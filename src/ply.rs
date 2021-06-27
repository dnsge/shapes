use ply_rs;
use ply_rs::ply::Property;

use std::convert::{TryFrom};
use std::{ops, fmt};
use std::ops::Index;

use crate::geo::Point3;

pub struct Face {
    vertices: Vec<Point3>,
}

impl Face {
    fn new(vertices: Vec<Point3>) -> Face {
        Face {
            vertices
        }
    }

    pub fn vertices(&self) -> &Vec<Point3> {
        &self.vertices
    }
}

impl ops::Index<usize> for Face {
    type Output = Point3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl ops::IndexMut<usize> for Face {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vertices[index]
    }
}

pub struct Object {
    center: Point3,
    bounds: (f32, f32, f32),

    vertices: Vec<Point3>,
    faces: Vec<Face>,
    face_indexes: Vec<Vec<usize>>,
}

// todo: consider returning references throughout program
impl Object {
    fn get_vertex_safe(&self, index: usize) -> Point3 {
        if index >= self.vertices.len() {
            Point3::default()
        } else {
            self.vertices[index]
        }
    }

    pub fn vertices(&self) -> &Vec<Point3> {
        &self.vertices
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    pub fn scale(&mut self, by: f32) {
        let center = self.center;
        self.vertices.iter_mut().for_each(|v| {
            let mut from_center: Point3 = v.sub_point(center);
            *v = from_center.scale(by).add_point(center);
        });
        self.bounds = compute_bounds(&self.vertices);
        self.faces = map_faces(&self.face_indexes, &self.vertices);
    }

    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "center: {} bounds: {} x {} x {}", self.center, self.bounds.0, self.bounds.1, self.bounds.2)
    }
}

pub fn load(path: &str) -> Object {
    let mut f = std::fs::File::open(path).unwrap();
    let p = ply_rs::parser::Parser::<ply_rs::ply::DefaultElement>::new();
    let ply = p.read_ply(&mut f);

    assert!(ply.is_ok());
    let mut ply = ply.unwrap();
    println!("Loaded object | Header: {:#?}", ply.header);

    let mut vertices = Vec::<Point3>::new();
    let vertex_count = ply.header.elements["vertex"].count;
    vertices.reserve(vertex_count);

    for p in &ply.payload["vertex"] {
        if let Some(x) = scalar_to_float(&p["x"]) {
            if let Some(y) = scalar_to_float(&p["y"]) {
                if let Some(z) = scalar_to_float(&p["z"]) {
                    vertices.push(Point3::new([x, y, z]));
                }
            }
        }
    }

    let bounds = compute_bounds(&vertices);
    let center = Point3::new([bounds.0 / 2.0, bounds.1 / 2.0, bounds.2 / 2.0]);

    vertices.iter_mut().for_each(|p| {
        *p = p.add_point(center)
    });

    let vertex_index_name = ply.header.elements["face"].properties.iter().next().unwrap().0;

    let mut face_indexes: Vec<Vec<usize>> = Vec::new();
    let face_count = ply.header.elements["face"].count;
    face_indexes.reserve(face_count);

    for mut f in ply.payload.remove("face").unwrap() {
        let vi = f.remove(vertex_index_name);
        if let Some(t) = vi {
            let face_vec: Vec<usize> = match t {
                Property::ListChar(l) => conv_vec_to_usize(l),
                Property::ListUChar(l) => conv_vec_to_usize(l),
                Property::ListShort(l) => conv_vec_to_usize(l),
                Property::ListUShort(l) => conv_vec_to_usize(l),
                Property::ListInt(l) => conv_vec_to_usize(l),
                Property::ListUInt(l) => conv_vec_to_usize(l),
                v => panic!("Unexpected property value {:#?}", v),
            };

            // make sure nothing is out of bounds
            for (n, &vertex_index) in face_vec.iter().enumerate() {
                if vertex_index >= vertex_count {
                    panic!("out of bounds vertex index on face {}: {}", n, vertex_index)
                }
            }

            if face_vec.len() < 3 { // invalid face
                panic!("invalid face with {} vertices", face_vec.len())
            }

            face_indexes.push(face_vec);
        }
    }

    let faces = map_faces(&face_indexes, &vertices);

    Object {
        center,
        bounds,
        vertices,
        faces,
        face_indexes,
    }
}

fn compute_bounds(vertices: &Vec<Point3>) -> (f32, f32, f32) {
    let mut min_x: f32 = 0.0;
    let mut max_x: f32 = 0.0;
    let mut min_y: f32 = 0.0;
    let mut max_y: f32 = 0.0;
    let mut min_z: f32 = 0.0;
    let mut max_z: f32 = 0.0;

    for v in &vertices {
        min_x = f32::min(min_x, v[0]);
        max_x = f32::max(max_x, v[0]);
        min_y = f32::min(min_y, v[1]);
        max_y = f32::max(max_y, v[1]);
        min_z = f32::min(min_z, v[2]);
        max_z = f32::max(max_z, v[2]);
    }

    (max_x - min_x, max_y - min_y, max_z - min_z)
}

fn map_faces(face_indexes: &Vec<Vec<usize>>, vertices: &Vec<Point3>) -> Vec<Face> {
    face_indexes.iter().map(|si| {
        Face::new(si.iter().map(|&n| vertices[n]).collect())
    }).collect()
}

fn conv_vec_to_usize<T>(v: Vec<T>) -> Vec<usize> where usize: TryFrom<T> {
    v.into_iter().map(|i| {
        usize::try_from(i)
            .unwrap_or_else(|a| panic!("Failed to cast to usize"))
    }).collect()
}

fn scalar_to_float(prop: &Property) -> Option<f32> {
    match *prop {
        Property::Float(n) => Some(n),
        Property::Double(n) => Some(n as f32),
        Property::Char(n) => Some(n as f32),
        Property::Short(n) => Some(n as f32),
        Property::UChar(n) => Some(n as f32),
        Property::UShort(n) => Some(n as f32),
        Property::Int(n) => Some(n as f32),
        Property::UInt(n) => Some(n as f32),
        _ => None
    }
}

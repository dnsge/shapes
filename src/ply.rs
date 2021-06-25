use ply_rs;
use crate::geo;
use crate::geo::Point3;
use ply_rs::ply::Property;
use std::convert::{TryFrom, TryInto};
use std::ops::Index;

pub struct Object<const N: usize> {
    pub offset: Point3,
    vertices: Vec<Point3>,
    faces: Vec<[usize; N]>,
}

impl<const N: usize> Object<N> {
    fn get_vertex_safe(&self, index: usize) -> Point3 {
        if index >= self.vertices.len() {
            Point3::default()
        } else {
            self.vertices[index]
        }
    }

    fn get_vertex_safe_with_offset(&self, index: usize) -> Point3 {
        if index >= self.vertices.len() {
            Point3::default()
        } else {
            self.vertices[index].add_point(self.offset)
        }
    }

    pub fn vertices(&self) -> Vec<Point3> {
        self.vertices.iter().map(|v| v.add_point(self.offset)).collect()
    }
}

impl Object<3> {
    pub fn surfaces(&self) -> Vec<[Point3; 3]> {
        self.faces.iter().map(|arr| {
            [
                self.get_vertex_safe_with_offset(arr[0]),
                self.get_vertex_safe_with_offset(arr[1]),
                self.get_vertex_safe_with_offset(arr[2]),
            ]
        }).collect()
    }
}

impl Object<4> {
    pub fn surfaces(&self) -> Vec<[Point3; 3]> {
        self.faces.iter().flat_map(|arr| {
            let mut sp_v: Vec<Point3> = vec![
                self.get_vertex_safe(arr[0]),
                self.get_vertex_safe(arr[1]),
                self.get_vertex_safe(arr[2]),
                self.get_vertex_safe(arr[3]),
            ];

            sp_v.sort_by(|p1, p2| {
                if let Some(res) = p1[0].partial_cmp(&p2[0]) {
                    res
                } else {
                    std::cmp::Ordering::Less
                }
            });

            let sp: [Point3; 4] = sp_v.try_into().unwrap_or_else(|e| panic!("failed to unwrap sp_v"));

            [[sp[0], sp[1], sp[2]], [sp[1], sp[2], sp[3]]]
        }).map(|p| {
            [
                p[0].add_point(self.offset),
                p[1].add_point(self.offset),
                p[2].add_point(self.offset)
            ]
        }).collect()
    }
}

pub fn load<const N: usize>(path: &str) -> Object<N> {
    let mut f = std::fs::File::open(path).unwrap();
    let p = ply_rs::parser::Parser::<ply_rs::ply::DefaultElement>::new();
    let ply = p.read_ply(&mut f);

    assert!(ply.is_ok());
    let mut ply = ply.unwrap();
    println!("Header: {:#?}", ply.header);

    let mut vertices = Vec::<Point3>::new();
    vertices.reserve(ply.header.elements["vertex"].count);
    for p in &ply.payload["vertex"] {
        if let Some(x) = scalar_to_float(&p["x"]) {
            if let Some(y) = scalar_to_float(&p["y"]) {
                if let Some(z) = scalar_to_float(&p["z"]) {
                    vertices.push(Point3::new([x, y, z]));
                }
            }
        }
    }

    let vertex_index_name = ply.header.elements["face"].properties.iter().next().unwrap().0;
    let mut faces = Vec::<[usize; N]>::new();
    faces.reserve(ply.header.elements["face"].count);
    for mut f in ply.payload.remove("face").unwrap() {
        let vi = f.remove(vertex_index_name);
        if let Some(t) = vi {
            let surface_vec: Vec<usize> = match t {
                Property::ListChar(l) => conv_vec_to_usize(l),
                Property::ListUChar(l) => conv_vec_to_usize(l),
                Property::ListShort(l) => conv_vec_to_usize(l),
                Property::ListUShort(l) => conv_vec_to_usize(l),
                Property::ListInt(l) => conv_vec_to_usize(l),
                Property::ListUInt(l) => conv_vec_to_usize(l),
                v => panic!("Unexpected property value {:#?}", v),
            };

            let surface_arr: [usize; N] = surface_vec.try_into()
                .unwrap_or_else(|v: Vec<usize>| panic!("Expected a Vec of length {} but it was {}", N, v.len()));

            faces.push(surface_arr);
        }
    }

    Object {
        offset: Point3::default(),
        vertices,
        faces,
    }
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

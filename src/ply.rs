use std::convert::TryFrom;
use std::fs;
use std::io::Error;

use ply_rs::parser::Parser;
use ply_rs::ply::{DefaultElement, Property};

use crate::world::three_dim::{compute_center, Object};
use crate::world::Point3;

pub fn load(path: &str) -> Result<Object, Error> {
    let mut f = fs::File::open(path).unwrap();
    let p = Parser::<DefaultElement>::new();
    let ply = p.read_ply(&mut f);

    if let Err(e) = ply {
        return Err(e);
    }

    let mut ply = ply.unwrap();
    println!("Loaded object | {:#?}", ply.header);

    let vertex_count = ply.header.elements["vertex"].count;
    let mut vertices = Vec::<Point3>::with_capacity(vertex_count);

    for p in &ply.payload["vertex"] {
        if let Some(x) = scalar_to_float(&p["x"]) {
            if let Some(y) = scalar_to_float(&p["y"]) {
                if let Some(z) = scalar_to_float(&p["z"]) {
                    vertices.push(Point3::new([x, y, z]));
                }
            }
        }
    }

    let center = compute_center(&vertices);

    // Move object center to (0, 0, 0)
    vertices.iter_mut().for_each(|p| *p = *p - center);

    let vertex_index_name = ply.header.elements["face"]
        .properties
        .iter()
        .next()
        .unwrap()
        .0;

    let face_count = ply.header.elements["face"].count;
    let mut face_indexes: Vec<Vec<usize>> = Vec::with_capacity(face_count);

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

            if face_vec.len() < 3 {
                // invalid face
                panic!("invalid face with {} vertices", face_vec.len())
            }

            face_indexes.push(face_vec);
        }
    }

    Ok(Object::new(vertices, face_indexes))
}

fn conv_vec_to_usize<T>(v: Vec<T>) -> Vec<usize>
where
    usize: TryFrom<T>,
{
    v.into_iter()
        .map(|i| usize::try_from(i).unwrap_or_else(|_| panic!("Failed to cast to usize")))
        .collect()
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
        _ => None,
    }
}

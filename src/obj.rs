use crate::world::three_dim::compute_center;
use crate::world::{Object, Point3};

use obj::raw::object::Polygon;
use obj::raw::{parse_obj, RawObj};
use obj::{LoadError, LoadErrorKind, Obj, ObjError, ObjResult, Vertex};

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufReader;

pub fn load(path: &str) -> Result<Object, ObjError> {
    let reader = BufReader::new(File::open(path).unwrap());
    let raw_object = match parse_obj(reader) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let object = match custom_process(raw_object) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let mut vertices: Vec<Point3> = Vec::with_capacity(object.vertices.len());
    for vert in object.vertices {
        vertices.push(Point3::new(vert.position));
    }

    let center = compute_center(&vertices);
    vertices.iter_mut().for_each(|p| *p = *p - center);

    assert_eq!(object.indices.len() % 3, 0);
    let mut face_indexes: Vec<Vec<usize>> = Vec::with_capacity(object.indices.len() / 3);
    for chunk in object.indices.chunks_exact(3) {
        let face = chunk.iter().map(|v| *v as usize).collect();
        face_indexes.push(face);
    }

    Ok(Object::new(vertices, face_indexes))
}

// adapted from obj-rs::Vertex::process
fn custom_process(raw_object: RawObj) -> ObjResult<Obj> {
    let positions = raw_object.positions;
    let normals = raw_object.normals;
    let polygons = raw_object.polygons;

    let mut vb: Vec<Vertex> = Vec::with_capacity(polygons.len() * 3);
    let mut ib: Vec<u16> = Vec::with_capacity(polygons.len() * 3);
    {
        let mut cache = HashMap::new();
        let mut map = |pi: usize, ni: usize, has_normals: bool| {
            // Look up cache
            let index = match cache.entry((pi, ni, has_normals)) {
                // Cache miss -> make new, store it on cache
                Entry::Vacant(entry) => {
                    let p = positions[pi];
                    let vertex = if has_normals {
                        let n = normals[ni];
                        Vertex {
                            position: [p.0, p.1, p.2],
                            normal: [n.0, n.1, n.2],
                        }
                    } else {
                        Vertex {
                            position: [p.0, p.1, p.2],
                            normal: [0.0, 0.0, 0.0],
                        }
                    };
                    let index: u16 =
                        u16::try_from(vb.len()).expect("Unable to convert the index from usize");
                    vb.push(vertex);
                    entry.insert(index);
                    index
                }
                // Cache hit -> use it
                Entry::Occupied(entry) => *entry.get(),
            };
            ib.push(index)
        };

        for polygon in polygons {
            match polygon {
                Polygon::P(ref vec) if vec.len() == 3 => {
                    for &pi in vec {
                        map(pi, 0, false)
                    }
                }
                Polygon::PT(ref vec) if vec.len() == 3 => {
                    for &(pi, _) in vec {
                        map(pi, 0, false)
                    }
                }
                Polygon::PN(ref vec) if vec.len() == 3 => {
                    for &(pi, ni) in vec {
                        map(pi, ni, true)
                    }
                }
                Polygon::PTN(ref vec) if vec.len() == 3 => {
                    for &(pi, _, ni) in vec {
                        map(pi, ni, true)
                    }
                }
                _ => {
                    return Err(std::convert::From::from(LoadError::new(
                        LoadErrorKind::UntriangulatedModel,
                        "Model should be triangulated first to be loaded properly",
                    )))
                }
            }
        }
    }

    Ok(Obj {
        name: raw_object.name,
        vertices: vb,
        indices: ib,
    })
}

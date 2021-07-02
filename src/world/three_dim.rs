use std::{fmt, ops};

use crate::matrix::Matrix;
use crate::world::{Point2, Point3};

pub struct Face {
    vertices: Vec<Point3>,
}

impl Face {
    fn new(vertices: Vec<Point3>) -> Face {
        Face { vertices }
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
    size: (f32, f32, f32),

    vertices: Vec<Point3>,
    faces: Vec<Face>,
    face_indexes: Vec<Vec<usize>>,
}

// todo: consider returning references throughout program
impl Object {
    pub fn new(vertices: Vec<Point3>, face_indexes: Vec<Vec<usize>>) -> Object {
        let size = compute_size(&vertices);
        let faces = map_faces(&face_indexes, &vertices);

        Object {
            size,
            vertices,
            faces,
            face_indexes,
        }
    }

    pub fn vertices(&self) -> &Vec<Point3> {
        &self.vertices
    }

    pub fn normalize_size(&mut self, largest_dimension_target: f32) {
        let largest_dimension = f32::max(self.size.0, f32::max(self.size.1, self.size.2));
        self.scale(largest_dimension_target / largest_dimension);
    }

    pub fn scale(&mut self, by: f32) {
        if by == 1.0 {
            return;
        }

        self.vertices.iter_mut().for_each(|v| {
            *v = *v * by;
        });

        self.size = compute_size(&self.vertices);
        self.faces = map_faces(&self.face_indexes, &self.vertices);
    }

    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Object(size: {} x {} x {})",
            self.size.0, self.size.1, self.size.2
        )
    }
}

pub fn compute_extremes(vertices: &Vec<Point3>) -> (Point3, Point3) {
    let mut min_x: f32 = 0.0;
    let mut max_x: f32 = 0.0;
    let mut min_y: f32 = 0.0;
    let mut max_y: f32 = 0.0;
    let mut min_z: f32 = 0.0;
    let mut max_z: f32 = 0.0;

    for v in vertices {
        min_x = f32::min(min_x, v[0]);
        max_x = f32::max(max_x, v[0]);
        min_y = f32::min(min_y, v[1]);
        max_y = f32::max(max_y, v[1]);
        min_z = f32::min(min_z, v[2]);
        max_z = f32::max(max_z, v[2]);
    }

    (
        Point3::new([min_x, min_y, min_z]),
        Point3::new([max_x, max_y, max_z]),
    )
}

pub fn compute_size(vertices: &Vec<Point3>) -> (f32, f32, f32) {
    let extremes = compute_extremes(vertices);
    (extremes.1 - extremes.0).into()
}

pub fn compute_center(vertices: &Vec<Point3>) -> Point3 {
    let extremes = compute_extremes(vertices);
    extremes.0.midpoint(extremes.1)
}

pub fn map_faces(face_indexes: &Vec<Vec<usize>>, vertices: &Vec<Point3>) -> Vec<Face> {
    face_indexes
        .iter()
        .map(|si| Face::new(si.iter().map(|&n| vertices[n]).collect()))
        .collect()
}

pub fn make_focal_matrix(cam_x: f32, cam_y: f32) -> Matrix<3, 4> {
    Matrix::new([
        [1.0, 0.0, 0.0, -cam_x],
        [0.0, 1.0, 0.0, -cam_y],
        [0.0, 0.0, 1.0, 0.0],
    ])
}

pub fn make_scaling_matrix(
    pixel_size: f32,
    viewport_width: usize,
    viewport_height: usize,
) -> Matrix<3, 3> {
    Matrix::new([
        [1.0 / pixel_size, 0.0, (viewport_width as f32) / 2.0],
        [0.0, 1.0 / pixel_size, (viewport_height as f32) / 2.0],
        [0.0, 0.0, 1.0],
    ])
}

pub fn make_rotation_matrix(rx: f32, ry: f32, rz: f32) -> Matrix<3, 3> {
    // aliases
    let sin = f32::sin;
    let cos = f32::cos;

    // see https://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
    Matrix::new([
        [
            cos(rx) * cos(ry),
            cos(rx) * sin(ry) * sin(rz) - sin(rx) * cos(rz),
            cos(rx) * sin(ry) * cos(rz) + sin(rx) * sin(rz),
        ],
        [
            sin(rx) * cos(ry),
            sin(rx) * sin(ry) * sin(rz) + cos(rx) * cos(rz),
            sin(rx) * sin(ry) * cos(rz) - cos(rx) * sin(rz),
        ],
        [-sin(ry), cos(ry) * sin(rz), cos(ry) * cos(rz)],
    ])
}

pub fn rotate_point_with_matrix(p: Point3, center: Point3, rot_matrix: &Matrix<3, 3>) -> Point3 {
    // 1. Translate p so that center is now at origin
    let mut n = p - center;
    // 2. Rotate n about origin by rot
    n = *rot_matrix * n;
    // 3. Translate p back towards center
    n + center
}

pub fn rotate_point(p: Point3, center: Point3, rot: (f32, f32, f32)) -> Point3 {
    rotate_point_with_matrix(p, center, &make_rotation_matrix(rot.0, rot.1, rot.2))
}

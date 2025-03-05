use crate::matrix::Matrix;
use crate::world::three_dim::make_rotation_matrix;
use crate::world::{Point2, Point3};

use super::projection::ProjectedPoint;

pub struct Camera {
    position: Point3,
    rotation: (f32, f32, f32),
    view_matrix: Matrix<4, 4>,
    focal_matrix: Matrix<3, 4>,
    combined_matrix: Matrix<3, 4>,
    modified: bool,
}

impl Camera {
    pub fn new(position: Point3, aspect_ratio: f32) -> Camera {
        let rotation = (0.0, 0.0, 0.0);
        let view_matrix = rotation_view_matrix(position, rotation);
        let focal_matrix = make_focal_matrix(0.0, 0.0, aspect_ratio);

        Camera {
            position,
            rotation,
            view_matrix,
            focal_matrix,
            combined_matrix: focal_matrix * view_matrix,
            modified: false,
        }
    }

    pub fn position(&self) -> Point3 {
        self.position
    }

    pub fn rotation(&self) -> (f32, f32, f32) {
        self.rotation
    }

    pub fn move_to(&mut self, point: Point3) {
        self.position = point;
    }

    pub fn set_rotation(&mut self, rotation: (f32, f32, f32)) {
        self.rotation = rotation;
    }

    pub fn point_to(&mut self, point: Point3) {
        self.view_matrix = point_to_view_matrix(self.position, point, Y_AXIS);
    }

    pub fn update(&mut self) {
        self.view_matrix = rotation_view_matrix(self.position, self.rotation);
        self.combined_matrix = self.focal_matrix * self.view_matrix;
        self.modified = true;
    }

    pub fn project_point(&self, p: Point3) -> Point2 {
        (self.combined_matrix * p.euc_to_hom()).hom_to_euc()
    }

    pub fn project_point_with_depth(&self, p: Point3) -> ProjectedPoint {
        let proj = self.project_point(p);
        let dist_squared = (p - self.position).magnitude_2();
        ProjectedPoint {
            x: proj[0],
            y: proj[1],
            z: dist_squared,
        }
    }

    pub fn get_and_clear_modified(&mut self) -> bool {
        if self.modified {
            self.modified = false;
            true
        } else {
            false
        }
    }
}

const X_AXIS: Point3 = Point3::new([1.0, 0.0, 0.0]);
const Y_AXIS: Point3 = Point3::new([0.0, 1.0, 0.0]);
const Z_AXIS: Point3 = Point3::new([0.0, 0.0, 1.0]);

fn rotation_view_matrix(origin: Point3, rotation: (f32, f32, f32)) -> Matrix<4, 4> {
    let rot_matrix = make_rotation_matrix(rotation.0, rotation.1, rotation.2);
    let new_x = rot_matrix * X_AXIS;
    let new_y = rot_matrix * Y_AXIS;
    let new_z = rot_matrix * Z_AXIS;

    axes_transformation_matrix(new_x, new_y, new_z, origin).inverse()
}

fn point_to_view_matrix(origin: Point3, target: Point3, up: Point3) -> Matrix<4, 4> {
    let z_axis: Point3 = (target - origin).normalize(); // in direction from camera to target
    let x_axis: Point3 = up.cross(z_axis).normalize(); // right from z axis
    let y_axis: Point3 = z_axis.cross(x_axis).normalize();

    axes_transformation_matrix(x_axis, y_axis, z_axis, origin).inverse()
}

fn axes_transformation_matrix(
    new_x: Point3,
    new_y: Point3,
    new_z: Point3,
    origin: Point3,
) -> Matrix<4, 4> {
    Matrix::new([
        [new_x[0], new_y[0], new_z[0], origin[0]],
        [new_x[1], new_y[1], new_z[1], origin[1]],
        [new_x[2], new_y[2], new_z[2], origin[2]],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

fn make_focal_matrix(cam_x: f32, cam_y: f32, aspect_ratio: f32) -> Matrix<3, 4> {
    Matrix::new([
        [aspect_ratio.recip(), 0.0, 0.0, -cam_x],
        [0.0, 1.0, 0.0, -cam_y],
        [0.0, 0.0, 1.0, 0.0],
    ])
}

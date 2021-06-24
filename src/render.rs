use crate::geo::{Point2};
use crate::matrix::{Matrix};

pub struct Screen {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        Screen {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn get_coords(&mut self, x: usize, y: usize) -> Option<&mut u32> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.buffer.get_mut(y * self.width + x)
        }
    }

    pub fn get_pixel(&mut self, pixel: (usize, usize)) -> Option<&mut u32> {
        self.get_coords(pixel.0, pixel.1)
    }

    pub fn get_pixel_i(&mut self, pixel: (isize, isize)) -> Option<&mut u32> {
        if pixel.0 < 0 || pixel.1 < 0 {
            None
        } else {
            self.get_coords(pixel.0 as usize, pixel.1 as usize)
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }
}

pub fn make_focal_matrix(cam_x: f32, cam_y: f32) -> Matrix<3, 4> {
    Matrix::new([
        [1.0, 0.0, 0.0, -cam_x],
        [0.0, 1.0, 0.0, -cam_y],
        [0.0, 0.0, 1.0, 0.0],
    ])
}

pub fn make_scaling_matrix(pixel_size: f32, viewport_width: usize, viewport_height: usize) -> Matrix<3, 3> {
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
        [cos(rx) * cos(ry), cos(rx) * sin(ry) * sin(rz) - sin(rx) * cos(rz), cos(rx) * sin(ry) * cos(rz) + sin(rx) * sin(rz)],
        [sin(rx) * cos(ry), sin(rx) * sin(ry) * sin(rz) + cos(rx) * cos(rz), sin(rx) * sin(ry) * cos(rz) - cos(rx) * sin(rz)],
        [-sin(ry), cos(ry) * sin(rz), cos(ry) * cos(rz)]
    ])
}

fn projection_to_ndc(p: Point2, width: usize, height: usize) -> Point2 {
    Point2::new([
        (p[0] + (width as f32 / 2.0)) / (width as f32),
        (p[1] + (height as f32 / 2.0)) / (height as f32),
    ])
}

fn ndc_to_screen(ndc: Point2, screen_size: (usize, usize)) -> (isize, isize) {
    (
        (ndc[0] * screen_size.0 as f32).floor() as isize,
        ((1.0 - ndc[1]) * screen_size.1 as f32).floor() as isize,
    )
}

pub fn projection_to_screen(p: Point2, proj_size: (usize, usize), screen_size: (usize, usize)) -> (isize, isize) {
    let ndc = projection_to_ndc(p, proj_size.0, proj_size.1); // move to normalized device coordinates
    ndc_to_screen(ndc, screen_size) // move to screen coordinates
}

use crate::{matrix, geo};

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

pub fn make_focal_matrix(cam_x: f32, cam_y: f32) -> matrix::Matrix34 {
    matrix::Matrix34([
        [1.0, 0.0, 0.0, -cam_x],
        [0.0, 1.0, 0.0, -cam_y],
        [0.0, 0.0, 1.0, 0.0],
    ])
}

pub fn make_scaling_matrix(pixel_size: f32, viewport_width: usize, viewport_height: usize) -> matrix::Matrix33 {
    matrix::Matrix33([
        [1.0 / pixel_size, 0.0, (viewport_width as f32) / 2.0],
        [0.0, 1.0 / pixel_size, (viewport_height as f32) / 2.0],
        [0.0, 0.0, 1.0],
    ])
}

fn projection_to_ndc(p: geo::Point2, width: usize, height: usize) -> geo::Point2 {
    geo::Point2(
        (p.0 + (width as f32 / 2.0)) / (width as f32),
        (p.1 + (height as f32 / 2.0)) / (height as f32),
    )
}

fn ndc_to_raster(ndc: geo::Point2, screen_size: (usize, usize)) -> (usize, usize) {
    (
        (ndc.0 * screen_size.0 as f32).floor() as usize,
        ((1.0 - ndc.1) * screen_size.1 as f32).floor() as usize,
    )
}

pub fn projection_to_raster(p: geo::Point2, proj_size: (usize, usize), screen_size: (usize, usize)) -> (usize, usize) {
    let ndc = projection_to_ndc(p, proj_size.0, proj_size.1);
    ndc_to_raster(ndc, screen_size)
}

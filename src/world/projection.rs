use crate::world::Point2;

#[derive(Clone, Debug)]
pub struct ProjectedPoint {
    pub x: f32, // screen x
    pub y: f32, // screen y
    pub z: f32, // depth
}

#[derive(Clone, Debug)]
pub struct ProjectedTriangle {
    pub v0: ProjectedPoint,
    pub v1: ProjectedPoint,
    pub v2: ProjectedPoint,
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

pub fn projection_to_screen(
    p: Point2,
    proj_size: (usize, usize),
    screen_size: (usize, usize),
) -> (isize, isize) {
    let ndc = projection_to_ndc(p, proj_size.0, proj_size.1); // move to normalized device coordinates
    ndc_to_screen(ndc, screen_size) // move to screen coordinates
}

fn projected_point_to_ndc(p: ProjectedPoint, width: usize, height: usize) -> ProjectedPoint {
    ProjectedPoint {
        x: (p.x + (width as f32 / 2.0)) / (width as f32),
        y: (p.y + (height as f32 / 2.0)) / (height as f32),
        z: p.z,
    }
}

fn ndc_point_to_screen(ndc: ProjectedPoint, screen_size: (usize, usize)) -> ProjectedPoint {
    ProjectedPoint {
        x: (ndc.x * screen_size.0 as f32),
        y: ((1.0 - ndc.y) * screen_size.1 as f32),
        z: ndc.z,
    }
}

pub fn projected_point_to_screen(
    p: ProjectedPoint,
    proj_size: (usize, usize),
    screen_size: (usize, usize),
) -> ProjectedPoint {
    let ndc = projected_point_to_ndc(p, proj_size.0, proj_size.1); // move to normalized device coordinates
    ndc_point_to_screen(ndc, screen_size) // move to screen coordinates
}

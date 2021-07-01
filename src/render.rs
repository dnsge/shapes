use crate::matrix::Matrix;
use crate::scene::Renderer;
use crate::screen_buffer::ScreenBuffer;
use crate::world::three_dim::{
    make_rotation_matrix, projection_to_screen, rotate_point_with_matrix, Object,
};
use crate::world::{Point2, Point3};

const RENDER_DEBUG: bool = false;

impl ScreenBuffer {
    // Attempts to bring a point inside the screen along a line
    //
    // For example:
    //                    +
    //                   /
    //                  /
    //       +---------+----------------+
    //       |        /                 |
    //       |       /                  |
    //       |      /                   |
    //
    // A line is projected through the original point to the intersection with the window.
    fn bring_inside(&self, p: (isize, isize), slope: f32) -> (isize, isize) {
        // precondition: point is outside of screen

        if slope.is_nan() {
            // dy = dx = 0 leads to indeterminate form 0.0/0.0 = NaN
            return p;
        }

        let is_vertical: bool = slope.is_infinite();
        let below = p.1 < 0;
        let above = p.1 >= self.height() as isize;
        let left = p.0 < 0;
        let right = p.0 >= self.width() as isize;

        if (above || below) && slope == 0.0 {
            return p;
        }

        if (left || right) && is_vertical {
            return p;
        }

        let old_x = p.0 as f32;
        let old_y = p.1 as f32;
        let mut new_x = old_x;
        let mut new_y = old_y;

        if is_vertical {
            if below {
                new_y = 0.0;
            } else if above {
                new_y = (self.height() - 1) as f32;
            }
        } else {
            if left || right {
                if left {
                    new_x = 0.0;
                } else {
                    new_x = (self.width() as isize - 1) as f32;
                }
                let dx = new_x - old_x;
                new_y = dx * slope + old_y;
            } else if above || below {
                // at this point above || below should always be true, but whatever
                if below {
                    new_y = 0.0
                } else {
                    new_y = (self.height() as isize - 1) as f32;
                }
                let dy = new_y - old_y;
                new_x = dy / slope + old_x;
            }
        }

        (new_x as isize, new_y as isize)
    }

    // adapted from http://www.sunshine2k.de/java.html#bresenham
    pub fn draw_line(&mut self, mut p1: (isize, isize), mut p2: (isize, isize), color: u32) {
        let slope: f32 = (p2.1 as f32 - p1.1 as f32) / (p2.0 as f32 - p1.0 as f32);

        if self.outside_screen(p1) {
            p1 = self.bring_inside(p1, slope);
        }

        if self.outside_screen(p2) {
            p2 = self.bring_inside(p2, slope);
        }

        if self.outside_screen(p1) || self.outside_screen(p2) {
            return;
        }

        let mut x = p1.0;
        let mut y = p1.1;

        let mut dx = isize::abs(p2.0 - p1.0);
        let mut dy = isize::abs(p2.1 - p1.1);
        let sign_x = isize::signum(p2.0 - p1.0);
        let sign_y = isize::signum(p2.1 - p1.1);

        let mut swapped = false;
        if dy > dx {
            // swap
            swapped = true;
            let tmp = dy;
            dy = dx;
            dx = tmp;
        }

        let mut err = (dy as f32) / (dx as f32) - 0.5;

        for _ in 1..=dx {
            self.set_pixel_i((x, y), color);

            while err >= 0.0 {
                if swapped {
                    x += sign_x;
                } else {
                    y += sign_y;
                }
                err -= 1.0;
            }

            if swapped {
                y += sign_y;
            } else {
                x += sign_x;
            }

            err += (dy as f32) / (dx as f32);
        }

        self.set_pixel_i(p2, color);
    }

    pub fn fill_triangle(
        &mut self,
        p1: (isize, isize),
        p2: (isize, isize),
        p3: (isize, isize),
        color: u32,
    ) {
        if !self.triangle_inside_screen(p1, p2, p3) {
            return;
        }

        let mut points = vec![p1, p2, p3];
        points.sort_by_key(|p| {
            // sort by point y values
            p.1
        });

        if points[1].1 == points[2].1 {
            self.fill_bottom_triangle(points[0], points[1], points[2], color);
        } else if points[0].1 == points[1].1 {
            self.fill_top_triangle(points[0], points[1], points[2], color);
        } else {
            let p4 = (
                (points[0].0 as f32
                    + (((points[1].1 - points[0].1) as f32) / ((points[2].1 - points[0].1) as f32))
                        * (points[2].0 - points[0].0) as f32) as isize,
                points[1].1,
            );
            self.fill_bottom_triangle(points[0], points[1], p4, color);
            self.fill_top_triangle(points[1], p4, points[2], color);
        }
    }

    pub fn fill_triangle_edge(
        &mut self,
        p1: (isize, isize),
        p2: (isize, isize),
        p3: (isize, isize),
        color: u32,
        edge_color: u32,
    ) {
        if !self.triangle_inside_screen(p1, p2, p3) {
            return;
        }

        self.fill_triangle(p1, p2, p3, color);
        self.draw_line(p1, p2, edge_color);
        self.draw_line(p2, p3, edge_color);
        self.draw_line(p1, p3, edge_color);
    }

    fn fill_bottom_triangle(
        &mut self,
        p1: (isize, isize),
        p2: (isize, isize),
        p3: (isize, isize),
        color: u32,
    ) {
        let slope1 = (p2.0 - p1.0) as f32 / (p2.1 - p1.1) as f32;
        let slope2 = (p3.0 - p1.0) as f32 / (p3.1 - p1.1) as f32;

        let mut cur_x_1 = p1.0 as f32;
        let mut cur_x_2 = p1.0 as f32;

        for y_val in p1.1..=p2.1 {
            self.draw_line((cur_x_1 as isize, y_val), (cur_x_2 as isize, y_val), color);
            cur_x_1 += slope1;
            cur_x_2 += slope2;
        }
    }

    fn fill_top_triangle(
        &mut self,
        p1: (isize, isize),
        p2: (isize, isize),
        p3: (isize, isize),
        color: u32,
    ) {
        let slope1 = (p3.0 - p1.0) as f32 / (p3.1 - p1.1) as f32;
        let slope2 = (p3.0 - p2.0) as f32 / (p3.1 - p2.1) as f32;

        let mut cur_x_1 = p3.0 as f32;
        let mut cur_x_2 = p3.0 as f32;

        for y_val in (p1.1..=p3.1).rev() {
            self.draw_line((cur_x_1 as isize, y_val), (cur_x_2 as isize, y_val), color);
            cur_x_1 -= slope1;
            cur_x_2 -= slope2;
        }
    }
}

struct Triangle {
    vertices: [(isize, isize); 3],
    color: u32,
}

#[derive(Eq, PartialEq)]
enum SurfaceOrientation {
    TowardsCamera,
    AwayFromCamera,
}

struct Surface {
    vertices: Vec<Point3>,
    surface_normal: Point3,
    camera_surface_dot: f32,
    orientation: SurfaceOrientation,
}

impl Surface {
    pub fn min_z(&self) -> f32 {
        let mut min = self.vertices[0][2];
        for i in 1..self.vertices.len() {
            min = f32::min(min, self.vertices[i][2]);
        }
        min
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct ObjectOrientation {
    pub position: Point3,
    pub rotation: (f32, f32, f32),
}

impl Renderer<ObjectOrientation> for Object {
    fn render(&self, screen: &mut ScreenBuffer, camera: &Matrix<3, 4>, state: ObjectOrientation) {
        // Rendering the object performs the following steps:
        // 1a. Rotate every surface around the object center
        // 1b. Transform object to position
        // 2. Project each surface to z=1 plane
        // 3. Convert to screen coordinates
        // 4. Break surfaces into triangles
        // 5. Raster triangles

        // Rotate surfaces and transform to position
        // todo: combine actions into single world matrix operation
        let transform_vector: Point3 = state.position;
        let rotation_matrix =
            make_rotation_matrix(state.rotation.0, state.rotation.1, state.rotation.2);
        let mut surfaces: Vec<Surface> = self
            .faces()
            .iter()
            .map(|f| {
                f.vertices()
                    .iter()
                    .map(|&p| {
                        let rotated =
                            rotate_point_with_matrix(p, Point3::default(), &rotation_matrix);
                        rotated + transform_vector
                    })
                    .collect()
            })
            .filter(|s: &Vec<Point3>| {
                // remove surfaces where z < 1
                let mut inside = false;
                for p in s {
                    if p[2] >= 1.0 {
                        inside = true;
                        break;
                    }
                }
                inside
            })
            .map(|s| {
                // Let triangle ABC be defined by the points s[0], s[1], and s[2]
                //
                // 1. ABC has a surface normal N defined by the cross product of two of its legs,
                //     N = AB X AC
                // 2. ABC has a vector from the camera to its first vertex,
                //     D = A - C
                //    where C is the camera position.
                //
                // When D·N >= 0, the triangle should not be rendered.
                //
                // ref: https://en.wikipedia.org/wiki/Back-face_culling

                let vec1 = s[1] - s[0]; // vector A-->B
                let vec2 = s[2] - s[0]; // vector A-->C
                let surface_normal = vec1.cross(vec2).normalize();
                let dot = s[0].sub([0.0, 0.0, 0.0]).normalize().dot(surface_normal);

                let orientation = if dot < 0.0 {
                    SurfaceOrientation::TowardsCamera
                } else {
                    SurfaceOrientation::AwayFromCamera
                };

                Surface {
                    vertices: s,
                    surface_normal,
                    camera_surface_dot: dot,
                    orientation,
                }
            })
            .collect();

        surfaces.sort_by(|s1, s2| s2.min_z().partial_cmp(&s1.min_z()).unwrap());

        // todo: handle z = 0, out of viewport, clipping z < 1, etc.
        let mut triangles: Vec<Triangle> = Vec::new();
        for s in &surfaces {
            if s.orientation == SurfaceOrientation::AwayFromCamera {
                continue;
            }

            let z_space_points: Vec<Point2> = s
                .vertices
                .iter()
                .map(|p| (*camera * p.euc_to_hom()).hom_to_euc())
                .collect();

            let mut any_inside = false;
            for p in &z_space_points {
                if p[0].abs() <= 1.0 || p[1].abs() <= 1.0 {
                    any_inside = true;
                    break;
                }
            }

            if !any_inside {
                continue;
            }

            let projected_points: Vec<(isize, isize)> = z_space_points
                .iter()
                .map(|&p| projection_to_screen(p, (2, 2), screen.size()))
                .collect();

            triangles.push(Triangle {
                vertices: [
                    projected_points[0],
                    projected_points[1],
                    projected_points[2],
                ],
                color: make_gray_color(-s.camera_surface_dot, 0.0, 1.0),
            });
        }

        for triangle in triangles {
            screen.fill_triangle(
                triangle.vertices[0],
                triangle.vertices[1],
                triangle.vertices[2],
                triangle.color,
            );
        }

        if RENDER_DEBUG {
            render_center_point(transform_vector, screen, camera);
        }
    }
}

fn render_center_point(position: Point3, screen: &mut ScreenBuffer, camera: &Matrix<3, 4>) {
    let z_space = (*camera * position.euc_to_hom()).hom_to_euc();
    let screen_space = projection_to_screen(z_space, (2, 2), screen.size());
    screen.set_pixel_i(screen_space, 0xff0000);
}

fn make_gray_color(intensity: f32, min: f32, max: f32) -> u32 {
    let scaled = intensity * (max - min) + min;
    let c = (scaled * 255.0) as u32;
    (c << 16) | (c << 8) | c
}

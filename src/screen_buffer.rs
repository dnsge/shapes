use core::f32;

use crate::world::projection::ProjectedTriangle;

pub struct ScreenBuffer {
    buffer: Vec<u32>,
    z_buffer: Vec<f32>,
    width: usize,
    height: usize,
}

impl ScreenBuffer {
    pub fn new(width: usize, height: usize) -> ScreenBuffer {
        ScreenBuffer {
            buffer: vec![0; width * height],
            z_buffer: vec![0.0; width * height],
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

    pub fn set_pixel(&mut self, pixel: (usize, usize), value: u32) -> bool {
        if let Some(p) = self.get_pixel(pixel) {
            *p = value;
            true
        } else {
            false
        }
    }

    pub fn set_pixel_i(&mut self, pixel: (isize, isize), value: u32) -> bool {
        if pixel.0 < 0 || pixel.1 < 0 {
            false
        } else {
            self.set_pixel((pixel.0 as usize, pixel.1 as usize), value)
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
        self.z_buffer.fill(f32::MAX);
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

    pub fn inside_screen(&self, p: (isize, isize)) -> bool {
        (0 < p.0 && p.0 < (self.width as isize)) // inside x
            && (0 < p.1 && p.1 < (self.height as isize)) // inside y
    }

    pub fn outside_screen(&self, p: (isize, isize)) -> bool {
        !self.inside_screen(p)
    }

    /// Fills a projected triangle onto the screen buffer. This method exists
    /// here to have optimized, unchecked access into the buffer and z buffer.
    pub fn fill_projected_triangle(&mut self, triangle: &ProjectedTriangle, color: u32) {
        // Extract vertices
        let v0 = &triangle.v0;
        let v1 = &triangle.v1;
        let v2 = &triangle.v2;

        // Find bounding box (clamped to screen boundaries)
        let min_x = v0.x.min(v1.x).min(v2.x).max(0.0) as i32;
        let min_y = v0.y.min(v1.y).min(v2.y).max(0.0) as i32;
        let max_x = v0.x.max(v1.x).max(v2.x).min(self.width as f32 - 1.0) as i32;
        let max_y = v0.y.max(v1.y).max(v2.y).min(self.height as f32 - 1.0) as i32;

        // Calculate triangle area (to determine winding order)
        let area = 0.5 * ((v1.x - v0.x) * (v2.y - v0.y) - (v2.x - v0.x) * (v1.y - v0.y));

        // Skip degenerate triangles
        if area.abs() < 1e-6 {
            return;
        }

        // We'll define edge functions in a consistent manner:
        // For each edge (v_i -> v_i+1), we want:
        // - Points to the LEFT of the edge to be positive
        // - Points to the RIGHT of the edge to be negative
        //
        // Edge function: E(x,y) = (x - x_i) * (y_i+1 - y_i) - (y - y_i) * (x_i+1 - x_i)
        //
        // For a counter-clockwise triangle, a point is INSIDE when all edge functions are positive

        // Edge functions: E(x, y) = (y_i - y_j) * x + (x_j - x_i) * y + (x_i * y_j - x_j * y_i)
        // Edges: v0->v1, v1->v2, v2->v0
        let edge0 = (
            v0.y - v1.y,               // A
            v1.x - v0.x,               // B
            v0.x * v1.y - v1.x * v0.y, // C
        );

        let edge1 = (v1.y - v2.y, v2.x - v1.x, v1.x * v2.y - v2.x * v1.y);

        let edge2 = (v2.y - v0.y, v0.x - v2.x, v2.x * v0.y - v0.x * v2.y);

        // Step values for edge functions when moving in x or y direction
        let step_x = [edge0.0, edge1.0, edge2.0];
        let step_y = [edge0.1, edge1.1, edge2.1];

        // Precompute starting position (top-left corner of bounding box)
        let start_x = min_x as f32 + 0.5; // Center of pixel
        let start_y = min_y as f32 + 0.5;

        // Initialize edge function values for the first row
        let mut row_edge_vals = [
            edge0.0 * start_x + edge0.1 * start_y + edge0.2,
            edge1.0 * start_x + edge1.1 * start_y + edge1.2,
            edge2.0 * start_x + edge2.1 * start_y + edge2.2,
        ];

        // Iterate over each pixel in the bounding box
        for y in min_y..=max_y {
            // Initialize edge values for this row
            let mut edge_vals = row_edge_vals.clone();

            for x in min_x..=max_x {
                // Check if pixel is inside triangle
                // A point is inside if it's on the left side of all edges
                // With our edge function definition, "left" means positive values
                let inside = edge_vals[0] >= 0.0 && edge_vals[1] >= 0.0 && edge_vals[2] >= 0.0;

                if inside {
                    // Calculate barycentric coordinates
                    // For correct interpolation, make sure they're normalized
                    let mut w = [0.0; 3];
                    w[0] = edge_vals[1].abs() / (2.0 * area.abs()); // alpha (weight for v0)
                    w[1] = edge_vals[2].abs() / (2.0 * area.abs()); // beta (weight for v1)
                    w[2] = edge_vals[0].abs() / (2.0 * area.abs()); // gamma (weight for v2)

                    // Normalize weights to ensure they sum to 1
                    let sum = w[0] + w[1] + w[2];
                    if sum > 1e-6 {
                        w[0] /= sum;
                        w[1] /= sum;
                        w[2] /= sum;
                    }

                    // Perspective-correct depth interpolation
                    // Interpolate 1/z instead of z directly
                    let one_over_z =
                        w[0] * (1.0 / v0.z) + w[1] * (1.0 / v1.z) + w[2] * (1.0 / v2.z);
                    let z_interpolated = 1.0 / one_over_z;

                    // Z-buffer test
                    let buffer_index = y as usize * self.width + x as usize;
                    if z_interpolated < self.z_buffer[buffer_index] {
                        // Update z-buffer
                        self.z_buffer[buffer_index] = z_interpolated;

                        // Update pixel buffer
                        self.buffer[buffer_index] = color;
                    }
                }

                // Move to the next pixel (x+1)
                edge_vals[0] += step_x[0];
                edge_vals[1] += step_x[1];
                edge_vals[2] += step_x[2];
            }

            // Move to the next row (y+1)
            row_edge_vals[0] += step_y[0];
            row_edge_vals[1] += step_y[1];
            row_edge_vals[2] += step_y[2];
        }
    }
}

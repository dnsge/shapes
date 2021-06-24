mod matrix;
mod geo;
mod render;

use minifb::{Key, Window, WindowOptions};
use geo::{Point2, Point3};
use matrix::{Matrix};

// in units
const PIXEL_SIZE: f32 = 1.0;

// in pixels
const WIDTH: usize = 512;
const HEIGHT: usize = 512;

// in frames
const FPS: u64 = 60;

struct Cube {
    pub origin: Point3,
    pub size: f32,
}

impl Cube {
    fn vertices(&self) -> [Point3; 8] {
        let half = self.size / 2.0;
        [
            self.origin.add([half, half, half]),
            self.origin.add([half, half, -half]),
            self.origin.add([half, -half, half]),
            self.origin.add([half, -half, -half]),
            self.origin.add([-half, half, half]),
            self.origin.add([-half, half, -half]),
            self.origin.add([-half, -half, half]),
            self.origin.add([-half, -half, -half]),
        ]
    }

    fn map_to_screen(&self, fm: Matrix<3, 4>, cm: Matrix<3, 3>) -> [Point2; 8] {
        let mut res: [Point2; 8] = [Point2::default(); 8];
        let vertices = self.vertices();

        for i in 0..7 {
            let projected: Option<Matrix<3, 1>> = fm * (vertices[i].euc_to_hom().to_tall_matrix());
            if let Some(proj_point) = projected {
                let converted: Option<Matrix<3, 1>> = cm * proj_point;
                if let Some(conv_point) = converted {
                    res[i] = conv_point.tall_to_point().hom_to_euc();
                }
            }

        }

        return res;
    }
}

fn main() {
    let focal_matrix = render::make_focal_matrix(0.0, 0.0);
    // let coordinate_mapping_matrix = render::make_scaling_matrix(PIXEL_SIZE, 4, 4);

    let mut my_cube = Cube {
        origin: Point3::new([-10.0, 0.0, 10.0]),
        size: 4.0,
    };

    let mut screen = render::Screen::new(WIDTH, HEIGHT);
    let mut window = Window::new(
        "Shapes",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(1000000 / FPS)));

    let now = std::time::SystemTime::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = now.elapsed().unwrap().as_secs_f32();

        my_cube.origin = Point3::new([
            f32::cos(elapsed) * 10.0,
            f32::sin(elapsed) * 5.0,
            10.0,
        ]);
        my_cube.size = f32::abs(f32::cos(elapsed) * 4.0);

        screen.clear(0x000000);
        for v in my_cube.vertices() {
            // project point onto z = 1
            let projected = (focal_matrix * (v.euc_to_hom().to_tall_matrix())).unwrap().tall_to_point().hom_to_euc();
            // convert point at (x, y, 1) to screen space
            let screen_point = render::projection_to_raster(projected, (4, 4), screen.size());

            if let Some(pixel) = screen.get_pixel(screen_point) {
                *pixel = 0xffffff;
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(screen.buffer(), screen.width(), screen.height())
            .unwrap();
    }
}

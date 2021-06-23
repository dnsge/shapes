mod matrix;
mod geo;
mod render;

use minifb::{Key, Window, WindowOptions};
use geo::{Point2, Point3};

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

    fn map_to_screen(&self, fm: matrix::Matrix34, cm: matrix::Matrix33) -> [Point2; 8] {
        let mut res: [Point2; 8] = [Point2::default(); 8];
        let vertices = self.vertices();

        for i in 0..7 {
            res[i] = cm.dot_point3(fm.dot_point4(vertices[i].euc_to_hom())).hom_to_euc()
        }

        return res;
    }
}

fn main() {
    let focal_matrix = render::make_focal_matrix(0.0, 0.0);
    let coordinate_mapping_matrix = render::make_scaling_matrix(PIXEL_SIZE, WIDTH * 2, HEIGHT);

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

    // ~30 fps
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
        for v in my_cube.vertices() { //my_cube.map_to_screen(focal_matrix, coordinate_mapping_matrix) {
            let projected = focal_matrix.dot_point4(v.euc_to_hom()).hom_to_euc();
            // let screen_point = coordinate_mapping_matrix.dot_point3(projected).hom_to_euc();
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

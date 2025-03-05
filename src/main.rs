use crate::world::camera::Camera;
use crate::world::{Object, Point3};
use core::f32;
use std::{env, path, process};

mod matrix;
mod obj;
mod ply;
mod render;
mod scene;
mod screen_buffer;
mod world;

// in pixels
const WIDTH: usize = 750;
const HEIGHT: usize = 750;
const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return Err("missing file argument".to_string());
    }

    let file_name = &args[1];
    if !path::Path::new(file_name).exists() {
        return Err(format!("file \"{}\" not found", file_name));
    }

    let mut fps: u64 = 30;
    if args.len() >= 3 {
        let fps_string = &args[2];
        match fps_string.parse::<u64>() {
            Ok(val) => fps = val,
            Err(_) => {
                return Err(format!("invalid fps: {}", fps_string));
            }
        };
    }

    let mut scale: f32 = 0.0;
    if args.len() >= 4 {
        let scale_string = &args[3];
        match scale_string.parse::<f32>() {
            Ok(val) => scale = val,
            Err(_) => {
                return Err(format!("invalid scale: {}", scale_string));
            }
        };
    }

    let mut object: Object = if file_name.ends_with(".ply") {
        match ply::load(file_name) {
            Ok(o) => o,
            Err(e) => return Err(format!("failed to load file: {}", e.to_string())),
        }
    } else {
        match obj::load(file_name) {
            Ok(o) => o,
            Err(e) => return Err(format!("failed to load file: {}", e.to_string())),
        }
    };

    if scale != 0.0 {
        object.scale(scale);
    } else {
        object.normalize_size(5.0);
    }

    println!("Object details: {}", object);

    let mut cam = Camera::new(Point3::new([3.0, 2.0, -2.0]), ASPECT_RATIO);
    cam.point_to(Point3::new([0.0, 0.0, 4.0]));
    cam.update();

    let now = std::time::SystemTime::now();
    let mut scene = scene::Scene::new(
        object,
        "Shapes - ESC to quit",
        (WIDTH, HEIGHT),
        fps.max(1),
        cam,
        0xf7ffff,
        |_, window, cam, delta| {
            handle_camera_controls(
                window,
                cam,
                0.01 * (delta.as_millis() as f32),
                0.001 * (delta.as_millis() as f32),
            );

            let elapsed = now.elapsed().unwrap().as_secs_f32();

            render::ObjectOrientation {
                position: Point3::new([0.0, 0.0, 4.0]),
                rotation: (0.0, f32::to_radians(elapsed * 20.0), f32::to_radians(-90.0)),
            }
        },
    );

    if fps == 0 {
        let frame = scene.draw_and_export_frame(render::ObjectOrientation {
            position: Point3::new([0.0, 0.0, 4.0]),
            rotation: (0.0, 0.0, f32::to_radians(0.0)),
        });

        let buf_data = &rgb8_to_u8_vec(frame)[..];
        let save_res = image::save_buffer(
            "output.png",
            buf_data,
            WIDTH as u32,
            HEIGHT as u32,
            image::ColorType::Rgb8,
        );

        match save_res {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    } else {
        scene.run();
        Ok(())
    }
}

fn rgb8_to_u8_vec(rgb: &[u32]) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::with_capacity(rgb.len() * 3);
    for &pixel in rgb {
        res.push(((pixel >> 16) & 0xff) as u8);
        res.push(((pixel >> 8) & 0xff) as u8);
        res.push((pixel & 0xff) as u8);
    }
    res
}

fn handle_camera_controls(
    window: &minifb::Window,
    camera: &mut Camera,
    speed: f32,
    rotation_speed: f32,
) {
    // Get camera rotation
    let (rot_x, rot_y, rot_z) = camera.rotation();

    // Calculate camera's basis vectors for left-handed system (+x right, +y up, +z into screen)

    // Forward vector (where the camera is pointing, +z direction)
    // For pitch: positive rot_x means looking up, negative means looking down
    let forward = Point3::new([
        rot_y.sin() * rot_x.cos(),
        -rot_x.sin(), // Negative Y component when looking down (negative rot_x)
        rot_y.cos() * rot_x.cos(),
    ]);

    // Right vector (perpendicular to forward in the horizontal plane, +x direction)
    let right = Point3::new([rot_y.cos(), 0.0, -rot_y.sin()]);

    // Up vector (world up, not camera up, to maintain stable movement)
    let up = Point3::new([0.0, 1.0, 0.0]);

    // Initialize movement vector
    let mut movement = Point3::new([0.0, 0.0, 0.0]);

    // Add movement components based on key presses
    if window.is_key_down(minifb::Key::W) {
        // Move forward - directly use the forward vector's components
        // This should allow proper movement in the direction you're looking
        movement[0] += forward[0] * speed;
        movement[1] += forward[1] * speed;
        movement[2] += forward[2] * speed;
    }
    if window.is_key_down(minifb::Key::S) {
        // Move backward - opposite of forward direction
        movement[0] -= forward[0] * speed;
        movement[1] -= forward[1] * speed;
        movement[2] -= forward[2] * speed;
    }
    if window.is_key_down(minifb::Key::A) {
        // Move left
        movement[0] -= right[0] * speed;
        movement[1] -= right[1] * speed;
        movement[2] -= right[2] * speed;
    }
    if window.is_key_down(minifb::Key::D) {
        // Move right
        movement[0] += right[0] * speed;
        movement[1] += right[1] * speed;
        movement[2] += right[2] * speed;
    }
    if window.is_key_down(minifb::Key::E) {
        // Move up
        movement[0] += up[0] * speed;
        movement[1] += up[1] * speed;
        movement[2] += up[2] * speed;
    }
    if window.is_key_down(minifb::Key::Q) {
        // Move down
        movement[0] -= up[0] * speed;
        movement[1] -= up[1] * speed;
        movement[2] -= up[2] * speed;
    }

    // Apply movement to camera position
    if movement != Point3::new([0.0, 0.0, 0.0]) {
        let current_pos = camera.position();
        camera.move_to(Point3::new([
            current_pos[0] + movement[0],
            current_pos[1] + movement[1],
            current_pos[2] + movement[2],
        ]));
    }

    // Rotation controls - Arrow keys
    let mut rotation_delta = (0.0f32, 0.0f32, 0.0f32);

    // Pitch control (z-axis rotation)
    if window.is_key_down(minifb::Key::Up) {
        rotation_delta.2 -= rotation_speed;
    }
    if window.is_key_down(minifb::Key::Down) {
        rotation_delta.2 += rotation_speed;
    }

    // Yaw control (y-axis rotation)
    if window.is_key_down(minifb::Key::Left) {
        rotation_delta.1 -= rotation_speed;
    }
    if window.is_key_down(minifb::Key::Right) {
        rotation_delta.1 += rotation_speed;
    }

    // Roll control (x-axis rotation) - Z and X keys
    if window.is_key_down(minifb::Key::Z) {
        rotation_delta.0 -= rotation_speed;
    }
    if window.is_key_down(minifb::Key::X) {
        rotation_delta.0 += rotation_speed;
    }

    // Apply rotation changes
    if rotation_delta != (0.0, 0.0, 0.0) {
        let (rot_x, rot_y, rot_z) = camera.rotation();
        camera.set_rotation((
            rot_x + rotation_delta.0,
            rot_y + rotation_delta.1,
            f32::clamp(
                rot_z + rotation_delta.2,
                -f32::consts::FRAC_PI_2,
                f32::consts::FRAC_PI_2,
            ),
        ));
    }

    // Update camera matrices if any changes were made
    if movement != Point3::new([0.0, 0.0, 0.0]) || rotation_delta != (0.0, 0.0, 0.0) {
        camera.update();
    }
}

fn main() {
    process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("error: {}", e);
            1
        }
    })
}

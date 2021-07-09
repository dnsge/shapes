use crate::world::camera::Camera;
use crate::world::{Object, Point3};
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
        |_, _, _| {
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

fn main() {
    process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("error: {}", e);
            1
        }
    })
}

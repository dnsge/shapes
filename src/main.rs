mod geo;
mod matrix;
mod ply;
mod render;
mod scene;

use geo::Point3;
use std::{env, path, process};

// in pixels
const WIDTH: usize = 750;
const HEIGHT: usize = 750;

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

    let mut object: ply::Object = ply::load(file_name);
    if scale != 0.0 {
        object.scale(scale);
    } else {
        object.normalize_size(4.0);
    }

    println!("Object details: {}", object);

    let now = std::time::SystemTime::now();
    let mut scene = scene::Scene::new(
        object,
        "Shapes - ESC to quit",
        (WIDTH, HEIGHT),
        fps,
        0xffffff,
        |screen, window| {
            let elapsed = now.elapsed().unwrap().as_secs_f32();

            render::ObjectOrientation {
                position: Point3::new([0.0, 0.0, 4.0]),
                rotation: (0.0, f32::to_radians(elapsed * 20.0), f32::to_radians(-90.0)),
            }
        },
    );

    scene.run();
    Ok(())
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

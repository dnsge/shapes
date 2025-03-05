use crate::screen_buffer::ScreenBuffer;
use crate::world::camera::Camera;
use minifb::{Key, Window, WindowOptions};

pub trait Renderer<S> {
    fn render(&self, screen: &mut ScreenBuffer, camera: &Camera, state: S);
}

pub struct Scene<T, S, F>
where
    T: Renderer<S>,
    F: Fn(&ScreenBuffer, &Window, &mut Camera, std::time::Duration) -> S,
    S: Default + Copy + PartialEq,
{
    screen: ScreenBuffer,
    window: Window,
    object: T,

    last_frame: std::time::Instant,
    frame_time: std::time::Duration,
    camera: Camera,
    background_color: u32,

    update_func: F,
    last_state: Option<S>,
}

impl<T, S, F> Scene<T, S, F>
where
    T: Renderer<S>,
    F: Fn(&ScreenBuffer, &Window, &mut Camera, std::time::Duration) -> S,
    S: Default + Copy + PartialEq,
{
    fn draw_frame(&mut self, state: S) {
        self.screen.clear(self.background_color);
        self.object.render(&mut self.screen, &self.camera, state);
        self.last_state = Some(state);
    }

    pub fn draw_and_export_frame(&mut self, state: S) -> &[u32] {
        self.screen.clear(self.background_color);
        self.object.render(&mut self.screen, &self.camera, state);
        self.last_state = Some(state);
        self.screen.buffer()
    }

    pub fn run(&mut self) {
        // Set FPS
        self.window.limit_update_rate(Some(self.frame_time));

        // Render loop
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let now = std::time::Instant::now();
            let delta = now - self.last_frame;
            self.last_frame = now;

            // Get next state
            let new_state: S =
                (self.update_func)(&self.screen, &self.window, &mut self.camera, delta);

            let state_changed: bool = match self.last_state {
                Some(old_state) => old_state != new_state,
                None => true,
            };
            let camera_changed = self.camera.get_and_clear_modified();

            // Only render if something has changed
            if state_changed || camera_changed {
                self.draw_frame(new_state);
            }

            // Render buffer to screen
            self.window
                .update_with_buffer(
                    self.screen.buffer(),
                    self.screen.width(),
                    self.screen.height(),
                )
                .unwrap();
        }
    }

    pub fn new(
        object: T,
        title: &str,
        size: (usize, usize),
        fps: u64,
        camera: Camera,
        background_color: u32,
        update_func: F,
    ) -> Scene<T, S, F> {
        let screen = ScreenBuffer::new(size.0, size.1);
        let window = Window::new(title, size.0, size.1, WindowOptions::default()).unwrap();

        Scene {
            screen,
            window,
            object,
            last_frame: std::time::Instant::now(),
            frame_time: std::time::Duration::from_micros(1_000_000 / fps),
            camera,
            background_color,
            update_func,
            last_state: None,
        }
    }
}

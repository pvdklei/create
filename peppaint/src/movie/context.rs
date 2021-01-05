use crate::{
    Window,
    Vec3,
    Key,
    PainterDepricated,
    Mat4
};

use nalgebra_glm as glm;
use std::time::SystemTime;

trait Context {
    fn create() -> Self;
    fn update(&mut self);
}

pub struct MyContext {
    pub painter: PainterDepricated,
    pub window: Window, 
    view_projection: Mat4,
    view_projection_glm: glm::Mat4,
    pub real_time: SystemTime,
    pub pnoise: perlin_noise::PerlinNoise,
    pub rng: rand::rngs::ThreadRng,
    route: fn(f32) -> Vec3,
    framerate: usize,
    dt: f64,
    movie_time: f64,
    location: Vec3,
    heading: Vec3,
}

impl MyContext {

    pub fn create() -> Self {
        let window = Window::set_up(500, 500, "PepMovie");
        let view_projection = Mat4::identity();
        let view_projection_glm = glm::identity();
        let real_time = SystemTime::now();
        let movie_time = 0.0f64;
        let route = |_| { Vec3(0.0, 0.0, 0.0) };
        let framerate = 60;
        let dt = 1.0 / framerate as f64;
        let mut painter = PainterDepricated::ne();
        painter.origin_top_left_projection(window.get_width(), window.get_height());
        let pnoise = perlin_noise::PerlinNoise::new(); 
        let rng = rand::thread_rng();
        Self {
            window, 
            view_projection,
            view_projection_glm,
            framerate,
            dt,
            movie_time,
            real_time, 
            route,
            painter,
            pnoise,
            rng,
            location: Vec3(0.0, 0.0, 0.0),
            heading: Vec3(0.0, 0.0, 0.0)
        }
    }

    pub fn sign(&mut self, width: usize, height: usize, title: &str) {
        self.set_window_size(width, height);
        self.window.set_title(title);
    }

    pub fn set_window_size(&mut self, width: usize, height: usize) {
        self.window.set_size(width, height);
        self.painter.origin_top_left_projection(width, height);
    }

    pub fn set_framerate(&mut self, fr: usize) {
        self.framerate = fr;
        self.dt = 1.0 / fr as f64;
    }

    pub fn get_vp(&self) -> Mat4 { self.view_projection }
    pub fn set_vp(&mut self, vp: &Mat4) { self.view_projection = *vp }
    pub fn get_width(&self) -> f32 { self.window.get_width() as f32 }
    pub fn get_height(&self) -> f32 { self.window.get_height() as f32 }
    pub fn set_route(&mut self, route: fn(f32) -> Vec3) { self.route = route; }
    pub fn get_route(&mut self) -> fn(f32) -> Vec3 { self.route }
    pub fn get_location(&self) -> Vec3 { self.location }
    pub fn get_heading(&self) -> Vec3 { self.heading }

    // INPUT
 
    pub fn is_key_pressed(&self, k: Key) -> bool {
        self.window.is_key_pressed(k)
    }

    pub fn is_mouse_pressed(&self) -> bool {
        self.window.is_mouse_presses()
    }

    pub fn get_cursor_pos(&self) -> (f32, f32) {
        self.window.get_cursor_pos() 
    }

    // a tuple containing the normalized offsets from the middle
    pub fn get_cursor_direction(&self) -> (f32, f32) {
        let (xcur, ycur) = self.window.get_cursor_pos();
        let xoff = (xcur - self.get_width() / 2.0) / (self.get_width() / 2.0);
        let yoff = (ycur - self.get_height() / 2.0) / (self.get_height() / 2.0);
        (xoff, yoff)
    }

    // TIME

    pub fn time(&self) -> f64 {
        self.movie_time
    }

    pub fn dt(&self) -> f64 {
        self.dt
    }

    pub fn real_time(&self) -> f64 {
        self.real_time.elapsed().unwrap().as_secs_f64()
    }

    pub fn timestep(&mut self) {
        self.movie_time += self.dt;
        self.location = (self.route)(self.movie_time as f32);
        self.heading = (self.route)(self.movie_time as f32 + crate::EPSILON) - self.location
    }

    pub fn new_frame(&mut self) {
        self.timestep();
        self.window.show();
    }
}


use peppaint::{
    HeadingProjCam,
    Actor,
    MyContext,
};

use nalgebra_glm as glm;

pub struct ControlableCamera {
    pos: glm::Vec3,
    heading: glm::Vec3,
    speed: f32,
    sensitivity: f32
} 

impl ControlableCamera {
    pub fn new() -> Self {
        Self {
            pos: glm::vec3(0.0, 0.0, 5.0),
            heading: -glm::Vec3::z(), 
            speed: 1.0, sensitivity: 1.0
        }
    }

    pub fn get_vp(&self) -> glm::Mat4 {
        glm::look_at(&self.pos, &(self.pos + self.heading), &glm::Vec3::y())
    }
}

// impl Actor for ControlableCamera {
//     fn act(&mut self, ctx: &mut MyContext) {
//         let dt = ctx.dt();
//         if ctx.is_key_pressed(Key::W) {
//             self.pos += self.heading * speed * dt;
//         } 
//         if ctx.is_key_pressed(Key::S) {
//             self.pos -= self.heading * speed * dt;
//         } 
//     }
// }

pub struct FirstPersonRollerCoasterCam {
    cam: HeadingProjCam
}

impl FirstPersonRollerCoasterCam {
    pub fn ne() -> Self {
        Self { cam: HeadingProjCam::new(70.0) }
    }
}

impl Actor for FirstPersonRollerCoasterCam {
    fn act(&mut self, ctx: &mut MyContext) {
        self.cam.pos = ctx.get_location();
        self.cam.heading = ctx.get_heading();
        ctx.set_vp(&self.cam.get_vp());
    }
}
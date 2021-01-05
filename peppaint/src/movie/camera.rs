use crate::types::*;
use crate::constants::*;

pub struct OrthoCam {
    pub pos: Vec3,
    pub rotation: Vec3,
    pub dims: [Vec2; 3],
}

impl OrthoCam {
    pub fn new(width: usize, height: usize) -> Self {
        let pos = Vec3(0.0, 0.0, 0.0);
        let rotation = Vec3(0.0, 0.0, 0.0);
        let dims = [
            Vec2(0.0, width as f32),
            Vec2(0.0, height as f32),
            Vec2(-1.0, 1.0),
        ];
        Self { pos, dims, rotation }
    }

    pub fn get_vp(&self) -> Mat4 {
        let trans = Mat4::translate(-self.pos.0, -self.pos.1, -self.pos.2);
        let rotx = Mat4::rotation(self.rotation.0, &crate::E1);
        let roty = Mat4::rotation(self.rotation.1, &crate::E2);
        let rotz = Mat4::rotation(self.rotation.2, &crate::E3);
        let orth = Mat4::ortho(
            self.dims[0].0, self.dims[0].1, 
            self.dims[1].0, self.dims[1].1, 
            self.dims[2].0, self.dims[2].1
        );
        orth * trans * rotx * roty * rotz 
    }
}

pub struct HeadingProjCam {
    pub fov: Vec2,
    pub close_far: Vec2,
    pub pos: Vec3,
    pub heading: Vec3, 
}

impl HeadingProjCam {

    pub fn new(far: f32) -> Self {
        Self {
            fov: Vec2(QUARTER_PI, QUARTER_PI),
            close_far: Vec2(0.1, far),
            pos: Vec3(0.0, 0.0, 0.0),
            heading: Vec3(0.0, 0.0, 1.0),
        }
    }

    pub fn get_vp(&self) -> Mat4 {
        let trans = Mat4::translate(-self.pos.0, -self.pos.1, -self.pos.2);
        let proj = Mat4::perspective(self.close_far.0, self.close_far.1, self.fov.0, self.fov.1);
        let rot = Mat4::inv_rotation_from_heading(&E3, &self.heading);
        proj * rot * trans
    }
}

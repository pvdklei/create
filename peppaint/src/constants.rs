use gl::types::GLfloat;
use crate::types::Vec3;

pub const GL_MAJOR: usize = 4;
pub const GL_MINOR: usize = 1;

pub const PI: GLfloat = 3.141592;
pub const HALF_PI: GLfloat = PI / 2.0;
pub const TWO_PI: GLfloat = PI * 2.0;
pub const QUARTER_PI: GLfloat = PI / 4.0;

pub const MICROS_PER_SECOND: u32 = 1_000_000;

pub const EPSILON: f32 = 0.001;

pub const FRAME_TIME_60FPS_IN_MICROS: u128 = (MICROS_PER_SECOND as u128)/60;
pub const FRAME_TIME_30FPS_IN_MICROS: u128 = (MICROS_PER_SECOND as u128)/30;

pub const E1: Vec3 = Vec3(1.0, 0.0, 0.0);
pub const E2: Vec3 = Vec3(0.0, 1.0, 0.0);
pub const E3: Vec3 = Vec3(0.0, 0.0, 1.0);

pub const FLOAT: gl::types::GLenum = gl::FLOAT;
pub const INT: gl::types::GLenum = gl::INT;
pub const UINT: gl::types::GLenum = gl::UNSIGNED_INT;
use crate::{
    Vec2
};

pub trait FloatFloat {
    fn unpack(&self) -> (f32, f32);
}

impl FloatFloat for (f32, f32) {
    fn unpack(&self) -> (f32, f32) {
        (self.0, self.1)
    }
}

impl FloatFloat for Vec2 {
    fn unpack(&self) -> (f32, f32) {
        (self.0, self.1)
    }
}
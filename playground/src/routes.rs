use peppaint::{Vec3};

pub fn f1(t: f32) -> Vec3 {
    let x = t.tan();
    let y = 3.0 * t + 0.1 * t.powi(2);
    let z = 4.0 * t.cos();
    Vec3(x, y, z) 
}

pub fn fa1(t: f32) -> Vec3 {
    let x = t.tan().powi(2) + 1.0;
    let y = 3.0 + 0.2 * t;
    let z = - 4.0 * t.sin();
    Vec3(x, y, z) 
}

pub fn f2(t: f32) -> Vec3 {
    let x = 8.0 * t.sin() + 3.0 * (t * 2.0).cos();
    let y = 3.0 * t + 0.1 * t.powi(2);
    let z = 4.0 * t.cos();
    Vec3(x, y, z) 
}

pub fn fa2(t: f32) -> Vec3 {
    let x = 8.0 * t.cos() - 6.0 * (t * 2.0).sin();
    let y = 3.0 + 0.2 * t;
    let z = - 4.0 * t.sin();
    Vec3(x, y, z) 
}

pub fn f3(t: f32) -> Vec3 {
    Vec3(t, t, t) 
}

pub fn fa3(_: f32) -> Vec3 {
    Vec3(1.0, 1.0, 1.0) 
}

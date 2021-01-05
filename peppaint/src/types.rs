use gl::types::*;
use crate::constants::EPSILON;

pub enum Type {
    Float,
    Int,
    Uint,
    Byte,
    Ubyte
}

impl Type {
    pub fn to_gl_type_enum(&self) -> gl::types::GLenum {
        use Type::*;
        match self {
            Float => gl::FLOAT,
            Int => gl::INT,
            Uint => gl::UNSIGNED_INT,
            Byte => gl::BYTE,
            Ubyte => gl::UNSIGNED_BYTE
        }
    }
}

pub type Float = GLfloat;
pub type Uint = GLuint;
pub type Int = GLint;
pub type Byte = GLbyte;
pub type Ubyte = GLubyte;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Vec2(pub GLfloat, pub GLfloat);

impl Vec2 {
    pub fn rotate(&self, angle: Float) -> Self {
        Self(
            self.0 * angle.cos() - self.1 * angle.sin(),
            self.0 * angle.sin() + self.1 * angle.cos(),
        )
    }

    pub fn mag(&self) -> Float {
        ((self.0).powi(2) + (self.1).powi(2)).sqrt()
    }

    pub fn resize_me(&mut self, fac: Float) {
        let mag = self.mag();
        self.0 *= fac / mag;
        self.1 *= fac / mag;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Vec2(self.0 - b.0, self.1 - b.1)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Vec2(self.0 + b.0, self.1 + b.1)
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, b: Self) {
        self.0 -= b.0;
        self.1 -= b.1;
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, b: Self) {
        self.0 += b.0;
        self.1 += b.1;
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub GLfloat, pub GLfloat, pub GLfloat);

impl Vec3 {

    pub fn e1() -> Self { Self(1.0, 0.0, 0.0) }
    pub fn e2() -> Self { Self(0.0, 1.0, 0.0) }
    pub fn e3() -> Self { Self(0.0, 0.0, 1.0) }

    pub fn mag(&self) -> GLfloat {
        (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }

    pub fn add(a: &Self, b: &Self) -> Self {
        Vec3(
            a.0 + b.0,
            a.1 + b.1,
            a.2 + b.2,
        )
    }

    pub fn sub(a: &Self, b: &Self) -> Self {
        Vec3(
            a.0 - b.0,
            a.1 - b.1,
            a.2 - b.2,
        )
    }

    pub fn resize(a: &Self, b: GLfloat) -> Self {
        Self(
            a.0 * b,
            a.1 * b,
            a.2 * b
        )
    }

    pub fn resize_me(&mut self, factor: GLfloat) {
        self.0 *= factor;
        self.1 *= factor;
        self.2 *= factor;
    }

    pub fn normalize_me(&mut self) {
        self.resize_me(1.0 / self.mag());
    }

    pub fn cross(a: &Self, b: &Self) -> Self {
        let c1 = a.1 * b.2 - a.2 * b.1;
        let c2 = a.2 * b.0 - a.0 * b.2;
        let c3 = a.0 * b.1 - a.1 * b.0;
        Self(c1, c2, c3)
    }

    pub fn dot(a: &Self, b: &Self) -> GLfloat {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2 
    }

    pub fn angle(a: &Self, b: &Self) -> GLfloat {
        let num = Self::dot(a, b);
        let den = a.mag() * b.mag();
        let cos_angle = num / den;
        let angle = cos_angle.acos();
        angle
    }

    pub fn to_homo(a: &Self) -> Vec4 {
        Vec4(a.0, a.1, a.2, 1.0)
    }

    pub fn same(a: &Self, b: &Self) -> bool {
        let dif = *a - *b;
        if  dif.0.abs() > EPSILON ||
            dif.1.abs() > EPSILON ||
            dif.2.abs() > EPSILON 
        { return false; }
        true
    }

    pub fn print(&self) {
        unsafe { println!("This Vec3: ({}, {}, {})", self.0, self.1, self.2) }
    }
}

impl std::ops::Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, b: Self) -> Self {
        Vec3::add(&self, &b)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, b: Self) -> Self {
        Vec3::sub(&self, &b)
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Vec4(pub GLfloat, pub GLfloat, pub GLfloat, pub GLfloat);

impl Vec4 {

    pub fn mag(&self) -> GLfloat {
        (self.0*self.0 + self.1*self.1 + self.2*self.2 + self.3*self.3).sqrt()
    }

    pub fn dot(&self, v: Self) -> GLfloat {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2 + self.3 * v.3
    }

    pub fn resize_me(&mut self, factor: GLfloat) {
        self.0 *= factor;
        self.1 *= factor;
        self.2 *= factor;
        self.3 *= factor;
    }

    pub fn eucl(&self) -> Vec3 {
        Vec3(
            self.0 / self.3,
            self.1 / self.3,
            self.2 / self.3
        )
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Mat4(pub Vec4, pub Vec4, pub Vec4, pub Vec4);

impl Mat4 {

    // STATICS

    pub fn identity() -> Self {
        Self(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0)
        )
    }

    pub fn uninit() -> Self {
        use std::mem::MaybeUninit;
        unsafe { MaybeUninit::uninit().assume_init() }
    }

    pub fn scale(sx: Float, sy: Float, sz: Float) -> Self {
        Mat4(
            Vec4(sx, 0.0, 0.0, 0.0),
            Vec4(0.0, sy, 0.0, 0.0),
            Vec4(0.0, 0.0, sz, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn translate(tx: GLfloat, ty: GLfloat, tz: GLfloat) -> Self {
        Self(
            Vec4(1.0, 0.0, 0.0, tx),
            Vec4(0.0, 1.0, 0.0, ty),
            Vec4(0.0, 0.0, 1.0, tz),
            Vec4(0.0, 0.0, 0.0, 1.0)
        )
    }

    pub fn rotation(theta: GLfloat, n: &Vec3) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        let omc = 1.0 - c;
        let n1s = n.0.powi(2);
        let n2s = n.1.powi(2);
        let n3s = n.2.powi(2);

        let m11 = c + n1s*omc;
        let m12 = n.0*n.1*omc - n.2*s;
        let m13 = n.0*n.2*omc + n.1*s;
        let m21 = n.0*n.1*omc + n.2*s;
        let m22 = c + n2s*omc;
        let m23 = n.1*n.2*omc - n.0*s;
        let m31 = n.0*n.2*omc - n.1*s;
        let m32 = n.1*n.2*omc + n.0*s;
        let m33 = c + n3s*omc;

        Self(
            Vec4(m11, m12, m13, 0.0),
            Vec4(m21, m22, m23, 0.0),
            Vec4(m31, m32, m33, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0)
        )
    }

    pub fn euler_rotation(x: f32, y: f32, z: f32) -> Self {
        let x = Self::rotation(x, &Vec3::e1());
        let y = Self::rotation(y, &Vec3::e2());
        let z = Self::rotation(z, &Vec3::e3());
        x * y * z
    }

    pub fn glitch_rotation(heading: &Vec3, nul_heading: &Vec3) -> Self {
        
        let v = Vec3::cross(nul_heading, heading);
        let c = Vec3::dot(nul_heading, heading);
        let vboxed = Mat4(
            Vec4(0.0, -v.2, v.1, 0.0),
            Vec4(v.2, 0.0, -v.0, 0.0),
            Vec4(-v.1, v.0, 0.0, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0)
        );
        let factor = 1.0 / (c + 1.0);
        let mut vboxsquare = vboxed * vboxed;
        vboxsquare.resize_me(factor);
        let identity = Mat4::identity();
        let r = identity + vboxed + vboxsquare;
        r
    }

    pub fn rotation_from_heading(nul_heading: &Vec3, heading: &Vec3) -> Self {
        if Vec3::same(nul_heading, heading) {
            return Mat4::identity();
        }
        let mut axis = Vec3::cross(nul_heading, heading);
        axis.normalize_me();
        let angle = Vec3::angle(nul_heading, heading);
        let rot = Mat4::rotation(angle, &axis);
        rot
    }

    pub fn inv_rotation_from_heading(nul_heading: &Vec3, heading: &Vec3) -> Self {
        if Vec3::same(nul_heading, heading) {
            return Mat4::identity();
        }
        let mut axis = Vec3::cross(nul_heading, heading);
        axis.normalize_me();
        let angle = - Vec3::angle(nul_heading, heading);
        let rot = Mat4::rotation(angle, &axis);
        rot
    }

    pub fn perspective(
        near: Float, 
        far: Float, 
        x_angle: Float, 
        y_angle: Float
    ) -> Self {
        let fx = 1.0 / x_angle.tan();
        let fy = 1.0 / y_angle.tan();
        let a = (far + near) / (far - near);
        let b = (1.0 - a) * far;
        Self(
            Vec4(fx, 0.0, 0.0, 0.0),
            Vec4(0.0, fy, 0.0, 0.0),
            Vec4(0.0, 0.0, a, b),
            Vec4(0.0, 0.0, 1.0, 0.0)
        )
    }

    pub fn perspective_good(
        near: Float, 
        far: Float, 
        fov: Float, 
        aspect: Float
    ) -> Self {
        let tanfov = fov.tan();
        let m11 = 1.0 / (aspect * tanfov);
        let m22 = 1.0 / tanfov;
        let m33 = - (far + near) / (far - near);
        let m34 = - (2.0 * far * near) / (far - near);
        Self(
            Vec4(m11, 0.0, 0.0, 0.0),
            Vec4(0.0, m22, 0.0, 0.0),
            Vec4(0.0, 0.0, m33, m34),
            Vec4(0.0, 0.0, -1.0, 0.0)
        )
    }

    pub fn ortho(
        left: Float, right: Float,
        bottom: Float, top: Float,
        far: Float, near: Float,
    ) -> Self {
        Self(
            Vec4(2.0 / (right - left), 0.0, 0.0, -(right + left) / (right - left)),
            Vec4(0.0, 2.0 / (top - bottom), 0.0, -(top + bottom) / (top - bottom)),
            Vec4(0.0, 0.0, 2.0 / (far - near), -(far + near) / (far - near)),
            Vec4(0.0, 0.0, 0.0, 1.0)
        )
    }

    pub fn model(trans: Vec3, scale: Vec3, rot: Vec3) -> Self {
        let trans = Mat4::translate(trans.0, trans.1, trans.2);
        let scale = Mat4::scale(scale.0, scale.1, scale.2);
        let rot = Mat4::euler_rotation(rot.0, rot.1, rot.2);
        trans * scale * rot
    }

    pub fn transpose(mat: &Mat4) -> Self {
        let mut x1 = (mat.0).0;
        let mut x2 = (mat.1).0;
        let mut x3 = (mat.2).0;
        let mut x4 = (mat.3).0;
        let r1 = Vec4(x1, x2, x3, x4);

        x1 = (mat.0).1;
        x2 = (mat.1).1;
        x3 = (mat.2).1;
        x4 = (mat.3).1;
        let r2 = Vec4(x1, x2, x3, x4);

        x1 = (mat.0).2;
        x2 = (mat.1).2;
        x3 = (mat.2).2;
        x4 = (mat.3).2;
        let r3 = Vec4(x1, x2, x3, x4);

        x1 = (mat.0).3;
        x2 = (mat.1).3;
        x3 = (mat.2).3;
        x4 = (mat.3).3;
        let r4 = Vec4(x1, x2, x3, x4);

        Mat4(r1, r2, r3, r4)
    }

    pub fn mult(a: &Self, b: &Self) -> Self {

        let b = Mat4::transpose(b);
        let m11 = b.0.dot(a.0);
        let m12 = b.1.dot(a.0);
        let m13 = b.2.dot(a.0);
        let m14 = b.3.dot(a.0);

        let m21 = b.0.dot(a.1);
        let m22 = b.1.dot(a.1);
        let m23 = b.2.dot(a.1);
        let m24 = b.3.dot(a.1);

        let m31 = b.0.dot(a.2);
        let m32 = b.1.dot(a.2);
        let m33 = b.2.dot(a.2);
        let m34 = b.3.dot(a.2);

        let m41 = b.0.dot(a.3);
        let m42 = b.1.dot(a.3);
        let m43 = b.2.dot(a.3);
        let m44 = b.3.dot(a.3);

        let r1 = Vec4(m11, m12, m13, m14);
        let r2 = Vec4(m21, m22, m23, m24);
        let r3 = Vec4(m31, m32, m33, m34);
        let r4 = Vec4(m41, m42, m43, m44);

        Self(r1, r2, r3, r4)
    }

    pub fn mult_vec4(a: &Self, b: &Vec4) -> Vec4 {
        let x1 = b.dot(a.0);
        let x2 = b.dot(a.1);
        let x3 = b.dot(a.2);
        let x4 = b.dot(a.3);
        Vec4(x1, x2, x3, x4)
    }

    // IMMUTUALS

    pub fn as_float_const_ptr(&self) -> *const GLfloat {
        unsafe { &(self.0).0 as *const GLfloat }
    }

    pub fn as_float_mut_ptr(&mut self) -> *mut GLfloat {
        unsafe { &mut (self.0).0 as *mut GLfloat }
    }

    // MUTUALS

    pub fn resize_me(&mut self, fac: GLfloat) {
        (self.0).0 *= fac;
        (self.0).1 *= fac;
        (self.0).2 *= fac;
        (self.0).3 *= fac;

        (self.1).0 *= fac;
        (self.1).1 *= fac;
        (self.1).2 *= fac;
        (self.1).3 *= fac;

        (self.2).0 *= fac;
        (self.2).1 *= fac;
        (self.2).2 *= fac;
        (self.2).3 *= fac;

        (self.3).0 *= fac;
        (self.3).1 *= fac;
        (self.3).2 *= fac;
        (self.3).3 *= fac;
    }

    pub fn transpose_me(&mut self) {
        let mut x1 = (self.0).0;
        let mut x2 = (self.1).0;
        let mut x3 = (self.2).0;
        let mut x4 = (self.3).0;
        let r1 = Vec4(x1, x2, x3, x4);

        x1 = (self.0).1;
        x2 = (self.1).1;
        x3 = (self.2).1;
        x4 = (self.3).1;
        let r2 = Vec4(x1, x2, x3, x4);

        x1 = (self.0).2;
        x2 = (self.1).2;
        x3 = (self.2).2;
        x4 = (self.3).2;
        let r3 = Vec4(x1, x2, x3, x4);

        x1 = (self.0).3;
        x2 = (self.1).3;
        x3 = (self.2).3;
        x4 = (self.3).3;
        let r4 = Vec4(x1, x2, x3, x4);

        self.0 = r1;
        self.1 = r2;
        self.2 = r3;
        self.3 = r4;
    }
}

impl std::ops::Mul for Mat4 {

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::mult(&self, &other)
    }
}

impl std::ops::MulAssign for Mat4 {

    fn mul_assign(&mut self, other: Self) {
        *self = Self::mult(&other, self);
    }
}

impl std::ops::Add for Mat4 {
    type Output = Self;

    fn add(self, b: Self) -> Self {
        let m00 = (self.0).0 + (b.0).0;
        let m01 = (self.0).1 + (b.0).1;
        let m02 = (self.0).2 + (b.0).2;
        let m03 = (self.0).3 + (b.0).3;

        let m10 = (self.1).0 + (b.1).0;
        let m11 = (self.1).1 + (b.1).1;
        let m12 = (self.1).2 + (b.1).2;
        let m13 = (self.1).3 + (b.1).3;

        let m20 = (self.2).0 + (b.2).0;
        let m21 = (self.2).1 + (b.2).1;
        let m22 = (self.2).2 + (b.2).2;
        let m23 = (self.2).3 + (b.2).3;

        let m30 = (self.3).0 + (b.3).0;
        let m31 = (self.3).1 + (b.3).1;
        let m32 = (self.3).2 + (b.3).2;
        let m33 = (self.3).3 + (b.3).3;

        Mat4(
            Vec4(m00, m01, m02, m03),
            Vec4(m10, m11, m12, m13),
            Vec4(m20, m21, m22, m23),
            Vec4(m30, m31, m32, m33)
        )

    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct Color(pub GLfloat, pub GLfloat, pub GLfloat, pub GLfloat);


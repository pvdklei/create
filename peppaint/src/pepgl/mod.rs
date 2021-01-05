pub mod shader;
pub use shader::*;

pub mod vao;
pub use vao::*;

pub mod window;
pub use window::*;

pub mod vertex;
pub use vertex::*;

pub mod mesh;
pub use mesh::*;

pub mod model;
pub use model::*;

pub mod buffer;
pub use buffer::*;

pub mod texture;
pub use texture::Texture;

use gl;

pub fn gl_draw_tris(n_indices: usize) {
    unsafe { 
        gl::DrawElements(
            gl::TRIANGLES, 
            n_indices as i32, 
            gl::UNSIGNED_INT, 
            0 as *const gl::types::GLvoid
        ); 
    }
}

pub fn gl_enable_depth() {
    unsafe { 
        if gl::IsEnabled(gl::DEPTH_TEST) == gl::TRUE { return }
        gl::Enable(gl::DEPTH_TEST); 
    }
}

pub fn gl_disable_depth() {
    unsafe { 
        if gl::IsEnabled(gl::DEPTH_TEST) == gl::FALSE { return }
        gl::Disable(gl::DEPTH_TEST); 
    }
}

pub fn gl_flush_error() {
    unsafe {
        while gl::GetError() != gl::NO_ERROR {};
    }
}

pub fn gl_check_error() {
    unsafe {
        loop {
            let error = gl::GetError();
            if error == gl::NO_ERROR { break }
            println!("[GL ERROR CODE] {}", error);
        }
    }
}

pub fn gl_fill() {
    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    }
}

pub fn gl_nofill() {
    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }
}

use gl::types::*;
use std::ffi::{CString};
use crate::types::{Mat4, Vec4};
use std::collections::HashMap;

pub struct ShaderProgram {
    id: GLuint,
    loc_cache: HashMap<String, GLint>
}

impl ShaderProgram {

    pub fn from_frag_and_vert_src(fs: &str, vs: &str) -> Result<Self, String> {
        let fs = Shader::from_source(
            fs,
            gl::FRAGMENT_SHADER,
        ).unwrap();
    
        let vs = Shader::from_source(
            vs,
            gl::VERTEX_SHADER,
        ).unwrap();

        Self::from_frag_and_vert_structs(fs, vs)
    }

    pub fn from_frag_and_vert_path(fs: &str, vs: &str) -> Result<Self, String> {
        let fs = Shader::from_path(
            fs,
            gl::FRAGMENT_SHADER,
        ).unwrap();
    
        let vs = Shader::from_path(
            vs,
            gl::VERTEX_SHADER,
        ).unwrap();

        Self::from_frag_and_vert_structs(fs, vs)
    }

    fn from_frag_and_vert_structs(fs: Shader, vs: Shader) -> Result<Self, String> {
        unsafe {
            let id = gl::CreateProgram();
            gl::AttachShader(id, fs.id);
            gl::AttachShader(id, vs.id);
            gl::LinkProgram(id);
    
            // ERROR HANDLING
            let mut it_worked = gl::FALSE as gl::types::GLint;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut it_worked);
            if it_worked == 0 {
                let error = String::from("ShaderProgram was not correctly made");
                return Err(error);
            }
            // END ERROR HANDLING
    
            gl::DetachShader(id, fs.id);
            gl::DetachShader(id, vs.id);

            let loc_cache = HashMap::new();
    
            Ok( Self { id, loc_cache } )
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn get_location(&mut self, name: &str) -> GLint {
        match self.loc_cache.get(name) {
            Some(loc) => *loc,
            None => {
                let loc: GLint = unsafe { gl::GetUniformLocation(self.id, const_char_ptr!(name) as *const i8) };
                if loc == -1 { println!("uniform name does not exist or starts with reserved prefix"); }
                else { self.loc_cache.insert(name.to_string(), loc); }
                loc
            }
        }
    }

    pub fn set_uint(&mut self, name: &str, uint: GLuint) {
        unsafe {
            let loc = self.get_location(name);
            gl::Uniform1ui(loc, uint);
        }
    }

    pub fn set_int(&mut self, name: &str, int: i32) {
        unsafe {
            let loc = self.get_location(name);
            gl::Uniform1i(loc, int);
        }
    }

    pub fn set_vec4f_depricated(&mut self, name: &str, vec4f: &Vec4) {
        unsafe {
            let loc = self.get_location(name);
            gl::Uniform4f(loc, vec4f.0, vec4f.1, vec4f.2, vec4f.3);
        }
    }

    pub fn set_float(&mut self, name: &str, float: GLfloat) {
        unsafe {
            let loc = self.get_location(name);
            gl::Uniform1f(loc, float);
        }
    }

    pub fn set_mat4f(&mut self, name: &str, mat: *const f32) {
        unsafe {
            let loc = self.get_location(name);
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat);
        }
    }

    pub fn set_mat4f_array(&mut self, name: &str, mat: *const f32, n: usize) {
        unsafe {
            let loc = self.get_location(name);
            gl::UniformMatrix4fv(loc, n as i32, gl::FALSE, mat);
        }
    }

    pub fn set_mat4f_array_generic<T>(&mut self, name: &str, mats: &[T]) {
        unsafe {
            let loc = self.get_location(name);
            gl::UniformMatrix4fv(loc, mats.len() as i32, gl::FALSE, mats.as_ptr() as *const f32);
        }
    }

    pub fn set_mat3f_array_generic<T>(&mut self, name: &str, mats: &[T]) {
        unsafe {
            let loc = self.get_location(name);
            gl::UniformMatrix3fv(loc, mats.len() as i32, gl::FALSE, mats.as_ptr() as *const f32);
        }
    }

    pub fn set_mat4f_depricated(&mut self, name: &str, mat: &Mat4) {
        unsafe {
            let loc = self.get_location(name);
            gl::UniformMatrix4fv(loc, 1, gl::TRUE, mat.as_float_const_ptr());
        }
    }

    pub fn set_mat4f_array_depricated(&mut self, name: &str, mats: &[Mat4], n: usize) {
        unsafe {
            let loc = self.get_location(name);
            gl::UniformMatrix4fv(loc, n as i32, gl::TRUE, mats[0].as_float_const_ptr());
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}

struct Shader {
    pub id: GLuint
}

impl Shader {
    pub fn from_source(src: &str, shader_type: GLuint) -> Result<Self, String> {
        unsafe {

            let src = CString::new(src).unwrap();
            let src = src.as_c_str();

            // let src = src.as_bytes();
            // let src = src.to_vec();
            // let src = CString::from_vec_unchecked(src);
            // let src = src.as_c_str();
    
            let id = gl::CreateShader(shader_type);
            gl::ShaderSource(id, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        
            // ERROR HANDLING
            let mut it_worked: GLint = gl::FALSE as GLint;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut it_worked);
        
            if it_worked != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                let buf = String::from_utf8_unchecked(buf);
                return Err(format!("Shader could not be made, LOG: {:?}", buf));
            }
            // END ERROR HANDLING
        
            Ok( Self { id } )
        }
    }

    pub fn from_path<'a>(path: &'a str, shader_type: GLuint) -> Result<Self, String> {
        let path = std::path::Path::new(path);
        let src = std::fs::read_to_string(path).expect(
            format!("Could not find shader source at {:?}", path).as_str()
        );
        Self::from_source(src.as_str(), shader_type)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}


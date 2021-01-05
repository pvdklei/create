use crate::pepgl::shader::ShaderProgram;
use crate::pepgl::vao::VertexArrayObject;
use crate::pepgl::vertex::Vertex;

use std::fmt::Debug;

// TODO: make similar to painting

const SHADER_ERROR: &str = "Shader was not loaded in this Mesh";

pub struct Mesh<T: Vertex + Copy + Debug> {
    program: Option<*mut ShaderProgram>,
    vao: VertexArrayObject,
    vertices: Vec<T>,
    indices: Vec<u32>, 
}

impl<T: Vertex + Copy + Debug> Mesh<T> {
    
    pub fn with_content(
        vertices: Vec<T>,
        indices: Vec<u32>,
    ) -> Self {
        let vao = VertexArrayObject::new(crate::DrawType::Static);
        let this = Self {
            vao, 
            program: None, 
            vertices, 
            indices, 
        };
        this.buffer();
        this
    }

    pub fn empty() -> Self {
        let vertices: Vec<T> = Vec::new();
        let indices: Vec<u32> = Vec::new();
        let vao = VertexArrayObject::new(crate::DrawType::Static);
        Self {
            vao, 
            program: None, 
            vertices, 
            indices, 
        }
    }

    pub fn static_with_capacity(n_vertices: usize, n_indices: usize) -> Self {
        let vertices: Vec<T> = Vec::with_capacity(n_vertices);
        let indices: Vec<u32> = Vec::with_capacity(n_indices);
        let vao = VertexArrayObject::new(crate::DrawType::Static);
        Self {
            vao, 
            program: None, 
            vertices, 
            indices, 
        }
    }

    pub fn dynamic_with_capacity(n_vertices: usize, n_indices: usize) -> Self {
        let vertices: Vec<T> = Vec::with_capacity(n_vertices);
        let indices: Vec<u32> = Vec::with_capacity(n_indices);
        let vao = VertexArrayObject::new(crate::DrawType::Dynamic);
        Self {
            vao, 
            program: None,  
            vertices, 
            indices, 
        }
    }

    pub fn bind(&self) { 
        self.bind_vao();
        self.bind_program();
    }

    pub fn bind_vao(&self) { self.vao.bind(); }
    pub fn bind_program(&self) { unsafe { (*self.program.expect(SHADER_ERROR)).bind(); } }

    pub fn set_program(&mut self, p: &mut ShaderProgram) { self.program = Some(p); }
    pub fn get_program(&self) -> &mut ShaderProgram { unsafe { &mut *self.program.expect(SHADER_ERROR) } }

    pub fn buffer(&self) {
        self.vao.bind();
        self.vao.buffer(&self.vertices, &self.indices);
    }

    pub fn subbuffer(&self) {
        self.vao.bind();
        self.vao.subbuffer(&self.vertices, &self.indices, 0, 0);
    }

    pub fn init_buffers(&self) {
        self.vao.bind();
        self.vao.init_buffers::<T>(self.vertices.capacity(), self.indices.capacity());
    }

    pub fn show(&self) {
        crate::gl_draw_tris(self.indices.len());
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn full(&self) -> bool {
        self.vertices.len() >= self.vertices.capacity() ||
        self.indices.len() >= self.indices.capacity()
    }

    pub fn push_data(&mut self, vertices: &[T], indices: &[u32]) {
        self.vertices.extend_from_slice(vertices);
        self.indices.extend_from_slice(indices);
    }

    pub fn push_tri(
        &mut self,
        v1: &T, v2: &T, v3: &T
    ) {
        let base = self.vertices.len() as u32;
        self.indices.push(base);
        self.indices.push(base + 1);
        self.indices.push(base + 2);

        self.vertices.push(*v1);
        self.vertices.push(*v2);
        self.vertices.push(*v3);
    } 

    pub fn push_quad(
        &mut self,
        v1: &T, v2: &T, v3: &T, v4: &T
    ) {
        let base = self.vertices.len() as u32;
        self.indices.push(base);
        self.indices.push(base + 2);
        self.indices.push(base + 1);
        self.indices.push(base);
        self.indices.push(base + 3);
        self.indices.push(base + 2);

        self.vertices.push(*v1);
        self.vertices.push(*v2);
        self.vertices.push(*v3);
        self.vertices.push(*v4);
    }

    pub fn push_fan(&mut self, v0: &T, v: &[T]) {
        let base = self.vertices.len() as u32;
        let mut first = base + 1;
        let mut next: u32;
        for _ in 1..v.len() {
            next = first + 1; 
            self.indices.push(base);
            self.indices.push(first);
            self.indices.push(next);
            first = next;
        }
        self.vertices.push(*v0);
        for v in v.iter() { self.vertices.push(*v) }
    }

    // DEBUG 

    pub fn print_vertices(&self) {
        println!("Vertices: \n\n{:#?}", self.vertices);
    }
}
use gl::types::*;
use std::path::Path;
use crate::{Vertex, Buffer, BufferType, DrawType};

pub struct VertexArrayObject {
    vao: u32,
    vbo: Buffer,
    vbos: Vec<Buffer>,
    ibo: Buffer, 
    drawtype: DrawType
}

impl VertexArrayObject { 

    pub fn new(drawtype: DrawType) -> Self {
        let vbo = Buffer::new(
            BufferType::Vertex, 
            drawtype
        );
        let ibo = Buffer::new(
            BufferType::Index, 
            drawtype
        );
        let vbos = Vec::new();
        let mut vao: u32 = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao); };
        Self { vao, vbo, vbos, ibo, drawtype }
    }

    pub fn new_static() -> Self {
        Self::new(DrawType::Static)
    }

    pub fn new_dynamic() -> Self {
        Self::new(DrawType::Dynamic)
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.vao) }
    }

    pub fn init_buffers<T: Vertex>(&self, verlen: usize, indlen: usize) {
        self.vbo.bind();
        self.vbo.init(verlen * std::mem::size_of::<T>());
        self.ibo.bind();
        self.ibo.init(indlen * std::mem::size_of::<crate::Uint>());
        self.set_vertex_layout::<T>();
    }

    pub fn buffer<T: Vertex>(&self, vertices: &[T], indices: &[u32]) {
        self.ibo.bind();
        self.ibo.buffer(indices);
        self.vbo.bind();
        self.vbo.buffer(vertices);
        self.set_vertex_layout::<T>();
    }

    pub fn buffer_indices(&self, indices: &[u32]) {
        self.ibo.bind();
        self.ibo.buffer(indices);
    }

    pub fn buffer_floats(&self, vertices: &[f32], indices: &[u32]) {
        self.ibo.bind();
        self.ibo.buffer(indices);
        self.vbo.bind();
        self.vbo.buffer(vertices);
    }

    pub fn subbuffer<T: Vertex>(
        &self, 
        vertices: &[T], 
        indices: &[u32], 
        vertex_offset: usize, 
        index_offset: usize
    ) {
        self.vbo.bind();
        self.vbo.subbuffer(vertices, vertex_offset);
        self.ibo.bind();
        self.ibo.subbuffer(indices, index_offset);
    }

    pub fn buffer_to_new_vbo<T>(&mut self, content: &[T]) {
        let vbo = Buffer::new(
            BufferType::Vertex,
            self.drawtype
        );
        vbo.bind();
        vbo.buffer(content);
        self.vbos.push(vbo);
    }

    fn set_vertex_layout<T: Vertex>(&self) {
        let vl = T::get_layout();
        for al in vl.attrib_layouts {
            self.set_attrib_layout(al.location, al.n_elements, vl.stride, al.byte_offset, al.type_);
        }
    }

    pub fn set_attrib_layout(
        &self, 
        location: usize, 
        n_elements: usize, 
        stride: usize, 
        byte_offset: usize, 
        type_: crate::Type,
    ) {
        unsafe {
            gl::EnableVertexAttribArray(location as GLuint);
            gl::VertexAttribPointer(
                location as GLuint, 
                n_elements as GLint, 
                type_.to_gl_type_enum(), 
                gl::FALSE, 
                stride as GLsizei, 
                byte_offset as *const GLvoid
            );
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.vao as *const GLuint) }
    }
}

// pub struct VertexArrayObject {
//     vao: u32,
//     vbo: Buffer,
//     ibo: Buffer, 
// }

// impl VertexArrayObject { 

//     pub fn new(drawtype: DrawType) -> Self {
//         let vbo = Buffer::new(
//             BufferType::Vertex, 
//             drawtype
//         );
//         let ibo = Buffer::new(
//             BufferType::Index, 
//             drawtype
//         );
//         let mut vao: u32 = 0;
//         unsafe { gl::GenVertexArrays(1, &mut vao); };
//         Self { vao, vbo, ibo }
//     }

//     pub fn new_static() -> Self {
//         Self::new(DrawType::Static)
//     }

//     pub fn new_dynamic() -> Self {
//         Self::new(DrawType::Dynamic)
//     }

//     pub fn bind(&self) {
//         unsafe { gl::BindVertexArray(self.vao) }
//     }

//     pub fn init_buffers<T: Vertex>(&self, verlen: usize, indlen: usize) {
//         self.vbo.bind();
//         self.vbo.init(verlen * std::mem::size_of::<T>());
//         self.ibo.bind();
//         self.ibo.init(indlen * std::mem::size_of::<crate::Uint>());
//         self.set_vertex_layout::<T>();
//     }

//     pub fn buffer<T: Vertex>(&self, vertices: &[T], indices: &[u32]) {
//         self.ibo.bind();
//         self.ibo.buffer(indices);
//         self.vbo.bind();
//         self.vbo.buffer(vertices);
//         self.set_vertex_layout::<T>();
//     }

//     pub fn buffer_floats(&self, vertices: &[f32], indices: &[u32]) {
//         self.ibo.bind();
//         self.ibo.buffer(indices);
//         self.vbo.bind();
//         self.vbo.buffer(vertices);
//     }

//     pub fn subbuffer<T: Vertex>(
//         &self, 
//         vertices: &[T], 
//         indices: &[u32], 
//         vertex_offset: usize, 
//         index_offset: usize
//     ) {
//         self.vbo.bind();
//         self.vbo.subbuffer(vertices, vertex_offset);
//         self.ibo.bind();
//         self.ibo.subbuffer(indices, index_offset);
//     }

//     fn set_vertex_layout<T: Vertex>(&self) {
//         let vl = T::get_layout();
//         for al in vl.attrib_layouts {
//             self.set_attrib_layout(al.location, al.n_elements, vl.stride, al.byte_offset, al.type_);
//         }
//     }

//     pub fn set_attrib_layout(
//         &self, 
//         location: usize, 
//         n_elements: usize, 
//         stride: usize, 
//         byte_offset: usize, 
//         type_: gl::types::GLenum
//     ) {
//         unsafe {
//             gl::EnableVertexAttribArray(location as GLuint);
//             gl::VertexAttribPointer(
//                 location as GLuint, 
//                 n_elements as GLint, 
//                 type_, 
//                 gl::FALSE, 
//                 stride as GLsizei, 
//                 byte_offset as *const GLvoid
//             );
//         }
//     }
// }

// impl Drop for VertexArrayObject {
//     fn drop(&mut self) {
//         unsafe { gl::DeleteVertexArrays(1, &self.vao as *const GLuint) }
//     }
// }
use gl::types::*;

pub struct Buffer {
    buffertype: gl::types::GLenum,
    drawtype: gl::types::GLenum, 
    id: u32
}

pub enum BufferType {
    Index, 
    Vertex
}

#[derive(Copy, Clone)]
pub enum DrawType {
    Static,
    Dynamic
}

impl Buffer {

    pub fn new(
        buffertype: BufferType,
        drawtype: DrawType,
    ) -> Self {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id); }
        let buffertype = match buffertype {
            BufferType::Index => gl::ELEMENT_ARRAY_BUFFER,
            BufferType::Vertex => gl::ARRAY_BUFFER
        };
        let drawtype = match drawtype {
            DrawType::Static => gl::STATIC_DRAW,
            DrawType::Dynamic => gl::DYNAMIC_DRAW
        };
        Self { buffertype, drawtype, id }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.buffertype, self.id); }
    }

    pub fn init(&self, size: usize) {
        unsafe {
            gl::BufferData(
                self.buffertype,
                size as GLsizeiptr,
                std::ptr::null(),
                self.drawtype
            );
        }
    }

    pub fn buffer<T>(&self, content: &[T]) {
        unsafe {
            gl::BufferData(
                self.buffertype,
                (content.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                content.as_ptr() as *const gl::types::GLvoid,
                self.drawtype
            );
        }
    }

    pub fn subbuffer<T>(&self, content: &[T], offset: usize) {
        unsafe {
            gl::BufferSubData(
                self.buffertype,
                offset as GLintptr,
                (content.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                content.as_ptr() as *const gl::types::GLvoid,
            );
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}
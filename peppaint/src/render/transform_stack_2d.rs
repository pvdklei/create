use nalgebra_glm as glm;

pub struct TransformStack2D {
    stack: Vec<glm::Mat3>,
    pub in_the_making: bool
}

impl TransformStack2D {
    pub fn new(len: usize) -> Self {
        let mut stack = Vec::<glm::Mat3>::with_capacity(len);
        stack.push(glm::identity());
        Self {
            stack,
            in_the_making: false
        }
    }

    pub fn new_matrix(&mut self) {
        if self.in_the_making {
            self.stack.push(*self.stack.last().unwrap());
        } else {
            self.stack.push(glm::identity());
            self.in_the_making = true;
        }
    }
    pub fn done(&mut self) {
        self.in_the_making = false;
    }

    pub fn get_stack(&self) -> &[glm::Mat3] {
        &self.stack
    }
    pub fn on_new_painting(&mut self) {
        if self.in_the_making {
            let current = self.stack.pop().unwrap(); 
            self.stack.clear();
            self.stack.push(glm::identity());
            self.stack.push(current);
        } else {
            self.stack.clear();
            self.stack.push(glm::identity());
        };
    }
    pub fn full(&self) -> bool {
        self.stack.len() == self.stack.capacity()
    }

    pub fn get_current_index(&self) -> usize {
        if self.in_the_making {
            return self.stack.len() - 1
        } 0
    }
    // MATRIX OPERATIONS

    pub fn set(&mut self, m: &glm::Mat3) {
        let last_index = self.get_current_index();
        self.stack[last_index] = *m;
    }
    pub fn mult(&mut self, m: &glm::Mat3) {
        let last_index = self.get_current_index();
        self.stack[last_index] *= *m;
    }
    pub fn rotate(&mut self, a: f32) {
        let index = self.get_current_index();
        self.stack[index] = glm::rotate2d(&self.stack[index], a);
    }
    pub fn translate(&mut self, tx: f32, ty: f32) {
        let index = self.get_current_index();
        self.stack[index] = glm::translate2d(&self.stack[index], &glm::vec2(tx, ty));
    }
    pub fn scale(&mut self, sx: f32, sy: f32) {
        let index = self.get_current_index();
        self.stack[index] = glm::scale2d(&self.stack[index], &glm::vec2(sx, sy));
    }
    pub fn scale_at(&mut self, sx: f32, sy: f32, x: f32, y: f32) {
        self.translate(-x, -y);
        self.scale(sx, sy);
        self.translate(x, y);
    }
    pub fn rotate_at(&mut self, a: f32, x: f32, y: f32) {
        self.translate(-x, -y);
        self.rotate(a);
        self.translate(x, y);
    }
}
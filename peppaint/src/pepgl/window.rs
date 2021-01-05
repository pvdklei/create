use glfw::ffi::*;
use gl;
use std::os::raw::*;

pub type Screen = Window;

pub struct Window {
    window: *mut GLFWwindow,
    width: usize,
    height: usize
}

impl Window {
    pub fn set_up(width: usize, height: usize, title: &str) -> Self {

        unsafe {

            if glfwInit() == FALSE {
                panic!("Could not set up Window!");
            }

            glfwWindowHint(OPENGL_PROFILE, OPENGL_CORE_PROFILE);
            glfwWindowHint(CONTEXT_VERSION_MAJOR, crate::GL_MAJOR as i32);
            glfwWindowHint(CONTEXT_VERSION_MINOR, crate::GL_MINOR as i32);
            glfwWindowHint(OPENGL_FORWARD_COMPAT, gl::TRUE as i32);
            // glfwWindowHint(SAMPLES, 4);
    
            glfwSetErrorCallback(Some(on_error));
    
            let window: *mut GLFWwindow = glfwCreateWindow(
                width as c_int, 
                height as c_int, 
                const_char_ptr!(title), 
                std::ptr::null_mut(), 
                std::ptr::null_mut()
            );

            if window.is_null() {
                panic!("Window could not be made!");
            }

            glfwMakeContextCurrent(window);

            gl::load_with(|s| glfwGetProcAddress(const_char_ptr!(s)) as *const std::os::raw::c_void);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);  
            gl::Enable(gl::BLEND); 
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
            // gl::Enable(gl::MULTISAMPLE);

            let this = Self { 
                window,
                width,
                height
            };

            this.make_currect();
            this
        }
    }

    pub fn get_width(&self) -> usize { self.width }
    pub fn get_height(&self) -> usize { self.height }

    pub fn set_background(&self, r: f32, g: f32, b: f32) {
        unsafe { gl::ClearColor(r, g, b, 1.0); }
    }

    pub fn set_size(&mut self, w: usize, h: usize) {
        unsafe { glfwSetWindowSize(self.window, w as c_int, h as c_int) }
        self.width = w;
        self.height = h;
    }

    pub fn set_title(&self, title: &str) {
        unsafe { glfwSetWindowTitle(self.window, const_char_ptr!(title)) }
    }

    pub fn show(&self) { 
        unsafe { 
            glfwSwapBuffers(self.window);
            gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT); 
        }
    }

    pub fn should_close(&self) -> bool {  
        unsafe { 
            glfwPollEvents();
            glfwWindowShouldClose(self.window) == TRUE 
        } 
    }

    pub fn make_currect(&self) {
        unsafe { glfwMakeContextCurrent(self.window); }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        unsafe {
            glfwGetKey(self.window, key.to_glfw()) == PRESS
        }
    }

    pub fn is_mouse_presses(&self) -> bool {
        unsafe {
            glfwGetMouseButton(self.window, MOUSE_BUTTON_LEFT) == PRESS
        }
    }

    pub fn get_cursor_pos(&self) -> (f32, f32) {
        let mut x = 0.0f64;
        let mut y = 0.0f64;
        unsafe { 
            glfwGetCursorPos(self.window, &mut x as *mut c_double, &mut y as *mut c_double);
        }
        (x as f32, y as f32)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { glfwDestroyWindow(self.window); }
    }
}

extern "C" fn on_error(error: c_int, des: *const c_char) {
    println!("Error {}: {:?}", error, des);
}

pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Escape,
    Space,
    A, W, D, S
}

impl Key {
    fn to_glfw(&self) -> c_int {
        use Key::*;
        match self {
            Up => KEY_UP,
            Down => KEY_DOWN,
            Left => KEY_LEFT,
            Right => KEY_RIGHT,
            Escape => KEY_ESCAPE,
            Space => KEY_SPACE,
            A => KEY_A,
            W => KEY_W,
            D => KEY_D,
            S => KEY_S
        }
    }
}

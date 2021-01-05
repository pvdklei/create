pub mod movielib;
pub use movielib::*;
pub mod actors;
pub use actors::*;
pub mod routes;
pub use routes::*; 

use peppaint::movie::movie::*;

use nalgebra_glm as glm;
use glm::{ Mat4 };
use std::path::Path;

fn main() {
    MoviePlayer::play_debug::<Test3>();
    MoviePlayer::play_debug::<Test2>();
    MoviePlayer::play_debug::<movielib::spiral_thing::SpiralThing>();
    MoviePlayer::play_debug::<movielib::square_adjuster::SquareAdjuster>();
    MoviePlayer::play_debug::<movielib::tunnel_simulator::TunnelSimulator>();
}

struct Test3 {
    p: Painter
}
impl Movie for Test3 {
    fn setup(ctx: &mut MyContext) -> Self { 
        ctx.window.set_background(0.2, 0.4, 0.0);
        let p = Painter::ne();
        Self { p }
    }
    fn show(&mut self, _ctx: &mut MyContext) {
        self.p.grey(1.0);

        self.p.new_model();
        self.p.alpha(0.1);
        self.p.translate(0.2, 0.3);
        self.p.grey(1.0);
        self.p.rect(0.0, 0.0, 0.2, 0.1);

        self.p.new_model();
        self.p.scale(3.0, 3.0);
        self.p.translate(-0.2, -0.3);
        self.p.grey(0.5);
        self.p.rect(0.0, 0.0, 0.2, 0.1);

        self.p.end_model();
        self.p.paint();
    }
}

use peppaint::{
    Vec2, 
    Key, 
    VertexArrayObject, 
    VertexLayout, 
    AttributeLayout, 
    Vertex, 
    Texture, 
    //Mat4,
    Painter,
    Mesh,
    MyContext,
    Actor,
    ShaderProgram,
    Vec3,
    Vec4,
    std::vertices,
};

use crate::{
    ring_tunnel::TunnelVertex
};

struct Test2 {
    program: ShaderProgram,
    pos: Vec3,
    model: peppaint::Model,
    camangles: (f32, f32)
}

impl Movie for Test2 {
    fn setup(_ctx: &mut MyContext) -> Self {

        let path = Path::new("/Users/pvdklei/Desktop/Code/graphics/playground/src/assets/rose/Models and Textures/rose.obj");

        let model = peppaint::Model::dot_obj(&path).unwrap();

        let program = ShaderProgram::from_frag_and_vert_path(
            "src/shaders/ringtunnel.frag",
            "src/shaders/ringtunnel.vert"
        ).unwrap();

        let pos = Vec3(0.0, 0.0, -5.0);
        let camangles = (0.0, 0.0);

        Self { program, pos, model, camangles }
    }
    fn update(&mut self, ctx: &mut MyContext) {

        if ctx.is_key_pressed(Key::W) {
            self.pos.2 += 0.5;
        } 
        if ctx.is_key_pressed(Key::S) {
            self.pos.2 -= 0.5;
        } 

        if ctx.is_key_pressed(Key::A) {
            self.pos.0 += 0.5;
        } 
        if ctx.is_key_pressed(Key::D) {
            self.pos.0 -= 0.5;
        } 

        if ctx.is_key_pressed(Key::Up) {
            self.pos.1 -= 0.4;
        } 
        if ctx.is_key_pressed(Key::Down) {
            self.pos.1 += 0.4;
        } 

        if ctx.is_mouse_pressed() {
            let (offx, offy) = ctx.get_cursor_direction();
            let fac = 0.05;
            self.camangles.0 += offy * fac;
            self.camangles.1 += offx * fac;
        } 

        // self.pos.print();

        let scale = glm::scaling(&glm::vec3(0.1, 0.1, 0.1));

        let trans = glm::translation(&glm::vec3(self.pos.0, self.pos.1, self.pos.2));
        let pers = glm::perspective(1.0, glm::half_pi(), 0.5, 50.0);
        let rotx = glm::rotation(self.camangles.0, &glm::vec3(1.0, 0.0, 0.0));
        let roty = glm::rotation(self.camangles.1, &glm::vec3(0.0, 1.0, 0.0));

        let model = scale;
        let view = pers * rotx * roty * trans;
        let mvp = view * model;

        self.program.bind();
        self.program.set_mat4f("uMVP", mvp.as_ptr());
        self.program.set_vec4f_depricated("uColor", &Vec4(1.0, 1.0 , 1.0, 1.0));

        self.model.show();

        peppaint::gl_check_error();
        peppaint::gl_flush_error();
        
    }
}

struct Test {
    pos: Vec2,
    vao: VertexArrayObject,
    program: peppaint::ShaderProgram,
    cam: FirstPersonRollerCoasterCam,
    tun: RingTunnel,
}

impl Movie for Test {

    fn setup(ctx: &mut MyContext) -> Self {

        ctx.sign(800, 500, "yoooo");
        ctx.window.set_background(1.0, 1.0, 1.0);
        ctx.set_route(crate::f2);

        let tun = RingTunnel::ne_def(ctx.get_route());

        let ver = &[
            TexVertex(-0.5, 0.5, 0.0, 1.0),
            TexVertex(0.5, 0.5, 1.0, 1.0),
            TexVertex(0.5, -0.5, 1.0, 0.0),
            TexVertex(-0.5, -0.5, 0.0, 0.0),
        ];
        let ind = &[0, 2, 1, 0, 3, 2];
        let vao = VertexArrayObject::new_static();
        vao.bind();
        vao.buffer(ver, ind);
        let program = peppaint::ShaderProgram::from_frag_and_vert_path(
            "src/shaders/texture.frag", 
            "src/shaders/texture.vert"
        ).unwrap();
        let texture = Texture::from_path("src/assets/chess.jpeg");

        Self { 
            pos: Vec2(200.0, 200.0),
            vao,
            program,
            cam: FirstPersonRollerCoasterCam::ne(),
            tun
        }
    }

    fn update(&mut self, ctx: &mut MyContext) {

        
        let speed = 3.0;
        if ctx.is_key_pressed(Key::Up) {
            self.pos += Vec2(0.0, -speed)
        }
        if ctx.is_key_pressed(Key::Down) {
            self.pos += Vec2(0.0, speed)
        }
        if ctx.is_key_pressed(Key::Left) {
            self.pos += Vec2(-speed, 0.0)
        }
        if ctx.is_key_pressed(Key::Right) {
            self.pos += Vec2(speed, 0.0)
        }

        let cpos = ctx.get_cursor_pos();
        let p = &mut ctx.painter;
        p.grey(0.2);
        p.circle(self.pos.0, self.pos.1, 70.0);
        p.color_alpha(0.5, 0.3, 1.0, 1.0);
        p.line(0.0, 0.0, 50.0, 300.0, 2.0);
        p.circle(90.0, 100.0, 10.0);
        p.circle(cpos.0, cpos.1, 10.0);
        p.rounded_rect(cpos.0, cpos.1, 200.0, 100.0, 50.0);
        p.paint();

        // self.cam.act(ctx);
        // self.tun.act(ctx);

        // self.vao.bind();
        // self.program.bind();
        // peppaint::gl_draw_tris(6);
    }
}

#[repr(C, packed)]
#[derive(Clone, Debug, Copy)]
pub struct TexVertex(f32, f32, f32, f32);

impl Vertex for TexVertex {
    fn get_layout() -> VertexLayout {
        let stride = 16;
        let al1 = AttributeLayout {
            location: 0,
            n_elements: 2, 
            byte_offset: 0,
            type_: peppaint::Type::Float
        };
        let al2 = AttributeLayout {
            location: 1,
            n_elements: 2,
            byte_offset: 8,
            type_: peppaint::Type::Float
        };
        let als = vec![al1, al2];
        VertexLayout {
            stride,
            attrib_layouts: als
        }
    }
}
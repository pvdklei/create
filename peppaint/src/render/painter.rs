use crate::{
    Mesh,
    types::*,
    std::PaintingVertex,
    constants::TWO_PI,
    ShaderProgram,
    transform_stack_2d::TransformStack2D
};

use nalgebra_glm as glm;

const STD_CIRLCE_PREC: usize = 200;
const STD_ROUNDED_CORNER_PREC: usize = 30;

// TODO: implement textures/text, etc
const MAX_MODEL_MATS: usize = 128;
const MAX_VERTICES: usize = 10000;
const MAX_INDICES: usize = 7000;

pub struct Painter {
    painting: Mesh<PaintingVertex>,
    settings: PainterSettings,
    projection: glm::Mat4,
    transforms: TransformStack2D,
    program: ShaderProgram
} 

impl Painter {

    pub fn ne() -> Self {
        // let painting = Painting::empty();
        let painting = Mesh::dynamic_with_capacity(MAX_VERTICES, MAX_INDICES);
        let program = ShaderProgram::from_frag_and_vert_src(
            include_str!("../std/shaders/painting.frag"), 
            include_str!("../std/shaders/painting.vert")
        ).expect("Could not find Painter Shaders");
        painting.init_buffers();

        let color = Color(1.0, 1.0, 1.0, 1.0);
        let projection = glm::identity();
        let transforms = TransformStack2D::new(MAX_MODEL_MATS);

        let settings = PainterSettings {
            color: glm::vec3(1.0, 1.0, 1.0),
            alpha: 1.0,
            foreground: true,
            fill: true,
        };

        Self { 
            painting,
            projection, 
            transforms,
            settings,
            program
        }
    }

    pub fn paint(&mut self) {
        crate::gl_disable_depth();
        if !self.settings.fill {
            crate::gl_nofill();
        }
        self.paint_default();
        crate::gl_enable_depth();
        if !self.settings.fill {
            crate::gl_fill();
        }
    }

    pub fn paint_default(&mut self) {
        self.program.bind();
        self.program.set_mat4f_array_generic("uViewProjection", &[self.projection]);
        self.program.set_mat3f_array_generic("uModels", self.transforms.get_stack());
        self.program.set_int("doForeground", self.settings.foreground as i32);
        self.painting.subbuffer();
        self.painting.show();
        self.new_painting();
    }

    pub fn new_painting(&mut self) {
        self.painting.clear();
        self.transforms.on_new_painting();
    }

    // OPTIONS

    pub fn fill(&mut self) {
        self.settings.fill = true;
    }

    pub fn nofill(&mut self) {
        self.settings.fill = false;
    }

    pub fn do_background(&mut self) {
        self.settings.foreground = false;
    }

    pub fn do_foreground(&mut self) {
        self.settings.foreground = true;
    }

    // OPTIONS: COLORS

    pub fn color(
        &mut self,
        red: Float, 
        green: Float, 
        blue: Float 
    ) {
        self.color_alpha(red, green, blue, 1.0);
    }

    pub fn grey(&mut self, val: Float) {
        self.color_alpha(val, val, val, 1.0)
    }

    pub fn color_alpha(
        &mut self, 
        red: Float, 
        green: Float, 
        blue: Float, 
        alpha: Float
    ) {
        self.settings.color = glm::vec3(red, green, blue);
        self.settings.alpha = alpha;
    }

    pub fn alpha(&mut self, a: f32) {
        self.settings.alpha = a;
    }

    // TRANSFORMS

    pub fn adjust_to_screen(&mut self, w: usize, h: usize) {
        self.projection = glm::ortho(
            0.0, w as f32,
            h as f32, 0.0, 
            1.0, -1.0
        );
    }

    pub fn new_model(&mut self) { 
        if self.transforms.full() {
            self.paint();
        } 
        self.transforms.new_matrix() 
    }
    pub fn end_model(&mut self) { self.transforms.done(); }
    pub fn translate(&mut self, tx: f32, ty: f32) { self.transforms.translate(tx, ty) }
    pub fn scale(&mut self, sx: f32, sy: f32) { self.transforms.scale(sx, sy) }
    pub fn rotate(&mut self, a: f32) { self.transforms.rotate(a) }
    pub fn scale_at(&mut self, sx: f32, sy: f32, x: f32, y: f32) { self.transforms.scale_at(sx, sy, x, y) }
    pub fn rotate_at(&mut self, a: f32, x: f32, y: f32) { self.transforms.rotate_at(a, x, y) }

    // SHAPES 

    fn make_vertex(&self, x: Float, y: Float) -> PaintingVertex {
        let color = (
            self.settings.color.x, 
            self.settings.color.y, 
            self.settings.color.z, 
            self.settings.alpha
        );
        PaintingVertex {
            pos: (x, y), 
            color, 
            model: self.transforms.get_current_index() as f32
        }
    }

    pub fn triangle(
        &mut self,
        x1: Float, y1: Float, 
        x2: Float, y2: Float,
        x3: Float, y3: Float,
    ) {
        if self.painting.full() {
            self.paint();
        };
        let v1 = self.make_vertex(x1, y1);
        let v2 = self.make_vertex(x2, y2);
        let v3 = self.make_vertex(x3, y3);
        self.painting.push_tri(&v1, &v2, &v3);
    }

    pub fn quad(
        &mut self, 
        x1: Float, y1: Float, 
        x2: Float, y2: Float,
        x3: Float, y3: Float,
        x4: Float, y4: Float
    ) {
        if self.painting.full() {
            self.paint();
        };
        let v1 = self.make_vertex(x1, y1);
        let v2 = self.make_vertex(x2, y2);
        let v3 = self.make_vertex(x3, y3);
        let v4 = self.make_vertex(x4, y4);

        self.painting.push_quad(&v1, &v2, &v3, &v4);
    }

    pub fn fan<T: crate::traits::FloatFloat>(&mut self, x0: f32, y0: f32, fan: &[T]) {
        let v0 = self.make_vertex(x0, y0);
        let mut vfan: Vec<PaintingVertex> = Vec::with_capacity(fan.len());
        for i in 0..fan.len() {
            let (x, y) = fan[i].unpack();
            let v = self.make_vertex(x, y);
            vfan.push(v);
        }
        self.painting.push_fan(&v0, vfan.as_slice());
    }

    pub fn line(
        &mut self,
        x1: Float, y1: Float, 
        x2: Float, y2: Float,
        width: Float
    ) {

        let dir = Vec2(x2 - x1, y2 - y1);
        let mut offset = dir.rotate(crate::HALF_PI);
        offset.resize_me(width / 2.0);
        self.quad(
            x1 - offset.0, y1 - offset.1,
            x1 + offset.0, y1 + offset.1,
            x2 + offset.0, y2 + offset.1,
            x2 - offset.0, y2 - offset.1,
        )
    }

    pub fn rect(&mut self, x: Float, y: Float, width: Float, height: Float) {
        self.quad(x, y, x + width, y, x + width, y + height, x, y + height);
    }

    pub fn square(&mut self, x: Float, y: Float, width: Float) {
        self.rect(x, y, width, width);
    }

    pub fn circle(&mut self, x: Float, y: Float, rad: Float) {
        self.circle_prec(x, y, rad, STD_CIRLCE_PREC);
    }

    pub fn circle_prec(&mut self, x0: Float, y0: Float, rad: Float, precision: usize) {
        let step = TWO_PI / (precision as f32);
        let mut total = 0f32;
        let mut fan: Vec<(f32, f32)> = Vec::with_capacity(precision);
        for _ in 0..(precision+1) {
            let x = rad * total.cos() + x0;
            let y = rad * total.sin() + y0;
            fan.push((x, y));
            total += step;
        }
        self.fan(x0, y0, fan.as_slice());
    }

    pub fn rounded_rect(
        &mut self, 
        x: Float, y: Float, 
        width: Float, height: Float,
        rad: Float
    ) {
        self.rounded_rect_prec(x, y, width, height, rad, STD_ROUNDED_CORNER_PREC);
    }

    pub fn rounded_rect_prec(
        &mut self, 
        x: Float, y: Float, 
        width: Float, height: Float,
        rad: Float, prec: usize
    ) {
        let m = Vec2(x + width / 2.0, y + height / 2.0);
        let bl = Vec2(x + rad, y + rad);
        let br = Vec2(x + width - rad, y + rad);
        let tr = Vec2(x + width - rad, y + height - rad);
        let tl = Vec2(x + rad, y + height - rad);

        let mut fan = Vec::<Vec2>::with_capacity(4 * (prec + 1));

        let step = TWO_PI / ((4 * prec) as f32);
        let mut total = 0f32;
        let mut circlefan = Vec::<Vec2>::with_capacity(4 * prec);
        for _ in 0..(prec * 4) {
            let x = rad * total.cos();
            let y = rad * total.sin();
            circlefan.push(Vec2(x, y));
            total += step;
        }

        for i in 0..(prec+1) {
            let offset = circlefan[i];
            fan.push(tr + offset); 
        }

        for i in prec..(2 * prec + 1) {
            let offset = circlefan[i];
            fan.push(tl + offset); 
        }

        for i in (2 * prec)..(3 * prec + 1) {
            let offset = circlefan[i];
            fan.push(bl + offset); 
        }

        for i in (3 * prec)..(4 * prec) {
            let offset = circlefan[i];
            fan.push(br + offset); 
        }

        fan.push(tr + circlefan[0]);

        self.fan(m.0, m.1, fan.as_slice());
    }
}

struct PainterSettings { 
    color: glm::Vec3,
    alpha: f32,
    foreground: bool,
    fill: bool
}

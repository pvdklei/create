use crate::{
    Mesh,
    types::*,
    std::DepricPaintingVertex,
    constants::TWO_PI,
    ShaderProgram,
};

const STD_CIRLCE_PREC: usize = 200;
const STD_ROUNDED_CORNER_PREC: usize = 30;

// TODO: implement textures/text, etc
const MAX_MODEL_MATS: usize = 128;
const MAX_VERTICES: usize = 10000;
const MAX_INDICES: usize = 7000;


pub struct PainterDepricated {
    // painting: Painting,
    painting: Mesh<DepricPaintingVertex>,
    color: Color,
    projection: Mat4,
    view: Mat4,
    models: [Mat4; MAX_MODEL_MATS],
    model_ptr: usize,
    current_model_ptr: usize,
    making_model: bool,
    fill: bool,
    z: f32,
    program: Box<ShaderProgram>
} 

impl PainterDepricated {

    pub fn ne() -> Self {
        // let painting = Painting::empty();
        let mut painting = Mesh::dynamic_with_capacity(MAX_VERTICES, MAX_INDICES);
        let program = Box::new(ShaderProgram::from_frag_and_vert_src(
            include_str!("../std/shaders/painting_depric.frag"), 
            include_str!("../std/shaders/painting_depric.vert")
        ).expect("Could not find Painter Shaders"));
        painting.init_buffers();

        let color = Color(1.0, 1.0, 1.0, 1.0);
        let projection = Mat4::identity();
        let view = Mat4::identity();
        let mut models = [Mat4::uninit(); MAX_MODEL_MATS];
        models[0] = Mat4::identity();

        Self { 
            painting,
            color, 
            projection, 
            view,
            models,
            model_ptr: 0,
            current_model_ptr: 0,
            making_model: false,
            fill: true,
            z: -1.0 + crate::EPSILON,
            program
        }
    }

    pub fn paint(&mut self) {
        crate::gl_disable_depth();
        if !self.fill {
            crate::gl_nofill();
        }
        self.paint_default();
        crate::gl_enable_depth();
        if !self.fill {
            crate::gl_fill();
        }
    }

    pub fn paint_default(&mut self) {
        let vp = self.projection * self.view;
        self.program.bind();
        self.program.set_mat4f_depricated("uViewProjection", &vp);
        self.program.set_mat4f_array_depricated("uModels", &self.models, self.model_ptr + 1);
        self.painting.subbuffer();
        self.painting.show();
        self.new_painting();
    }

    pub fn new_painting(&mut self) {
        self.painting.clear();
        if self.making_model {
            self.model_ptr = 1;
            let current_model = self.models[MAX_MODEL_MATS - 1];
            self.set_model(&current_model);
        } else {
            self.model_ptr = 0;
        }
    }

    // OPTIONS

    pub fn fill(&mut self) {
        self.fill = true;
    }

    pub fn nofill(&mut self) {
        self.fill = false;
    }

    pub fn set_z(&mut self, val: f32) { 
        use crate::EPSILON;
        if val + EPSILON > 1.0 {
            self.z = 1.0 - EPSILON;
        } else if val - EPSILON < -1.0 {
            self.z = -1.0 + EPSILON;
        } else {
            self.z = val;
        }
    }

    pub fn do_background(&mut self) {
        self.z = -1.0 + crate::EPSILON;
    }

    pub fn do_foreground(&mut self) {
        self.z = 1.0 - crate::EPSILON;
    }

    // COLORS

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
        self.color = Color(red, green, blue, alpha);
    }

    // PROJECTION

    pub fn origin_top_left_projection(&mut self, w: usize, h: usize) {
        self.set_projection(
            &Mat4::ortho(
                0.0, w as f32,
                h as f32, 0.0, 
                -1.0, 1.0
            )
        );
    }

    pub fn origin_middle_projection(&mut self, w: usize, h: usize) {
        let w2 = w as f32 / 2.0;
        let h2 = h as f32 / 2.0;
        let ortho = Mat4::ortho(-w2, w2, -h2, h2, 1.0, -1.0);
        self.set_projection(&ortho);
    }

    pub fn ident_projection(&mut self) {
        self.projection = Mat4::identity();
    }

    pub fn set_projection(&mut self, mat: &Mat4) {
        self.projection = *mat;
    }

    pub fn mult_projection(&mut self, mat: &Mat4) {
        self.projection *= *mat;
    }

    // VIEW

    pub fn ident_view(&mut self) {
        self.view = Mat4::identity();
    }

    pub fn set_view(&mut self, mat: &Mat4) {
        self.view = *mat;
    }

    pub fn mult_view(&mut self, mat: &Mat4) {
        self.view *= *mat;
    }

    pub fn rotate_view(&mut self, a: f32) {
        let rot = Mat4::rotation(a, &crate::E3);
        self.mult_view(&rot);
    }

    // MODEL

    pub fn new_model(&mut self) {
        if self.model_ptr + 1 == MAX_MODEL_MATS {
            self.paint();
        }
        self.model_ptr += 1;
        self.current_model_ptr = self.model_ptr; 
        self.making_model = true;
    }

    pub fn new_clean_model(&mut self) {
        self.new_model();
        self.ident_model();
    }

    pub fn new_similar_model(&mut self) {
        let prev_model = self.get_model();
        self.new_model();
        self.set_model(&prev_model);
    }

    pub fn end_modelling(&mut self) {
        self.current_model_ptr = 0;
        self.making_model = false;
    }

    pub fn ident_model(&mut self) {
        self.models[self.model_ptr] = Mat4::identity();
    }

    pub fn rotate_model(&mut self, a: f32) {
        let rot = Mat4::rotation(a, &crate::E3);
        self.mult_model(&rot);
    }

    pub fn translate_model(&mut self, dx: Float, dy: Float) {
        let mat = Mat4::translate(dx, dy, 0.0);
        self.mult_model(&mat);
    }

    pub fn set_model(&mut self, m: &Mat4) {
        self.models[self.model_ptr] = *m;
    }

    pub fn get_model(&self) -> Mat4 {
        self.models[self.model_ptr]
    }
 
    pub fn mult_model(&mut self, m: &Mat4) {
        self.models[self.model_ptr] *= *m;
    }

    // SHAPES 

    fn make_vertex(&self, x: Float, y: Float) -> DepricPaintingVertex {
        DepricPaintingVertex {
            pos: Vec3(x, y, self.z), 
            col: self.color, 
            model: self.current_model_ptr as f32
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
        let mut vfan: Vec<DepricPaintingVertex> = Vec::with_capacity(fan.len());
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

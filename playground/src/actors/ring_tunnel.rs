use peppaint::{
    Actor,
    MyContext,
    Mesh,
    types::*,
    vertex::*,
    constants::*,
    ShaderProgram,
};

impl Actor for RingTunnel {
    fn start_acting(&mut self, ctx: &mut MyContext) {
        self.f = ctx.get_route();
    }
    fn act(&mut self, ctx: &mut MyContext) {
        self.update(ctx.time() as f32);
        self.paint(&ctx.get_vp());
    }
}

#[allow(dead_code)]
pub struct RingTunnel {
    tunnelpieces: Vec<RingTunnelPiece>, 
    f: fn(f32) -> Vec3,
    final_t: f32, 
    step_t: f32,
    p: Box<ShaderProgram>
}

impl RingTunnel {

    pub fn ne_def(f: fn(f32) -> Vec3) -> Self {
        Self::from_function(f, 0.2, 50, Vec3(2.0, 2.0, 0.5))
    }

    pub fn from_function(
        f: fn(f32) -> Vec3, 
        step_size: f32, 
        n: usize, 
        size: Vec3
    ) -> Self {

        let mut p = Box::new(ShaderProgram::from_frag_and_vert_src(
            include_str!("../shaders/ringtunnel.frag"),
            include_str!("../shaders/ringtunnel.vert")
        ).unwrap());

        let mut tunnelpieces: Vec<RingTunnelPiece> = vec![];

        for i in 0..n {
            let t = step_size * (i as f32);
            let pos = f(t);
            let heading = f(t + EPSILON) - pos;
            let piece = RingTunnelPiece::new(
                60,
                0.1,
                heading,
                pos,
                Vec4(0.2 * (i as f32) % 1.0, 0.2, 0.6 * (i as f32) % 1.0, 1.0),
                size,
                t,
                &mut *p
            );
            tunnelpieces.push(piece);
        };

        let final_t = step_size * (n as f32);

        Self { 
            tunnelpieces,
            f, 
            final_t, 
            step_t: step_size,
            p
        }
    }

    pub fn paint(&self, vp: &Mat4) {
        for piece in &self.tunnelpieces {
            piece.paint(vp);
        }
    }

    pub fn update(&mut self, t: f32) {
        let len = self.tunnelpieces.len() as f32;
        for piece in &mut self.tunnelpieces {
            if piece.toc < (t - len / 2.0 * self.step_t) {
                piece.position = (self.f)(self.final_t);
                piece.heading = (self.f)(self.final_t + EPSILON);
                piece.toc = self.final_t;
                self.final_t += self.step_t;
            }
            piece.size.2 = 8.0 * t.sin();
        }
    }
}

struct RingTunnelPiece {
    mesh: Mesh<TunnelVertex>, 
    pub heading: Vec3,
    pub position: Vec3,
    pub color: Vec4,
    pub size: Vec3,
    pub toc: f32,
}

impl RingTunnelPiece {

    pub fn new(
        precision: usize, 
        width: f32,
        heading: Vec3,
        position: Vec3,
        color: Vec4,
        size: Vec3,
        toc: f32,
        p: &mut ShaderProgram,
    ) -> Self {
        
        let mut mesh = Mesh::<TunnelVertex>::empty();
        mesh.set_program(p);

        let mut rotation: f32 = 0.0;
        let rotation_step: f32 = TWO_PI / (precision as f32);

        let z1 = - width / 2.0;
        let z2 = width / 2.0;
        for _ in 0..precision {

            let x1 = 0.5 * rotation.cos();
            let y1 = 0.5 * rotation.sin();
            rotation += rotation_step;
            let x2 = 0.5 * rotation.cos();
            let y2 = 0.5 * rotation.sin();
            
            mesh.push_quad(
                &TunnelVertex {
                    pos: Vec3(x1, y1, z2)
                },
                &TunnelVertex {
                    pos: Vec3(x1, y1, z1)
                },
                &TunnelVertex {
                    pos: Vec3(x2, y2, z1)
                },
                &TunnelVertex {
                    pos: Vec3(x2, y2, z2)
                }
            );
        }

        mesh.buffer();

        Self { 
            mesh,
            heading,
            position,
            color, 
            size, 
            toc
        }
        
    }

    pub fn paint(&self, vp: &Mat4) {
        let model = self.calc_model();
        let mvp = (*vp) * model;
        self.mesh.bind();
        self.mesh.get_program().set_mat4f_depricated(
            "uMVP", 
            &mvp
        );
        self.mesh.get_program().set_vec4f_depricated(
            "uColor",
            &self.color
        );
        self.mesh.show();
    }

    fn calc_model(&self) -> Mat4 {

        // rotation
        let rot = Mat4::rotation_from_heading(&E3, &self.heading);

        // translation and scaling
        let trans = Mat4::translate(self.position.0, self.position.1, self.position.2);
        let scale = Mat4::scale(self.size.0, self.size.1, self.size.2);

        let model = trans * rot * scale;
        model
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct TunnelVertex {
    pub pos: Vec3
}

impl Vertex for TunnelVertex {
    fn get_layout() -> VertexLayout {
        let stride = 12;
        let al1 = AttributeLayout {
            location: 0,
            n_elements: 3,
            type_: peppaint::Type::Float,
            byte_offset: 0
        };
        let attrib_layouts = vec![al1];
        VertexLayout {
            stride,
            attrib_layouts
        }
    }
}
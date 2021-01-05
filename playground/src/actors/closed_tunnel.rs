use peppaint::{
    Mesh,
    types::*,
    vertex::*,
    constants::*,
    Actor,
    MyContext,
    ShaderProgram,
    std::vertices::PosColor
};

impl Actor for ClosedTunnel {
    fn act(&mut self, ctx: &mut MyContext) {
        self.update(ctx.time() as f32);
        self.show(&ctx.get_vp());
    }
}

pub struct ClosedTunnel {
    parts: Vec<ClosedTunnelPart>,
    dt: f32, 
    t_mid: f32, 
    f: fn(f32) -> Vec3, 
    radius: f32,
    p: Box<ShaderProgram>
}

impl ClosedTunnel {

    pub fn ne_def(f: fn(f32) -> Vec3) -> Self {
        Self::threepiece(f, 2.0, 0.5)
    }
 
    pub fn threepiece(f: fn(f32) -> Vec3, dt_part: f32, radius: f32) -> Self {

        let mut p = Box::new(peppaint::std::shaders::pos_color());

        let mut parts: Vec<ClosedTunnelPart> = vec![];
        let mut start = 0.0;
        for _ in 0..3 {
            let end = start + dt_part;
            let part = ClosedTunnelPart::from_function(
                f, 
                start, 
                end, 
                0.05, 
                50, 
                radius,
                &mut *p
            );
            parts.push(part);
            start = end;
        }

        let t_mid = parts[1].tstart + dt_part / 2.0;

        Self {
            parts, 
            dt: dt_part,
            t_mid,
            f, 
            radius,
            p,
        }
    }

    pub fn update(&mut self, t: f32) {
        if t > self.t_mid {
            let new_part = ClosedTunnelPart::from_function(
                self.f, 
                self.parts[2].tend, self.parts[2].tend + self.dt, 
                0.1, 
                50, 
                self.radius,
                &mut self.p
            );
            self.t_mid = self.parts[1].tend + self.dt / 2.0;
            self.parts.push(new_part);
            self.parts.remove(0);
        }
    }

    pub fn show(&mut self, vp: &Mat4) {
        self.p.bind();
        self.p.set_mat4f_depricated("uVP", vp);
        for part in &self.parts {
            part.show();
        }
    }
}

pub struct ClosedTunnelPart {
    pub mesh: Mesh<PosColor>, 
    pub tstart: f32,
    pub tend: f32,
}


impl ClosedTunnelPart {
    pub fn from_function(
        f: fn(f32) -> Vec3,
        start: f32,
        end: f32,
        step: f32, 
        prec: usize,
        radius: f32,
        p: &mut ShaderProgram
    ) -> Self {

        let mut mesh = Mesh::<PosColor>::empty();
        mesh.set_program(p);

        // create circle of points around z_axis at origin
        let mut circle: Vec<Vec4> = vec![];
        let rotation_step = TWO_PI / (prec as f32);
        let mut total_rotation: f32 = 0.0;
        for _ in 0..prec {
            let x = radius * total_rotation.cos();
            let y = radius * total_rotation.sin();
            let v = Vec4(x, y, 0.0, 1.0);
            circle.push(v);
            total_rotation += rotation_step;
        }

        // create circles around function and connect them
        // create the first one and then loop
        let mut t = start;
        
        let mut pos = f(t);
        let mut vel = f(t + EPSILON) - pos;

        // get model matrix
        let mut model = Mat4::rotation_from_heading(&E3, &vel);
        (model.0).3 = pos.0; // transformation
        (model.1).3 = pos.1;
        (model.2).3 = pos.2;

        // move circle to right position
        let mut circle_at_t: Vec<Vec3> = vec![];
        for point in &circle {
            let moved_point = Mat4::mult_vec4(&model, &point).eucl();
            circle_at_t.push(moved_point);
        }
        circle_at_t.push(circle_at_t[0]); // makes sure the last quad is also made

        while t < end {
            t += step;
            pos = f(t);
            vel = f(t + EPSILON) - pos;

            // get model matrix
            let mut model = Mat4::rotation_from_heading(&E3, &vel);
            (model.0).3 = pos.0; // transformation
            (model.1).3 = pos.1;
            (model.2).3 = pos.2;

            // move circle to right position
            let mut circle_at_ta: Vec<Vec3> = vec![];
            for point in &circle {
                let moved_point = Mat4::mult_vec4(&model, &point).eucl();
                circle_at_ta.push(moved_point);
            }

            circle_at_ta.push(circle_at_ta[0]); // makes sure the last quad is also made

            // connect the two
            for i in 1..circle_at_t.len() {
                let ia = (i as f32) / 1.3;
                let v1 = circle_at_t[i - 1];
                let v2 = circle_at_t[i];
                let va1 = circle_at_ta[i - 1];
                let va2 = circle_at_ta[i];
                let color = Color((t.sin() * ia.sin() + 1.0) / 2.0, (ia.cos() + 1.0) / 2.0, 0.5, 1.0);
                mesh.push_quad(
                    &PosColor { pos: v1, color }, 
                    &PosColor { pos: v2, color }, 
                    &PosColor { pos: va2, color }, 
                    &PosColor { pos: va1, color }
                );
            }

            circle_at_t = circle_at_ta;
        }

        mesh.buffer();

        Self { mesh, tend: t, tstart: start }

    }

    pub fn show(&self) {
        self.mesh.bind();
        self.mesh.show();
    }
}
use peppaint::{
    Float,
    PainterDepricated,
    Actor,
    MyContext
};
use perlin_noise::PerlinNoise;

pub struct StroboSquare {
    pub x: Float,
    pub y: Float,
    pub w: Float,
    pub a: Float,
    pub b: Float
}

impl StroboSquare {
    pub fn ne_def() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 0.5,
            a: 800.0,
            b: 1.0
        }
    }
    pub fn paint(&mut self, p: &mut PainterDepricated) {
        p.ident_projection();
        p.grey(1.0);
        p.square(self.x, self.y, self.w);
        p.paint();
    }
    pub fn update(&mut self, noise: &PerlinNoise, mut time: f32) {
        time *= 100.0;
        let n = noise.get((time/1379.0) as f64) as f32;
        self.x = 0.5 * (n * 2.0 - 1.0);
        let n = noise.get((time/1892.0) as f64) as f32;
        self.y = 0.8 * (n * 2.0 - 1.0);
        self.w = (time/self.a).sin() + self.b;

        let mut pow = 0.98;
        if time as usize % 6000 > 3000 { pow = 1.0/pow; }
        self.a = self.a.powf(pow);
        self.b = 0.3 * (time/300.0).cos();
    }
}

impl Actor for StroboSquare {
    fn act(&mut self, ctx: &mut MyContext) {
        self.update(&ctx.pnoise, ctx.real_time() as f32);
        self.paint(&mut ctx.painter);
    }
}
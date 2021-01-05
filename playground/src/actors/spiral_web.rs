use peppaint::{
    Actor,
    MyContext
};

pub struct SpiralWeb {
    timefac: f32, 
    linedis: f32,
    linewidth: f32,
    linelenfac: f32,
    n_lines: usize
}

impl SpiralWeb {
    pub fn ne_def() -> Self {
        let timefac = 0.005;
        let linedis = 2.0;
        let linewidth = 2.0;
        let n_lines = 300;
        let linelenfac = 3.0;
        Self { 
            timefac, linedis,
            n_lines, linewidth,
            linelenfac
        }
    }
}

impl Actor for SpiralWeb {
    fn act(&mut self, ctx: &mut MyContext) {
        ctx.painter.origin_middle_projection(ctx.window.get_width(), ctx.window.get_height());

        let linelen = self.linelenfac * ctx.get_width();

        let time = ctx.time() as f32 * self.timefac; 

        peppaint::gl_check_error();
        peppaint::gl_flush_error();
        
        ctx.painter.color(0.9, 0.8, 0.0);

        for i in 0..self.n_lines {
            let ia = i as f32;
            ctx.painter.new_similar_model();
            ctx.painter.rotate_model(time.sin() * peppaint::PI);
            ctx.painter.line(-linelen, ia * self.linedis, linelen, ia * self.linedis, self.linewidth);
        }

        ctx.painter.end_modelling();
        ctx.painter.paint();
    }
}
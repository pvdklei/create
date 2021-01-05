use peppaint::{
    MyContext,
    Movie,
};

pub struct SpiralThing {
    square: crate::StroboSquare,
    spiral: crate::SpiralWeb
}

impl Movie for SpiralThing {

    fn setup(ctx: &mut MyContext) -> Self {

        ctx.sign(500, 300, "yoooo");
        ctx.window.set_background(1.0, 1.0, 0.7);
        ctx.set_framerate(60);

        Self { 
            square: crate::StroboSquare::ne_def(),
            spiral: crate::SpiralWeb::ne_def()
        }
    }

    fn show(&mut self, ctx: &mut MyContext) {

        use peppaint::Actor;
        self.spiral.act(ctx);
        self.square.act(ctx);
    }
}

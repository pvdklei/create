
use peppaint::Movie;
use peppaint::MyContext; 
use crate::StroboSquare;

pub struct SquareAdjuster {
    square: StroboSquare,
}

impl Movie for SquareAdjuster {

    fn setup(_ctx: &mut MyContext) -> Self {
        Self { 
            square: StroboSquare::ne_def()
        }
    }

    fn show(&mut self, ctx: &mut MyContext) {
        use peppaint::Actor;
        self.square.act(ctx);
    }

}
use crate::MyContext;

pub trait Actor {
    fn start_acting(&mut self, _ctx: &mut MyContext) {}
    fn stop_acting(&mut self, _ctx: &mut MyContext) {}
    fn act(&mut self, ctx: &mut MyContext);
}
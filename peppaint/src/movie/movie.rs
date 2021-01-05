use crate::{ MyContext, Key };
use std::time::SystemTime;
use std::thread::sleep;
use std::time::Duration;

pub struct MoviePlayer {}
    
impl MoviePlayer {

    /* 
    Usage: 
    MoviePlayer::play::<YourMovieStruct>(); 
    */
    pub fn play_debug<T: Movie>() {

        println!("Started movie");

        let mut ctx = MyContext::create();
        let mut movie = T::setup(&mut ctx);

        println!("Setted up");
        let mut time = SystemTime::now();

        while movie.is_playing() {

            get_fr! {  

                movie.update(&mut ctx);
                movie.show(&mut ctx);

                ctx.window.show();
                ctx.timestep();
                if ctx.window.should_close() ||
                ctx.is_key_pressed(Key::Escape) {
                    break;
                }

            }

            let dt = ctx.dt();
            let elapsed = time.elapsed().unwrap().as_secs_f64();
            if dt > elapsed {
                let time_to_sleep = ctx.dt() - elapsed;
                sleep(Duration::from_secs_f64(time_to_sleep))
            }
            time = SystemTime::now();
        }
    }
}

pub trait Movie {
    fn setup(ctx: &mut MyContext) -> Self;
    fn update(&mut self, _ctx: &mut MyContext) {}
    fn show(&mut self, _ctx: &mut MyContext) {}
    fn is_playing(&self) -> bool { true }
}


extern crate sdl2;
#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate stdweb;
extern crate num;

#[cfg(target_os = "emscripten")]
mod emscripten;
mod utils;
mod app;
mod mandelbrot;

use app::App;
use sdl2::ttf;

#[cfg(target_os = "emscripten")]
fn main() {
    stdweb::initialize();

    js! {
        Module.startApp = function() {
            @{start}();
        };
    }
}

#[cfg(not(target_os = "emscripten"))]
fn main() {
    start();
}

fn start() {
    let ctx = sdl2::init().unwrap();
    let ttf_context = ttf::init().unwrap();

    let mut app = App::new(&ctx, &ttf_context);
    app.start();

    #[cfg(target_os = "emscripten")]
    stdweb::event_loop();
}


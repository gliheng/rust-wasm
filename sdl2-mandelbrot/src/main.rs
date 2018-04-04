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

fn main() {
    #[cfg(target_os = "emscripten")]
    stdweb::initialize();

    let mut app = App::new();
    app.start();

    #[cfg(target_os = "emscripten")]
    stdweb::event_loop();
}

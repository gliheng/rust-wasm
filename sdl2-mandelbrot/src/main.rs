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

#[cfg(target_os = "emscripten")]
fn main() {
    stdweb::initialize();

    fn start() {
        let mut app = App::new();
        app.start();
        stdweb::event_loop();
    }

    js! {
        Module.startApp = function() {
            @{start}();
        };
    }
}

#[cfg(not(target_os = "emscripten"))]
fn main() {
    let mut app = App::new();
    app.start();
}

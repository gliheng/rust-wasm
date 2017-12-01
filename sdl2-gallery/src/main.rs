extern crate sdl2;
#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate stdweb;

use std::process;
use std::thread::sleep;
use std::time::{Instant, Duration};
use stdweb::web::ArrayBuffer;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;

#[cfg(not(target_os = "emscripten"))]
use sdl2::gfx::primitives::DrawRenderer;

#[cfg(target_os = "emscripten")]
mod emscripten;
mod frame_rate;
mod utils;

use frame_rate::FrameRate;


const FRAME_TIME: u32 = 1_000_000_000 / 60;
fn main() {
    #[cfg(target_os = "emscripten")]
    stdweb::initialize();


    utils::fetch("http://localhost:8000", |buf: ArrayBuffer| {
        println!("callback called {}", buf.len());
    });


    let (width, height) = utils::get_window_dimensiton();

    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();
    // TODO set dpi
    // println!("dpi {:?}", video.display_dpi(0));

    // Enable anti-aliasing
    let gl_attr = video.gl_attr();
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let window  = match video
        .window("wasm-demo", width, height)
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };

    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();

    let white = Color::RGB(255, 255, 255);
    let black = Color::RGB(0, 0, 0);
    let green = Color::RGB(0, 255, 0);

    let mut events = ctx.event_pump().unwrap();

    let mut frame_rate = FrameRate::new(100);
    let mut main_loop = || {
        frame_rate.tick();

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                _ => {}
            }
        }

        canvas.set_draw_color(black);
        canvas.clear();

        #[cfg(not(target_os = "emscripten"))]
        let _ = canvas.string(10, 10, frame_rate.mean().to_string().as_str(), green);

        canvas.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    {
        let frame_time = Duration::new(0, FRAME_TIME);
        loop {
            let frame_start = Instant::now();

            main_loop();

            let draw_time = Instant::now().duration_since(frame_start);
            if frame_time > draw_time {
                // framerate control
                sleep(frame_time - draw_time);
            }
        }
    }
}

#[macro_use]
extern crate stdweb;
extern crate sdl2;

use std::process;
use std::thread::sleep;
use std::time::{Instant, Duration};
use stdweb::web::ArrayBuffer;
use stdweb::web::TypedArray;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;
use sdl2::rwops::RWops;
use sdl2::image::ImageRWops;

use sdl2::surface::Surface;

mod emscripten;
mod frame_rate;
mod utils;

use frame_rate::FrameRate;


const FRAME_TIME: u32 = 1_000_000_000 / 60;
fn main() {
    stdweb::initialize();

    utils::fetch("http://localhost:8000/img/img1.jpg", |buf: TypedArray<u8>| {
        let data = buf.to_vec();
        println!("callback called {}", data.len());
        let mut rw = RWops::from_bytes(data.as_slice()).unwrap();
        Surface::load_bmp_rw(&mut rw);
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

        // let _ = canvas.string(10, 10, frame_rate.mean().to_string().as_str(), green);

        canvas.present();
    };

    use emscripten::{emscripten};
    emscripten::set_main_loop_callback(main_loop);
}

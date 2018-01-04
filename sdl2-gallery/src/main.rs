#[macro_use]
extern crate stdweb;
extern crate sdl2;

mod emscripten;
mod frame_rate;
mod utils;
mod display;

use std::process;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use display::{Image, Scene};
use frame_rate::FrameRate;

const FRAME_TIME: u32 = 1_000_000_000 / 60;
fn main() {
    use emscripten::{emscripten};

    stdweb::initialize();

    let ctx = sdl2::init().unwrap();
    let (width, height) = utils::get_window_dimensiton();
    let video = ctx.video().unwrap();

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

    let black = Color::RGB(0, 0, 0);

    let mut events = ctx.event_pump().unwrap();
    let mut scene = Scene::new(canvas.texture_creator());
    scene.add(Image::new_with_dimension("img/img1.jpg".to_string(), 0, 0, 200, 200));
    scene.add(Image::new_with_dimension("img/img2.jpg".to_string(), 200, 0, 200, 200));
    scene.add(Image::new_with_dimension("img/img3.jpg".to_string(), 0, 200, 200, 200));
    scene.add(Image::new_with_dimension("img/img4.jpg".to_string(), 200, 200, 200, 200));

    let mut frame_rate = FrameRate::new(100);
    let main_loop = || {
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
        scene.render(&mut canvas);
        canvas.present();
    };

    emscripten::set_main_loop_callback(main_loop);
}

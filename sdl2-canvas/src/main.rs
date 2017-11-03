extern crate sdl2;

use std::process;
use std::io::Write;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::{Point, Rect};
use sdl2::keyboard::Keycode;


#[cfg(target_os = "emscripten")]
pub mod emscripten;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    // Enable anti-aliasing
    let gl_attr = video_ctx.gl_attr();
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let window  = match video_ctx
        .window("wasm-demo", 640, 480)
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

    let mut rect = Rect::new(10, 10, 50, 50);

    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    let mut events = ctx.event_pump().unwrap();

    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    rect.x -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    rect.x += 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    rect.y -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    rect.y += 10;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(black);
        canvas.clear();
        canvas.set_draw_color(white);
        canvas.fill_rect(rect);
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.draw_line(Point::new(150, 10), Point::new(250, 110));
        canvas.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}
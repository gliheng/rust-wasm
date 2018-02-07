#[macro_use]
extern crate stdweb;
extern crate sdl2;

mod utils;
mod frame_rate;
mod emscripten;

use std::process;
use stdweb::unstable::TryInto;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use frame_rate::FrameRate;
use utils::glyph_renderer::GlyphRenderer;

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
        .window("wasm gallery", width, height)
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

    let mut glyph_renderer = GlyphRenderer::new(canvas.texture_creator(), font, Color::RGB(0, 255, 0));
    let mut frame_rate = FrameRate::new();
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

        // render framerate
        let n = frame_rate.mean().to_string();
        glyph_renderer.render(&mut canvas, &n, 0, 0);

        canvas.present();
    };

    emscripten::set_main_loop_callback(main_loop);
}

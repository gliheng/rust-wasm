extern crate sdl2;

use std::process;
use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::{Point, Rect};
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::gfx::primitives::DrawRenderer;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

fn main() {
    // sdl2_image::init();
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();

    // Enable anti-aliasing
    let gl_attr = video.gl_attr();
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let window  = match video
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

    let tc = canvas.texture_creator();
    let texture = tc.load_texture(Path::new("icon.png")).expect("Cannot load image");

    let black = Color::RGB(0, 0, 0);
    let white = Color::RGB(255, 255, 255);
    let green = Color::RGB(0, 255, 0);
    let yellow = Color::RGB(255, 255, 0);
    let red = Color::RGB(255, 0, 0);
    let cyon = Color::RGB(0, 255, 255);
    let purple = Color::RGB(255, 0, 255);

    let mut events = ctx.event_pump().unwrap();

    let mut main_loop = || {
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
        canvas.set_draw_color(white);
        canvas.fill_rect(Rect::new(10, 10, 50, 50));
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        let _ = canvas.string(300, 200, "hello", green);
        let _ = canvas.rounded_rectangle(10, 10, 200, 200, 10, green);
        let _ = canvas.aa_ellipse(400, 400, 100, 40, green);
        let _ = canvas.thick_line(300, 300, 400, 400, 5, yellow);
        let _ = canvas.aa_polygon(&[20, 40, 60, 80, 100, 120, 3],
                                  &[100, 200, 100, 200, 100, 300, 300],
                                  yellow);
        let _ = canvas.hline(400, 600, 150, cyon);
        let _ = canvas.aa_line(150, 10, 250, 110, red);
        let _ = canvas.copy(&texture, None, Rect::new(70, 10, 50, 50));
        canvas.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}

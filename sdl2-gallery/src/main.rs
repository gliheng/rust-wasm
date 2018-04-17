#![recursion_limit="128"]

#[macro_use]
extern crate stdweb;
extern crate sdl2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod emscripten;
mod frame_rate;
mod utils;
mod display;
mod model;
mod view;
mod transition;
mod config;
mod gesture;
mod actions;

use std::process;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Rect;
use model::Gallery;
use view::{GalleryView, Preview};
use display::{Stage, Display};
use std::rc::Rc;
use config::{Config};
use sdl2::image::{self, INIT_JPG, INIT_PNG};
#[cfg(feature = "fps")]
use frame_rate::FrameRate;
#[cfg(feature = "fps")]
use utils::glyph_renderer::GlyphRenderer;
#[cfg(feature = "fps")]
use sdl2::ttf;

fn main() {
    use emscripten::{emscripten};
    stdweb::initialize();

    let gallery: &Gallery = Config::get_gallery().expect("Cannot find gallery config");

    let ctx = sdl2::init().unwrap();
    let _ = image::init(INIT_PNG | INIT_JPG).unwrap();

    let width = *Config::get_u32("width").unwrap();
    let height = *Config::get_u32("height").unwrap();
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

    let stage = Stage::new(canvas.texture_creator());
    {
        let mut s = stage.borrow_mut();
        s.add_scene("gallery", GalleryView::new(stage.clone()));
        s.add_scene("preview", Preview::new(stage.clone()));
        s.start("gallery");
    }

    #[cfg(feature = "fps")]
    let ttf_context = ttf::init().unwrap();
    #[cfg(feature = "fps")]
    let mut glyph_renderer = None;
    #[cfg(feature = "fps")]
    match ttf_context.load_font("./assets/Supermercado-Regular.ttf", 50) {
        Ok(font) => {
            let mut g = GlyphRenderer::new(canvas.texture_creator(), font, Color::RGB(0, 255, 0));
            glyph_renderer = Some(g);
        },
        Err(e) => {
            println!("Cannot load font {:?}", e);
        },
    }

    #[cfg(feature = "fps")]
    let mut frame_rate = FrameRate::new();

    let main_loop = || {
        #[cfg(feature = "fps")]
        frame_rate.tick();

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                _ => {}
            }
            stage.borrow_mut().handle_events(&event);
        }
        canvas.set_draw_color(black);
        canvas.clear();
        stage.borrow().update();
        stage.borrow().render(&mut canvas, Rect::new(0, 0, width, height));

        // render framerate
        #[cfg(feature = "fps")]
        {
            if let Some(ref mut r) = glyph_renderer {
                let n = frame_rate.mean().to_string();
                r.render(&mut canvas, &n, 0, 0);
            }
        }
        canvas.present();
    };

    emscripten::set_main_loop_callback(main_loop);
}

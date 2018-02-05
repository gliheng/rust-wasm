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

use std::process;
use stdweb::unstable::TryInto;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Rect;
use sdl2::ttf;
use model::Gallery;
use view::GalleryView;
use display::{Scene, Display};
use std::rc::Rc;
use frame_rate::FrameRate;
use utils::glyph_renderer::GlyphRenderer;
use config::Config;

fn main() {
    use emscripten::{emscripten};
    stdweb::initialize();

    let gallery = js! {
        return Module.gallery;
    };
    let gallery: Gallery = gallery.try_into().unwrap();
    println!("{:?}", gallery);

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
    let scene = Scene::new(canvas.texture_creator());

    let mut config = Config::get_instance();
    if let Ok(mut c) = config.write() {
        c.set("gallery", &gallery);
        c.set("width", &width);
        c.set("height", &height);
    }

    scene.borrow_mut().add_child(GalleryView::new(scene.clone()));

    let ttf_context = ttf::init().unwrap();
    let mut glyph_renderer = None;
    match ttf_context.load_font("./assets/Supermercado-Regular.ttf", 50) {
        Ok(font) => {
            let mut g = GlyphRenderer::new(canvas.texture_creator(), font, Color::RGB(0, 255, 0));
            glyph_renderer = Some(g);
        },
        Err(e) => {
            println!("Cannot load font {:?}", e);
        },
    }

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
            scene.borrow_mut().handle_events(&event);
        }
        canvas.set_draw_color(black);
        canvas.clear();
        scene.borrow().update();
        scene.borrow().render(&mut canvas, Rect::new(0, 0, width, height));

        // render framerate
        if let Some(ref mut r) = glyph_renderer {
            let n = frame_rate.mean().to_string();
            r.render(&mut canvas, &n, 0, 0);
        }
        canvas.present();
    };

    emscripten::set_main_loop_callback(main_loop);
}

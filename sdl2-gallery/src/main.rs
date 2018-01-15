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

use std::process;
use stdweb::unstable::TryInto;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Rect;
use model::Gallery;
use view::GalleryView;
use display::{Scene, Display};
use frame_rate::FrameRate;


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
    let green = Color::RGB(0, 255, 0);

    let mut events = ctx.event_pump().unwrap();
    let mut scene = Scene::new(canvas.texture_creator());

    scene.add(GalleryView::new(gallery, width, height));

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
            scene.handle_events(&event);
        }
        canvas.set_draw_color(black);
        canvas.clear();
        scene.render(&mut canvas, Rect::new(0, 0, width, height));
        let _ = canvas.string(10, 10, &frame_rate.get().to_string(), green);
        canvas.present();
    };

    emscripten::set_main_loop_callback(main_loop);
}

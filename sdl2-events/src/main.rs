extern crate sdl2;
#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate stdweb;

use std::process;
use std::thread::sleep;
use std::time::{Instant, Duration};

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
mod box_shape;
mod utils;

use frame_rate::FrameRate;
use box_shape::Box;

const FRAME_TIME: u32 = 1_000_000_000 / 60;
fn main() {
    #[cfg(target_os = "emscripten")]
    stdweb::initialize();

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

    let box_w = 100;
    let box_h = 100;
    let mut box0 = Box::from_center(Point::new(width as i32/2, height as i32/2), box_w as u32, box_h as u32, white);
    let mut events = ctx.event_pump().unwrap();

    let mut last_touch_id = None;
    let mut last_click_x = 0;
    let mut last_click_y = 0;
    let mut start_x = 0;
    let mut start_y = 0;
    let mut frame_rate = FrameRate::new(100);
    let mut main_loop = || {
        frame_rate.tick();

        box0.update();
        box0.confine(0, 0, width as i32, height as i32);

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    let x = box0.x() - 10;
                    box0.set_x(x);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    let x = box0.x() + 10;
                    box0.set_x(x);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    let y = box0.y() - 10;
                    box0.set_y(y);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    let y = box0.y() + 10;
                    box0.set_y(y);
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if box0.point_inside(x, y) {
                        box0.set_drag(true);
                        start_x = box0.x();
                        start_y = box0.y();
                    } else {
                        box0.set_drag(false);
                    }
                    last_click_x = x;
                    last_click_y = y;
                },
                Event::MouseMotion { x, y, .. } => {
                    if box0.drag() {
                        box0.move_to(start_x + x - last_click_x, start_y + y - last_click_y);
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                    box0.set_drag(false);
                },
                Event::FingerDown { x, y, touch_id, .. } => {
                    let x = utils::convert(width, x);
                    let y = utils::convert(height, y);
                    if last_touch_id.is_some() {
                        continue;
                    }

                    last_touch_id = Some(touch_id);

                    if box0.point_inside(x, y) {
                        box0.set_drag(true);
                        start_x = box0.x();
                        start_y = box0.y();
                    } else {
                        box0.set_drag(false);
                    }
                    last_click_x = x;
                    last_click_y = y;
                },
                Event::FingerMotion { x, y, .. } => {
                    let x = utils::convert(width, x);
                    let y = utils::convert(height, y);

                    if box0.drag() {
                        box0.move_to(start_x + x - last_click_x, start_y + y - last_click_y);
                    }
                },
                Event::FingerUp {touch_id, .. } => {
                    if last_touch_id == Some(touch_id) {
                        last_touch_id = None;
                        box0.set_drag(false);
                    }
                },

                _ => {}
            }
        }

        canvas.set_draw_color(black);
        canvas.clear();

        box0.render(&mut canvas);

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

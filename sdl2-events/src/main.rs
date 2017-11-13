extern crate sdl2;

use std::process;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;


#[cfg(target_os = "emscripten")]
pub mod emscripten;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    // Enable anti-aliasing
    let gl_attr = video_ctx.gl_attr();
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let window  = match video_ctx
        .window("wasm-demo", WIDTH, HEIGHT)
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


    let white = sdl2::pixels::Color::RGB(255, 255, 255);
    let box_w = 50;
    let box_h = 50;
    let mut box0 = Box::from_center(Point::new(WIDTH as i32/2, HEIGHT as i32/2), box_w as u32, box_h as u32, white);

    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let mut events = ctx.event_pump().unwrap();

    let mut mouse_down = false;
    let mut drag_box = false;
    let mut last_click_x = 0;
    let mut last_click_y = 0;
    let mut start_x = 0;
    let mut start_y = 0;
    let mut main_loop = || {
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
                    mouse_down = true;
                    if box0.point_inside(x, y) {
                        drag_box = true;
                        start_x = box0.x();
                        start_y = box0.y();
                    } else {
                        drag_box = false;
                    }
                    last_click_x = x;
                    last_click_y = y;
                },
                Event::MouseMotion { x, y, .. } => {
                    if mouse_down && drag_box {
                        box0.update(start_x + x - last_click_x, start_y + y - last_click_y);
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, x, y, .. } => {
                    mouse_down = false;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(black);
        canvas.clear();

        box0.render(&mut canvas);

        canvas.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}

struct Box {
    shape: Rect,
    color: Color,
}

impl Box {
    fn new(x: i32, y: i32, width: u32, height: u32, color: Color) -> Box {
        Box{shape: Rect::new(x, y, width, height), color: color}
    }

    fn from_center<P>(center: P, width: u32, height: u32, color: Color) -> Box
        where P: Into<Point> {
        Box{shape: Rect::from_center(center, width, height), color: color}
    }

    fn x(&self) -> i32 {
        self.shape.x()
    }

    fn y(&self) -> i32 {
        self.shape.y()
    }

    fn set_x(&mut self, x: i32) {
        self.shape.set_x(x)
    }

    fn set_y(&mut self, y: i32) {
        self.shape.set_y(y)
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.shape);
    }

    fn update(&mut self, x: i32, y: i32) {
        self.shape.set_x(x);
        self.shape.set_y(y);
    }

    fn point_inside(&self, x: i32, y: i32) -> bool {
        let shape = &self.shape;
        x >= shape.x() && x <= shape.x() + shape.width() as i32
            && y >= shape.y() && y <= shape.y() + shape.height() as i32
    }
}

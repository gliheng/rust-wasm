use std::process;
use std::thread::sleep;
use std::time::{Instant, Duration};
use sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::video::Window;
use utils;
use mandelbrot::{Mandelbrot};

pub struct App {
    canvas: Canvas<Window>,
    events: EventPump,
    mandelbrot: Mandelbrot,
    start_point: Option<Point>,
    drag_point: Option<Point>,
}

impl App {
    pub fn new() -> Self {
        let (width, height) = utils::get_window_dimention();

        let ctx = sdl2::init().unwrap();
        let video = ctx.video().unwrap();

        // Enable anti-aliasing
        let gl_attr = video.gl_attr();
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);

        let window  = match video
            .window("mandelbrot demo", width, height)
            .position_centered()
            .opengl()
            .build() {
                Ok(window) => window,
                Err(err)   => panic!("failed to create window: {}", err)
            };

        let canvas = window
            .into_canvas()
            .build()
            .unwrap();

        let events = ctx.event_pump().unwrap();
        let mandelbrot = Mandelbrot::new(&canvas);

        App {
            canvas, events, mandelbrot,
            start_point: None,
            drag_point: None,
        }
    }

    pub fn start(&mut self) {
        #[cfg(target_os = "emscripten")]
        use emscripten;

        #[cfg(target_os = "emscripten")]
        emscripten::set_main_loop_callback(self.mainloop);

        #[cfg(not(target_os = "emscripten"))]
        {
            loop {
                self.mainloop();
            }
        }
    }

    fn mainloop(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown {keycode: Some(Keycode::Space), ..} => {
                    self.mandelbrot.reset();
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    self.start_point = Some(Point::new(x, y));
                },
                Event::MouseMotion { x, y, .. } => {
                    if self.start_point.is_some() {
                        self.drag_point = Some(Point::new(x, y));
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                    if self.start_point.is_some() && self.drag_point.is_some() {
                        let rect = utils::rect_from_points(self.start_point.as_ref().unwrap(),
                                                           self.drag_point.as_ref().unwrap());
                        self.mandelbrot.update_rect(&rect);
                    }
                    self.start_point = None;
                    self.drag_point = None;
                },
                _ => {}
            }
        }

        let black = Color::RGB(0, 0, 0);
        let green = Color::RGB(0, 255, 0);

        self.canvas.set_draw_color(black);
        self.canvas.clear();
        self.mandelbrot.render(&mut self.canvas);
        if self.start_point.is_some() && self.drag_point.is_some() {
            self.canvas.set_draw_color(green);
            let rect = utils::rect_from_points(self.start_point.as_ref().unwrap(),
                                               self.drag_point.as_ref().unwrap());
            self.canvas.draw_rect(rect);
        }
        self.canvas.present();
    }
}

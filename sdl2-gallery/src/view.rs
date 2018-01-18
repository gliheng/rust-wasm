use display::{Image, Scene};
use model::Gallery;
use display::Display;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::event::Event;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::time::{Duration, Instant};
use std::mem::drop;

enum TransitionState {
    Running,
    AtEnd,
}
pub struct Transition {
    start_time: Instant,
    duration: Duration,
    start_val: i32,
    target_val: i32,
    state: TransitionState,
}

impl Transition {
    pub fn new(start_val: i32, target_val: i32, duration: Duration) -> Transition {
        Transition {
            start_time: Instant::now(),
            start_val,
            target_val,
            duration,
            state: TransitionState::Running,
        }
    }
    pub fn at_end(&self) -> bool {
        match self.state {
            TransitionState::AtEnd => true,
            _ => false,
        }
    }
    pub fn step(&mut self) -> f64 {
        let mut t = Transition::to_f64(self.start_time.elapsed());
        let d = Transition::to_f64(self.duration);
        let b = self.start_val;
        let c = self.target_val - self.start_val;
        if t >= d {
            self.state = TransitionState::AtEnd;
            return self.target_val as f64;
        }
        // using easing fucntions from http://www.gizma.com/easing/
        // linear
        // c as f64 * t / d + b as f64
        // easeOutQuad
        t /= d;
	    return -c as f64 * t * (t-2.) + b as f64;
    }
    fn to_f64(d: Duration) -> f64 {
        d.as_secs() as f64
            + d.subsec_nanos() as f64 * 1e-9
    }
}

pub struct GalleryView {
    parent: Weak<RefCell<Scene>>,
    curr: Rc<RefCell<Display>>,
    next: Rc<RefCell<Display>>,
    width: u32,
    height: u32,
    dragging: bool,
    translate_x: i32,
    transition: Option<Transition>,
}

impl GalleryView {
    pub fn new(parent: Rc<RefCell<Scene>>, config: Gallery, width: u32, height: u32) -> Rc<RefCell<GalleryView>> {
        let mut urls = config.urls.iter();

        let curr = ScrollView::new(Image::new_with_dimension(urls.next().unwrap().to_owned(), width, height));
        curr.borrow_mut().set_rect(0, 0, width, height);

        let next = ScrollView::new(Image::new_with_dimension(urls.next().unwrap().to_owned(), width, height));
        next.borrow_mut().set_rect(0, 0, width, height);

        Rc::new(RefCell::new(GalleryView {
            parent: Rc::downgrade(&parent),
            curr,
            next,
            width,
            height,
            dragging: false,
            translate_x: 0,
            transition: None,
        }))
    }

    fn move_to(&mut self, x: i32, duration: Duration) {
        self.transition = Some(Transition::new(self.translate_x,
                                               x,
                                               duration));
    }
}

impl Display for GalleryView {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        let mut r1 = rect.clone();
        r1.offset(self.translate_x, 0);
        self.curr.borrow().render(canvas, r1);
        let mut r2 = rect.clone();
        r2.offset(self.translate_x + self.width as i32, 0);
        self.next.borrow().render(canvas, r2);
    }
    fn handle_events(&mut self, event: &Event) {
        match event {
            &Event::FingerDown { x, y, touch_id, .. } => {
                self.dragging = true;
                self.transition = None;
            },
            &Event::FingerMotion { x, y, dx, dy, .. } => {
                if self.dragging {
                    self.translate_x += (self.width as f32 * dx) as i32;
                }
            },
            &Event::FingerUp {touch_id, .. } => {
                self.dragging = false;
                let mov = (((self.translate_x as f64 + self.width as f64 / 2.) / self.width as f64).floor() as i32).min(0);
                let target_x = mov * self.width as i32;
                println!("move: {}", mov);
                self.move_to(target_x, Duration::from_millis(300));
            },
            _ => (),
        }
    }
    fn is_interactive(&self) -> bool {
        true
    }
    fn interact(&mut self) {
        let mut in_transition = !self.dragging && self.transition.is_some();
        if in_transition {
            if let Some(ref mut transition) = self.transition {
                self.translate_x = transition.step() as i32;
                if transition.at_end() {
                    in_transition = false;
                }
            }
            if !in_transition {
                self.transition = None;
            }
        }
    }
}

pub struct ScrollView {
    content: Rc<Image>,
    rect: Rect,
}

impl ScrollView {
    fn new(content: Rc<Image>) -> Rc<RefCell<ScrollView>> {
        Rc::new(RefCell::new(ScrollView {
            content,
            rect: Rect::new(0, 0, 0, 0),
        }))
    }

    fn set_rect(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.rect.set_x(x);
        self.rect.set_y(y);
        self.rect.set_width(w);
        self.rect.set_height(h);
    }
}

impl Display for ScrollView {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        self.content.render(canvas, rect);
    }
}

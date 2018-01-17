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


pub struct Transition<T> {
    start_time: Instant,
    duration: Duration,
    curr_val: T,
    target_val: T,
}

pub struct GalleryView {
    parent: Weak<RefCell<Scene>>,
    curr: Rc<RefCell<Display>>,
    next: Rc<RefCell<Display>>,
    width: u32,
    height: u32,
    dragging: bool,
    move_x: i32,
    transition: Option<Transition<i32>>,
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
            move_x: 0,
            transition: None,
        }))
    }

    fn move_to(&mut self, x: i32) {
        self.transition = Some(Transition {
            start_time: Instant::now(),
            duration: Duration::from_millis(2000),
            curr_val: self.move_x,
            target_val: x,
        });
    }
}

impl Display for GalleryView {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        let mut r1 = rect.clone();
        r1.offset(self.move_x, 0);
        self.curr.borrow().render(canvas, r1);
        let mut r2 = rect.clone();
        r2.offset(self.move_x + self.width as i32, 0);
        self.next.borrow().render(canvas, r2);
    }
    fn handle_events(&mut self, event: &Event) {
        match event {
            &Event::FingerDown { x, y, touch_id, .. } => {
                self.dragging = true;
            },
            &Event::FingerMotion { x, y, dx, dy, .. } => {
                if self.dragging {
                    self.move_x += (self.width as f32 * dx) as i32;
                }
            },
            &Event::FingerUp {touch_id, .. } => {
                self.dragging = false;
                self.move_to(0);
            },
            _ => (),
        }
    }
    fn is_interactive(&self) -> bool {
        true
    }
    fn interact(&mut self) {
        if !self.dragging {
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

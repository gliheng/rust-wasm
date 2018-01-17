use display::{Image, Scene};
use model::Gallery;
use display::Display;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::event::Event;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct GalleryView {
    curr: Rc<RefCell<Display>>,
    next: Rc<RefCell<Display>>,
    width: u32,
    height: u32,
}

impl GalleryView {
    pub fn new(config: Gallery, width: u32, height: u32) -> Rc<RefCell<GalleryView>> {
        let mut urls = config.urls.iter();

        let curr = ScrollView::new(Image::new_with_dimension(urls.next().unwrap().to_owned(), width, height));
        curr.borrow_mut().set_rect(0, 0, width, height);

        let next = ScrollView::new(Image::new_with_dimension(urls.next().unwrap().to_owned(), width, height));
        next.borrow_mut().set_rect(0, 0, width, height);

        Rc::new(RefCell::new(GalleryView {
            curr,
            next,
            width,
            height,
        }))
    }
}

impl Display for GalleryView {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        self.curr.borrow().render(canvas, rect.clone());
        self.next.borrow().render(canvas, rect.clone());
    }
    fn handle_events(&mut self, event: &Event) {
        match event {
            &Event::FingerDown { x, y, touch_id, .. } => {
                println!("down");
            },
            &Event::FingerMotion { x, y, .. } => {
            },
            &Event::FingerUp {touch_id, .. } => {
                println!("up");
            },
            _ => (),
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
        if let Some(r) = self.rect.intersection(rect) {
            self.content.render(canvas, r);
        }
    }
}

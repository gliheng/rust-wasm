use display::{Image, Scene};
use model::Gallery;
use display::Display;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::event::Event;
use std::rc::Rc;

pub struct GalleryView {
    curr: Rc<ScrollView>,
    next: Rc<ScrollView>,
    width: u32,
    height: u32,
}

impl GalleryView {
    pub fn new(config: Gallery, width: u32, height: u32) -> Rc<GalleryView> {
        let mut urls = config.urls.iter();

        let mut curr = ScrollView::new(Image::new_with_dimension(urls.next().unwrap().to_owned(), width, height));
        if let Some(mut v) = Rc::get_mut(&mut curr) {
            v.set_rect(0, 0, width, height);
        }

        let mut next = ScrollView::new(Image::new_with_dimension(urls.next().unwrap().to_owned(), width, height));
        if let Some(mut v) = Rc::get_mut(&mut next) {
            v.set_rect(0, 0, width, height);
        }

        Rc::new(GalleryView {
            curr,
            next,
            width,
            height,
        })
    }
}

impl Display for GalleryView {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        self.curr.render(canvas, rect.clone());
        self.next.render(canvas, rect.clone());
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
    fn new(content: Rc<Image>) -> Rc<ScrollView> {
        Rc::new(ScrollView {
            content,
            rect: Rect::new(0, 0, 0, 0),
        })
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

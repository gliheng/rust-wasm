use display::{Image, Scene};
use model::Gallery;
use display::Display;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;

pub struct GalleryView {
    curr: ScrollView,
    next: ScrollView,
    width: u32,
    height: u32,
}

impl GalleryView {
    pub fn new(config: Gallery, width: u32, height: u32) -> GalleryView {
        let mut urls = config.urls.iter();

        let mut curr = ScrollView::new(Image::new_with_dimension(urls.next().unwrap().to_owned(), width, height));
        curr.set_rect(0, 0, width, height);

        let mut next = ScrollView::new(Image::new_with_dimension(urls.next().unwrap().to_owned(), width, height));
        next.set_rect(width as i32, 0, width, height);

        GalleryView {
            curr,
            next,
            width,
            height,
        }
    }
}

impl Display for GalleryView {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        self.curr.render(canvas, rect.clone());
        self.next.render(canvas, rect.clone());
    }
}

pub struct ScrollView {
    content: Image,
    rect: Rect,
}

impl ScrollView {
    fn new(content: Image) -> ScrollView {
        ScrollView {
            content,
            rect: Rect::new(0, 0, 0, 0),
        }
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

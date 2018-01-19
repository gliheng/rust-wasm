use display::{Image, Scene};
use model::Gallery;
use display::Display;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::event::Event;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::time::{Duration};
use std::mem::drop;
use transition::Transition;
use gesture::{GestureDetector, GestureEvent};

const GAP: i32 = 30;

pub struct GalleryView {
    parent: Weak<RefCell<Scene>>,
    prev: Rc<RefCell<ScrollView>>,
    curr: Rc<RefCell<ScrollView>>,
    next: Rc<RefCell<ScrollView>>,
    config: Gallery,
    width: u32,
    height: u32,
    dragging: bool,
    translate_x: i32,
    translate_x_pre: i32,
    img_idx: usize,
    transition: Option<Transition>,
    gesture_detector: GestureDetector,
}

impl GalleryView {
    pub fn new(parent: Rc<RefCell<Scene>>, config: Gallery, width: u32, height: u32) -> Rc<RefCell<GalleryView>> {
        let prev = ScrollView::new(Image::new_with_dimension("".to_owned(), width, height));
        prev.borrow_mut().set_rect(0, 0, width, height);

        let curr = ScrollView::new(Image::new_with_dimension("".to_owned(), width, height));
        curr.borrow_mut().set_rect(0, 0, width, height);

        let next = ScrollView::new(Image::new_with_dimension("".to_owned(), width, height));
        next.borrow_mut().set_rect(0, 0, width, height);

        let mut g = GalleryView {
            parent: Rc::downgrade(&parent),
            prev,
            curr,
            next,
            config,
            width,
            height,
            dragging: false,
            translate_x: 0,
            translate_x_pre: 0,
            img_idx: 0,
            transition: None,
            gesture_detector: GestureDetector::new(),
        };
        g.set_curr_image(0);
        Rc::new(RefCell::new(g))
    }

    fn rotate(&mut self) {
        println!("rotate with translate_x: {}", self.translate_x);
        let p = self.img_idx as isize - 1;
        if self.translate_x > 0 && p >= 0 {
            println!("rotate left");
            self.translate_x -= self.width as i32 + GAP;
            self.set_curr_image(p as usize);
        } else if self.translate_x < 0 && self.img_idx + 1 < self.config.urls.len() {
            println!("rotate right");
            self.translate_x += self.width as i32 + GAP;
            let i = self.img_idx + 1;
            self.set_curr_image(i);
        }
    }

    fn set_curr_image(&mut self, idx: usize) {
        //  set prev scrollview
        let mut scrollview = self.prev.borrow_mut();
        {
            let mut img = scrollview.content.borrow_mut();

            let i = idx as isize - 1;
            if i < 0 {
                img.set_src("");
            } else if let Some(url) = self.config.urls.get(i as usize) {
                println!("set prev {}", i);
                img.set_src(url);
            } else {
                img.set_src("");
            }
        }
        scrollview.set_scale(1.);

        //  set curr scrollview
        let mut scrollview = self.curr.borrow_mut();
        {
            let mut img = scrollview.content.borrow_mut();
            if let Some(url) = self.config.urls.get(idx) {
                img.set_src(url);
                println!("set curr {}", idx);

            } else {
                img.set_src("");
            }
        }
        scrollview.set_scale(1.);

        //  set next scrollview
        let mut scrollview = self.next.borrow_mut();
        {
            let mut img = scrollview.content.borrow_mut();
            if let Some(url) = self.config.urls.get(idx + 1) {
                println!("set next {}", idx+1);
                img.set_src(url);
            } else {
                img.set_src("");
            }
        }
        scrollview.set_scale(1.);

        self.img_idx = idx;
    }

    fn move_to(&mut self, x: i32, duration: Duration) {
        self.transition = Some(Transition::new(self.translate_x,
                                               x,
                                               duration));
    }
}

impl Display for GalleryView {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        if self.translate_x > 0 {
            let mut r0 = rect.clone();
            r0.offset(self.translate_x - self.width as i32 - GAP, 0);
            self.prev.borrow().render(canvas, r0);
        }

        let mut r1 = rect.clone();
        r1.offset(self.translate_x, 0);
        self.curr.borrow().render(canvas, r1);

        if self.translate_x < 0 {
            let mut r2 = rect.clone();
            r2.offset(self.translate_x + self.width as i32 + GAP, 0);
            self.next.borrow().render(canvas, r2);
        }
    }
    fn handle_events(&mut self, event: &Event) {
        {
            let mut scrollview = self.curr.borrow_mut();

            self.gesture_detector.feed(event);
            for event in &self.gesture_detector.poll() {
                match event {
                    &GestureEvent::DoubleTap => {
                        if scrollview.zoom_mode {
                            // exit zoom
                            scrollview.exit_zoom();
                        } else {
                            scrollview.enter_zoom();
                        }
                    },
                    _ => ()
                }
            }

            if scrollview.zoom_mode {
                scrollview.handle_events(event);
                return;
            }

        }

        // handle horizontal move
        match event {
            &Event::FingerDown { x, y, touch_id, .. } => {
                self.dragging = true;
                self.transition = None;
                self.translate_x_pre = self.translate_x;
            },
            &Event::FingerMotion { x, y, dx, dy, .. } => {
                if self.dragging {
                    self.translate_x += (self.width as f32 * dx) as i32;
                }
            },
            &Event::FingerUp {touch_id, .. } => {
                self.dragging = false;
                // move direction: -1 to left, 1 to right, 0 restore
                let delta = self.translate_x - self.translate_x_pre;
                let threshold = 150; // threshold for the move
                let mut mov = if delta > threshold {
                    1
                } else if delta < -threshold {
                    -1
                } else {
                    0
                };

                // avoid invalid move
                if mov == -1 && self.img_idx >= self.config.urls.len() - 1
                    || mov == 1 && self.img_idx == 0 {
                        mov = 0;
                    }
                let target_x = mov * (self.width as i32 + GAP);
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
                    // end transition
                    in_transition = false;
                    // self.transition_x = transition.target_val();
                }
            }
            if !in_transition {
                self.transition = None;
                self.rotate();
            }
        }
    }
}

pub struct ScrollView {
    pub content: Rc<RefCell<Image>>,
    rect: Rect,
    scale: f64,
    offset_x: i32,
    offset_y: i32,
    zoom_mode: bool,
    dragging: bool,
}

impl ScrollView {
    fn new(content: Rc<RefCell<Image>>) -> Rc<RefCell<ScrollView>> {
        Rc::new(RefCell::new(ScrollView {
            content,
            rect: Rect::new(0, 0, 0, 0),
            scale: 1.0,
            offset_x: 0,
            offset_y: 0,
            zoom_mode: false,
            dragging: false,
        }))
    }

    fn enter_zoom(&mut self) {
        self.cover();
        self.zoom_mode = true;
    }

    fn exit_zoom(&mut self) {
        self.contain();
        self.offset_x = 0;
        self.offset_y = 0;
        self.zoom_mode = false;
    }

    fn handle_events(&mut self, event: &Event) {
        // handle drag
        match event {
            &Event::FingerDown { x, y, touch_id, .. } => {
                self.dragging = true;
            },
            &Event::FingerMotion { x, y, dx, dy, .. } => {
                if self.dragging {
                    self.offset_x += (dx * self.rect.width() as f32) as i32;
                    self.offset_y += (dy * self.rect.height() as f32) as i32;
                }
            },
            &Event::FingerUp {touch_id, .. } => {
                self.dragging = false;
            },
            _ => ()
        }
    }

    fn is_interactive(&self) -> bool {
        false
    }

    fn set_rect(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.rect.set_x(x);
        self.rect.set_y(y);
        self.rect.set_width(w);
        self.rect.set_height(h);
    }

    fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
        if scale == 1. {
            self.zoom_mode = false;
        }
    }

    fn contain(&mut self) {
        self.set_scale(1.);
    }

    fn cover(&mut self) {
        let w = self.rect.width();
        let h = self.rect.height();
        let mut r = 2.;
        if let Some((img_w, img_h)) = self.content.borrow().get_img_size() {
            let (w1, _) = Image::cover_size(img_w, img_h, w, h);
            r = w1 as f64 / w as f64;
        }
        self.set_scale(r);
    }
}

impl Display for ScrollView {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        canvas.set_clip_rect(rect);
        let r = Rect::from_center(rect.center().offset(self.offset_x, self.offset_y),
                                  (rect.width() as f64 * self.scale) as u32,
                                  (rect.height() as f64 * self.scale) as u32);
        self.content.borrow().render(canvas, r);
        canvas.set_clip_rect(None);
    }
}

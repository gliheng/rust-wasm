use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::default::Default;
use std::os::raw::{c_void, c_char};
use std::marker::{Send};
use std::ffi::{CString};
use std::sync::Mutex;
use sdl2::video::{Window, WindowContext};
use sdl2::image::{LoadSurface};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::event::Event;
use stdweb::web::TypedArray;
use utils;
use std::sync::Arc;

static mut TEXTURE_CREATOR: Option<TextureCreator<WindowContext>> = None;
lazy_static!{
    static ref LOAD_REGISTER: Mutex<HashMap<String, SizedTexture>> = Mutex::new(HashMap::new());
}

struct SizedTexture(u32, u32, Texture);
unsafe impl Send for SizedTexture {}

pub trait Display {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect);
    fn handle_events(&mut self, event: &Event) {}
    fn is_interactive(&self) -> bool { false }
    fn update(&mut self) {}
}

pub struct Scene {
    children: Vec<Rc<RefCell<Display>>>,
    listeners: Vec<Rc<RefCell<Display>>>,
}

impl Scene {
    pub fn new(tc: TextureCreator<WindowContext>) -> Rc<RefCell<Scene>> {
        unsafe {
            TEXTURE_CREATOR = Some(tc);
        }
        Rc::new(RefCell::new(Scene {
            children: Vec::new(),
            listeners: Vec::new(),
        }))
    }
    pub fn add_child(&mut self, c: Rc<RefCell<Display>>) {
        self.children.push(c.clone());
        if c.borrow().is_interactive() {
            self.listeners.push(c.clone());
        }
    }
    pub fn update(&self) {
        for c in &self.listeners {
            c.borrow_mut().update();
        }
    }
}

impl Display for Scene {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        for c in &self.children {
            c.borrow().render(canvas, rect.clone());
        }
    }
    fn handle_events(&mut self, event: &Event) {
        for c in &self.listeners {
            c.borrow_mut().handle_events(&event);
        }
    }
}

pub struct Image {
    dirty: bool,
    src: String,
    w: u32,
    h: u32,
}

impl Image {
    pub fn new(src: String) -> Rc<RefCell<Image>> {
        if src != "" {
            load_img(&src);
        }

        Rc::new(RefCell::new(Image {
            dirty: false,
            src,
            ..Default::default()
        }))
    }
    pub fn new_with_dimension(src: String, w: u32, h: u32) -> Rc<RefCell<Image>> {
        if src != "" {
            load_img(&src);
        }
        Rc::new(RefCell::new(Image {
            dirty: false,
            src,
            w,
            h,
            ..Default::default()
        }))
    }
    pub fn set_src(&mut self, src: &str) {
        self.src = src.to_string();
        if src != "" {
            load_img(src);
        }
    }
    pub fn get_img_size(&self) -> Option<(u32, u32)> {
        let m = LOAD_REGISTER.lock().unwrap();
        if let Some(&SizedTexture(img_w, img_h, ..)) = m.get(&self.src) {
            Some((img_w, img_h))
        } else {
            None
        }
    }

    pub fn cover_size(img_w: u32, img_h: u32, w: u32, h: u32) -> (u32, u32) {
        let img_r = img_w as f64 / img_h as f64;
        let r = w as f64 / h as f64;
        if img_r > r {
            ((h as f64 * img_r) as u32, h)
        } else {
            (w, (w as f64 / img_r) as u32)
        }
    }

    pub fn contain_size(img_w: u32, img_h: u32, w: u32, h: u32) -> (u32, u32) {
        let img_r = img_w as f64 / img_h as f64;
        let r = w as f64 / h as f64;
        if img_r > r {
            (w, (w as f64 / img_r) as u32)
        } else {
            ((img_r * h as f64) as u32, h)
        }
    }
}

impl Display for Image {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        let m = LOAD_REGISTER.lock().unwrap();
        if let Some(&SizedTexture(img_w, img_h, ref tex)) = m.get(&self.src) {
            let s_rect = Rect::new(0, 0, img_w, img_h);
            let (w, h) = Self::contain_size(img_w, img_h, rect.width(), rect.height());
            let t_rect = Rect::new((rect.width() as i32 - w as i32) / 2 + rect.x(),
                                   (rect.height() as i32 - h as i32) / 2 + rect.y(),
                                   w, h);
            let _ = canvas.copy(tex,
                                s_rect,
                                t_rect);
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            dirty: false,
            src: "".to_string(),
            w: 0,
            h: 0,
        }
    }
}

pub fn load_img(src: &str) {
    // check if already loaded
    let m = LOAD_REGISTER.lock().unwrap();
    if m.get(src).is_some() {
        return;
    }

    let bsrc = Box::into_raw(Box::new(src.to_string()));
    let ext = (match src.rfind('.') {
        Some(i) => &src[i+1..],
        None => "",
    }).to_string();

    utils::fetch(src, move |buf: TypedArray<u8>| {
        use emscripten::{emscripten};
        let v = buf.to_vec();
        unsafe {
            emscripten::emscripten_run_preload_plugins_data(
                v.as_ptr(),
                v.len(),
                CString::new(ext).unwrap().as_ptr(),
                bsrc as *const c_void,
                loaded, load_err
            );
        }
    });
}

extern "C" fn loaded(src: *const c_void, file: *mut c_char) {
    unsafe {
        let mut m = LOAD_REGISTER.lock().unwrap();
        let src = Box::from_raw(src as *mut String);
        let file = CString::from_raw(file).into_string().unwrap();
        if let Ok(surf) = Surface::from_file(file) {
            if let Some(ref tc) = TEXTURE_CREATOR {
                let w = surf.width();
                let h = surf.height();
                let tex = tc.create_texture_from_surface(surf).expect("failed to create texture fron surface");
                m.insert(*src, SizedTexture(w, h, tex));
            } else {
                println!("load err");
            }
        }
    }
}

extern "C" fn load_err(src: *const c_void) {
    unsafe {
        let src = Box::from_raw(src as *mut String);
        println!("load failed! src: {}", src);
    }
}

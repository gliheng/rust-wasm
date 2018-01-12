use std::cell::RefCell;
use std::collections::HashMap;
use std::default::Default;
use std::os::raw::{c_void, c_char};
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
use std::marker::{Send};
use utils;

static mut TEXTURE_CREATOR: Option<TextureCreator<WindowContext>> = None;
lazy_static!{
    static ref LOAD_REGISTER: Mutex<HashMap<String, SizedTexture>> = Mutex::new(HashMap::new());
}

struct SizedTexture(u32, u32, Texture);
unsafe impl Send for SizedTexture {}

pub trait Display {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect);
    // fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32, w: u32, h: u32);
}

pub struct Scene {
    children: Vec<Box<Display>>,
}

impl Scene {
    pub fn new(tc: TextureCreator<WindowContext>, ) -> Scene {
        unsafe {
            TEXTURE_CREATOR = Some(tc);
        }
        Scene {
            children: vec![],
        }
    }
    pub fn add(&mut self, ui: Box<Display>) {
        self.children.push(ui);
    }
    pub fn handle_events(&mut self, event: &Event) {
        
    }
}

impl Display for Scene {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        for c in &self.children {
            c.render(canvas, rect.clone());
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
    pub fn new(src: String) -> Image {
        load_img(&src);
        Image {
            dirty: false,
            src,
            ..Default::default()
        }
    }
    pub fn new_with_dimension(src: String, w: u32, h: u32) -> Image {
        load_img(&src);
        Image {
            dirty: false,
            src,
            w,
            h,
        }
    }
}

impl Display for Image {
    fn render(&self, canvas: &mut Canvas<Window>, rect: Rect) {
        let m = LOAD_REGISTER.lock().unwrap();
        if let Some(&SizedTexture(w, h, ref tex)) = m.get(&self.src) {
            let _ = canvas.copy(tex,
                                Rect::new(0, 0, w, h),
                                rect);
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

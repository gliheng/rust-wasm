use std::cell::RefCell;
use std::collections::HashMap;
use std::default::Default;
use std::os::raw::{c_void, c_char};
use std::ffi::{CString};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{LoadSurface};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use stdweb::web::TypedArray;
use utils;

static mut TEXTURE_CREATOR: Option<TextureCreator<WindowContext>> = None;
thread_local!(static LOAD_REGISTER: RefCell<HashMap<String, (u32, u32, Texture)>> = RefCell::new(HashMap::new()));

pub struct Scene {
    children: Vec<Image>,
}

impl Scene {
    pub fn new(tc: TextureCreator<WindowContext>) -> Scene {
        unsafe {
            TEXTURE_CREATOR = Some(tc);
        }
        Scene {
            children: vec![],
        }
    }
    pub fn add(&mut self, img: Image) {
        self.children.push(img);
    }
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for c in &self.children {
            c.render(canvas);
        }
    }
}

pub struct Display {
}

impl Display {
}

pub struct Image {
    dirty: bool,
    src: String,
    x: i32,
    y: i32,
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
    pub fn new_with_dimension(src: String, x: i32, y: i32, w: u32, h: u32) -> Image {
        load_img(&src);
        Image {
            dirty: false,
            src,
            x,
            y,
            w,
            h,
        }
    }
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        LOAD_REGISTER.with(|m| {
            if let Some(&(w, h, ref tex)) = m.borrow().get(&self.src) {
                let _ = canvas.copy(tex,
                                    Rect::new(0, 0, w, h),
                                    Rect::new(self.x, self.y, self.w, self.h));
            }
        });
    }
}


impl Default for Image {
    fn default() -> Self {
        Self {
            dirty: false,
            src: "".to_string(),
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }
}

pub fn load_img(src: &str) {
    println!("loading! {:?}", src);
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
        LOAD_REGISTER.with(|m| {
            let src = Box::from_raw(src as *mut String);
            let file = CString::from_raw(file).into_string().unwrap();
            println!("loaded: {} {}", src, file);

            if let Ok(surf) = Surface::from_file(file) {
                if let Some(ref tc) = TEXTURE_CREATOR {
                    let w = surf.width();
                    let h = surf.height();
                    let tex = tc.create_texture_from_surface(surf).expect("failed to create texture fron surface");
                    m.borrow_mut().insert(*src, (w, h, tex));
                    println!("set tex!");
                } else {
                    println!("load err");
                }
            }
        });
    }
}

extern "C" fn load_err(src: *const c_void) {
    unsafe {
        let src = Box::from_raw(src as *mut String);
        println!("load failed! src: {}", src);
    }
}

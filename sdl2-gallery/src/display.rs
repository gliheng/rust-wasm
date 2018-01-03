use std::cell::RefCell;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use sdl2::video::{Window, WindowContext};
use std::os::raw::{c_int, c_void, c_char, c_float};
use std::ffi::{CStr, CString};
use sdl2::image::{LoadSurface};
use sdl2::render::{TextureCreator};
use stdweb::web::TypedArray;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use std::collections::HashMap;
use utils;

static mut TEXTURE_CREATOR: Option<TextureCreator<WindowContext>> = None;
static mut TEX: Option<Texture> = None;

static mut LOAD_ID: u32 = 0;
thread_local!(static LOAD_REGISTER: RefCell<HashMap<u32, Box<Fn(String)>>> = RefCell::new(HashMap::new()));

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
}

impl Image {
    pub fn new(src: String) -> Image {
        let mut inst = Image {
            dirty: false,
            src,
        };

        let p: *mut Image = &mut inst;
        utils::fetch(&inst.src, move |buf: TypedArray<u8>| {
            use emscripten::{emscripten};
            let v = buf.to_vec();
            unsafe {
                let id = LOAD_ID;
                LOAD_ID += 1;

                LOAD_REGISTER.with(|m| {
                    m.borrow_mut().insert(id, Box::new(move |filename| {
                        let img = &*p;
                        img.loaded(filename);
                    }));
                });
                emscripten::emscripten_run_preload_plugins_data(
                    v.as_ptr(),
                    v.len(),
                    CString::new("jpg").unwrap().as_ptr(),
                    Box::into_raw(Box::new(id)) as *const c_void,
                    loaded, load_err
                );
            }
        });

        inst
    }
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        unsafe {
            if let Some(ref tex) = TEX {
                let _ = canvas.copy(tex, None, None);
            }
        }
    }
    pub fn loaded(&self, filename: String) {
        if let Ok(surf) = Surface::from_file(filename) {
            unsafe {
                if let Some(ref tc) = TEXTURE_CREATOR {
                    let tex = tc.create_texture_from_surface(surf).expect("failed to create texture fron surface");
                    TEX = Some(tex);
                } else {
                    println!("load err")
                }
            }
        }
    }
}

extern "C" fn loaded(arg: *const c_void, file: *mut c_char) {
    unsafe {
        let p = arg as *mut u32;
        let id = Box::from_raw(p);
        LOAD_REGISTER.with(|m| {
            if let Some(cbk) = m.borrow_mut().remove(&id) {
                cbk(CString::from_raw(file).into_string().unwrap());
            }
        });
    }
}

extern "C" fn load_err(arg: *const c_void) {
    println!("load failed!");
}

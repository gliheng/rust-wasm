use std::collections::HashMap;
use model::Gallery;
use stdweb::unstable::TryInto;
use utils;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

pub struct Config {
    fields: HashMap<&'static str, u32>,
}

impl Config {
    pub fn new() -> Config {
        let mut fields = HashMap::new();
        let (width, height) = utils::get_window_dimention();
        unsafe {
            fields.insert("width", width);
            fields.insert("height", height);
            fields.insert("gallery", Box::into_raw(Box::new(get_gallery())) as u32);
        }

        Config {
            fields
        }
    }
    pub fn get_instance() -> &'static Config {
        &CONFIG
    }
    pub fn get_u32(key: &'static str) -> Option<&'static u32> {
        CONFIG.fields.get(key)
    }
    pub fn get_gallery() -> Option<&'static Gallery> {
        if let Some(&v) = CONFIG.fields.get("gallery") {
            unsafe {
                let g: &Gallery = &*(v as *const Gallery);
                return Some(g);
            }
        }
        None
    }
}

fn get_gallery() -> Gallery {
    let gallery = js! {
        return Module.gallery;
    };
    gallery.try_into().unwrap()
}
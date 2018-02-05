use std::collections::HashMap;
use model::Gallery;
use std::sync::RwLock;
use std::any::Any;
use std::mem::transmute;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::new());
}

pub struct Config {
    fields: HashMap<&'static str, [usize;2]>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            fields: HashMap::new()
        }
    }
    pub fn get_instance() -> &'static RwLock<Config> {
        &CONFIG
    }
    pub fn set(&mut self, key: &'static str, v: &Any) {
        unsafe {
            self.fields.insert(key, transmute::<&Any, [usize;2]>(v));
        }
    }
    pub fn get(&self, key: &'static str) -> Option<&'static Any> {
        match self.fields.get(key) {
            Some(v) => {
                unsafe {
                    let p = transmute::<[usize;2], &Any>(*v);
                    Some(p)
                }
            },
            None => None,
        }
    }
    pub fn get_u32(&self, key: &'static str) -> Option<&u32> {
        if let Some(v) = self.get(key) {
            unsafe {
                return (*v).downcast_ref::<u32>();
            }
        }
        None
    }
    pub fn get_gallery(&self, key: &'static str) -> Option<&Gallery> {
        if let Some(v) = self.get(key) {
            unsafe {
                return (*v).downcast_ref::<Gallery>();
            }
        }
        None
    }
}

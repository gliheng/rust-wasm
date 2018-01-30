#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::os::raw::{c_void, c_int, c_float, c_char, c_double};
extern "C" {
    pub fn emscripten_set_element_css_size(target: *const c_char, width: c_double, height: c_double) -> c_int;
    pub fn emscripten_get_element_css_size(target: *const c_char, width: *mut c_double, height: *mut c_double) -> c_int;
}

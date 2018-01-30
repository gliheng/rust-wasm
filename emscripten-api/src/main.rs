mod emsdk;

use std::os::raw::{c_char, c_int, c_void};
use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;
use emsdk::{emscripten, html5};

fn main() {
    {
        println!("test emscripten_asm_const");

        unsafe {
            let s = concat!("console.log(123)", "\0");
            emscripten::emscripten_asm_const(s as *const _ as *const c_char);

            let s1 = concat!("return $0 * 10;", "\0");
            let ret = emscripten::emscripten_asm_const_int(s1 as *const _ as *const c_char, 20_i32);
            println!("ret: {}", ret);
        }
    }

    {
        println!("test emscripten_async_wget");

        unsafe {
            let url = concat!("icon.png", "\0");
            let file = concat!("icon.png", "\0");
            emscripten::emscripten_async_wget(url as *const _ as *const c_char,
                                              file as *const _ as *const c_char,
                                              onload, onerror);
        }
    }

    {
        println!("test emscripten_get_element_css_size");
        let dom = CString::new("canvas").unwrap();
        // println!("ret: {}", html5::emscripten_set_element_css_size(dom.as_ptr(), 400_f64, 400_f64));
        let mut w = 0_f64;
        let mut h = 0_f64;
        unsafe {
            println!("ret: {}", html5::emscripten_get_element_css_size(dom.as_ptr(), &mut w, &mut h));
            println!("w: {} h: {}", w, h);
        }
    }

    {
        let s = concat!("\
FS.mkdir('/persist');\
FS.mount(IDBFS, {}, '/persist');\
FS.syncfs(true, function (err) {\
    assert(!err);\
    ccall('test');\
});\
", "\0");
        unsafe {
            emscripten::emscripten_asm_const(s as *const _ as *const c_char);
            emscripten::emscripten_exit_with_live_runtime();
        }
    }
}

#[no_mangle]
pub extern "C" fn onload(file: *const c_char) {
    unsafe {
        let file = CString::from_raw(file as *mut c_char).into_string().unwrap();
        println!("file {} loaded", file);
    }
}

#[no_mangle]
pub extern "C" fn onerror(file: *const c_char) {
    unsafe {
        let file = CString::from_raw(file as *mut c_char).into_string().unwrap();
        println!("file {} load error", file);
    }
}

#[no_mangle]
pub extern "C" fn test() {
    let file_name = "/persist/foo.txt";

    if let Ok(mut file) = File::open(file_name) {
        let mut txt = String::new();
        file.read_to_string(&mut txt).unwrap();
        println!("File content: {}", txt);
    } else {
        let mut mf = File::create(file_name).unwrap();
        mf.write_all("Hello, emscripten!".as_bytes());
        println!("Write file {}", file_name);

        let s = concat!("FS.syncfs(function(err) {\
assert(!err);\
})", "\0");
        unsafe {
            emscripten::emscripten_asm_const(s as *const _ as *const c_char);
        }
    }

}

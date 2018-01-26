use std::mem;
use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub fn rust_pow(number: i32, power: i32) -> i32 {
    number.pow(power as u32)
}

#[no_mangle]
pub fn hello() {
    println!("Hello, world!");
}

#[no_mangle]
pub fn draw(sel: *mut c_char) {
    unsafe {
        match CString::from_raw(sel).into_string() {
            Ok(sel) => {
                println!("I'll draw on: {}", sel);
                mem::forget(sel);
            },
            Err(_) => ()
        }
    }
}

fn main() {
}

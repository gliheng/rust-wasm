#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::cell::RefCell;
use std::ptr::null_mut;
use std::os::raw::{c_void, c_int, c_float, c_double, c_char};

type em_callback_func = extern "C" fn();
type em_str_callback_func = extern "C" fn(*const c_char);
type em_arg_callback_func = extern "C" fn(*const c_void);
type em_run_preload_plugins_data_onload_func = extern "C" fn(*const c_void, *mut c_char);

extern "C" {
    pub fn emscripten_set_main_loop(func: em_callback_func,
                                    fps: c_int,
                                    simulate_infinite_loop: c_int);

    pub fn emscripten_cancel_main_loop();
    pub fn emscripten_pause_main_loop();
    pub fn emscripten_get_now() -> c_float;
    pub fn emscripten_run_preload_plugins(file: *const c_char,
                                          onload: em_str_callback_func,
                                          onerror: em_str_callback_func) -> i32;
    pub fn emscripten_run_preload_plugins_data(data: *const u8,
                                               size: usize,
                                               suffix: *const c_char,
                                               arg: *const c_void,
                                               onload: em_run_preload_plugins_data_onload_func,
                                               onerror: em_arg_callback_func);

    pub fn emscripten_asm_const(code: *const c_char);
    pub fn emscripten_asm_const_int(code: *const c_char, ...) -> c_int;
    pub fn emscripten_asm_const_double(code: *const c_char, ...) -> c_double;

    pub fn emscripten_exit_with_live_runtime();

    pub fn emscripten_debugger();
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

pub fn set_main_loop_callback<F>(callback: F)
    where F: FnMut()
{
    MAIN_LOOP_CALLBACK
        .with(|log| { *log.borrow_mut() = &callback as *const _ as *mut c_void; });

    unsafe {
        emscripten_set_main_loop(wrapper::<F>, -1, 1);
    }

    extern "C" fn wrapper<F>()
        where F: FnMut()
    {
        MAIN_LOOP_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            unsafe {
                (*closure)();
            }
        });
    }
}


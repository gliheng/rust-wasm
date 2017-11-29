#[cfg(target_os = "emscripten")]
use stdweb::unstable::TryInto;

#[cfg(target_os = "emscripten")]
pub fn get_window_dimensiton() -> (u32, u32) {
    let w = js! {
        return document.body.clientWidth;
    };
    let h = js! {
        return document.body.clientHeight;
    };
    (w.try_into().unwrap(), h.try_into().unwrap())
}

/// convert FingerMotion coordinates to px
pub fn convert(total: f32, ratio: f32) -> f32 {
    total * ratio
}

#[cfg(not(target_os = "emscripten"))]
pub fn get_window_dimensiton() -> (u32, u32) {
    (640, 500)
}

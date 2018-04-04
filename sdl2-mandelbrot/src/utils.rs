use sdl2::rect::{Rect, Point};

#[cfg(target_os = "emscripten")]
use stdweb::unstable::TryInto;

#[cfg(target_os = "emscripten")]
pub fn get_window_dimention() -> (u32, u32) {
    let w = js! {
        return document.body.clientWidth;
    };
    let h = js! {
        return document.body.clientHeight;
    };
    (w.try_into().unwrap(), h.try_into().unwrap())
}

#[cfg(not(target_os = "emscripten"))]
pub fn get_window_dimention() -> (u32, u32) {
    (640, 500)
}

/// convert FingerMotion coordinates to px
pub fn convert(total: f32, ratio: f32) -> f32 {
    total * ratio
}

pub fn rect_from_points(p1: &Point, p2: &Point) -> Rect {
    let (x1, y1) = (*p1).into();
    let (x2, y2) = (*p2).into();

    Rect::new(x1.min(x2),
              y1.min(y2),
              (x1 - x2).abs() as u32,
              (y1 - y2).abs() as u32)
}
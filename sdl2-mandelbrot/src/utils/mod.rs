pub mod glyph_renderer;

use std::time::Duration;
use sdl2::rect::{Rect, Point};

#[cfg(target_os = "emscripten")]
use stdweb::unstable::TryInto;

#[cfg(target_os = "emscripten")]
pub fn get_window_dimention() -> (u32, u32) {
    let w = js! {
        return Module.canvas.clientWidth;
    };
    let h = js! {
        return Module.canvas.clientHeight;
    };
    (w.try_into().unwrap(), h.try_into().unwrap())
}

#[cfg(not(target_os = "emscripten"))]
pub fn get_window_dimention() -> (u32, u32) {
    (640, 500)
}

pub fn rect_from_points(p1: &Point, p2: &Point) -> Rect {
    let (x1, y1) = (*p1).into();
    let (x2, y2) = (*p2).into();

    Rect::new(x1.min(x2),
              y1.min(y2),
              (x1 - x2).abs() as u32,
              (y1 - y2).abs() as u32)
}

pub fn format_duration(dur: Duration) -> f32 {
    dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1_000_000_000.
}

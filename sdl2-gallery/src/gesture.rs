use std::f32;
use sdl2::event::Event;
use std::mem;
use std::time::{Instant, Duration};

// struct GestureEventIterator {
// }

// impl Iterator<GestureEvent> for GestureEventIterator {
// }

pub enum GestureEvent {
    Tap,
    DoubleTap,
}

pub struct GestureDetector {
    last_x: f32,
    last_y: f32,
    last_tap_time: Instant,
    pool: Vec<GestureEvent>,
}

impl GestureDetector {
    pub fn new() -> GestureDetector {
        GestureDetector {
            last_x: f32::NAN,
            last_y: f32::NAN,
            last_tap_time: Instant::now(),
            pool: vec![],
        }
    }
    pub fn feed(&mut self, evt: &Event) {
        match evt {
            &Event::FingerDown { x, y, touch_id, .. } => {
                if x == self.last_x && y == self.last_y && self.last_tap_time.elapsed() < Duration::from_millis(500) {
                    self.pool.push(GestureEvent::DoubleTap);
                } else {
                    self.last_x = x;
                    self.last_y = y;
                    self.last_tap_time = Instant::now();
                }
            },
            _ => ()
        }
    }
    // this can be optimized by using an iterator
    pub fn poll(&mut self) -> Vec<GestureEvent> {
        let mut p = vec![];
        mem::swap(&mut self.pool, &mut p);
        return p;
    }
}

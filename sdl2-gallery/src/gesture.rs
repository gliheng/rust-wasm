use std::f32;
use sdl2::event::Event;
use std::mem;

// struct GestureEventIterator {
// }

// impl Iterator<GestureEvent> for GestureEventIterator {
// }

pub enum GestureEvent {
    Tap,
    DoubleTap,
}

pub struct GestureDetector {
    first_tap: Option<Event>,
    pool: Vec<GestureEvent>,
}

impl GestureDetector {
    pub fn new() -> GestureDetector {
        GestureDetector {
            first_tap: None,
            pool: vec![],
        }
    }
    pub fn feed(&mut self, evt: &Event) {
        match evt {
            &Event::FingerDown { x, y, dx, dy, touch_id, finger_id, timestamp, pressure } => {
                // get distance between two points
                let mut double_tapped = false;
                if let Some(ref tap) = self.first_tap {
                    if let &Event::FingerDown {x: x0, y: y0, touch_id: touch_id0, timestamp: timestamp0, ..} = tap {
                        if touch_id == touch_id0 {
                            let dist = ((x - x0).powi(2) + (y - y0).powi(2)).sqrt();
                            if dist < 10. && timestamp - timestamp0 < 300 {
                                self.pool.push(GestureEvent::DoubleTap);
                                double_tapped = true;
                            }
                        }
                    }
                }
                if !double_tapped {
                    self.first_tap = Some(Event::FingerDown {
                        x,
                        y,
                        dx,
                        dy,
                        touch_id,
                        finger_id,
                        timestamp,
                        pressure,
                    });
                }
            },
            &Event::FingerMotion { x, y, touch_id, .. } => {
                // release the first tap if moved
                let mut done = false;
                if let Some(ref tap) = self.first_tap {
                    if let &Event::FingerDown {touch_id: touch_id0, ..} = tap {
                        if touch_id0 == touch_id {
                            done = true;
                        }
                    }
                }
                if done {
                    self.first_tap = None;
                }
            },
            &Event::FingerUp { x, y, touch_id, .. } => {
            },
            &Event::MultiGesture {d_dist, d_theta, ..} => {
                println!("pinched: {} {}", d_dist, d_theta);
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

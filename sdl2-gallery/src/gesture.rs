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
    PanStart {
        timestamp: u32,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    Pan {
        timestamp: u32,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    PanEnd {
        timestamp: u32,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
}


pub struct GestureDetector {
    pool: Vec<GestureEvent>,
    detectors: Vec<Box<Detector>>,
}

impl GestureDetector {
    pub fn new() -> GestureDetector {
        GestureDetector {
            pool: vec![],
            detectors: vec![Box::new(DoubleTapDetector::new()), Box::new(PanDetector::new())]
        }
    }
    pub fn feed(&mut self, evt: &Event) {
        for d in &mut self.detectors {
            if let Some(g) = d.feed(evt) {
                self.pool.push(g);
            }
        }
    }
    // this can be optimized by using an iterator
    pub fn poll(&mut self) -> Vec<GestureEvent> {
        let mut p = vec![];
        mem::swap(&mut self.pool, &mut p);
        return p;
    }
}

trait Detector {
    fn feed(&mut self, evt: &Event) -> Option<GestureEvent>;
}

pub struct DoubleTapDetector {
    first_tap: Option<Event>,
}

impl DoubleTapDetector {
    fn new() -> DoubleTapDetector {
        DoubleTapDetector {
            first_tap: None,
        }
    }
}

impl Detector for DoubleTapDetector {
    fn feed(&mut self, evt: &Event) -> Option<GestureEvent> {
        match evt {
            &Event::FingerDown { x, y, dx, dy, touch_id, finger_id, timestamp, pressure } => {
                // get distance between two points
                if let Some(ref tap) = self.first_tap {
                    if let &Event::FingerDown {x: x0, y: y0, touch_id: touch_id0, timestamp: timestamp0, finger_id: finger_id0, ..} = tap {
                        if finger_id == finger_id0 {
                            let dist = ((x - x0).powi(2) + (y - y0).powi(2)).sqrt();
                            if dist < 10. && timestamp - timestamp0 < 300 {
                                return Some(GestureEvent::DoubleTap);
                            }
                        }
                    }
                }
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
            },
            &Event::FingerMotion { x, y, finger_id, .. } => {
                // release the first tap if moved
                let mut done = false;
                if let Some(ref tap) = self.first_tap {
                    if let &Event::FingerDown {finger_id: finger_id0, ..} = tap {
                        if finger_id0 == finger_id {
                            done = true;
                        }
                    }
                }
                if done {
                    self.first_tap = None;
                }
            },
            &Event::FingerUp { x, y, finger_id, .. } => {
            },
            _ => ()
        }
        None
    }
}

pub struct PanDetector {
    curr_touch_id: i64,
    curr_finger_id: i64,
}

impl PanDetector {
    fn new() -> PanDetector {
        PanDetector {
            curr_touch_id: -1,
            curr_finger_id: -1,
        }
    }
}
impl Detector for PanDetector {
    fn feed(&mut self, evt: &Event) -> Option<GestureEvent> {
        match evt {
            &Event::FingerDown { x, y, dx, dy, touch_id, finger_id, timestamp, pressure } => {
                self.curr_touch_id = touch_id;
                self.curr_finger_id = finger_id;
                return Some(GestureEvent::PanStart{x, y, dx, dy, timestamp, pressure});
            },
            &Event::FingerMotion { x, y, dx, dy, touch_id, finger_id, timestamp, pressure } => {
                if self.curr_touch_id == touch_id && self.curr_finger_id == finger_id {
                    return Some(GestureEvent::Pan{x, y, dx, dy, timestamp, pressure});
                }
            },
            &Event::FingerUp { x, y, dx, dy, touch_id, finger_id, timestamp, pressure, .. } => {
                if self.curr_touch_id == touch_id && self.curr_finger_id == finger_id {
                    return Some(GestureEvent::PanEnd{x, y, dx, dy, timestamp, pressure});
                }
            },
            _ => ()
        }
        None
    }
}

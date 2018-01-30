use std::f32;
use sdl2::event::Event;
use sdl2::touch::num_touch_fingers;

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
        self.pool.drain(0..).collect()
    }
}

trait Detector {
    fn feed(&mut self, evt: &Event) -> Option<GestureEvent>;
}

pub struct DoubleTapDetector {
    prev_finger_down: Option<Event>,
}

impl DoubleTapDetector {
    fn new() -> DoubleTapDetector {
        DoubleTapDetector {
            prev_finger_down: None,
        }
    }
}

impl Detector for DoubleTapDetector {
    fn feed(&mut self, evt: &Event) -> Option<GestureEvent> {
        match evt {
            &Event::FingerDown { x, y, dx, dy, touch_id, finger_id, timestamp, pressure } => {
                // get distance between two points
                if let Some(ref tap) = self.prev_finger_down {
                    if let &Event::FingerDown {x: x0, y: y0, touch_id: touch_id0, timestamp: timestamp0, finger_id: finger_id0, ..} = tap {
                        if finger_id == finger_id0 {
                            let dist = get_dist(x, x0, y, y0);
                            if dist < 30. && timestamp - timestamp0 < 300 {
                                return Some(GestureEvent::DoubleTap);
                            }
                        }
                    }
                }
                self.prev_finger_down = Some(Event::FingerDown {
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
            &Event::FingerMotion { x, y, finger_id, dx, dy, .. } => {
                // release the first tap if moved
                let mut cancel = false;
                if let Some(ref down) = self.prev_finger_down {
                    if let &Event::FingerDown {finger_id: finger_id0, x: x0, y: y0, ..} = down {
                        if finger_id0 == finger_id && get_dist(x, x0, y, y0) >= 0.01 {
                            // moved too far, cancel the tap
                            cancel = true;
                        }
                    }
                }
                if cancel {
                    self.prev_finger_down = None;
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
                let fingers = num_touch_fingers(1);
                if fingers == 1 && self.curr_touch_id == touch_id && self.curr_finger_id == finger_id {
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

fn get_dist(x: f32, x0: f32, y: f32, y0: f32) -> f32 {
    ((x - x0).powi(2) + (y - y0).powi(2)).sqrt()
}

use std::f32;
use sdl2::event::Event;
use sdl2::touch::num_touch_fingers;

pub enum GestureEvent {
    Tap(f32, f32),
    DoubleTap(f32, f32),
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


pub enum GestureDetectorTypes {
    Tap, Pan,
}

pub struct GestureDetector {
    pool: Vec<GestureEvent>,
    detectors: Vec<Box<Detector>>,
}

impl GestureDetector {
    pub fn new(types: Vec<GestureDetectorTypes>) -> GestureDetector {
        GestureDetector {
            pool: vec![],
            detectors: types.iter().map(|t| {
                match t {
                    &GestureDetectorTypes::Tap => {
                        Box::new(TapDetector::new()) as Box<Detector>
                    },
                    &GestureDetectorTypes::Pan => {
                        Box::new(PanDetector::new()) as Box<Detector>
                    },
                }
            }).collect()
        }
    }
    pub fn feed(&mut self, evt: &Event) {
        for d in &mut self.detectors {
            if let Some(g) = d.feed(evt) {
                self.pool.push(g);
            }
        }
    }
    pub fn poll(&mut self) -> Vec<GestureEvent> {
        self.pool.drain(0..).collect()
    }
}

trait Detector {
    fn feed(&mut self, evt: &Event) -> Option<GestureEvent>;
}

const TAP_DURATION: u32 = 150;
const DOUBLETAP_DURATION: u32 = 300;
const TAP_DIST: f32 = 0.04;

pub struct TapDetector {
    prev_finger_down: Option<Event>,
    prev_tap: Option<(f32, f32, u32)>,
}

impl TapDetector {
    fn new() -> TapDetector {
        TapDetector {
            prev_finger_down: None,
            prev_tap: None,
        }
    }
}

impl Detector for TapDetector {
    fn feed(&mut self, evt: &Event) -> Option<GestureEvent> {
        match evt {
            &Event::FingerDown { x, y, dx, dy, touch_id, finger_id, timestamp, pressure } => {
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
                // release the first tap if moved too much
                let mut cancel = false;
                if let Some(ref down) = self.prev_finger_down {
                    if let &Event::FingerDown {finger_id: finger_id0, x: x0, y: y0, ..} = down {
                        if finger_id0 == finger_id && get_dist(x, y, x0, y0) >= 0.02 {
                            // moved too far, cancel the tap
                            cancel = true;
                        }
                    }
                }
                if cancel {
                    self.prev_finger_down = None;
                }
            },
            &Event::FingerUp { x, y, finger_id, timestamp, .. } => {
                // get distance between two points
                if let Some(ref down) = self.prev_finger_down {
                    if let &Event::FingerDown {x: x0, y: y0, touch_id: touch_id0, timestamp: timestamp0, finger_id: finger_id0, ..} = down {
                        if finger_id == finger_id0 {
                            if get_dist(x, y, x0, y0) < TAP_DIST && timestamp - timestamp0 < TAP_DURATION {
                                // we got a Tap or a DoubleTap
                                let mut single = true;
                                if let Some(ref tap) = self.prev_tap {
                                    if get_dist(x, y, tap.0, tap.1) < TAP_DIST && timestamp - tap.2 < DOUBLETAP_DURATION {
                                        single = false;
                                    }
                                }
                                if single {
                                    self.prev_tap = Some((x, y, timestamp));
                                    return Some(GestureEvent::Tap(x, y));
                                }
                                return Some(GestureEvent::DoubleTap(x, y));
                            }
                        }
                    }
                }
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

fn get_dist(x: f32, y: f32, x0: f32, y0: f32) -> f32 {
    ((x - x0).powi(2) + (y - y0).powi(2)).sqrt()
}

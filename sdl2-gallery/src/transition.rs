use std::time::{Duration, Instant};

enum TransitionState {
    Running,
    AtEnd,
}

pub struct Transition {
    start_time: Instant,
    duration: Duration,
    start_val: i32,
    target_val: i32,
    state: TransitionState,
}

impl Transition {
    pub fn new(start_val: i32, target_val: i32, duration: Duration) -> Transition {
        Transition {
            start_time: Instant::now(),
            start_val,
            target_val,
            duration,
            state: TransitionState::Running,
        }
    }
    pub fn at_end(&self) -> bool {
        match self.state {
            TransitionState::AtEnd => true,
            _ => false,
        }
    }
    pub fn step(&mut self) -> f64 {
        let mut t = Transition::to_f64(self.start_time.elapsed());
        let d = Transition::to_f64(self.duration);
        let b = self.start_val;
        let c = self.target_val - self.start_val;
        if t >= d {
            self.state = TransitionState::AtEnd;
            return self.target_val as f64;
        }
        // using easing fucntions from http://www.gizma.com/easing/
        // linear
        // c as f64 * t / d + b as f64
        // easeOutQuad
        t /= d;
	    return -c as f64 * t * (t-2.) + b as f64;
    }
    fn to_f64(d: Duration) -> f64 {
        d.as_secs() as f64
            + d.subsec_nanos() as f64 * 1e-9
    }
}

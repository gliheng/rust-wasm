use std::time::{Instant, Duration};
use std::collections::VecDeque;
use utils::mean::Mean;

pub struct FrameRate {
    times: VecDeque<Instant>,
    mean: Mean<u32>
}

impl FrameRate {
    pub fn new() -> Self {
        FrameRate {
            times: VecDeque::with_capacity(100),
            mean: Mean::new(5)
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let duration = Duration::from_secs(1);

        self.times.push_back(now);

        while self.times.front().is_some() && now.duration_since(*self.times.front().unwrap()) > duration {
            self.times.pop_front();
        }
    }

    /// Get the average framerate over the previous samples
    pub fn mean(&mut self) -> u32 {
        self.mean.push(self.times.len() as u32);
        self.mean.get() as u32
    }

    /// Get the current framerate
    pub fn get(&self) -> u8 {
        self.times.len() as u8
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use super::*;

    #[test]
    fn framerate_works() {
        let mut rate = FrameRate::new(200);
        for _ in 0..10 {
            rate.tick();
        }
        assert!(rate.get() == 10);
        sleep(Duration::new(1, 100_000_000));
        rate.tick();
        assert!(rate.get() == 1);
    }
}

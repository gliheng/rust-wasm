use std::collections::VecDeque;
use std::time::{Instant, Duration};

struct Mean {
    histroy: VecDeque<u8>,
    total: u32 // we get a total, divide by n to get a mean
}

impl Mean {
    fn new(capacity: usize) -> Self {
        Mean {
            histroy: VecDeque::with_capacity(capacity),
            total: 0
        }
    }
    fn push(&mut self, n: u8) {
        if self.histroy.len() == self.histroy.capacity() {
            if let Some(n) = self.histroy.pop_front() {
                self.total -= n as u32;
            }
        }
        self.histroy.push_back(n);
        self.total += n as u32;
    }
    fn get(&self) -> u32 {
        self.total / self.histroy.len() as u32
    }
}

pub struct FrameRate {
    times: VecDeque<Instant>,
    mean: Mean
}

impl FrameRate {
    pub fn new(n: u8) -> Self {
        FrameRate {
            times: VecDeque::with_capacity(n as usize),
            mean: Mean::new(10usize)
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

    pub fn mean(&mut self) -> u8 {
        self.mean.push(self.times.len() as u8);
        self.mean.get() as u8
    }

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

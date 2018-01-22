use std::collections::VecDeque;
use std::ops::{AddAssign, SubAssign};

pub struct Mean<T> {
    histroy: VecDeque<T>,
    total: T // we get a total, divide by n to get a mean
}

impl <T: AddAssign + SubAssign + Default> Mean<T> {
    pub fn new(capacity: usize) -> Self {
        Mean {
            histroy: VecDeque::with_capacity(capacity),
            total: Self::default()
        }
    }
    pub fn push(&mut self, n: T) {
        if self.histroy.len() == self.histroy.capacity() {
            if let Some(n) = self.histroy.pop_front() {
                self.total -= n;
            }
        }
        self.histroy.push_back(n);
        self.total += n;
    }
    pub fn get(&self) -> T {
        (self.total as f64 / self.histroy.len() as f64) as T
    }
}


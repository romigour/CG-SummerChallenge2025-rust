use std::time::{Instant, Duration};

pub struct Timer {
    start: Instant,
    limit: Duration,
}

impl Timer {
    pub fn new(start: Instant, limit: Duration) -> Self {
        Self { start, limit }
    }
    pub fn is_time_up(&self) -> bool {
        Instant::now().duration_since(self.start) >= self.limit
    }
}
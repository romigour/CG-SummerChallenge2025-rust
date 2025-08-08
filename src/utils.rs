use std::time::{Instant, Duration};
use std::eprintln;

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

pub struct Debug {
}

impl Debug {
    pub fn debug(label: &str, params: &[(&str, String)]) {
        eprintln!("=== DEBUG: {} ===", label);
        for (name, value) in params {
            eprintln!(" {}: {}", name, value);
        }
        eprintln!("======================");
    }
}
use std::time::{Instant, Duration};
use std::eprintln;
use std::fmt::Display;

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
        eprintln!();
    }

    pub fn debug_vec<T: std::fmt::Debug>(label: &str, values: &[T]) {
        eprintln!("=== DEBUG: {} ===", label);
        for value in values.iter() {
            eprintln!("{:?}", value);
        }
        eprintln!();
    }
}

pub struct Math {
}

impl Math {
    pub fn manhattan(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
        (x1 - x2).abs() + (y1 - y2).abs()
    }
}
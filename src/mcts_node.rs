use std::collections::HashMap;
use crate::action::Action;

#[derive(Debug)]
pub struct MCTSNode {
    pub visits: usize,
    pub value: f64,
    pub children: HashMap<Action, MCTSNode>,
}

impl MCTSNode {
    pub fn new() -> Self {
        Self {
            visits: 0,
            value: 0.0,
            children: HashMap::new(),
        }
    }

    pub fn uct(&self, parent_visits: usize) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }
        self.value / self.visits as f64 + (1.41 * (parent_visits as f64).ln() / self.visits as f64).sqrt()
    }
}
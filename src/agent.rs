use crate::game::GameState;
use crate::mcts::{Mcts, MctsConfig};
use crate::utils::Timer;

pub struct MctsAgent {
    config: MctsConfig,
}

impl MctsAgent {
    pub fn new() -> Self {
        Self { config: MctsConfig { exploration_constant: 1.41 } }
    }

    pub fn search<S: GameState>(&self, state: &S, iterations: u32, start: std::time::Instant) -> S::Action {
        let timer = Timer::new(start, std::time::Duration::from_millis(45));
        let mut mcts = Mcts::new(state.clone(), self.config.clone());
        mcts.search(iterations, &timer)
    }
}
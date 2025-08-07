use crate::game::GameState;
use crate::utils::Timer;

/// Paramètres de MCTS
#[derive(Clone, Debug)]
pub struct MctsConfig {
    pub exploration_constant: f32,
}

/// Noeud pour MCTS générique
pub struct Node<S: GameState> {
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub state: S,
    pub action_from_parent: Option<S::Action>,
    pub visits: u32,
    pub value: f32,
}

pub struct Mcts<S: GameState> {
    pub nodes: Vec<Node<S>>,
    pub config: MctsConfig,
}

impl<S: GameState> Mcts<S> {
    pub fn new(root_state: S, config: MctsConfig) -> Self {
        let root = Node {
            parent: None,
            children: vec![],
            state: root_state,
            action_from_parent: None,
            visits: 0,
            value: 0.0,
        };
        Self { nodes: vec![root], config }
    }

    pub fn search(&mut self, iterations: u32, timer: &Timer) -> S::Action {
        for _ in 0..iterations {
            if timer.is_time_up() { break; }
            let leaf_idx = self.select();
            let reward = self.simulate(leaf_idx);
            self.backpropagate(leaf_idx, reward);
        }
        self.best_action()
    }

    fn select(&mut self) -> usize { /* sélection UCT */ unimplemented!() }
    fn expand(&mut self, idx: usize) -> usize { /* expansion */ unimplemented!() }
    fn simulate(&self, idx: usize) -> f32 { /* rollout */ unimplemented!() }
    fn backpropagate(&mut self, idx: usize, reward: f32) { /* maj valeurs */ unimplemented!() }
    fn best_action(&self) -> S::Action { /* renvoyer le meilleur coup*/ unimplemented!() }
}

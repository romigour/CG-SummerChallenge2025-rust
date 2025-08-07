mod game;
mod mcts;
mod agent;
mod utils;

use game::{MyGameState, GameState};
use agent::MctsAgent;
use std::time::Instant;

fn main() {
    // Lecture des entrées de Codingame, initialisation
    let mut state = MyGameState::new();
    // Boucle de jeu
    while !state.is_terminal() {
        // Mettre à jour l'état depuis l'entrée standard
        state.read_input();
        // Choisir l'action via MCTS
        let now = Instant::now();
        let agent = MctsAgent::new();
        let best_action = agent.search(&state, 1000, now);
        // Appliquer l'action
        state.write_output(&best_action);
        state.apply_action(best_action);
    }
}
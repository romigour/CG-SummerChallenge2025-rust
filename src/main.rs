mod action;
mod state;
mod agent;
mod utils;
mod grid;
mod ia;

use crate::state::State;
use crate::ia::IA;
use crate::utils::Debug;

fn main() {
    let mut state = State::new();

    let ia = IA::new();

    // 1. Lecture des inputs initiaux
    State::init_input(&mut state);

    // 2. Boucle de jeu
    loop {
        // Lecture des inputs du tour
        State::update_input(&mut state);

        Debug::debug("State",
                     &[
                         ("turn", state.turn.to_string()),
                         ("state", format!("{:?}", state)),
                     ],
        );

        // IA
        let best_actions =  ia.decide_actions(&state);

        Debug::debug_vec("Best Actions", &best_actions);

        // Output
        state.play(best_actions);

    }
}
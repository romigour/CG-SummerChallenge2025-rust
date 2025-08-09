mod action;
mod state;
mod agent;
mod utils;
mod grid;
mod ia;
mod scorer;
mod ia2;

use crate::ia2::IA2;
use crate::state::State;
use crate::utils::{Debug, Timer};
use std::time::Duration;

fn main() {
    let mut state = State::new();

    let ia = IA2::new();
    let mut timer = Timer::new(Duration::from_millis(45));

    // 1. Lecture des inputs initiaux
    State::init_input(&mut state);

    // 2. Boucle de jeu
    loop {
        // Lecture des inputs du tour
        State::update_input(&mut state);

        timer.start();

        // Debug::debug("State",
        //              &[
        //                  ("turn", state.turn.to_string()),
        //                  ("state", format!("{:?}", state)),
        //              ],
        // );

        // IA
        let best_actions =  ia.decide_actions(&state, &timer);

        //Debug::debug_vec("Best Actions", &best_actions);
        Debug::debug_simple(format!("Time {:?}ms", timer.time()));

        // Output
        state.play(best_actions);

    }
}
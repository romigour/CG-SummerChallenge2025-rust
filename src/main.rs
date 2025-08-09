mod action;
mod state;
mod agent;
mod utils;
mod grid;
mod ia;
mod scorer;

use std::fmt::format;
use std::time::{Duration, Instant};
use crate::state::State;
use crate::ia::IA;
use crate::utils::{Debug, Timer};

fn main() {
    let mut state = State::new();

    let ia = IA::new();
    let mut timer = Timer::new(Duration::from_millis(50));

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

        Debug::debug_vec("Best Actions", &best_actions);
        Debug::debug_simple(format!("Time {:?}ms", timer.time()));

        // Output
        state.play(best_actions);

    }
}
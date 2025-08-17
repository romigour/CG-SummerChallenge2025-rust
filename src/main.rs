mod action;
mod state;
mod agent;
mod utils;
mod grid;
mod scorer;
mod io_helper;
mod mcts_node;
mod ia;
mod tests;

use crate::state::State;
use crate::utils::{Debug, Timer};
use std::time::Duration;
use crate::ia::IA;
use crate::io_helper::InputSource;

fn main() {
    let mut input_source = InputSource::from_stdin();

    let mut state = State::new();

    let ia = IA::new();
    let mut timer = Timer::new(Duration::from_millis(55));

    Debug::debug_simple("A".parse().unwrap());
    // 1. Lecture des inputs initiaux
    State::init_input(&mut state, &mut input_source);

    // 2. Boucle de jeu
    loop {
        // Lecture des inputs du tour
        State::update_input(&mut state, &mut input_source);

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
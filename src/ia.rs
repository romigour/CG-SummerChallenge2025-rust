use crate::action::Action;
use crate::state::State;

pub struct IA;

impl IA {
    pub fn new() -> Self {
        IA
    }
    pub fn decide_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Exemple : pour chaque unité, aller à droite
        for unit in &state.my_agents {
            actions.push(Action::HunkerDown);
        }

        actions
    }
}
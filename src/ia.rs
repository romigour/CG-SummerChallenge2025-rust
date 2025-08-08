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
        for idx in &state.my_idx_arr {
            let agent = &state.agents[*idx];
            actions.push(Action::hunker_down(agent.id, agent.x + 1, agent.y));
        }

        actions
    }
}
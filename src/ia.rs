use crate::action::Action;
use crate::scorer::Scorer;
use crate::state::State;
use crate::utils::{Debug, Timer};

pub struct IA;

impl IA {
    pub fn new() -> Self {
        IA
    }
    pub fn decide_actions(&self, state: &State, timer: &Timer) -> Vec<Action> {
        let mut actions = Vec::new();

        for idx in &state.my_idx_arr {
            let agent = &state.agents[*idx];
            if agent.is_dead {
                continue;
            }
            let actionsAgent = state.legal_actions_for_agent(agent);
            let mut best_action: Option<Action> = None;
            let mut best_score = i32::MIN;
            Debug::debug_vec(format!("Agent n°{:?}", agent.id).as_str(), &actionsAgent);
            for action in actionsAgent {
                let mut current_state = state.clone();
                current_state.apply_actions(action);

                // Debug::debug("Current State",
                //              &[
                //                  ("action", action.display()),
                //                  ("current_state", format!("{:?}", current_state)),
                //              ],
                // );
                // Debug::debug_simple(format!("Time1: {:?}", timer.time()));

                let score = Scorer::score(current_state);
                // Debug::debug_simple(format!("Time1: {:?}", timer.time()));
                if score > best_score {
                    best_score = score;
                    best_action = Option::from(action);
                }
            }

            //Debug::debug_vec(format!("Action agent n°{:?}", agent.id).as_str(), &actionsAgent);
            if best_action.is_none() {
                actions.push(Action::hunker_down(agent.id, agent.x, agent.y));
            } else {
                Debug::debug_simple(format!("Score agent n° {:?}: {:?} -> {:?}", agent.id, best_score, best_action.unwrap()));
                actions.push(best_action.unwrap());
            }

        }

        actions
    }
}
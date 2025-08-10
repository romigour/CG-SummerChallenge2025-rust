use crate::action::{Action, TypeAction};
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

        let mut actions_per_agent = Vec::new();
        let mut nb_actions_per_agent = Vec::new();
        for idx in &state.my_idx_arr {
            let agent = &state.agents[*idx];
            if agent.is_dead {
                continue;
            }

            let actions_for_agent_filter = filter_actions(state.legal_actions_for_agent(agent));
            let actions_for_agent_filter_len = actions_for_agent_filter.len();

            //Debug::debug_vec(format!("Agent n°{}", agent.id).as_str(), &actions_for_agent);

            actions_per_agent.push(actions_for_agent_filter);
            nb_actions_per_agent.push(actions_for_agent_filter_len);
        }
        //Debug::debug_simple(format!("nb_actions_per_agent {:?}", nb_actions_per_agent));
        let mut all_combinations = combine_actions(&actions_per_agent);
        Debug::debug_simple(format!("Size combos {:?}", all_combinations.len()));
        let mut best_score = i32::MIN;

        for combo in &mut all_combinations {
            let mut current_state = state.clone();
            for action in &mut *combo {
                current_state.apply_actions(*action);
            }

            // Debug::debug("Current State",
            //              &[
            //                  ("actions", format!("{:?}", combo)),
            //                  ("current_state", format!("{:?}", current_state)),
            //              ],
            // );
            // Debug::debug_simple(format!("Time1: {:?}", timer.time()));

            let score = Scorer::score(current_state);


            // Debug::debug_simple(format!("Time1: {:?}", timer.time()));

            if score > best_score {
                best_score = score;
                actions = combo.clone();
            }

            if timer.is_time_up() {
                Debug::debug_simple("IS TIME UP".parse().unwrap());
                break;
            }
        }
        Debug::debug_simple(format!("best_score: {:?}", best_score));
        actions
    }


}

fn combine_actions(actions: &[Vec<Action>]) -> Vec<Vec<Action>> {
    if actions.is_empty() {
        return vec![vec![]];
    }

    let first_agent_actions = &actions[0];
    let rest_combinations = combine_actions(&actions[1..]);

    let mut result = Vec::new();

    for &action in first_agent_actions {
        for combo in &rest_combinations {
            let mut new_combo = vec![action];
            new_combo.extend_from_slice(&combo);
            // result.push(new_combo);
            // Vérifie que toutes les coordonnées sont uniques
            let mut coords = std::collections::HashSet::new();
            let all_unique = new_combo.iter().all(|a| coords.insert((a.mx, a.my)));

            if all_unique {
                result.push(new_combo);
            }
        }
    }

    result
}

fn filter_actions(actions: Vec<Action>) -> Vec<Action> {
    let mut hunker_down: Vec<Action> = Vec::new();
    let mut shoot: Vec<Action> = Vec::new();
    let mut throw: Vec<Action> = Vec::new();

    // 1. Séparer par type
    for a in actions {
        match a.type_action {
            TypeAction::HunkerDown => hunker_down.push(a),
            TypeAction::Shoot => shoot.push(a),
            TypeAction::Throw => throw.push(a),
        }
    }

    // 2. Trier et limiter
    shoot.sort_by(|a, b| b.score.cmp(&a.score));
    shoot.truncate(6);

    throw.sort_by(|a, b| b.score.cmp(&a.score));
    throw.truncate(2);

    // 3. Fusionner dans un seul Vec
    let mut result = Vec::new();
    result.extend(hunker_down);
    result.extend(shoot);
    result.extend(throw);

    result
}
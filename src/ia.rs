use crate::action::Action;
use crate::mcts_node::MCTSNode;
use crate::scorer::Scorer;
use crate::state::State;
use crate::utils::{Debug, Timer};
use std::collections::HashMap;
//use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom; // <-- c'est celui-ci
use rand::thread_rng;
use crate::agent::Team;

pub struct IA;

impl IA {
    pub fn new() -> Self {
        IA
    }
    pub fn decide_actions(&self, root_state: &State, timer: &Timer) -> Vec<Action> {
        let mut simulation_count = 0;

        let mut agent_trees: HashMap<usize, MCTSNode> = HashMap::new();
        let agent_ids: Vec<usize> = root_state.agents.iter().filter(|a| !a.is_dead).map(|a| a.id as usize).collect();
        for agent_id in &agent_ids {
            let mut node_agent = MCTSNode::new(None);
            let legal_actions = root_state.legal_actions_for_idx_agent(agent_id - 1);
            let mut index = 0;
            for mut action in legal_actions {
                action.index = index;
                index += 1;
                node_agent.children.push(MCTSNode::new(Some(action)));
            }
            agent_trees.insert(*agent_id, node_agent);
        }

        // Debug::debug_simple(format!("LISTE {:?}", legal_actions_map));
        while !timer.is_time_up() {
            // while simulation_count < 5000 {
            let mut actions = vec![];
            let mut next_state = root_state.clone();
            for agent_id in &agent_ids {
                let node = agent_trees.get_mut(agent_id).unwrap();

                // Sélection via UCT
                let best_move_index = node.children.iter()
                    .enumerate()
                    .map(|(i, n)| (i, n.uct(node.visits)))
                    .max_by(|a, b| a.1.total_cmp(&b.1))
                    .map(|(i, _)| i)
                    .unwrap();

                actions.push(node.children.get(best_move_index).unwrap().action.unwrap());
            }

            // Simulation
            next_state.apply_actions_all(&actions);
            simulation_count += 1;

            simulate_random_playout(&mut next_state, 1);

            let mut score : [f64; 2] = [-1.0, -1.0];
            score[0] = Scorer::score(next_state.clone(), Team::Me);
            score[1] = Scorer::score(next_state.clone(), Team::Enemy);

            // Rétropropagation
            for action in &actions {
                let node = agent_trees.get_mut(&(action.id as usize)).unwrap();
                let mut child: &mut MCTSNode = node.children.get_mut(action.index).unwrap();
                child.visits += 1;
                child.value += score[0] - score[1];
                node.visits += 1;
            }
        }

        Debug::debug_simple(format!("Sim: {:?}", &simulation_count));

        // Choix final d’action pour chaque agent
        root_state.my_idx_arr.iter()
            .map(|agent_idx| agent_trees.get(&(agent_idx + 1)))
            .filter(|node| node.is_some())
            .map(|node| node.unwrap())
            .map(|node| {
                node.children.iter().max_by_key(|node| { node.visits }).map(|(a)| a.action.unwrap()).unwrap()
            }).collect()
    }
}

fn simulate_random_playout(current_state: &mut State, max_depth: usize) {
    let mut rng = thread_rng();

    let mut depth = 0;
    while max_depth > depth {
        if current_state.is_terminal() {
            break;
        }

        let actions: Vec<Action> = current_state.agents.iter().filter(|agent| !agent.is_dead).map(|agent| {
            let legal = current_state.legal_actions_for_idx_agent((agent.id - 1) as usize);
            legal.choose(&mut rng).unwrap().clone()
        }).collect();

        current_state.apply_actions_all(&actions);
        depth += 1;
    }
}
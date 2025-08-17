use crate::action::Action;
use crate::mcts_node::MCTSNode;
use crate::scorer::Scorer;
use crate::state::State;
use crate::utils::{Debug, Timer};
use std::collections::HashMap;
use rand::seq::SliceRandom; // <-- c'est celui-ci
use rand::thread_rng;

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
            agent_trees.insert(*agent_id, MCTSNode::new());
        }

        while !timer.is_time_up() {
            // while simulation_count < 5000 {
            let mut actions = vec![];
            let mut next_state = root_state.clone();
            for agent_id in &agent_ids {
                let node = agent_trees.get_mut(agent_id).unwrap();
                let legal_actions = next_state.legal_actions_for_idx_agent(*agent_id - 1);

                legal_actions.iter().for_each(|action| {
                    // Si l'action n'existe pas dans les enfants, on l'ajoute
                    if !node.children.contains_key(action) {
                        node.children.insert(*action, MCTSNode::new());
                    }
                });

                // Sélection via UCT
                let action = legal_actions.iter().max_by(|a, b| {
                    let a_node = node.children.get(*a).unwrap();
                    let b_node = node.children.get(*b).unwrap();
                    a_node
                        .uct(node.visits)
                        .partial_cmp(&b_node.uct(node.visits))
                        .unwrap()
                }).unwrap().clone();

                actions.push(action);
            }

            // Simulation
            next_state.apply_actions_all(&actions);
            simulation_count += 1;

            let reward = simulate_random_playout(&mut next_state, 1);

            // Rétropropagation
            for action in &actions {
                let node = agent_trees.get_mut(&(action.id as usize)).unwrap();
                let mut child: &mut MCTSNode = node.children.get_mut(action).unwrap();
                child.visits += 1;
                child.value += reward;
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
                node.children.iter().max_by(|(_, a), (_, b)| {
                    a.visits.cmp(&b.visits)
                }).map(|(a, _)| a.clone()).unwrap()
            }).collect()
    }
}

fn simulate_random_playout(current_state: &mut State, max_depth: usize) -> f64 {
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

    // Scorer::score(current_state)
    Scorer::score(current_state.clone())
}
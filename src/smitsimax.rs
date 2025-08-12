// use crate::action::{Action, TypeAction};
// use crate::node::Node;
// use crate::scorer::Scorer;
// use crate::state::State;
// use crate::utils::{Debug, Timer};
//
// pub struct Smitsimax;
//
// impl Smitsimax {
//     pub fn new() -> Self {
//         Smitsimax
//     }
//     pub fn decide_actions(&self, state: &State, timer: &Timer) -> Vec<Action> {
//
//         let mut actions = Vec::new();
//
//         while !timer.is_time_up() {
//             let mut current_state = state.clone();
//             let mut nodes: [Node; 10] = [Node::new_root(); 10];
//
//             for agent in current_state.agents {
//                 if agent.is_dead {
//                     continue
//                 }
//
//                 let mut node = nodes[(agent.id - 1) as usize];
//
//                 if node.visits == 1 {
//                     let mut actions = current_state.legal_actions_for_agent(&agent);
//                     for action in actions {
//                         node.children.push(Node::new_child(node, action));
//                     }
//                 }
//
//                 // Calcul des scores UCB
//                 // let ucb_scores: Vec<f64> = node.children.iter()
//                 //     .zip(node.visites.iter())
//                 //     .map(|(&avg, &count)| ucb1(avg, total_plays, count))
//                 //     .collect();
//
//             }
//         }
//
//
//         actions
//     }
// }
//
// fn ucb1(avg_reward: f64, total_plays: u32, action_plays: u32) -> f64 {
//     if action_plays == 0 {
//         return f64::INFINITY; // Pour forcer l'exploration au début
//     }
//     avg_reward + ((2.0 * (total_plays as f64).ln()) / (action_plays as f64)).sqrt()
// }

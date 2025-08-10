// use crate::tree::Tree;
// use rand::rngs::StdRng;
// use rand::SeedableRng;
// use crate::action::Action;
// use crate::scorer::Scorer;
// use crate::state::State;
//
// /// Moteur Smitsimax (générique)
// pub struct Smitsimax {
//     trees: Vec<Tree>, // 4 arbres (un par pod)
//     exploration: f64,
//     max_depth: usize,
//     sims: usize,
//     random_pulls: usize,  // ex: 10 (tirages aléatoires initiaux)
//     use_scale: bool,
//     rng: StdRng,
// }
//
// impl Smitsimax {
//     fn new(sims: usize, max_depth: usize, exploration: f64, random_pulls: usize, seed: u64) -> Self {
//         Smitsimax {
//             trees: (0..4).map(|_| Tree::new()).collect(),
//             exploration,
//             max_depth,
//             sims,
//             random_pulls,
//             use_scale: false,
//             rng: StdRng::seed_from_u64(seed),
//         }
//     }
//
//     /// lance la recherche et renvoie les indices d'enfants choisis à la racine (1 move par pod)
//     fn search(&mut self, base_state: &State) -> [Action; 4] {
//         // stat auto : lowest/highest si tu veux normaliser par pod
//         let mut lowest = [f64::INFINITY; 4];
//         let mut highest = [f64::NEG_INFINITY; 4];
//
//         for sim_id in 0..self.sims {
//             let mut state = base_state.clone();
//             // chemin sélectionné courant (index node) pour chaque pod au travers de la profondeur
//             let mut current = vec![0usize; 4];
//
//             for depth in 0..self.max_depth {
//                 let mut actions_to_apply = Vec::new();
//                 // pour chaque agent, sélectionner un enfant
//                 for agent in state.agents.iter() {
//                     let idx = (agent.id - 1) as usize;
//                     // si pas d'enfants, créer enfants à partir de moves possibles
//                     if self.trees[idx].nodes[current[idx]].children.is_empty() {
//                         let moves = state.legal_actions_for_agent(agent);
//                         self.trees[idx].make_children(current[idx], moves);
//                     }
//
//                     // sélectionner : au début aléatoire, ensuite UCB
//                     let chosen_child = if sim_id < self.random_pulls {
//                         // sélection aléatoire globale pendant les random_pulls premières simulations
//                         self.trees[idx].random_child(current[idx], &mut self.rng)
//                     } else {
//                         // compute scale param (optionnel)
//                         let scale = if self.use_scale {
//                             let delta = (highest[idx] - lowest[idx]).max(1e-6);
//                             delta
//                         } else {
//                             1.0
//                         };
//                         self.trees[idx].ucb_select(current[idx], self.exploration, scale)
//                     };
//
//                     // mettre à jour chemin courant
//                     current[idx] = chosen_child;
//
//                     // appliquer la move au pod simulé (on simule plus tard après avoir choisi tous les pods pour le tour)
//                     let action = self.trees[idx].nodes[chosen_child].action.clone();
//                     actions_to_apply.push(&action);
//                 } // fin boucle pods
//
//                 state.apply_actions_all(actions_to_apply);
//             }
//
//             // évaluer la fin de la simulation
//             let scores = Scorer::score(&state); // [f64; 4]
//
//             // mise à jour lowest/highest (si needed)
//             for p in 0..4 {
//                 if scores[p] < lowest[p] { lowest[p] = scores[p]; }
//                 if scores[p] > highest[p] { highest[p] = scores[p]; }
//             }
//
//             // backpropagate : pour chaque pod on remonte depuis current[pod] jusqu'à la racine
//             for pod in 0..4 {
//                 let mut idx = current[pod];
//                 loop {
//                     {
//                         let node = &mut self.trees[pod].nodes[idx];
//                         node.visits += 1;
//                         node.score += scores[pod];
//                     }
//                     // remonter
//                     if let Some(parent_idx) = self.trees[pod].nodes[idx].parent {
//                         idx = parent_idx;
//                     } else {
//                         break;
//                     }
//                 }
//             }
//         } // fin sims
//
//         // Après toutes les simulations : choisir la meilleure child de la racine pour chaque arbre
//         let mut result = [Action::default(); 4];
//         for pod in 0..4 {
//             let root = 0usize;
//             let tree = &self.trees[pod];
//             // choisir le child avec la moyenne la plus haute (ou visites max)
//             let mut best_child = tree.nodes[root].children[0];
//             let mut best_avg = std::f64::NEG_INFINITY;
//             for &c in &tree.nodes[root].children {
//                 let nd = &tree.nodes[c];
//                 let avg = if nd.visits>0 { nd.score / (nd.visits as f64) } else { nd.score };
//                 if avg > best_avg {
//                     best_avg = avg;
//                     best_child = c;
//                 }
//             }
//             result[pod] = tree.nodes[best_child].action.clone();
//         }
//
//         result
//     }
// }
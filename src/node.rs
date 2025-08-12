// use crate::action::Action;
// use crate::state::State;
//
// #[derive(Clone)]
// pub struct Node {
//     pub parent: Option<Box<Node>>,
//     pub children: Vec<Node>,
//     pub visits: usize,
//     pub score: f64, // somme des scores (on divisera par visits pour moyenne)
//     pub action: Option<Action>,
// }
//
// impl Node {
//     pub fn new_root() -> Self {
//         Node {
//             parent: None,
//             children: Vec::new(),
//             visits: 0,
//             score: 0.0,
//             action: None,
//         }
//     }
//
//     pub fn new_child(parent: Node, action: Action) -> Self {
//         Node {
//             parent: Some(Box::from(parent)),
//             children: Vec::new(),
//             visits: 0,
//             score: 0.0,
//             action: Some(action),
//         }
//     }
// }
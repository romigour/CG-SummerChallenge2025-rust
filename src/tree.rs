use crate::action::Action;
use crate::node::Node;
use rand::Rng;

pub struct Tree {
    nodes: Vec<Node>, // indexable, root = 0
}

impl Tree {
    pub fn new() -> Self {
        Tree { nodes: vec![Node::new_root()] }
    }

    fn make_children(&mut self, node_idx: usize, moves: Vec<Action>) {
        // créer un enfant par move
        for mv in moves {
            let child_idx = self.nodes.len();
            let child = Node::new_child(node_idx, mv);
            self.nodes.push(child);
            self.nodes[node_idx].children.push(child_idx);
        }
    }

    fn random_child(&self, node_idx: usize, rng: &mut impl Rng) -> usize {
        let children = &self.nodes[node_idx].children;
        let pick = rng.gen_range(0..children.len());
        children[pick]
    }

    /// Select child via UCB (retourne index du nœud enfant)
    fn ucb_select(&self, node_idx: usize, exploration: f64, scale_param: f64) -> usize {
        let parent_visits = (self.nodes[node_idx].visits.max(1) as f64).max(1.0);
        let mut best_idx = self.nodes[node_idx].children[0];
        let mut best_val = std::f64::NEG_INFINITY;

        for &c in &self.nodes[node_idx].children {
            let child = &self.nodes[c];
            let child_vis = (child.visits.max(1) as f64).max(1.0);
            let avg = child.score / child_vis; // moyenne
            // formule inspirée du post :
            let exploitation = avg / scale_param;
            let exploration_term = exploration * (parent_visits.ln().sqrt() / child_vis.sqrt());
            let ucb = exploitation + exploration_term;

            if ucb > best_val {
                best_val = ucb;
                best_idx = c;
            }
        }
        best_idx
    }
}
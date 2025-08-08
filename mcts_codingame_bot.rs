// mcts_codingame_bot.rs
// Version complète minimisée d'un bot CodinGame en Rust utilisant MCTS
// - Structure : GameInit (fixe), GameTurn (par tour), GameState (combiné)
// - Agents avec actions (Move, Attack, Wait)
// - MCTS basique : sélection (UCT), expansion, simulation (random playout), backpropagation
// - Évaluation simple : somme HP joueur - somme HP adversaire
// Adaptable au moteur réel du jeu : remplacer le parsing et la logique d'application des actions

use rand::prelude::*;
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::time::{Duration, Instant};

// -----------------------------
// agent.rs
// -----------------------------
#[derive(Clone, Debug)]
pub struct Agent {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub team: Team,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Team {
    Me,
    Enemy,
}

#[derive(Clone, Debug)]
pub enum Action {
    Move(i32, i32),
    Attack(u32), // target id
    Wait,
}

impl Action {
    pub fn as_output(&self) -> String {
        match self {
            Action::Move(x, y) => format!("MOVE {} {}", x, y),
            Action::Attack(id) => format!("ATTACK {}", id),
            Action::Wait => "WAIT".to_string(),
        }
    }
}

// -----------------------------
// state
// -----------------------------
#[derive(Clone, Debug)]
pub struct GameInit {
    pub map_w: i32,
    pub map_h: i32,
    // obstacles, terrain, constants...
}

impl GameInit {
    pub fn from_input() -> Self {
        // Placeholder parsing: in a real game read map and constants
        // Here we just read two ints if present, otherwise defaults
        let mut lines = stdin_lines();
        let first = lines.next();
        if let Some(ln) = first {
            let parts: Vec<_> = ln.split_whitespace().collect();
            if parts.len() >= 2 {
                if let (Ok(w), Ok(h)) = (parts[0].parse(), parts[1].parse()) {
                    return GameInit { map_w: w, map_h: h };
                }
            }
        }
        GameInit { map_w: 20, map_h: 20 }
    }
}

#[derive(Clone, Debug)]
pub struct GameTurn {
    pub my_agents: Vec<Agent>,
    pub enemy_agents: Vec<Agent>,
    pub turn: u32,
}

impl GameTurn {
    pub fn from_input() -> Self {
        // For CodinGame: parse per-turn lines. Here we support a simple format:
        // first line: turn_number
        // then: N (my agents), followed by N lines: id x y hp
        // then: M (enemy agents), followed by M lines: id x y hp
        let mut lines = stdin_lines();
        let mut turn = 0u32;
        if let Some(ln) = lines.next() {
            turn = ln.trim().parse().unwrap_or(0);
        }
        let mut my_agents = Vec::new();
        let mut enemy_agents = Vec::new();
        if let Some(ln) = lines.next() {
            let n: usize = ln.trim().parse().unwrap_or(0);
            for _ in 0..n {
                if let Some(ln) = lines.next() {
                    let parts: Vec<_> = ln.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let id: u32 = parts[0].parse().unwrap_or(0);
                        let x: i32 = parts[1].parse().unwrap_or(0);
                        let y: i32 = parts[2].parse().unwrap_or(0);
                        let hp: i32 = parts[3].parse().unwrap_or(0);
                        my_agents.push(Agent { id, x, y, hp, team: Team::Me });
                    }
                }
            }
        }
        if let Some(ln) = lines.next() {
            let m: usize = ln.trim().parse().unwrap_or(0);
            for _ in 0..m {
                if let Some(ln) = lines.next() {
                    let parts: Vec<_> = ln.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let id: u32 = parts[0].parse().unwrap_or(0);
                        let x: i32 = parts[1].parse().unwrap_or(0);
                        let y: i32 = parts[2].parse().unwrap_or(0);
                        let hp: i32 = parts[3].parse().unwrap_or(0);
                        enemy_agents.push(Agent { id, x, y, hp, team: Team::Enemy });
                    }
                }
            }
        }
        GameTurn { my_agents, enemy_agents, turn }
    }
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub init: GameInit,
    pub my_agents: Vec<Agent>,
    pub enemy_agents: Vec<Agent>,
    pub turn: u32,
}

impl GameState {
    pub fn new(init: &GameInit, turn: &GameTurn) -> Self {
        Self { init: init.clone(), my_agents: turn.my_agents.clone(), enemy_agents: turn.enemy_agents.clone(), turn: turn.turn }
    }

    pub fn is_terminal(&self) -> bool {
        self.my_agents.iter().all(|a| a.hp <= 0) || self.enemy_agents.iter().all(|a| a.hp <= 0)
    }

    pub fn evaluate(&self) -> f32 {
        let my_hp: i32 = self.my_agents.iter().map(|a| a.hp.max(0)).sum();
        let enemy_hp: i32 = self.enemy_agents.iter().map(|a| a.hp.max(0)).sum();
        (my_hp - enemy_hp) as f32
    }

    pub fn legal_actions_for_agent(&self, agent: &Agent) -> Vec<Action> {
        // Simple action set: stay, move in 4 directions within map, attack nearest enemy if within range 1
        let mut actions = Vec::new();
        actions.push(Action::Wait);
        let dirs = vec![(0,1),(0,-1),(1,0),(-1,0)];
        for (dx,dy) in dirs {
            let nx = agent.x + dx;
            let ny = agent.y + dy;
            if nx >= 0 && nx < self.init.map_w && ny >= 0 && ny < self.init.map_h {
                actions.push(Action::Move(nx, ny));
            }
        }
        // attack if any enemy adjacent
        let enemies = if agent.team == Team::Me { &self.enemy_agents } else { &self.my_agents };
        for e in enemies {
            let dist = (e.x - agent.x).abs() + (e.y - agent.y).abs();
            if dist <= 1 && e.hp > 0 {
                actions.push(Action::Attack(e.id));
            }
        }
        actions
    }

    pub fn legal_joint_actions(&self) -> Vec<Vec<Action>> {
        // For my agents only: returns a list of combinations (cartesian product) of actions per agent
        // WARNING: combinatorial explosion. We will cap by sampling below in MCTS.
        let mut lists: Vec<Vec<Action>> = Vec::new();
        for a in &self.my_agents {
            if a.hp <= 0 { lists.push(vec![Action::Wait]); continue; }
            lists.push(self.legal_actions_for_agent(a));
        }
        cartesian_product(&lists)
    }

    pub fn apply_joint_actions(&mut self, my_actions: &[Action], enemy_actions: Option<&[Action]>) {
        // Apply movements first
        // my_actions length should match number of my_agents; enemy_actions similarly if provided
        for (i, act) in my_actions.iter().enumerate() {
            if let Some(agent) = self.my_agents.get_mut(i) {
                if agent.hp <= 0 { continue; }
                match act {
                    Action::Move(nx, ny) => { agent.x = *nx; agent.y = *ny; }
                    _ => {}
                }
            }
        }
        if let Some(eacts) = enemy_actions {
            for (i, act) in eacts.iter().enumerate() {
                if let Some(agent) = self.enemy_agents.get_mut(i) {
                    if agent.hp <= 0 { continue; }
                    match act {
                        Action::Move(nx, ny) => { agent.x = *nx; agent.y = *ny; }
                        _ => {}
                    }
                }
            }
        }
        // Attacks: collect hits
        // For simplicity: each attack deals 10 damage
        let dmg = 10;
        // process my attacks
        for (i, act) in my_actions.iter().enumerate() {
            if let Some(att) = act {
                match att {
                    Action::Attack(target_id) => {
                        if let Some(target) = self.enemy_agents.iter_mut().find(|t| t.id == *target_id) {
                            if target.hp > 0 { target.hp -= dmg; }
                        }
                    }
                    _ => {}
                }
            }
        }
        // process enemy attacks
        if let Some(eacts) = enemy_actions {
            for (i, act) in eacts.iter().enumerate() {
                if let Some(att) = act {
                    match att {
                        Action::Attack(target_id) => {
                            if let Some(target) = self.my_agents.iter_mut().find(|t| t.id == *target_id) {
                                if target.hp > 0 { target.hp -= dmg; }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        self.turn += 1;
    }
}

// -----------------------------
// mcts/mod.rs (simple implementation)
// -----------------------------

#[derive(Clone)]
struct Node {
    state: GameState,
    parent: Option<usize>,
    children: Vec<usize>,
    visits: f32,
    value: f32,
    prior_joint_action: Option<Vec<Action>>,
}

struct MCTSTree {
    nodes: Vec<Node>,
}

impl MCTSTree {
    fn new(root_state: GameState) -> Self {
        let root = Node { state: root_state, parent: None, children: Vec::new(), visits: 0.0, value: 0.0, prior_joint_action: None };
        Self { nodes: vec![root] }
    }

    fn selection_ucb(&self, node_idx: usize, c: f32) -> Option<usize> {
        // choose child maximizing UCT
        let node = &self.nodes[node_idx];
        if node.children.is_empty() { return None; }
        let mut best = None;
        let mut best_score = f32::NEG_INFINITY;
        for &ci in &node.children {
            let child = &self.nodes[ci];
            if child.visits <= 0.0 {
                // favor unexplored
                return Some(ci);
            }
            let exploit = child.value / child.visits;
            let explore = c * ((node.visits.ln() / child.visits).sqrt());
            let score = exploit + explore;
            if score > best_score {
                best_score = score; best = Some(ci);
            }
        }
        best
    }

    fn expand(&mut self, node_idx: usize, joint_actions: Vec<Vec<Action>>) -> Option<usize> {
        // expand one child per joint action (or sample limited)
        if joint_actions.is_empty() { return None; }
        let mut created_idx = None;
        for joint in joint_actions.into_iter() {
            let mut new_state = self.nodes[node_idx].state.clone();
            // For expansion we need enemy actions: sample random
            let enemy_actions = sample_enemy_actions(&new_state);
            new_state.apply_joint_actions(&joint, Some(&enemy_actions));
            let node = Node { state: new_state, parent: Some(node_idx), children: Vec::new(), visits: 0.0, value: 0.0, prior_joint_action: Some(joint.clone()) };
            self.nodes.push(node);
            let idx = self.nodes.len() - 1;
            self.nodes[node_idx].children.push(idx);
            if created_idx.is_none() { created_idx = Some(idx); }
        }
        created_idx
    }

    fn backup(&mut self, mut idx: usize, value: f32) {
        loop {
            self.nodes[idx].visits += 1.0;
            self.nodes[idx].value += value;
            if let Some(p) = self.nodes[idx].parent {
                idx = p;
            } else { break; }
        }
    }
}

pub struct MCTS {
    pub iter_limit: usize,
    pub time_limit: Duration,
    pub c: f32,
}

impl MCTS {
    pub fn new(iter_limit: usize, time_limit_ms: u64) -> Self {
        Self { iter_limit, time_limit: Duration::from_millis(time_limit_ms), c: 1.4 }
    }

    pub fn search(&self, root_state: &GameState) -> Vec<Action> {
        let mut tree = MCTSTree::new(root_state.clone());
        let start = Instant::now();
        let mut iters = 0usize;
        while iters < self.iter_limit && start.elapsed() < self.time_limit {
            iters += 1;
            // 1. selection
            let mut node_idx = 0usize;
            loop {
                if tree.nodes[node_idx].state.is_terminal() { break; }
                if tree.nodes[node_idx].children.is_empty() { break; }
                if let Some(next) = tree.selection_ucb(node_idx, self.c) {
                    node_idx = next;
                } else { break; }
            }
            // 2. expansion
            if !tree.nodes[node_idx].state.is_terminal() {
                // generate a limited set of joint actions (sampled) to expand
                let joint_actions = sample_joint_actions_limited(&tree.nodes[node_idx].state, 10);
                if !joint_actions.is_empty() {
                    if let Some(child_idx) = tree.expand(node_idx, joint_actions) {
                        node_idx = child_idx;
                    }
                }
            }
            // 3. simulation from node_idx
            let value = rollout(&tree.nodes[node_idx].state, 10);
            // 4. backpropagate
            tree.backup(node_idx, value);
        }
        // choose best child of root by highest visits
        if tree.nodes[0].children.is_empty() {
            // fallback: pick greedy legal action set
            if let Some(joint) = greedy_joint_action(root_state) {
                return joint;
            }
            return Vec::new();
        }
        let mut best = None;
        let mut best_vis = -1.0f32;
        for &ci in &tree.nodes[0].children {
            let n = &tree.nodes[ci];
            if n.visits > best_vis {
                best_vis = n.visits; best = Some(ci);
            }
        }
        if let Some(bi) = best {
            if let Some(j) = &tree.nodes[bi].prior_joint_action {
                return j.clone();
            }
        }
        Vec::new()
    }
}

// -----------------------------
// Helpers: sampling, rollout
// -----------------------------

fn sample_enemy_actions(state: &GameState) -> Vec<Action> {
    // naive: for each enemy, choose random legal action
    let mut rng = thread_rng();
    let mut res = Vec::new();
    for e in &state.enemy_agents {
        if e.hp <= 0 { res.push(Action::Wait); continue; }
        let acts = state.legal_actions_for_agent(e);
        if acts.is_empty() { res.push(Action::Wait); }
        else { res.push(acts[rng.gen_range(0..acts.len())].clone()); }
    }
    res
}

fn sample_joint_actions_limited(state: &GameState, max_samples: usize) -> Vec<Vec<Action>> {
    // We avoid enumerating full cartesian product. Instead, sample joint actions by sampling per-agent actions.
    let mut rng = thread_rng();
    let n = state.my_agents.len();
    let mut samples = Vec::new();
    for _ in 0..max_samples {
        let mut joint = Vec::new();
        for a in &state.my_agents {
            if a.hp <= 0 { joint.push(Action::Wait); continue; }
            let acts = state.legal_actions_for_agent(a);
            if acts.is_empty() { joint.push(Action::Wait); }
            else { joint.push(acts[rng.gen_range(0..acts.len())].clone()); }
        }
        samples.push(joint);
    }
    // optionally deduplicate
    samples
}

fn rollout(state: &GameState, max_depth: usize) -> f32 {
    let mut s = state.clone();
    let mut rng = thread_rng();
    for _d in 0..max_depth {
        if s.is_terminal() { break; }
        // sample my joint actions and enemy actions
        let my_actions = sample_joint_actions_limited(&s, 1).pop().unwrap_or_else(|| Vec::new());
        let enemy_actions = sample_enemy_actions(&s);
        s.apply_joint_actions(&my_actions, Some(&enemy_actions));
    }
    s.evaluate()
}

fn greedy_joint_action(state: &GameState) -> Option<Vec<Action>> {
    // simple heuristic: each agent attacks if adjacent else move towards nearest enemy
    let mut joint = Vec::new();
    for a in &state.my_agents {
        if a.hp <= 0 { joint.push(Action::Wait); continue; }
        // find adjacent enemy
        if let Some(e) = state.enemy_agents.iter().find(|en| (en.x - a.x).abs() + (en.y - a.y).abs() <= 1 && en.hp > 0) {
            joint.push(Action::Attack(e.id)); continue;
        }
        if let Some(nearest) = nearest_enemy(state, a) {
            let dx = (nearest.x - a.x).signum();
            let dy = (nearest.y - a.y).signum();
            let nx = a.x + dx; let ny = a.y + dy;
            joint.push(Action::Move(nx, ny));
        } else { joint.push(Action::Wait); }
    }
    Some(joint)
}

fn nearest_enemy(state: &GameState, a: &Agent) -> Option<Agent> {
    state.enemy_agents.iter().filter(|e| e.hp > 0).min_by(|e1,e2| {
        let d1 = (e1.x - a.x).abs() + (e1.y - a.y).abs();
        let d2 = (e2.x - a.x).abs() + (e2.y - a.y).abs();
        d1.cmp(&d2)
    }).cloned()
}

// Utility: cartesian product of Vec<Vec<T>>
fn cartesian_product(lists: &Vec<Vec<Action>>) -> Vec<Vec<Action>> {
    let mut res: Vec<Vec<Action>> = vec![Vec::new()];
    for pool in lists {
        let mut tmp = Vec::new();
        for prefix in &res {
            for item in pool {
                let mut np = prefix.clone();
                np.push(item.clone());
                tmp.push(np);
            }
        }
        res = tmp;
    }
    res
}

// read stdin lines lazily (helper used in parsing) -- reads all stdin at once, returns iterator over lines
fn stdin_lines() -> impl Iterator<Item = String> {
    let stdin = io::stdin();
    let mut data = String::new();
    for line in stdin.lock().lines() {
        if let Ok(ln) = line { data.push_str(&ln); data.push('\n'); }
    }
    data.lines().map(|s| s.to_string()).collect::<Vec<_>>().into_iter()
}

// -----------------------------
// main.rs
// -----------------------------
fn main() {
    // Read initial input once
    let init = GameInit::from_input();

    // Create MCTS engine
    let mcts = MCTS::new(800, 90); // 800 iterations or 90 ms

    // Game loop: in a real CodinGame environment you would read per-turn inputs from stdin
    loop {
        // Try to read a turn; if none, break
        // For local testing we expect stdin to contain the whole sequence; our helper reads all lines at once.
        // So for a real loop you would parse line-by-line. Here we'll craft a simple interactive flow:
        let turn_state = GameTurn::from_input();
        // If no agents present, end
        if turn_state.my_agents.is_empty() && turn_state.enemy_agents.is_empty() { break; }
        let game_state = GameState::new(&init, &turn_state);
        let joint = mcts.search(&game_state);
        // Output one action per my agent (in order)
        if joint.is_empty() {
            // fallback
            if let Some(g) = greedy_joint_action(&game_state) {
                for a in g { println!("{}", a.as_output()); }
            } else {
                // no actions
                println!("WAIT");
            }
        } else {
            for a in joint { println!("{}", a.as_output()); }
        }
    }
}

// -----------------------------
// Notes:
// - This program is a template. To adapt to a real CodinGame problem, replace parsing functions with the exact input format.
// - The apply_joint_actions logic is intentionally simplified. You should implement the true game rules (order of actions, collisions, damage formulas, cooldowns...).
// - Tuning: iterations, time_limit, rollout depth, and evaluation should be tuned to the specific game.
// - Performance: use arena allocation, reduce cloning, and optimize state representation for speed in real contests.
// Good luck !

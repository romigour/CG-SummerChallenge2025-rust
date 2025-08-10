// Généré à 17:10:54 le 10-08-2025
mod action {
    #[derive(Clone, Debug, Copy, PartialEq, Eq)]
    pub enum TypeAction {
        Throw,
        Shoot,
        HunkerDown,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Action {
        pub id: i32,
        pub mx: i32,
        pub my: i32,
        pub type_action: TypeAction,
        pub x: i32,
        pub y: i32,
        pub enemy_id: i32,
        pub score: i32,
    }
    
    impl Action {
        pub fn new(id: i32, mx: i32, my: i32, type_action: TypeAction, x: i32, y: i32, enemy_id: i32, score: i32) -> Self {
            Action { id, mx, my, type_action, x, y, enemy_id, score }
        }
    
        pub fn shoot(id: i32, mx: i32, my: i32, enemy_id: i32, score: i32) -> Self {
            Self::new(id, mx, my, TypeAction::Shoot, 0, 0, enemy_id, score)
        }
    
        pub fn throw(id: i32, mx: i32, my: i32, x: i32, y: i32, score: i32) -> Self {
            Self::new(id, mx, my, TypeAction::Throw, x, y, 0, score)
        }
    
        pub fn hunker_down(id: i32, mx: i32, my: i32) -> Self {
            Self::new(id, mx, my, TypeAction::HunkerDown, 0, 0, 0, 0)
        }
    
        pub fn display(&self) -> String {
            match self.type_action {
                TypeAction::Throw => format!("{};MOVE {} {};THROW {} {}", self.id, self.mx, self.my, self.x, self.y),
                TypeAction::Shoot => format!("{};MOVE {} {};SHOOT {}", self.id, self.mx, self.my, self.enemy_id),
                TypeAction::HunkerDown => format!("{};MOVE {} {};HUNKER_DOWN", self.id, self.mx, self.my),
            }
        }
    }
}
mod state {
    use crate::action::{Action, TypeAction};
    use crate::agent::{Agent, Team};
    use crate::grid::Grid;
    
    use crate::io_helper::InputSource;
    use crate::utils::{Debug, Math};
    
    macro_rules! parse_input {
        ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
    }
    
    #[derive(Clone, Debug)]
    pub struct State {
        pub turn: i32,
        pub my_id: i32,
        pub width: i32,
        pub height: i32,
        pub agent_data_count: i32,
        pub my_idx_arr: Vec<usize>,
        pub enemy_idx_arr: Vec<usize>,
        pub grid: Grid,
        pub agents: [Agent; 10],
    }
    
    
    impl State {
    
        pub fn new() -> Self {
            Self {
                turn: 0,
                my_id: 0,
                width: 0,
                height: 0,
                agent_data_count: 0,
                my_idx_arr: Vec::new(),
                enemy_idx_arr: Vec::new(),
                grid: Grid::new(0, 0),
                agents: [Agent::default(); 10],
            }
        }
    
        pub fn init_input(state: &mut State, input: &mut InputSource) {
            let mut input_line = String::new();
            input.read_line(&mut input_line).unwrap();
            let my_id = parse_input!(input_line, i32);
            state.my_id = my_id;
            let mut input_line = String::new();
            input.read_line(&mut input_line).unwrap();
            let agent_data_count = parse_input!(input_line, i32);
            state.agent_data_count = agent_data_count;
            for _ in 0..agent_data_count as usize {
                let mut input_line = String::new();
                input.read_line(&mut input_line).unwrap();
                let inputs = input_line.split(" ").collect::<Vec<_>>();
                let agent_id = parse_input!(inputs[0], i32);
                let player = parse_input!(inputs[1], i32);
                let shoot_cooldown = parse_input!(inputs[2], i32);
                let optimal_range = parse_input!(inputs[3], i32);
                let soaking_power = parse_input!(inputs[4], i32);
                let splash_bombs = parse_input!(inputs[5], i32);
    
                let agent = Agent {
                    id: agent_id,
                    x: 0,
                    y: 0,
                    shoot_cooldown,
                    optimal_range,
                    soaking_power,
                    splash_bombs,
                    cooldown: 0,
                    wetness: 0,
                    team: if player == my_id { Team::Me } else { Team::Enemy },
                    is_dead: false,
                    hunker_down: false,
                };
    
                let agent_idx = (agent_id - 1) as usize;
                if agent.team == Team::Me {
                    state.my_idx_arr.push(agent_idx);
                } else {
                    state.enemy_idx_arr.push(agent_idx);
                }
                state.agents[agent_idx] = agent;
            }
            let mut input_line = String::new();
            input.read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let width = parse_input!(inputs[0], i32);
            state.width = width;
            let height = parse_input!(inputs[1], i32);
            state.height = height;
            state.grid = Grid::new(width as usize, height as usize);
            for _ in 0..height as usize {
                let mut input_line = String::new();
                input.read_line(&mut input_line).unwrap();
                let inputs = input_line.split_whitespace().collect::<Vec<_>>();
                for j in 0..width as usize {
                    let x = parse_input!(inputs[3*j], usize);
                    let y = parse_input!(inputs[3*j+1], usize);
                    let tile_type = parse_input!(inputs[3*j+2], i32);
                    state.grid.set(x, y, tile_type);
                }
            }
        }
    
        pub fn update_input(state: &mut State, input: &mut InputSource) {
            state.turn += 1;
            let mut input_line = String::new();
            input.read_line(&mut input_line).unwrap();
            let agent_count = parse_input!(input_line, i32); // Total number of agents still in the game
            for i in 0..10 {
                state.agents[i].is_dead = true;
            }
    
            for _ in 0..agent_count as usize {
                let mut input_line = String::new();
                input.read_line(&mut input_line).unwrap();
                let inputs = input_line.split(" ").collect::<Vec<_>>();
                let agent_id = parse_input!(inputs[0], i32);
                let x = parse_input!(inputs[1], i32);
                let y = parse_input!(inputs[2], i32);
                let cooldown = parse_input!(inputs[3], i32); // Number of turns before this agent can shoot
                let splash_bombs = parse_input!(inputs[4], i32);
                let wetness = parse_input!(inputs[5], i32); // Damage (0-100) this agent has taken
    
                let agent_idx = (agent_id - 1) as usize;
                state.agents[agent_idx].is_dead = false;
                state.agents[agent_idx].x = x;
                state.agents[agent_idx].y = y;
                state.agents[agent_idx].cooldown = cooldown;
                state.agents[agent_idx].splash_bombs = splash_bombs;
                state.agents[agent_idx].wetness = wetness;
            }
            let mut input_line = String::new();
            input.read_line(&mut input_line).unwrap();
            let my_agent_count = parse_input!(input_line, i32);
        }
    
        pub fn is_terminal(&self) -> bool {
            false
        }
    
        pub fn evaluate(&self) -> f32 {
            0.0
        }
    
        pub fn legal_actions_for_agent(&self, agent: &Agent) -> Vec<Action> {
            let mut actions = Vec::new();
    
            let dirs: Vec<(i32, i32)> = vec![(1,0),(0,1),(0,-1),(-1,0),(0,0)];
    
            for (dx,dy) in dirs {
                let nx = agent.x + dx;
                let ny = agent.y + dy;
                if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height {
    
                    if self.grid.get(nx as usize, ny as usize) > 0 {
                        continue
                    }
    
                    if agent.team == Team::Me {
                        // TODO quand on voudra gérer les actions des ennemis, il faudra modifier ces boucles:
                        // for enemy_idx in &self.enemy_idx_arr
    
                        // THROW
                        if agent.splash_bombs > 0 {
                            for enemy_idx in &self.enemy_idx_arr {
                                let enemy = self.agents[*enemy_idx];
                                let dist = Math::manhattan(nx, ny, enemy.x, enemy.y);
                                if dist <= 4 {
                                    actions.push(Action::throw(agent.id, nx, ny, enemy.x, enemy.y, 100 - enemy.wetness));
                                }
                            }
                        }
    
                        // SHOOT
                        if agent.cooldown <= 0 {
                            for enemy_idx in &self.enemy_idx_arr {
                                let enemy = &self.agents[*enemy_idx];
                                if enemy.is_dead {
                                    continue;
                                }
    
                                let dist = Math::manhattan(nx, ny, enemy.x, enemy.y);
                                if dist > agent.optimal_range * 2 {
                                    continue
                                }
    
                                let mut bonus = 0;
                                if dist < agent.optimal_range {
                                    bonus = 10;
                                }
    
                                let score = dist + bonus;
                                actions.push(Action::shoot(agent.id, nx, ny, enemy.id, score));
                            }
                        }
                    }
                    actions.push(Action::hunker_down(agent.id, nx, ny));
                }
            }
    
            actions
        }
    
        pub fn legal_joint_actions(&self) -> Vec<Vec<Action>> {
            // For my agents only: returns a list of combinations (cartesian product) of actions per agent
            // WARNING: combinatorial explosion. We will cap by sampling below in MCTS.
            // let mut lists: Vec<Vec<Action>> = Vec::new();
            // for a in &self.my_agents {
            //     if a.hp <= 0 { lists.push(vec![Action::Wait]); continue; }
            //     lists.push(self.legal_actions_for_agent(a));
            // }
            // cartesian_product(&lists)
            Vec::new()
        }
    
        pub fn apply_actions(&mut self, action: Action) {
            self.turn += 1;
    
            let agent = &mut self.agents[action.id as usize - 1];
            agent.x = action.mx;
            agent.y = action.my;
            let agent_x = agent.x;
            let agent_y = agent.y;
            let agent_cooldown = agent.cooldown;
            let agent_optimal_range = agent.optimal_range;
            let agent_soaking_power = agent.soaking_power as f32;
    
            agent.hunker_down = false;
            if action.type_action == TypeAction::HunkerDown {
                agent.hunker_down = true;
            } else if action.type_action == TypeAction::Throw {
                agent.splash_bombs -= 1;
                for a in self.agents.iter_mut() {
    
                    let dx = (a.x - action.x).abs();
                    let dy = (a.y - action.y).abs();
                    if dx <= 1 && dy <= 1 {
                        a.wetness += 30;
                    }
                }
    
            } else if action.type_action == TypeAction::Shoot {
                agent.cooldown = agent_cooldown;
                let (enemy_agent_x, enemy_agent_y, enemy_hunker_down) = {
                    let enemy_agent = &self.agents[action.enemy_id as usize - 1];
                    (enemy_agent.x, enemy_agent.y, enemy_agent.hunker_down)
                };
    
                let range_modifier = self.get_range_modifier(agent_x, agent_y, agent_optimal_range, enemy_agent_x, enemy_agent_y);
                let cover_modifier = self.get_cover_modifier(agent_x, agent_y, enemy_agent_x, enemy_agent_y);
                let hunker_down_bonus = if enemy_hunker_down { 0.25 } else { 0.0 };
    
                let enemy_agent = &mut self.agents[action.enemy_id as usize - 1];
                enemy_agent.wetness += (agent_soaking_power * range_modifier * (1.0 - hunker_down_bonus)) as i32;
                //enemy_agent.wetness += (agent_soaking_power) as i32;
            }
        }
    
        pub fn get_cover_modifier(&self, shooter_x: i32, shooter_y: i32, target_x: i32, target_y: i32) -> f32 {
            let dx = target_x - shooter_y;
            let dy = target_y - shooter_y;
            let dirs: Vec<(i32, i32)> = vec![(dx, 0), (0, dy)];
    
            let mut best_modifier = 1.0;
    
            for (dx, dy) in dirs {
                if dx.abs() > 1 || dy.abs() > 1 {
                    let adj_x = -dx.signum();
                    let adj_y = -dy.signum();
                    let cover_pos_x = target_x + adj_x;
                    let cover_pos_y = target_y + adj_y;
    
                    if Math::chebyshev_to(target_x, target_y, shooter_x, shooter_y) > 1 {
                        let mut cover_modifier = 1.0;
                        if (self.grid.get(cover_pos_x as usize, cover_pos_y as usize)) == 1 {
                            cover_modifier = 0.5;
                        } else if (self.grid.get(cover_pos_x as usize, cover_pos_y as usize)) == 2 {
                            cover_modifier = 0.25;
                        }
                        best_modifier = if best_modifier > cover_modifier { cover_modifier } else { best_modifier };
                    }
                }
            }
    
            best_modifier
        }
        // pub fn get_range_modifier(&self, shooter: Agent, target: Agent) -> f32 {
        pub fn get_range_modifier(&self, agent_x: i32, agent_y: i32, agent_optimal_shoot: i32, enemy_agent_x: i32, enemy_agent_y: i32) -> f32 {
            let dist = Math::manhattan(agent_x, agent_y, enemy_agent_x, enemy_agent_y);
            if dist <= agent_optimal_shoot {
                1.0
            } else if dist <= agent_optimal_shoot * 2 {
                0.5
            } else {
                0.0
            }
        }
        pub fn calcul_zone_couverture(&self, agent_id: i32, nx: i32, ny: i32) -> i32 {
            let mut my_zones = 0;
            let mut enemy_zones = 0;
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.grid.get(x as usize, y as usize) > 0 {
                        continue;
                    }
    
                    let mut dmy = 9999;
                    let mut denemy = 9999;
    
                    for my_idx in &self.my_idx_arr {
                        let my_agent = &self.agents[*my_idx];
                        if my_agent.is_dead {
                            continue;
                        }
    
                        let mut distance = Math::manhattan(my_agent.x, my_agent.y, x, y);
                        if my_agent.wetness >= 50 {
                            distance *= 2;
                        }
    
                        if distance < dmy {
                            dmy = distance;
                        }
                    }
    
                    for enemy_idx in &self.enemy_idx_arr {
                        let enemy = &self.agents[*enemy_idx];
                        if enemy.is_dead {
                            continue;
                        }
                        let mut distance = Math::manhattan(enemy.x, enemy.y, x, y);
                        if enemy.wetness >= 50 {
                            distance *= 2;
                        }
    
                        if distance < denemy {
                            denemy = distance;
                        }
                    }
    
                    if dmy < denemy {
                        my_zones += 1;
                    } else if denemy < dmy {
                        enemy_zones += 1;
                    }
                }
            }
            my_zones - enemy_zones
        }
    
        pub fn play(&self, actions: Vec<Action>) {
            for action in actions {
                println!("{}", action.display());
            }
        }
    }
}
mod agent {
    #[derive(Clone, Debug, Default, Copy)]
    pub struct Agent {
        pub id: i32,
        pub x: i32,
        pub y: i32,
        pub shoot_cooldown: i32,
        pub optimal_range: i32,
        pub soaking_power: i32,
        pub splash_bombs: i32,
        pub cooldown: i32,
        pub wetness: i32,
        pub team: Team,
        pub hunker_down: bool,
        pub is_dead: bool,
    }
    
    #[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
    pub enum Team {
        #[default]
        Me,
        Enemy,
    }
}
mod utils {
    use std::eprintln;
    use std::time::{Duration, Instant};
    
    pub struct Timer {
        start: Instant,
        limit: Duration,
    }
    
    impl Timer {
        pub fn new(limit: Duration) -> Self {
            Self { start: Instant::now(), limit }
        }
        pub fn start(&mut self) {
            self.start = Instant::now();
        }
    
        pub fn time(&self) -> f64 {
            let duration = Instant::now().duration_since(self.start);
            duration.as_micros() as f64 / 1000.0
        }
        pub fn is_time_up(&self) -> bool {
            Instant::now().duration_since(self.start) >= self.limit
        }
    }
    
    pub struct Debug {
    }
    
    impl Debug {
    
        pub fn debug_simple(value: String) {
            eprintln!("{:?}", value)
        }
    
        pub fn debug(label: &str, params: &[(&str, String)]) {
            eprintln!("=== {} ===", label);
            for (name, value) in params {
                eprintln!(" {}: {}", name, value);
            }
            eprintln!();
        }
    
        pub fn debug_vec<T: std::fmt::Debug>(label: &str, values: &[T]) {
            eprintln!("=== {} ===", label);
            for value in values.iter() {
                eprintln!("{:?}", value);
            }
            eprintln!();
        }
    }
    
    pub struct Math {
    }
    
    impl Math {
        pub fn manhattan(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
            (x1 - x2).abs() + (y1 - y2).abs()
        }
    
        pub fn chebyshev_to(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
            let dx = (x2 - x1).abs();
            let dy = (y2 - y1).abs();
            dx.max(dy)
        }
    }
}
mod grid {
    #[derive(Clone, Debug)]
    pub struct Grid {
        width: usize,
        height: usize,
        data: Vec<i32>,
    }
    
    impl Grid {
        pub(crate) fn new(width: usize, height: usize) -> Self {
            Self {
                width,
                height,
                data: vec![0; width * height],
            }
        }
    
        pub fn index(&self, x: usize, y: usize) -> usize {
            y * self.width + x
        }
    
        pub fn get(&self, x: usize, y: usize) -> i32 {
            self.data[self.index(x, y)]
        }
    
        pub fn set(&mut self, x: usize, y: usize, value: i32) {
            let idx = self.index(x, y);
            self.data[idx] = value;
        }
    }
}
mod scorer {
    use crate::agent::Team;
    use crate::state::State;
    use crate::utils::Debug;
    
    pub struct Scorer;
    
    impl Scorer {
        pub fn score(state: State) -> i32 {
            let mut my_position_score = 0;
            let mut my_wetness_score = 0;
            let mut enemy_wetness_score = 0;
            let mut nb_my_50wetness = 0;
            let mut nb_my_100wetness = 0;
            let mut nb_enemy_50wetness = 0;
            let mut nb_enemy_100wetness = 0;
    
            let zones = state.calcul_zone_couverture(0, 0, 0);
    
            //Debug::debug_simple(format!("zone {:?}", state.calcul_zone_couverture(0, 0, 0) * 100));
            for agent in &state.agents {
                if agent.is_dead {
                    continue
                }
    
                if agent.team == Team::Me {
                    my_wetness_score += agent.wetness;
                    if agent.wetness >= 50 {
                        nb_my_50wetness += 1
                    }
                    if agent.wetness >= 100 {
                        nb_my_100wetness += 1;
                    }
                    my_position_score += state.width - ((state.width / 2) - agent.x).abs();
                    my_position_score += state.height - ((state.height / 2) - agent.y).abs();
    
                    // for enemy in &state.agents {
                    //     if enemy.player != state.my_id {
                    //         let distance = ((agent.x - enemy.x).pow(2) + (agent.y - enemy.y).pow(2)) as f64;
                    //         score += (1000.0 / (1.0 + distance.sqrt())) as i32; // Closer enemies give more score
                    //     }
                    // }
                } else {
                    enemy_wetness_score += agent.wetness;
                    if agent.wetness >= 50 {
                        nb_enemy_50wetness += 1
                    }
                    if agent.wetness >= 100 {
                        nb_enemy_100wetness += 1;
                    }
                }
            }
    
            let mut score = 0;
            score += zones * 10;
    
           // score += my_position_score /10;
    
            score -= my_wetness_score * 1000;
            score -= nb_my_50wetness * 10000;
            score -= nb_my_100wetness * 100000;
    
            score += (enemy_wetness_score * 10) / 100;
            score += nb_enemy_50wetness * 100;
            score += nb_enemy_100wetness * 1000;
    
            score
    
        }
    }
}
mod ia {
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
}
mod node {
    use crate::action::Action;
    
    pub struct Node {
        pub parent: Option<usize>,
        pub children: Vec<usize>,
        pub visits: usize,
        pub score: f64, // somme des scores (on divisera par visits pour moyenne)
        pub action: Option<Action>,
    }
    
    impl Node {
        pub fn new_root() -> Self {
            Node {
                parent: None,
                children: Vec::new(),
                visits: 0,
                score: 0.0,
                action: None,
            }
        }
    
        pub fn new_child(parent: usize, action: Action) -> Self {
            Node {
                parent: Some(parent),
                children: Vec::new(),
                visits: 0,
                score: 0.0,
                action: Some(action),
            }
        }
    }
}
mod tree {
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
}
mod smitsimax {
    // use crate::tree::Tree;
    // use rand::rngs::StdRng;
    // use rand::SeedableRng;
    // use crate::action::Action;
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
    //                 // pour chaque pod, sélectionner un enfant
    //                 for agent in state.agents.iter() {
    //                     // si pas d'enfants, créer enfants à partir de moves possibles
    //                     if self.trees[pod].nodes[current[pod]].children.is_empty() {
    //                         let moves = state.legal_actions_for_agent(agent);
    //                         self.trees[pod].make_children(current[pod], moves);
    //                     }
    //
    //                     // sélectionner : au début aléatoire, ensuite UCB
    //                     let chosen_child = if sim_id < self.random_pulls {
    //                         // sélection aléatoire globale pendant les random_pulls premières simulations
    //                         self.trees[pod].random_child(current[pod], &mut self.rng)
    //                     } else {
    //                         // compute scale param (optionnel)
    //                         let scale = if self.use_scale {
    //                             let delta = (highest[pod] - lowest[pod]).max(1e-6);
    //                             delta
    //                         } else {
    //                             1.0
    //                         };
    //                         self.trees[pod].ucb_select(current[pod], self.exploration, scale)
    //                     };
    //
    //                     // mettre à jour chemin courant
    //                     current[pod] = chosen_child;
    //
    //                     // appliquer la move au pod simulé (on simule plus tard après avoir choisi tous les pods pour le tour)
    //                     let action = self.trees[pod].nodes[chosen_child].action.clone();
    //                     state.apply_actions(&action);
    //                 } // fin boucle pods
    //
    //                 // appliquer la physique & collisions pour ce pas de temps
    //                 G::simulate_step(&mut state);
    //             } // fin profondeur
    //
    //             // évaluer la fin de la simulation
    //             let scores = G::evaluate(&state); // [f64; 4]
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
    //             result[pod] = tree.nodes[best_child].mv.clone();
    //         }
    //
    //         result
    //     }
    // }
}
mod io_helper {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    
    pub enum InputSource {
        Stdin(io::Stdin),
        File(BufReader<File>),
    }
    
    impl InputSource {
        pub fn from_stdin() -> Self {
            InputSource::Stdin(io::stdin())
        }
    
        pub fn from_file(path: &str) -> io::Result<Self> {
            let file = File::open(path)?;
            Ok(InputSource::File(BufReader::new(file)))
        }
    
        pub fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
            buf.clear();
            match self {
                InputSource::Stdin(stdin) => stdin.read_line(buf),
                InputSource::File(reader) => reader.read_line(buf),
            }
        }
    }
}
mod tests {
    #[cfg(test)]
    mod tests {
        use std::fs::File;
        use std::io::{BufReader};
        use std::path::Path;
        use crate::io_helper::InputSource;
        use crate::state::State;
    
        #[test]
        fn test_init_and_update_input() {
            // 🔹 Création du State
            let mut state = State::new();
            assert_eq!(state.my_id, 0);
            // 📂 Chemins vers les fichiers (relatif au fichier tests.rs)
            // let init_path = Path::new("./files/input.txt");
            // let update_path = Path::new("./files/update.txt");
            //
            // // 📂 Ouverture des fichiers
            // let init_file = File::open(init_path).expect("Impossible d'ouvrir init.txt");
            // let update_file = File::open(&update_path).expect("Impossible d'ouvrir update.txt");
            //
            // let mut init_input_source = InputSource::File(BufReader::new(init_file));
            // let mut update_input_source = InputSource::File(BufReader::new(update_file));
    
            // // 🔹 Lecture init
            // State::init_input(&mut state, &mut init_input_source);
            //
            // // ✅ Assert sur l'init
            // assert_eq!(state.my_id, 0);
            // assert_eq!(state.agent_data_count, 2);
            // assert_eq!(state.width, 3);
            // assert_eq!(state.height, 3);
            // assert_eq!(state.my_idx_arr.len(), 1);
            // assert_eq!(state.enemy_idx_arr.len(), 1);
            //
            // // 🔹 Lecture update
            // State::update_input(&mut state, &mut update_input_source);
            //
            // // ✅ Assert sur le tour
            // assert_eq!(state.turn, 1);
            // assert_eq!(state.agents[0].x, 5);
            // assert_eq!(state.agents[0].y, 5);
            // assert_eq!(state.agents[1].x, 6);
            // assert_eq!(state.agents[1].y, 6);
            // assert_eq!(state.agents[1].wetness, 10);
        }
    }
}
use crate::ia::IA;
use crate::state::State;
use crate::utils::{Debug, Timer};
use std::time::Duration;
use crate::io_helper::InputSource;

fn main() {
    let mut input_source = InputSource::from_stdin();

    let mut state = State::new();

    let ia = IA::new();
    let mut timer = Timer::new(Duration::from_millis(45));

    // 1. Lecture des inputs initiaux
    State::init_input(&mut state, &mut input_source);

    // 2. Boucle de jeu
    loop {
        // Lecture des inputs du tour
        State::update_input(&mut state, &mut input_source);

        timer.start();

        // Debug::debug("State",
        //              &[
        //                  ("turn", state.turn.to_string()),
        //                  ("state", format!("{:?}", state)),
        //              ],
        // );

        // IA
        let best_actions =  ia.decide_actions(&state, &timer);

        //Debug::debug_vec("Best Actions", &best_actions);
        Debug::debug_simple(format!("Time {:?}ms", timer.time()));

        // Output
        state.play(best_actions);

    }
}
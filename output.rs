// Généré à 23:12:16 le 19-08-2025
mod action {
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub enum TypeAction {
        Throw,
        Shoot,
        HunkerDown,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Action {
        pub id: i32,
        pub mx: i32,
        pub my: i32,
        pub type_action: TypeAction,
        pub x: i32,
        pub y: i32,
        pub enemy_id: i32,
        pub score: i32,
        pub index: usize,
    }
    
    impl Action {
        pub fn new(id: i32, mx: i32, my: i32, type_action: TypeAction, x: i32, y: i32, enemy_id: i32, score: i32) -> Self {
            Action { id, mx, my, type_action, x, y, enemy_id, score, index: 0 }
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
            Debug::debug_input(input_line.clone());
            let my_id = parse_input!(input_line, i32);
            state.my_id = my_id;
            let mut input_line = String::new();
            input.read_line(&mut input_line).unwrap();
            Debug::debug_input(input_line.clone());
            let agent_data_count = parse_input!(input_line, i32);
            state.agent_data_count = agent_data_count;
            for _ in 0..agent_data_count as usize {
                let mut input_line = String::new();
                input.read_line(&mut input_line).unwrap();
                Debug::debug_input(input_line.clone());
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
            Debug::debug_input(input_line.clone());
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let width = parse_input!(inputs[0], i32);
            state.width = width;
            let height = parse_input!(inputs[1], i32);
            state.height = height;
            state.grid = Grid::new(width as usize, height as usize);
            for _ in 0..height as usize {
                let mut input_line = String::new();
                input.read_line(&mut input_line).unwrap();
                Debug::debug_input(input_line.clone());
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
            Debug::debug_input(input_line.clone());
            let agent_count = parse_input!(input_line, i32); // Total number of agents still in the game
            for i in 0..10 {
                state.agents[i].is_dead = true;
            }
    
            for _ in 0..agent_count as usize {
                let mut input_line = String::new();
                input.read_line(&mut input_line).unwrap();
                Debug::debug_input(input_line.clone());
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
            Debug::debug_input(input_line.clone());
            let my_agent_count = parse_input!(input_line, i32);
        }
    
        pub fn is_terminal(&self) -> bool {
            false
        }
    
        pub fn evaluate(&self) -> f32 {
            0.0
        }
    
        pub fn legal_actions_for_idx_agent(&self, idx_agent: usize) -> Vec<Action> {
            let agent = self.agents[idx_agent];
            self.legal_actions_for_agent(&agent)
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
                    } else {
                        // THROW
                        if agent.splash_bombs > 0 {
                            for enemy_idx in &self.my_idx_arr {
                                let enemy = self.agents[*enemy_idx];
                                let dist = Math::manhattan(nx, ny, enemy.x, enemy.y);
                                if dist <= 4 {
                                    actions.push(Action::throw(agent.id, nx, ny, enemy.x, enemy.y, 100 - enemy.wetness));
                                }
                            }
                        }
    
                        // SHOOT
                        if agent.cooldown <= 0 {
                            for enemy_idx in &self.my_idx_arr {
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
    
        pub fn apply_actions_all(&mut self, actions: &Vec<Action>) {
            for action in actions {
                self.apply_actions(*action);
            }
        }
    
        pub fn apply_actions(&mut self, action: Action) {
            self.turn += 1;
    
            let agent = &mut self.agents[action.id as usize - 1];
            agent.x = action.mx;
            agent.y = action.my;
            let agent_x = agent.x;
            let agent_y = agent.y;
            let agent_cooldown = agent.shoot_cooldown;
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
                enemy_agent.wetness += (agent_soaking_power * range_modifier * (cover_modifier - hunker_down_bonus)) as i32;
                //enemy_agent.wetness += (agent_soaking_power) as i32;
            }
        }
    
        pub fn get_cover_modifier(&self, shooter_x: i32, shooter_y: i32, target_x: i32, target_y: i32) -> f32 {
            let dx = target_x - shooter_x;
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
    
        pub fn debug_input(value: String) {
            let debug_input: bool = false;
    
            if debug_input {
                eprint!("{}", value);
            }
        }
    
        pub fn debug_simple(value: String) {
            eprintln!("{:?}", value)
        }
        pub fn display(value: String) {
            eprintln!("{}", value)
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
        pub fn score(state: State, team: Team) -> f64 {
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
            //score += zones * 10;
    
           // score += my_position_score /10;
    
            // score -= my_wetness_score * 1000;
            // score -= nb_my_50wetness * 10000;
            // score -= nb_my_100wetness * 100000;
    
            // score += (enemy_wetness_score * 10) / 100;
            // score += nb_enemy_50wetness * 100;
            // score += nb_enemy_100wetness * 1000;
    
            if Team::Me == team {
                score += zones / state.width * state.height;
                score += my_wetness_score / (my_wetness_score + enemy_wetness_score + 1);
                score += nb_my_50wetness / (nb_my_50wetness + nb_enemy_50wetness + 1);
                score += nb_my_100wetness / (nb_my_100wetness + nb_enemy_100wetness + 1);
            } else {
                score += -zones / state.width * state.height;
                score += enemy_wetness_score / (enemy_wetness_score + my_wetness_score + 1);
                score += nb_enemy_50wetness / (nb_enemy_50wetness + nb_enemy_50wetness + 1);
                score += nb_enemy_100wetness / (nb_enemy_100wetness + nb_enemy_100wetness + 1);
            }
    
            score as f64
    
        }
    }
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
mod mcts_node {
    use std::collections::HashMap;
    use crate::action::Action;
    
    #[derive(Debug)]
    pub struct MCTSNode {
        pub visits: usize,
        pub value: f64,
        pub children: Vec<MCTSNode>,
        pub action: Option<Action>,
    }
    
    impl MCTSNode {
        pub fn new(action: Option<Action>) -> Self {
            Self {
                visits: 0,
                value: 0.0,
                children: Vec::new(),
                action
            }
        }
    
        pub fn uct(&self, parent_visits: usize) -> f64 {
            if self.visits == 0 {
                return f64::INFINITY;
            }
            self.value / self.visits as f64 + (1.41 * (parent_visits as f64).ln() / self.visits as f64).sqrt()
        }
    }
}
mod ia {
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
}
mod tests {
    #[cfg(test)]
    mod tests {
        use std::fs::File;
        use std::io::{BufReader};
        use std::path::Path;
        use std::time::Duration;
        use crate::ia::IA;
        use crate::io_helper::InputSource;
        use crate::state::State;
        use crate::utils::Timer;
    
        #[test]
        fn test_init_and_update_input() {
            // 🔹 Création du State
            let mut state = State::new();
            let mut ia = IA::new();
            let mut timer = Timer::new(Duration::from_millis(9999999999));
            assert_eq!(state.my_id, 0);
            // 📂 Chemins vers les fichiers (relatif au fichier tests.rs)
            let init_path = Path::new("./files/input.txt");
            let update_path = Path::new("./files/update.txt");
            //
            // // 📂 Ouverture des fichiers
            let init_file = File::open(init_path).expect("Impossible d'ouvrir init.txt");
            let update_file = File::open(&update_path).expect("Impossible d'ouvrir update.txt");
            //
            let mut init_input_source = InputSource::File(BufReader::new(init_file));
            let mut update_input_source = InputSource::File(BufReader::new(update_file));
            State::init_input(&mut state, &mut init_input_source);
            State::update_input(&mut state, &mut update_input_source);
    
            timer.start();
            ia.decide_actions(&state, &timer);
            println!("state.my_id");
        }
    }
}
use crate::state::State;
use crate::utils::{Debug, Timer};
use std::time::Duration;
use crate::ia::IA;
use crate::io_helper::InputSource;

fn main() {
    let mut input_source = InputSource::from_stdin();

    let mut state = State::new();

    let ia = IA::new();
    let mut timer = Timer::new(Duration::from_millis(55));

    Debug::debug_simple("A".parse().unwrap());
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
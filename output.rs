// Généré à 15:43:50 le 08-08-2025
mod action {
    use std::fmt;
    
    #[derive(Clone, Debug)]
    pub enum Action {
        Move(u32, u32),
        Throw(u32, u32),
        Shoot(u32), // target id
        HunkerDown,
        Message(String),
    }
    impl fmt::Display for Action {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Action::Move(x, y) => write!(f, "MOVE {} {}", x, y),
                Action::Throw(x, y) => write!(f, "THROW {} {}", x, y),
                Action::Shoot(id) => write!(f, "SHOOT {}", id),
                Action::HunkerDown => write!(f, "HUNKER_DOWN"),
                Action::Message(text) => write!(f, "MESSAGE {}", text),
            }
        }
    }
}
mod state {
    use crate::action::Action;
    use crate::agent::{Agent, Team};
    use crate::grid::Grid;
    
    use std::io;
    
    macro_rules! parse_input {
        ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
    }
    
    #[derive(Debug)]
    pub struct GameConstants {
    
    }
    
    #[derive(Clone, Debug)]
    pub struct State {
        pub turn: u32,
        pub my_id: u32,
        pub width: u32,
        pub height: u32,
        pub agent_data_count: u32,
        pub my_idx_arr: Vec<u32>,
        pub enemy_idx_arr: Vec<u32>,
        pub grid: Grid,
        pub my_agents: Vec<Agent>,
        pub ennemy_agents: Vec<Agent>,
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
                my_agents: Vec::new(),
                ennemy_agents: Vec::new(),
            }
        }
    
        pub fn init_input(state: &mut State) {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let my_id = parse_input!(input_line, u32);
            state.my_id = my_id;
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let agent_data_count = parse_input!(input_line, u32);
            state.agent_data_count = agent_data_count;
            for i in 0..agent_data_count as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let inputs = input_line.split(" ").collect::<Vec<_>>();
                let agent_id = parse_input!(inputs[0], u32);
                let player = parse_input!(inputs[1], u32);
                let shoot_cooldown = parse_input!(inputs[2], u32);
                let optimal_range = parse_input!(inputs[3], u32);
                let soaking_power = parse_input!(inputs[4], u32);
                let splash_bombs = parse_input!(inputs[5], u32);
    
                let agent = Agent {
                    id: agent_id,
                    x: 0,
                    y: 0,
                    shoot_cooldown,
                    optimal_range,
                    soaking_power,
                    splash_bombs,
                    team: if player == my_id { Team::Me } else { Team::Enemy },
                };
    
                if agent.team == Team::Me {
                    state.my_idx_arr.push(agent_id - 1);
                    state.my_agents.push(agent);
                } else {
                    state.enemy_idx_arr.push(agent_id - 1);
                    state.ennemy_agents.push(agent);
                }
            }
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let width = parse_input!(inputs[0], u32);
            state.width = width;
            let height = parse_input!(inputs[1], u32);
            state.height = width;
            state.grid = Grid::new(width as usize, height as usize);
            for i in 0..height as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let inputs = input_line.split_whitespace().collect::<Vec<_>>();
                for j in 0..width as usize {
                    let x = parse_input!(inputs[3*j], usize);
                    let y = parse_input!(inputs[3*j+1], usize);
                    let tile_type = parse_input!(inputs[3*j+2], u32);
                    state.grid.set(x, y, tile_type);
                }
            }
        }
    
        pub fn update_input(state: &mut State) {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let agent_count = parse_input!(input_line, u32); // Total number of agents still in the game
            for i in 0..agent_count as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let inputs = input_line.split(" ").collect::<Vec<_>>();
                let agent_id = parse_input!(inputs[0], u32);
                let x = parse_input!(inputs[1], u32);
                let y = parse_input!(inputs[2], u32);
                let cooldown = parse_input!(inputs[3], u32); // Number of turns before this agent can shoot
                let splash_bombs = parse_input!(inputs[4], u32);
                let wetness = parse_input!(inputs[5], u32); // Damage (0-100) this agent has taken
            }
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let my_agent_count = parse_input!(input_line, u32);
        }
    
        pub fn is_terminal(&self) -> bool {
            false
        }
    
        pub fn evaluate(&self) -> f32 {
            0.0
        }
    
        pub fn legal_actions_for_agent(&self, agent: &Agent) -> Vec<Action> {
            // Simple action set: stay, move in 4 directions within map, attack nearest enemy if within range 1
            // let mut actions = Vec::new();
            // actions.push(Action::Wait);
            // let dirs = vec![(0,1),(0,-1),(1,0),(-1,0)];
            // for (dx,dy) in dirs {
            //     let nx = agent.x + dx;
            //     let ny = agent.y + dy;
            //     if nx >= 0 && nx < self.init.map_w && ny >= 0 && ny < self.init.map_h {
            //         actions.push(Action::Move(nx, ny));
            //     }
            // }
            // // attack if any enemy adjacent
            // let enemies = if agent.team == Team::Me { &self.enemy_agents } else { &self.my_agents };
            // for e in enemies {
            //     let dist = (e.x - agent.x).abs() + (e.y - agent.y).abs();
            //     if dist <= 1 && e.hp > 0 {
            //         actions.push(Action::Attack(e.id));
            //     }
            // }
            // actions
            Vec::new()
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
    
        pub fn apply_joint_actions(&mut self, my_actions: &[Action], enemy_actions: Option<&[Action]>) {
            self.turn += 1;
        }
    
        pub fn play(&self, actions: Vec<Action>) {
            for action in actions {
                println!("{}", action);
            }
        }
    }
}
mod agent {
    #[derive(Clone, Debug)]
    pub struct Agent {
        pub id: u32,
        pub x: u32,
        pub y: u32,
        pub shoot_cooldown: u32,
        pub optimal_range: u32,
        pub soaking_power: u32,
        pub splash_bombs: u32,
        pub team: Team,
    }
    
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Team {
        Me,
        Enemy,
    }
}
mod utils {
    use std::time::{Instant, Duration};
    use std::eprintln;
    
    pub struct Timer {
        start: Instant,
        limit: Duration,
    }
    
    impl Timer {
        pub fn new(start: Instant, limit: Duration) -> Self {
            Self { start, limit }
        }
        pub fn is_time_up(&self) -> bool {
            Instant::now().duration_since(self.start) >= self.limit
        }
    }
    
    pub struct Debug {
    }
    
    impl Debug {
        pub fn debug(label: &str, params: &[(&str, String)]) {
            eprintln!("=== DEBUG: {} ===", label);
            for (name, value) in params {
                eprintln!(" {}: {}", name, value);
            }
            eprintln!("======================");
        }
    }
}
mod grid {
    #[derive(Clone, Debug)]
    pub struct Grid {
        width: usize,
        height: usize,
        data: Vec<u32>,
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
    
        pub fn get(&self, x: usize, y: usize) -> u32 {
            self.data[self.index(x, y)]
        }
    
        pub fn set(&mut self, x: usize, y: usize, value: u32) {
            let idx = self.index(x, y);
            self.data[idx] = value;
        }
    }
}
mod ia {
    use crate::action::Action;
    use crate::state::State;
    
    pub struct IA;
    
    impl IA {
        pub fn new() -> Self {
            IA
        }
        pub fn decide_actions(&self, state: &State) -> Vec<Action> {
            let mut actions = Vec::new();
    
            // Exemple : pour chaque unité, aller à droite
            for unit in &state.my_agents {
                actions.push(Action::HunkerDown);
            }
    
            actions
        }
    }
}
use crate::state::State;
use crate::ia::IA;
use crate::utils::Debug;

fn main() {
    let mut state = State::new();

    let ia = IA::new();

    // 1. Lecture des inputs initiaux
    State::init_input(&mut state);

    // 2. Boucle de jeu
    loop {
        // Lecture des inputs du tour
        State::update_input(&mut state);

        Debug::debug("State",
                     &[
                         ("turn", state.turn.to_string()),
                         ("state", format!("{:?}", state)),
                     ],
        );

        // IA
        let best_actions =  ia.decide_actions(&state);

        Debug::debug("IA Decision",
                     &[
                         ("turn", state.turn.to_string()),
                         ("best_actions", format!("{:?}", best_actions.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", "))),
                     ],
        );

        // Output
        state.play(best_actions);

    }
}
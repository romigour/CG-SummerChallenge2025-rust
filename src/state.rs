use crate::action::{Action, TypeAction};
use crate::agent::{Agent, Team};
use crate::grid::Grid;

use crate::io_helper::InputSource;
use crate::utils::Math;

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
        for i in 0..state.agent_data_count as usize {
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

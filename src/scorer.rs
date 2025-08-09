use crate::state::State;
use crate::agent::{Team};
use crate::utils::Debug;

pub struct Scorer;

impl Scorer {
    pub fn new() -> Self {
        Scorer
    }
    pub fn score(state: State) -> i32 {
        let mut score = 0;
        let mut my_wetness_score = 0;
        let mut enemy_wetness_score = 0;
        let mut nb_enemy_50wetness = 0;

        //Debug::debug_simple(format!("{:?}", state));

        score += state.calcul_zone_couverture(0, 0, 0) * 1000;
        //Debug::debug_simple(format!("zone {:?}", state.calcul_zone_couverture(0, 0, 0) * 100));
        for agent in &state.agents {
            if agent.team == Team::Me {
                my_wetness_score += agent.wetness;

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
            }
        }

        score -= my_wetness_score;
        score += enemy_wetness_score * 10;
        score += nb_enemy_50wetness * 100;

        score

    }
}
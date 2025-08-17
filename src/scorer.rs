use crate::agent::Team;
use crate::state::State;
use crate::utils::Debug;

pub struct Scorer;

impl Scorer {
    pub fn score(state: State) -> f64 {
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

        score as f64

    }
}
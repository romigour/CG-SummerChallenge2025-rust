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
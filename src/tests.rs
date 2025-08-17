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
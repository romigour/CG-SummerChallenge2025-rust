use std::fmt::Debug;

/// Représente l'état de n'importe quel jeu 1v1
pub trait GameState: Clone + Debug {
    type Action: Clone + Debug;

    /// Renvoie toutes les actions légales pour le joueur courant
    fn legal_actions(&self) -> Vec<Self::Action>;
    /// Applique une action et retourne le nouvel état
    fn apply(&self, action: &Self::Action) -> Self;
    /// Vérifie si l'état est terminal
    fn is_terminal(&self) -> bool;
    /// Donne le score depuis la perspective du joueur courant
    fn evaluate(&self) -> f32;
    /// Lit l'état depuis l'entrée standard (Codingame)
    fn read_input(&mut self);
    /// Écrit l'action à la sortie standard (Codingame)
    fn write_output(&self, action: &Self::Action);
}

/// Exemple d'implémentation pour ton jeu spécifique
#[derive(Clone, Debug)]
pub struct MyGameState {
    // champs spécifiques: positions, PV, etc.
}

impl MyGameState {
    pub fn new() -> Self {
        // initialisation
        Self { /* ... */ }
    }
}

impl GameState for MyGameState {
    type Action = MyAction;

    fn legal_actions(&self) -> Vec<Self::Action> {
        // générer toutes les actions
        vec![]
    }
    fn apply(&self, action: &Self::Action) -> Self {
        // calcul de l'état suivant
        self.clone()
    }
    fn is_terminal(&self) -> bool {
        // condition de fin
        false
    }
    fn evaluate(&self) -> f32 {
        // heuristique ou reward final
        0.0
    }
    fn read_input(&mut self) {
        // parser l'entrée Codingame
    }
    fn write_output(&self, action: &Self::Action) {
        // formater la sortie
        println!("{}", action.to_string());
    }
}

#[derive(Clone, Debug)]
pub struct MyAction {
    // champs de l'action
}

impl ToString for MyAction {
    fn to_string(&self) -> String {
        // conversion en chaîne pour Codingame
        String::new()
    }
}
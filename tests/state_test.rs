use CG_SummerChallenge2025_rust::GameState;
use CG_SummerChallenge2025_rust::Mcts;
use CG_SummerChallenge2025_rust::MctsConfig;
use CG_SummerChallenge2025_rust::Timer;

#[test]
fn test() {
    let game_state = GameState::new();
    let mut mcts = Mcts::new(game_state.clone(), MctsConfig {
        exploration_constant: 1.41,
    });
    let start = std::time::Instant::now();
    let action = mcts.search(1000, &Timer::new(start, std::time::Duration::from_millis(45)));
    assert!(action.is_some(), "Expected an action to be returned");
    println!("Action: {:?}", action);
}
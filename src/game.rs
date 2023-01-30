use std::io::stdin;

#[allow(clippy::module_name_repetitions)]
pub enum GameState {
    ActiveGame,
    GameOver,
    Quit,
    StartNewGame,
}

pub(crate) fn start_new_game(game_state: &mut GameState) {
    if let GameState::GameOver = game_state {
        let mut user_input = String::new();
        println!("Start? (y/q)");

        match stdin().read_line(&mut user_input) {
            Ok(_) => {
                if user_input == "y\n" {
                    *game_state = GameState::StartNewGame;
                    log::info!("Commencing a new game.");
                }
                if user_input == "q\n" {
                    *game_state = GameState::Quit;
                    log::info!("User quitted.");
                }
            }
            Err(e) => {
                log::error!("Unable to start a new game: {}", e);
            }
        }
    }
}

pub(crate) fn logic() {
    // Deal Cards
    // New turn
}

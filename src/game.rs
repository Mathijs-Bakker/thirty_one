use std::io::stdin;

use crate::{card_deck::Card, player::Player};

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

const MAX_CARDS: usize = 3;

pub(crate) fn deal_table_cards<'a>(card_deck: &mut Vec<Card<'a>>, table: &mut Vec<Card<'a>>) {
    if table.is_empty() {
        for _ in 0..MAX_CARDS {
            get_card_on_top_and_swap(card_deck, table);
        }
    }
}

pub(crate) fn deal_player_cards<'a>(card_deck: &mut Vec<Card<'a>>, players: &mut Vec<Player<'a>>) {
    for player in players {
        if player.hand.is_empty() {
            for _ in 0..MAX_CARDS {
                get_card_on_top_and_swap(card_deck, &mut player.hand);
            }
        }
    }
}

fn get_card_on_top_and_swap<'a>(origin: &mut Vec<Card<'a>>, destination: &mut Vec<Card<'a>>) {
    let top_card = &origin[origin.len() - 1];
    destination.push(*top_card);
    origin.pop();
}

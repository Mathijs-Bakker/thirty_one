#![warn(clippy::all, clippy::pedantic)]

mod card_deck;
mod player;
use crate::card_deck::Card;
use player::Player;
use rand::{seq::SliceRandom, thread_rng};
use std::io::stdin;

use crate::card_deck::create_deck_of_cards;

enum GameState {
    ActiveGame,
    GameOver,
    Quit,
    StartNewGame,
}

fn main() {
    let mut game_state = GameState::GameOver;
    let mut players: Vec<Player> = vec![];

    // let mut deck: Vec<Card> = Vec::new();
    let mut deck: Vec<Card> = create_deck_of_cards();

    loop {
        match game_state {
            GameState::GameOver => start_new_game(&mut game_state),
            GameState::Quit => {
                println!("Program terminated.");
                break;
            }
            GameState::StartNewGame => {
                player::setup_players(&mut players);
                deck.shuffle(&mut thread_rng());
            }
            GameState::ActiveGame => todo!(),
        }

        for card in &deck {
            println!("Card: {:?}", card);
        }
    }
}

fn start_new_game(game_state: &mut GameState) {
    if let GameState::GameOver = game_state {
        let mut user_input = String::new();
        println!("Start? (y/q)");

        match stdin().read_line(&mut user_input) {
            Ok(_) => {
                if user_input == "y\n" {
                    *game_state = GameState::StartNewGame;
                    println!("Start new game!");
                }
                if user_input == "q\n" {
                    *game_state = GameState::Quit;
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}

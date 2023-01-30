#![warn(clippy::all, clippy::pedantic)]

mod card_deck;
mod game;
mod player;

use crate::{
    card_deck::Card,
    game::{deal_player_cards, deal_table_cards},
};
use game::GameState;
use player::Player;
use rand::{seq::SliceRandom, thread_rng};

use crate::card_deck::create_deck_of_cards;

fn main() {
    let mut game_state = game::GameState::GameOver;

    let mut deck: Vec<Card> = create_deck_of_cards();
    let mut players: Vec<Player> = Vec::new();
    let mut table: Vec<Card> = Vec::new();

    loop {
        match game_state {
            GameState::GameOver => game::start_new_game(&mut game_state),
            GameState::Quit => {
                log::info!("Application terminated.");
                break;
            }
            GameState::StartNewGame => {
                player::setup_players(&mut players);
                deck.shuffle(&mut thread_rng());
                game_state = GameState::ActiveGame;
            }
            GameState::ActiveGame => {
                deal_player_cards(&mut deck, &mut players);
                deal_table_cards(&mut deck, &mut table);
            }
        }
    }
}

#![warn(clippy::all, clippy::pedantic)]

mod card_deck;
mod game;
mod player;

use crate::card_deck::Card;
use game::GameState;
use player::Player;
use rand::{seq::SliceRandom, thread_rng};

use crate::card_deck::create_deck_of_cards;

fn main() {
    let mut game_state = game::GameState::GameOver;
    let mut players: Vec<Player> = Vec::new();

    let mut deck: Vec<Card> = create_deck_of_cards();

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
            GameState::ActiveGame => deal_cards(&mut players, &mut deck),
        }

        // for card in &deck {
        //     println!("Card: {:?}", card);
        // }
    }
}

fn deal_cards<'a>(players: &mut Vec<Player<'a>>, card_deck: &mut Vec<Card<'a>>) {
    for player in players {
        // if player.hand.is_empty() {
        let top_card = &card_deck[card_deck.len() - 1];
        println!("top card {:?}", top_card);

        player.hand.push(*top_card);

        println!("Computer dealt {:?} to player {:?}", top_card, player.name);
        card_deck.pop();
        // }
    }
}

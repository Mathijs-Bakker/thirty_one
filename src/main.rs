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

const MAX_CARDS: usize = 3;

fn deal_table_cards<'a>(card_deck: &mut Vec<Card<'a>>, table: &mut Vec<Card<'a>>) {
    if table.is_empty() {
        for _ in 0..MAX_CARDS {
            let top_card = &card_deck[card_deck.len() - 1];

            table.push(*top_card);
            log::info!("deal_table_cards() - Table got {:?} ", top_card);

            card_deck.pop();
        }
    }
}

fn deal_player_cards<'a>(card_deck: &mut Vec<Card<'a>>, players: &mut Vec<Player<'a>>) {
    for player in players {
        if player.hand.is_empty() {
            for _ in 0..MAX_CARDS {
                let top_card = &card_deck[card_deck.len() - 1];

                player.hand.push(*top_card);

                log::info!(
                    "deal_player_cards() - Player {:?} got {:?}",
                    player.name,
                    top_card
                );

                card_deck.pop();
            }
        }
    }
}

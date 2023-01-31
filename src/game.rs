use crate::{
    card_deck::Card,
    player::{Ai, Player},
};
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

const MAX_CARDS: usize = 3;

/// Deal cards on the table (when table has no cards).
pub(crate) fn deal_table_cards<'a>(card_deck: &mut Vec<Card<'a>>, table: &mut Vec<Card<'a>>) {
    if table.is_empty() {
        for _ in 0..MAX_CARDS {
            get_card_on_top_and_swap(card_deck, table);
        }
    }
}

/// Deal cards to players (when hands are empty).
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

fn render_cards(cards: &Vec<Card>) {
    let card1 = format!("  |{:?} {:?}|", cards[0].name, cards[0].suit,);
    let card2 = format!("  |{:?} {:?}|", cards[1].name, cards[1].suit,);
    let card3 = format!("  |{:?} {:?}|", cards[2].name, cards[2].suit,);

    println!("{card1}{card2}{card3}");
}

pub(crate) fn game_play(players: &Vec<Player>, table: &Vec<Card>) {
    // render table cards - V
    // render player cards - V
    // if player 31 -> Win
    // player turn
    // --> Bots using ai (todo)
    // --> Human player -> swap cards with table

    println!("Table:");
    render_cards(table);

    for player in players {
        if let Ai::Human = player.ai {
            println!("{:?}", player.name);
            render_cards(&player.hand);
        } else {
            println!("Bot");
            println!("| ? |  | ? |  | ? |");
        }
    }
}

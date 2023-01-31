use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card_deck::Card;

#[derive(Debug)]
pub enum Ai {
    Bot,
    Human,
}

#[derive(Debug)]
pub struct Player<'a> {
    pub ai: Ai,
    pub name: String,
    pub points: u8,
    pub hand: Vec<Card<'a>>,
}

impl<'a> Player<'a> {
    pub fn new(ai: Ai, name: String) -> Self {
        Self {
            ai,
            name,
            points: 5,
            hand: Vec::new(),
        }
    }
}

pub(crate) fn setup_players(players: &mut Vec<Player>) {
    log::info!("Setup players");
    players.clear();

    // Give player names
    // Todo: Names should be generated randomly, using real human names.
    let default_players: Vec<(Ai, String)> = vec![
        (Ai::Bot, "West".to_string()),
        (Ai::Bot, "North".to_string()),
        (Ai::Bot, "East".to_string()),
        (Ai::Human, "You".to_string()),
    ];

    for def in default_players {
        players.push(Player::new(def.0, def.1));
    }

    // Shuffle players to determine which player starts
    let mut rng = thread_rng();
    players.shuffle(&mut rng);
    log::info!("Players shuffled.");
}

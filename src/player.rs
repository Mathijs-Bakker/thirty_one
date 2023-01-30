use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card_deck::Card;

#[derive(Debug)]
pub struct Player<'a> {
    pub name: String,
    pub points: u8,
    pub hand: Vec<Card<'a>>,
}

impl<'a> Player<'a> {
    pub fn new(name: String) -> Self {
        Self {
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
    let player_names: Vec<String> = vec![
        "West".to_string(),
        "North".to_string(),
        "East".to_string(),
        "You".to_string(),
    ];

    for name in player_names {
        players.push(Player::new(name));
    }

    // Shuffle players to determine which player starts
    let mut rng = thread_rng();
    players.shuffle(&mut rng);
    log::info!("Players shuffled.");
}

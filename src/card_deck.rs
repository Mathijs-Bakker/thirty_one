#[derive(Debug, Clone, Copy)]
enum Suits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Clone, Copy)]
pub struct Card<'a> {
    value: u8,
    name: &'a str,
    suit: Suits,
}

const CARD_VALUES: &[(&str, u8); 8] = &[
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("10", 10),
    ("J", 10),
    ("Q", 10),
    ("K", 10),
    ("A", 11),
];

pub(crate) fn create_deck_of_cards() -> Vec<Card<'static>> {
    let mut deck: Vec<Card> = Vec::new();
    let suits = vec![Suits::Clubs, Suits::Hearts, Suits::Spades, Suits::Diamonds];

    for suit in suits {
        for val in CARD_VALUES {
            deck.push(Card {
                name: val.0,
                value: val.1,
                suit,
            });
        }
    }

    log::info!("Generated a new deck of {:?} cards.", deck.len());

    deck
}

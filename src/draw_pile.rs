use super::card::{Card, Color};
use super::DiscardPile;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Debug, Default)]
pub struct DrawPile {
    cards: Vec<Card>,
}

impl DrawPile {
    pub fn new() -> DrawPile {
        let mut cards = Vec::new();
        for color in &[Color::Red, Color::Blue, Color::Yellow, Color::Green] {
            for _ in 0..2 {
                for number in 0..=9 {
                    cards.push(Card::Number {
                        color: color.clone(),
                        number,
                    });
                }

                for card in &[
                    Card::Reverse {
                        color: color.clone(),
                    },
                    Card::Skip {
                        color: color.clone(),
                    },
                    Card::DrawTwo {
                        color: color.clone(),
                    },
                ] {
                    cards.push(card.clone());
                }
            }
        }

        for _ in 0..4 {
            cards.push(Card::Wild { color: Color::All });
            cards.push(Card::WildDrawFour);
        }

        cards.shuffle(&mut thread_rng());

        DrawPile { cards }
    }

    pub fn from_discard_pile(mut cards: Vec<Card>) -> DrawPile {
        cards.shuffle(&mut thread_rng());

        DrawPile { cards }
    }

    pub fn draw_cards(&mut self, n: usize, discard_pile: &mut DiscardPile) -> Vec<Card> {
        let mut drawn_cards: Vec<Card> = Vec::new();

        if self.cards.len() >= n {
            // ..n
            drawn_cards.append(&mut self.cards.drain(..n).collect::<Vec<Card>>());
        } else {
            let length = self.cards.len();
            // println!("\t\t{:?}", self.cards);
            // ..
            drawn_cards.append(&mut self.cards.drain(..).collect::<Vec<Card>>());
            let discarded_cards = discard_pile.reuse_cards();

            if discarded_cards.is_empty() {
                // No cards in draw AND discard pile, so we need a new deck (higly unlikely)
                // That means we are holding all of the 112 cards
                *self = DrawPile::new();
            } else {
                // Cards in discard pile, so we can reuse them
                *self = DrawPile::from_discard_pile(discarded_cards);
            }

            // ..n-length
            // println!("n: {}, length: {}", n, length);
            drawn_cards.append(&mut self.cards.drain(..n - length).collect::<Vec<Card>>());
        }

        drawn_cards
    }

    pub fn insert_and_shuffle(&mut self, card: Card) {
        self.cards.push(card);
        self.cards.shuffle(&mut thread_rng());
    }
}

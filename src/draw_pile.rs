use super::card::{Card, Color};
use super::DiscardPile;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Debug)]
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
            cards.push(Card::Wild);
            cards.push(Card::WildDrawFour);
        }

        cards.shuffle(&mut thread_rng());

        DrawPile { cards }
    }

    pub fn from_discard_pile(mut cards: Vec<Card>) -> DrawPile {
        cards.shuffle(&mut thread_rng());

        DrawPile { cards }
    }

    /* pub fn draw_cards2(&mut self, n: usize) -> Vec<Card> {
        let mut cards = self.draw_pile.draw(n);
        let draw_size = cards.len();

        if draw_size < n {
            // Empty draw pile, so we need to draw from discard pile
            if discarded_cards.len() == 0 {
                // No cards in draw AND discard pile, so we need a new deck (higly unlikely)
                // That means we are holding all of the 112 cards
                self.draw_pile = DrawPile::new();
            } else {
                // Cards in discard pile, so we can reuse them
                self.draw_pile = DrawPile::from_discard_pile(discarded_cards);
            }
        }

        cards
    } */

    pub fn draw_cards(&mut self, n: usize, discard_pile: &mut DiscardPile) -> Vec<Card> {
        let mut drawn_cards: Vec<Card> = Vec::new();

        if self.cards.len() > n {
            self.cards.drain(..n).for_each(|card| {
                drawn_cards.push(card);
            });
        } else {
            self.cards.drain(..).for_each(|card| {
                drawn_cards.push(card);
            });
            let discarded_cards = discard_pile.reuse_cards();

            if discarded_cards.len() == 0 {
                // No cards in draw AND discard pile, so we need a new deck (higly unlikely)
                // That means we are holding all of the 112 cards
                *self = DrawPile::new();
            } else {
                // Cards in discard pile, so we can reuse them
                *self = DrawPile::from_discard_pile(discarded_cards);
            }
        }

        drawn_cards
    }

    /* pub fn draw(&mut self, n: usize) -> Vec<Card> {
        let mut drawn_cards: Vec<Card> = Vec::new();

        if self.cards.len() > n {
            self.cards.drain(..n).for_each(|card| {
                drawn_cards.push(card);
            });
        } else {
            self.cards.drain(..).for_each(|card| {
                drawn_cards.push(card);
            });
        }

        drawn_cards
    } */
}

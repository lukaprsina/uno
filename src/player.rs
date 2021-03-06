use super::{Card, DiscardPile, DrawPile};
use std::cell::RefCell;

#[derive(Clone, Debug, Default)]
pub struct Player {
    hand: RefCell<Vec<Card>>,
    score: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hand: RefCell::new(Vec::new()),
            score: 0,
        }
    }

    pub fn add_cards(&self, cards: &[Card]) {
        self.hand.borrow_mut().extend_from_slice(cards);
    }

    pub fn choose_card(
        &self,
        other: &Card,
        draw_pile: &mut DrawPile,
        discard_pile: &mut DiscardPile,
    ) -> Option<Card> {
        let mut other_indexes: Vec<usize> = Vec::new();
        let mut wild_four_indexes: Vec<usize> = Vec::new();
        let mut indexes: Vec<usize> = Vec::new();

        for (position, this) in self.hand.borrow().iter().enumerate() {
            if this == other {
                if let Card::WildDrawFour = *this {
                    wild_four_indexes.push(position);
                } else {
                    other_indexes.push(position);
                }
            }
        }

        if other_indexes.is_empty() {
            if !wild_four_indexes.is_empty() {
                indexes = wild_four_indexes;
            }
        } else {
            indexes = other_indexes;
        }

        println!("\nMatching cards:\n");
        indexes
            .iter()
            .for_each(|&index| println!("{:?}", self.hand.borrow()[index]));

        println!("{}\nNon-matching cards:\n", "-".repeat(80));

        self.hand
            .borrow()
            .iter()
            .enumerate()
            .filter(|(index, _)| !indexes.contains(index))
            .for_each(|(_, card)| println!("{card:?}"));

        println!("{}", "-".repeat(80));

        if !indexes.is_empty() {
            Some(self.hand.borrow_mut().remove(indexes[0]))
        } else {
            let new_card = draw_pile.draw_cards(1, discard_pile).remove(0);

            println!("\tTaking: {new_card:?}");
            self.add_cards(&[new_card.clone()]);

            if &new_card == other {
                Some(new_card)
            } else {
                None
            }
        }
    }

    pub fn has_no_cards(&self) -> bool {
        self.hand.borrow().is_empty()
    }

    pub fn score_cards(&self) -> i32 {
        self.hand
            .borrow()
            .iter()
            .map(|card| match card {
                Card::Number { color: _, number } => *number as i32,
                Card::Reverse { color: _ } => 20i32,
                Card::Skip { color: _ } => 20i32,
                Card::DrawTwo { color: _ } => 20i32,
                Card::Wild { color: _ } => 50i32,
                Card::WildDrawFour => 50i32,
            })
            .sum()
    }

    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn clear_hand(&mut self) {
        self.hand.borrow_mut().clear();
    }
}

use super::{Card, DiscardPile, DrawPile};
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct Player {
    hand: RefCell<Vec<Card>>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hand: RefCell::new(Vec::new()),
        }
    }

    pub fn add_card(&self, cards: &[Card]) {
        self.hand.borrow_mut().extend_from_slice(cards);
    }

    pub fn choose_card(
        &self,
        other: &Card,
        draw_pile: &mut DrawPile,
        discard_pile: &mut DiscardPile,
    ) -> Option<Card> {
        let indexes: Vec<_> = self
            .hand
            .borrow()
            .iter()
            .enumerate()
            .filter_map(
                |(position, this)| {
                    if this == other {
                        Some(position)
                    } else {
                        None
                    }
                },
            )
            .collect();

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
            .for_each(|(_, card)| println!("{:?}", card));

        println!("{}", "-".repeat(80));

        let chosen_card: Card;

        if indexes.len() > 0 {
            let card = self.hand.borrow_mut().remove(indexes[0]);
            chosen_card = card;
        } else {
            let mut new_card: Card;
            loop {
                new_card = draw_pile.draw_cards(1, discard_pile)[0].clone();
                self.add_card(&[new_card.clone()]);
                println!("\tTaking: {:?}", new_card);

                if &new_card == other {
                    break;
                }
            }

            chosen_card = new_card;
        }

        if self.hand.borrow().len() == 0 {
            None
        } else {
            Some(chosen_card)
        }
    }
}

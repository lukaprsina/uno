use super::Card;

#[derive(Clone, Debug)]
pub struct DiscardPile {
    cards: Vec<Card>,
}

impl DiscardPile {
    pub fn new() -> DiscardPile {
        DiscardPile { cards: Vec::new() }
    }

    pub fn reuse_cards(&mut self) -> Vec<Card> {
        std::mem::replace(&mut self.cards, Vec::new())
    }

    pub fn place_cards(&mut self, cards: &[Card]) {
        self.cards.extend(cards.iter().cloned());
    }

    pub fn get_top_card(&self) -> &Card {
        self.cards.last().expect("No cards in discard pile")
    }
}

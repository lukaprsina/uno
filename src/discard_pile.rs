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
        let last_card = self
            .cards
            .last()
            .expect("Can't reuse cards from empty discard pile")
            .clone();
        std::mem::replace(&mut self.cards, vec![last_card])
    }

    // TODO: maybe dont clone
    pub fn place_cards(&mut self, cards: &[Card]) {
        self.cards.extend(cards.iter().cloned());
    }

    pub fn get_top_card(&self) -> &Card {
        self.cards.last().expect("No cards in discard pile")
    }
}

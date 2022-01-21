use super::Card;

#[derive(Clone, Debug, Default)]
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

    pub fn place_cards(&mut self, cards: &mut Vec<Card>) {
        self.cards.append(cards);
    }

    pub fn place_card(&mut self, card: &Card) {
        self.cards.push(card.clone());
    }

    pub fn get_top_card(&self) -> &Card {
        self.cards.last().expect("No cards in discard pile")
    }
}

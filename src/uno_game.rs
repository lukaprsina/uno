use super::{Card, DiscardPile, DrawPile, Player};

#[derive(Clone, Debug)]
pub struct Game {
    players: Vec<Player>,
    draw_pile: DrawPile,
    discard_pile: DiscardPile,
    clockwise: bool,
    index: i32,
}

impl Game {
    pub fn new(num_players: i32) -> Game {
        let discard_pile = DiscardPile::new();
        let draw_pile = DrawPile::new();

        let mut players = Vec::new();
        for _ in 0..num_players {
            players.push(Player::new());
        }

        Game {
            players,
            draw_pile,
            discard_pile,
            clockwise: true,
            index: 0i32,
        }
    }

    pub fn get_next_player(&self) -> i32 {
        let increment;

        if self.clockwise {
            increment = 1;
        } else {
            increment = -1;
        }

        let index = (self.index + increment) % self.players.len() as i32;
        if index.is_negative() {
            self.players.len() as i32 + index
        } else {
            index
        }
    }

    pub fn run_card_action(&mut self, card: &Card) {
        match card {
            Card::Number {
                color: _,
                number: _,
            } => (),
            Card::Reverse { color: _ } => self.clockwise = !self.clockwise,
            Card::Skip { color: _ } => self.index = self.get_next_player(),
            Card::DrawTwo { color: _ } => self.players[self.get_next_player() as usize]
                .add_card(&self.draw_pile.draw_cards(2, &mut self.discard_pile)),
            Card::Wild => (),
            Card::WildDrawFour => self.players[self.get_next_player() as usize]
                .add_card(&self.draw_pile.draw_cards(4, &mut self.discard_pile)),
        }
    }

    pub fn start(&mut self) {
        for _ in 0..7 {
            for player in &self.players {
                let cards = self.draw_pile.draw_cards(1, &mut self.discard_pile);
                player.add_card(&cards);
            }
        }

        let beginning_card = self.draw_pile.draw_cards(1, &mut self.discard_pile);
        self.discard_pile.place_cards(&beginning_card);

        loop {
            let card = self.discard_pile.get_top_card().clone();
            println!(
                "{}\n\nPlayer {}'s turn\nCard: {:?}",
                "-".repeat(80),
                self.index,
                &card
            );
            let player = &mut self.players[self.index as usize];
            if let Some(chosen_card) =
                player.choose_card(&card, &mut self.draw_pile, &mut self.discard_pile)
            {
                self.discard_pile.place_cards(&[chosen_card.clone()]);

                self.run_card_action(&chosen_card);
            } else {
                break;
            }

            self.index = self.get_next_player();
        }

        println!("Player {} won!\n{:?}", self.index, self.players);
    }
}

// a test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let game = Game {
            players: vec![Player::new(), Player::new()],
            clockwise: false,
            index: 0,
            discard_pile: DiscardPile::new(),
            draw_pile: DrawPile::new(),
        };
        println!("{}", game.get_next_player());
    }
}

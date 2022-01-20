use super::{card::Color, Card, DiscardPile, DrawPile, Player};

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

    pub fn run_card_action(&mut self, card: &mut Card, other: Option<&Card>) {
        match card {
            Card::Number {
                color: _,
                number: _,
            } => (),
            Card::Reverse { color: _ } => self.clockwise = !self.clockwise,
            Card::Skip { color: _ } => self.index = self.get_next_player(),
            Card::DrawTwo { color: _ } => self.players[self.get_next_player() as usize]
                .add_cards(&self.draw_pile.draw_cards(2, &mut self.discard_pile)),

            Card::Wild { color: _ } => match other {
                Some(other) => {
                    *card = Card::Wild {
                        color: self.choose_color(other),
                    };
                }
                None => (),
            },
            Card::WildDrawFour => self.players[self.get_next_player() as usize]
                .add_cards(&self.draw_pile.draw_cards(4, &mut self.discard_pile)),
        }
    }

    pub fn choose_color(&self, other: &Card) -> Color {
        match other {
            Card::Number { color, number: _ } => color.clone(),
            Card::Reverse { color } => color.clone(),
            Card::Skip { color } => color.clone(),
            Card::DrawTwo { color } => color.clone(),
            Card::Wild { color } => color.clone(),
            Card::WildDrawFour => Color::Blue,
        }
    }

    pub fn start(&mut self) {
        for _ in 0..7 {
            for player in &self.players {
                let cards = self.draw_pile.draw_cards(1, &mut self.discard_pile);
                player.add_cards(&cards);
            }
        }

        // self.draw_pile.test();

        let mut beginning_card: Card;
        loop {
            beginning_card = self
                .draw_pile
                .draw_cards(1, &mut self.discard_pile)
                .remove(0);

            match beginning_card {
                Card::WildDrawFour => {
                    self.draw_pile.insert_and_shuffle(beginning_card);
                }
                Card::Wild { color: _ } => {
                    beginning_card = Card::Wild {
                        color: self.choose_color(&beginning_card),
                    };
                    break;
                }
                _ => break,
            }
        }

        self.run_card_action(&mut beginning_card, None);
        self.discard_pile.place_cards(&[beginning_card]);

        loop {
            let card = self.discard_pile.get_top_card().clone();
            println!(
                "{}\n\nPlayer {}'s turn\nCard: {:?}",
                "-".repeat(80),
                self.index,
                &card
            );

            let player = &mut self.players[self.index as usize];

            if let Some(mut chosen_card) =
                player.choose_card(&card, &mut self.draw_pile, &mut self.discard_pile)
            {
                self.run_card_action(&mut chosen_card, Some(&card));
                self.discard_pile.place_cards(&[chosen_card]);
            }

            if self.players[self.index as usize].has_no_cards() {
                break;
            }

            self.index = self.get_next_player();
        }

        println!("Player {} won!\n{:?}", self.index, self.players);

        let score = self.score_cards();
        self.players[self.index as usize].add_score(score);
    }

    pub fn score_cards(&mut self) -> i32 {
        self.players
            .iter()
            .map(|player| player.score_cards())
            .sum::<i32>()
    }
}

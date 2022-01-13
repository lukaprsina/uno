#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    All,
}

#[derive(Clone, Debug)]
pub enum Card {
    Number { color: Color, number: u16 },
    Reverse { color: Color },
    Skip { color: Color },
    DrawTwo { color: Color },
    Wild { color: Color },
    WildDrawFour,
}

// TODO: the comparisons are not equal if turned around
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        use crate::Card::*;

        match (self, other) {
            (
                Number {
                    color: c1,
                    number: n1,
                },
                Number {
                    color: c2,
                    number: n2,
                },
            ) => c1 == c2 || n1 == n2,
            (
                Number {
                    color: c1,
                    number: _,
                },
                Reverse { color: c2 },
            ) => c1 == c2,
            (
                Number {
                    color: c1,
                    number: _,
                },
                Skip { color: c2 },
            ) => c1 == c2,
            (
                Number {
                    color: c1,
                    number: _,
                },
                DrawTwo { color: c2 },
            ) => c1 == c2,
            (
                Number {
                    color: c1,
                    number: _,
                },
                Wild { color: c2 },
            ) => match c2 {
                Color::All => true,
                _ => c1 == c2,
            },
            (
                Number {
                    color: _,
                    number: _,
                },
                WildDrawFour,
            ) => true,
            (
                Reverse { color: c1 },
                Number {
                    color: c2,
                    number: _,
                },
            ) => c1 == c2,
            (Reverse { color: c1 }, Reverse { color: c2 }) => c1 == c2,
            (Reverse { color: c1 }, Skip { color: c2 }) => c1 == c2,
            (Reverse { color: c1 }, DrawTwo { color: c2 }) => c1 == c2,
            (Reverse { color: c1 }, Wild { color: c2 }) => match c2 {
                Color::All => true,
                _ => c1 == c2,
            },
            (Reverse { color: _ }, WildDrawFour) => true,
            (
                Skip { color: c1 },
                Number {
                    color: c2,
                    number: _,
                },
            ) => c1 == c2,
            (Skip { color: c1 }, Reverse { color: c2 }) => c1 == c2,
            (Skip { color: c1 }, Skip { color: c2 }) => c1 == c2,
            (Skip { color: c1 }, DrawTwo { color: c2 }) => c1 == c2,
            (Skip { color: c1 }, Wild { color: c2 }) => match c2 {
                Color::All => true,
                _ => c1 == c2,
            },
            (Skip { color: _ }, WildDrawFour) => true,
            (
                DrawTwo { color: c1 },
                Number {
                    color: c2,
                    number: _,
                },
            ) => c1 == c2,
            (DrawTwo { color: c1 }, Reverse { color: c2 }) => c1 == c2,
            (DrawTwo { color: c1 }, Skip { color: c2 }) => c1 == c2,
            (DrawTwo { color: c1 }, DrawTwo { color: c2 }) => c1 == c2,
            (DrawTwo { color: c1 }, Wild { color: c2 }) => match c2 {
                Color::All => true,
                _ => c1 == c2,
            },
            (DrawTwo { color: _ }, WildDrawFour) => true,
            (
                Wild { color: c1 },
                Number {
                    color: c2,
                    number: _,
                },
            ) => match c1 {
                Color::All => true,
                _ => c1 == c2,
            },
            (Wild { color: c1 }, Reverse { color: c2 }) => match c1 {
                Color::All => true,
                _ => c1 == c2,
            },
            (Wild { color: c1 }, Skip { color: c2 }) => match c1 {
                Color::All => true,
                _ => c1 == c2,
            },
            (Wild { color: c1 }, DrawTwo { color: c2 }) => match c1 {
                Color::All => true,
                _ => c1 == c2,
            },
            (Wild { color: c1 }, Wild { color: c2 }) => {
                if c1 == &Color::All || c2 == &Color::All {
                    true
                } else {
                    c1 == c2
                }
            }
            (Wild { color: _ }, WildDrawFour) => true,
            (
                WildDrawFour,
                Number {
                    color: _,
                    number: _,
                },
            ) => true,
            (WildDrawFour, Reverse { color: _ }) => true,
            (WildDrawFour, Skip { color: _ }) => true,
            (WildDrawFour, DrawTwo { color: _ }) => true,
            (WildDrawFour, Wild { color: _ }) => true,
            (WildDrawFour, WildDrawFour) => true,
        }
    }
}

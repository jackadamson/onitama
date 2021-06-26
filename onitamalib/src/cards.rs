use crate::models::{Card, Point};

impl Card {
    pub fn moves(&self) -> Vec<Point> {
        match self {
            Card::Tiger => vec![Point { x: 0, y: -2 }, Point { x: 0, y: 1 }],
            Card::Dragon => vec![Point { x: -2, y: -1 }, Point { x: -1, y: 1 }, Point { x: 2, y: -1 }, Point { x: 1, y: 1 }],
            Card::Frog => vec![Point { x: -2, y: 0 }, Point { x: -1, y: -1 }, Point { x: 1, y: 1 }],
            Card::Rabbit => vec![Point { x: 1, y: -1 }, Point { x: 2, y: 0 }, Point { x: -1, y: 1 }],
            Card::Crab => vec![Point { x: 0, y: -1 }, Point { x: -2, y: 0 }, Point { x: 2, y: 0 }],
            Card::Elephant => vec![Point { x: 1, y: 0 }, Point { x: -1, y: -1 }, Point { x: 1, y: -1 }, Point { x: -1, y: 0 }],
            Card::Goose => vec![Point { x: -1, y: 0 }, Point { x: -1, y: -1 }, Point { x: 1, y: 0 }, Point { x: 1, y: 1 }],
            Card::Rooster => vec![Point { x: 1, y: 0 }, Point { x: 1, y: -1 }, Point { x: -1, y: 0 }, Point { x: -1, y: 1 }],
            Card::Monkey => vec![Point { x: -1, y: -1 }, Point { x: 1, y: -1 }, Point { x: -1, y: 1 }, Point { x: 1, y: 1 }],
            Card::Mantis => vec![Point { x: -1, y: -1 }, Point { x: 1, y: -1 }, Point { x: 0, y: 1 }],
            Card::Horse => vec![Point { x: 0, y: -1 }, Point { x: -1, y: 0 }, Point { x: 0, y: 1 }],
            Card::Ox => vec![Point { x: 0, y: -1 }, Point { x: 1, y: 0 }, Point { x: 0, y: 1 }],
            Card::Crane => vec![Point { x: 0, y: -1 }, Point { x: 1, y: 1 }, Point { x: -1, y: 1 }],
            Card::Boar => vec![Point { x: 0, y: -1 }, Point { x: 1, y: 0 }, Point { x: -1, y: 0 }],
            Card::Eel => vec![Point { x: 1, y: 0 }, Point { x: -1, y: -1 }, Point { x: -1, y: 1 }],
            Card::Cobra => vec![Point { x: -1, y: 0 }, Point { x: 1, y: -1 }, Point { x: 1, y: 1 }],
        }
    }
    pub fn index(&self) -> u32 {
        match self {
            Card::Tiger => 0,
            Card::Dragon => 1,
            Card::Frog => 2,
            Card::Rabbit => 3,
            Card::Crab => 4,
            Card::Elephant => 5,
            Card::Goose => 6,
            Card::Rooster => 7,
            Card::Monkey => 8,
            Card::Mantis => 9,
            Card::Horse => 10,
            Card::Ox => 11,
            Card::Crane => 12,
            Card::Boar => 13,
            Card::Eel => 14,
            Card::Cobra => 15,
        }
    }
}

impl From<u32> for Card {
    fn from(idx: u32) -> Self {
        match idx {
            0 => Card::Tiger,
            1 => Card::Dragon,
            2 => Card::Frog,
            3 => Card::Rabbit,
            4 => Card::Crab,
            5 => Card::Elephant,
            6 => Card::Goose,
            7 => Card::Rooster,
            8 => Card::Monkey,
            9 => Card::Mantis,
            10 => Card::Horse,
            11 => Card::Ox,
            12 => Card::Crane,
            13 => Card::Boar,
            14 => Card::Eel,
            15 => Card::Cobra,
            _ => panic!("invalid index for card"),
        }
    }
}

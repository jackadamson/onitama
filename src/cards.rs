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
}

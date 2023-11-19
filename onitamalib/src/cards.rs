use crate::models::{Card, CardDirection, CardSet, Point};

impl Card {
    pub fn moves(&self) -> Vec<Point> {
        match self {
            Card::Tiger => vec![Point { x: 0, y: -2 }, Point { x: 0, y: 1 }],
            Card::Dragon => vec![
                Point { x: -2, y: -1 },
                Point { x: -1, y: 1 },
                Point { x: 2, y: -1 },
                Point { x: 1, y: 1 },
            ],
            Card::Frog => vec![
                Point { x: -2, y: 0 },
                Point { x: -1, y: -1 },
                Point { x: 1, y: 1 },
            ],
            Card::Rabbit => vec![
                Point { x: 1, y: -1 },
                Point { x: 2, y: 0 },
                Point { x: -1, y: 1 },
            ],
            Card::Crab => vec![
                Point { x: 0, y: -1 },
                Point { x: -2, y: 0 },
                Point { x: 2, y: 0 },
            ],
            Card::Elephant => vec![
                Point { x: 1, y: 0 },
                Point { x: -1, y: -1 },
                Point { x: 1, y: -1 },
                Point { x: -1, y: 0 },
            ],
            Card::Goose => vec![
                Point { x: -1, y: 0 },
                Point { x: -1, y: -1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
            ],
            Card::Rooster => vec![
                Point { x: 1, y: 0 },
                Point { x: 1, y: -1 },
                Point { x: -1, y: 0 },
                Point { x: -1, y: 1 },
            ],
            Card::Monkey => vec![
                Point { x: -1, y: -1 },
                Point { x: 1, y: -1 },
                Point { x: -1, y: 1 },
                Point { x: 1, y: 1 },
            ],
            Card::Mantis => vec![
                Point { x: -1, y: -1 },
                Point { x: 1, y: -1 },
                Point { x: 0, y: 1 },
            ],
            Card::Horse => vec![
                Point { x: 0, y: -1 },
                Point { x: -1, y: 0 },
                Point { x: 0, y: 1 },
            ],
            Card::Ox => vec![
                Point { x: 0, y: -1 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
            ],
            Card::Crane => vec![
                Point { x: 0, y: -1 },
                Point { x: 1, y: 1 },
                Point { x: -1, y: 1 },
            ],
            Card::Boar => vec![
                Point { x: 0, y: -1 },
                Point { x: 1, y: 0 },
                Point { x: -1, y: 0 },
            ],
            Card::Eel => vec![
                Point { x: 1, y: 0 },
                Point { x: -1, y: -1 },
                Point { x: -1, y: 1 },
            ],
            Card::Cobra => vec![
                Point { x: -1, y: 0 },
                Point { x: 1, y: -1 },
                Point { x: 1, y: 1 },
            ],

            // Sensei's Path expansion
            Card::Fox => vec![
                Point { x: 1, y: -1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
            ],
            Card::Dog => vec![
                Point { x: -1, y: -1 },
                Point { x: -1, y: 0 },
                Point { x: -1, y: 1 },
            ],
            Card::Giraffe => vec![
                Point { x: -2, y: -1 },
                Point { x: 2, y: -1 },
                Point { x: 0, y: 1 },
            ],
            Card::Panda => vec![
                Point { x: -1, y: 1 },
                Point { x: 0, y: -1 },
                Point { x: 1, y: -1 },
            ],
            Card::Bear => vec![
                Point { x: 1, y: 1 },
                Point { x: 0, y: -1 },
                Point { x: -1, y: -1 },
            ],
            Card::Kirin => vec![
                Point { x: 0, y: 2 },
                Point { x: 1, y: -2 },
                Point { x: -1, y: -2 },
            ],
            Card::SeaSnake => vec![
                Point { x: -1, y: 1 },
                Point { x: 0, y: -1 },
                Point { x: 2, y: 0 },
            ],
            Card::Viper => vec![
                Point { x: 1, y: 1 },
                Point { x: 0, y: -1 },
                Point { x: -2, y: 0 },
            ],
            Card::Phoenix => vec![
                Point { x: -2, y: 0 },
                Point { x: -1, y: -1 },
                Point { x: 1, y: -1 },
                Point { x: 2, y: 0 },
            ],
            Card::Mouse => vec![
                Point { x: -1, y: 1 },
                Point { x: 0, y: -1 },
                Point { x: 1, y: 0 },
            ],
            Card::Rat => vec![
                Point { x: 1, y: 1 },
                Point { x: 0, y: -1 },
                Point { x: -1, y: 0 },
            ],
            Card::Turtle => vec![
                Point { x: -2, y: 0 },
                Point { x: -1, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 0 },
            ],
            Card::Tanuki => vec![
                Point { x: -1, y: 1 },
                Point { x: 0, y: -1 },
                Point { x: 2, y: -1 },
            ],
            Card::Iguana => vec![
                Point { x: 1, y: 1 },
                Point { x: 0, y: -1 },
                Point { x: -2, y: -1 },
            ],
            Card::Sable => vec![
                Point { x: -2, y: 0 },
                Point { x: -1, y: 1 },
                Point { x: 1, y: -1 },
            ],
            Card::Otter => vec![
                Point { x: 2, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: -1, y: -1 },
            ],
            // Way of the Wind Expansion
            Card::Goat => vec![
                Point { x: -1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: -1 },
            ],
            Card::Sheep => vec![
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: -1, y: -1 },
            ],

            // Promotional Cards
            Card::Lobster => vec![
                Point { x: -1, y: -1 },
                Point { x: -1, y: 2 },
                Point { x: 1, y: -1 },
                Point { x: 1, y: 2 },
            ],
            Card::Steer => vec![
                Point { x: 1, y: 0 },
                Point { x: -1, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: -1, y: 0 },
            ],
            Card::Hornet => vec![
                Point { x: -2, y: 2 },
                Point { x: 0, y: -1 },
                Point { x: 1, y: 0 },
            ],
            Card::Centipede => vec![
                Point { x: -1, y: 0 },
                Point { x: 0, y: -1 },
                Point { x: 2, y: 2 },
            ],
            Card::Cat => vec![
                Point { x: -1, y: 0 },
                Point { x: 0, y: -1 },
                Point { x: 0, y: 2 },
                Point { x: 2, y: 0 },
            ],
            Card::Serow => vec![
                Point { x: -2 y: 0 },
                Point { x: 0, y: -1 },
                Point { x: 0, y: 2 },
                Point { x: 1, y: 0 },
            ],
        }
    }
    pub fn direction(&self) -> CardDirection {
        match self {
            // Base Game
            Card::Tiger => CardDirection::Balanced,
            Card::Dragon => CardDirection::Balanced,
            Card::Frog => CardDirection::Left,
            Card::Rabbit => CardDirection::Right,
            Card::Crab => CardDirection::Balanced,
            Card::Elephant => CardDirection::Balanced,
            Card::Goose => CardDirection::Left,
            Card::Rooster => CardDirection::Right,
            Card::Monkey => CardDirection::Balanced,
            Card::Mantis => CardDirection::Balanced,
            Card::Horse => CardDirection::Left,
            Card::Ox => CardDirection::Right,
            Card::Crane => CardDirection::Balanced,
            Card::Boar => CardDirection::Balanced,
            Card::Eel => CardDirection::Left,
            Card::Cobra => CardDirection::Right,
            // Sensei's path expansion
            Card::Fox => CardDirection::Right,
            Card::Dog => CardDirection::Left,
            Card::Giraffe => CardDirection::Balanced,
            Card::Panda => CardDirection::Right,
            Card::Bear => CardDirection::Left,
            Card::Kirin => CardDirection::Balanced,
            Card::SeaSnake => CardDirection::Right,
            Card::Viper => CardDirection::Left,
            Card::Phoenix => CardDirection::Balanced,
            Card::Mouse => CardDirection::Right,
            Card::Rat => CardDirection::Left,
            Card::Turtle => CardDirection::Balanced,
            Card::Tanuki => CardDirection::Right,
            Card::Iguana => CardDirection::Left,
            Card::Sable => CardDirection::Right,
            Card::Otter => CardDirection::Left,
            Card::Goat => CardDirection::Right,
            Card::Sheep => CardDirection::Left,
            Card::Lobster => CardDirection::Balanced,
            Card::Steer => CardDirection::Balanced,
            Card::Hornet => CardDirection::Right,
            Card::Centipede => CardDirection::Left,
            Card::Cat => CardDirection::Right,
            Card::Serow => CardDirection::Left,
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
            // Sensei's path expansion
            Card::Fox => 16,
            Card::Dog => 17,
            Card::Giraffe => 18,
            Card::Panda => 19,
            Card::Bear => 20,
            Card::Kirin => 21,
            Card::SeaSnake => 22,
            Card::Viper => 23,
            Card::Phoenix => 24,
            Card::Mouse => 25,
            Card::Rat => 26,
            Card::Turtle => 27,
            Card::Tanuki => 28,
            Card::Iguana => 29,
            Card::Sable => 30,
            Card::Otter => 31,
            Card::Goat => 32,
            Card::Sheep => 33,
            Card::Lobster => 34,
            Card::Steer => 35,
            Card::Hornet => 36,
            Card::Centipede => 37,
            Card::Cat => 38,
            Card::Serow => 39,
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
            // Sensei's path expansion
            16 => Card::Fox,
            17 => Card::Dog,
            18 => Card::Giraffe,
            19 => Card::Panda,
            20 => Card::Bear,
            21 => Card::Kirin,
            22 => Card::SeaSnake,
            23 => Card::Viper,
            24 => Card::Phoenix,
            25 => Card::Mouse,
            26 => Card::Rat,
            27 => Card::Turtle,
            28 => Card::Tanuki,
            29 => Card::Iguana,
            30 => Card::Sable,
            31 => Card::Otter,
            32 => Card::Goat,
            33 => Card::Sheep,
            34 => Card::Lobster,
            35 => Card::Steer,
            36 => Card::Hornet,
            37 => Card::Centipede,
            38 => Card::Cat,
            39 => Card::Serow,
            _ => panic!("invalid index for card"),
        }
    }
}

impl CardSet {
    pub fn cards(&self) -> Vec<Card> {
        match &self {
            CardSet::Base => vec![
                Card::Tiger,
                Card::Dragon,
                Card::Frog,
                Card::Rabbit,
                Card::Crab,
                Card::Elephant,
                Card::Goose,
                Card::Rooster,
                Card::Monkey,
                Card::Mantis,
                Card::Horse,
                Card::Ox,
                Card::Crane,
                Card::Boar,
                Card::Eel,
                Card::Cobra,
            ],
            CardSet::SenseiPath => vec![
                Card::Fox,
                Card::Dog,
                Card::Giraffe,
                Card::Panda,
                Card::Bear,
                Card::Kirin,
                Card::SeaSnake,
                Card::Viper,
                Card::Phoenix,
                Card::Mouse,
                Card::Rat,
                Card::Turtle,
                Card::Tanuki,
                Card::Iguana,
                Card::Sable,
                Card::Otter,
            ],
            CardSet::WayOfTheWind => vec![Card::Goat, Card::Sheep],
            CardSet::PromotionalPack => vec![
                Card::Lobster, 
                Card::Steer, 
                Card::Hornet, 
                Card::Centipede, 
                Card::Cat, 
                Card::Serow,
            ],
            
        }
    }
}

use crate::models::{Card, CardDirection, CardSet, Point};

impl Card {
    pub fn moves(&self, is_king:bool, is_spirit:bool) -> Vec<Point> {
        match self {
            Card::Tiger => vec![
                Point { x: 0, y: -2 }, 
                Point { x: 0, y: 1 }
            ],
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

            // Sensei's Path
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
            // Way of the Wind
            Card::Bat => {
                if is_spirit {
                    vec![
                        Point { x: -2, y: -1 },
                        Point { x: -1, y: -1 },
                        Point { x: 1, y: -1 },
                        Point { x: 2, y: -1 },
                    ]
                } else {
                    vec![
                        Point { x: 0, y: 1 },
                        Point { x: 0, y: -1 },
                    ]    
                }
            },
            Card::Eagle => {
                if is_spirit {
                    vec![
                        Point { x: -2, y: -2 },
                        Point { x: 2, y: -2 },
                    ]
                } else {
                    vec![
                        Point { x: -1, y: -1 },
                        Point { x: 1, y: -1 },
                    ]    
                }
            },
            Card::Hawk => {
                if is_spirit {
                    vec![
                        Point { x: -2, y: 0 },
                        Point { x: -2, y: -1 },
                        Point { x: 2, y: -1 },
                        Point { x: 2, y: 0 },
                    ]
                } else {
                    vec![
                        Point { x: -1, y: -1 },
                        Point { x: -1, y: 1 },
                    ]    
                }
            },
            Card::Lion => {
                if is_spirit {
                    vec![
                        Point { x: 0, y: -1 },
                        Point { x: 0, y: -2 },
                    ]
                } else {
                    vec![
                        Point { x: -1, y: 1 },
                        Point { x: 1, y: -1 },
                    ]    
                }
            },
            Card::Octopus => {
                if is_spirit {
                    vec![
                        Point { x: -1, y: 0 },
                        Point { x: -1, y: 1 },
                        Point { x: 0, y: -1 },
                        Point { x: 0, y: 1 },    
                        Point { x: 1, y: 0 },
                        Point { x: 1, y: 1 },  
                    ]
                } else {
                    vec![
                        Point { x: -1, y: -1 },
                        Point { x: 1, y: 1 },
                    ]    
                }
            },
            Card::Rhinoceros => {
                if is_spirit {
                    vec![
                        Point { x: -2, y: 0 },
                        Point { x: -1, y: -1 },
                        Point { x: 0, y: -1 },
                        Point { x: 1, y: -1 },    
                        Point { x: 2, y: 0 },
                    ]
                } else {
                    vec![
                        Point { x: -1, y: -1 },
                        Point { x: 0, y: 1 },
                    ]    
                }
            },
            Card::Scorpion => {
                if is_spirit {
                    vec![
                        Point { x: -2, y: -1 },
                        Point { x: -1, y: -2 },
                        Point { x: 1, y: -2 },
                        Point { x: 2, y: -1 },    
                    ]
                } else {
                    vec![
                        Point { x: 1, y: 1 },
                        Point { x: 1, y: -1 },
                    ]    
                }
            },
            Card::Spider => {
                if is_spirit {
                    vec![
                        Point { x: -1, y: -1 },
                        Point { x: 0, y: -1 },
                        Point { x: 0, y: 1 },
                        Point { x: 1, y: -1 },    
                    ]
                } else {
                    vec![
                        Point { x: 1, y: -1 },
                        Point { x: 0, y: 1 },
                    ]    
                }
            },
            // Promotional Cards
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
                Point { x: -2, y: 0 },
                Point { x: 0, y: -1 },
                Point { x: 0, y: 2 },
                Point { x: 1, y: 0 },
            ],
            Card::Nessie => vec![
                Point { x: -2, y: -1 },
                Point { x: 2, y: 0 },
                Point { x: -1, y: 1 },
                Point { x: 1, y: 1 },
            ],
            Card::Butterfly => vec![
                Point { x: -2, y: -1 },
                Point { x: 2, y: -1 },
                Point { x: 0, y: 1 },
            ],
            Card::Moth => vec![
                Point { x: -2, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 0, y: -1 },
            ],
            Card::Okija => {
                if is_king {
                    vec![
                        Point { x: -2, y: -2 },
                        Point { x: 2, y: -2 },
                    ]
                } else {
                    vec![
                        Point { x: 0, y: -1 },
                    ]
                }
            },
            Card::Mejika => {
                if is_king {
                    vec![
                        Point { x: 0, y: -1 },
                    ]
                } else {
                    vec![
                        Point { x: -2, y: -1 },
                        Point { x: 2, y: -1 },
                    ]
                }
            },
            Card::Kumo => {
                if is_king {
                    vec![
                        Point { x: -2, y: 0 },
                        Point { x: 2, y: 0 },
                    ]
                } else {
                    vec![
                        Point { x: -1, y: -1 },
                        Point { x: 1, y: -1 },
                    ]
                }
            },
            Card::Sasori => {
                if is_king {
                    vec![
                        Point { x: -1, y: -2 },
                        Point { x: 1, y: -2 },
                    ]
                } else {
                    vec![
                        Point { x: -2, y: 0 },
                        Point { x: 2, y: 0 },
                    ]
                }
            },
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
            // Sensei's Path
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
            // Way of the Wind
            Card::Bat => CardDirection::Balanced,
            Card::Eagle => CardDirection::Balanced,
            Card::Hawk => CardDirection::Left,
            Card::Lion => CardDirection::Right,
            Card::Octopus => CardDirection::Left,
            Card::Rhinoceros => CardDirection::Left,
            Card::Scorpion => CardDirection::Right,
            Card::Spider => CardDirection::Right,
            // Promotional Cards
            Card::Goat => CardDirection::Right,
            Card::Sheep => CardDirection::Left,
            Card::Okija => CardDirection::Balanced,
            Card::Mejika => CardDirection::Balanced,
            Card::Kumo => CardDirection::Balanced,
            Card::Sasori => CardDirection::Balanced,
            Card::Lobster => CardDirection::Balanced,
            Card::Steer => CardDirection::Balanced,
            Card::Hornet => CardDirection::Right,
            Card::Centipede => CardDirection::Left,
            Card::Nessie => CardDirection::Balanced,
            Card::Cat => CardDirection::Right,
            Card::Serow => CardDirection::Left,
            Card::Butterfly => CardDirection::Balanced,
            Card::Moth => CardDirection::Balanced,

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
            // Sensei's Path
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
            // Way of the Wind
            Card::Bat => 32,
            Card::Eagle => 33,
            Card::Hawk => 34,
            Card::Lion => 35,
            Card::Octopus => 36,
            Card::Rhinoceros => 37,
            Card::Scorpion => 38,
            Card::Spider => 39,
            // Promotional Cards
            Card::Goat => 40,
            Card::Sheep => 41,
            Card::Okija => 42,
            Card::Mejika => 43,
            Card::Kumo => 44,
            Card::Sasori => 45,
            Card::Lobster => 46,
            Card::Steer => 47,
            Card::Hornet => 48,
            Card::Centipede => 49,
            Card::Nessie => 50,
            Card::Cat => 51,
            Card::Serow => 52,
            Card::Butterfly => 53,
            Card::Moth => 54,
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
            // Sensei's Path
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
            // Way of the Wind
            32 => Card::Bat,
            33 => Card::Eagle,
            34 => Card::Hawk,
            35 => Card::Lion,
            36 => Card::Octopus,
            37 => Card::Rhinoceros,
            38 => Card::Scorpion,
            39 => Card::Spider,
            // Promotional Cards
            40 => Card::Goat,
            41 => Card::Sheep,
            42 => Card::Okija,
            43 => Card::Mejika,
            44 => Card::Kumo,
            45 => Card::Sasori,
            46 => Card::Lobster,
            47 => Card::Steer,
            48 => Card::Hornet,
            49 => Card::Centipede,
            50 => Card::Nessie,
            51 => Card::Cat,
            52 => Card::Serow,
            53 => Card::Butterfly,
            54 => Card::Moth,
         
            _ => panic!("invalid index for card"),
        }
    }
}

impl CardSet {
    pub fn cards(&self) -> Vec<Card> {
        match &self {
            CardSet::Base => vec![
                Card::Boar,
                Card::Crab,
                Card::Elephant,
                Card::Dragon,
                Card::Monkey,
                Card::Tiger,
                Card::Crane,
                Card::Mantis,
                Card::Eel,
                Card::Cobra,
                Card::Frog,
                Card::Rabbit,
                Card::Goose,
                Card::Rooster,
                Card::Horse,
                Card::Ox,
            ],
            CardSet::SenseiPath => vec![
                Card::Giraffe,
                Card::Kirin,
                Card::Phoenix,
                Card::Turtle,
                Card::Bear,
                Card::Panda,
                Card::Dog,
                Card::Fox,
                Card::Iguana,
                Card::Tanuki,
                Card::Rat,
                Card::Mouse,
                Card::Otter,
                Card::Sable,
                Card::Viper,
                Card::SeaSnake,
            ],            
            CardSet::WayOfTheWind => vec![
                Card::Bat,
                Card::Eagle,
                Card::Hawk,
                Card::Lion,
                Card::Octopus,
                Card::Rhinoceros,
                Card::Scorpion,
                Card::Spider,
            ],
            CardSet::PromotionalPack => vec![
                Card::Sheep,
                Card::Goat,
                Card::Okija,
                Card::Mejika,
                Card::Kumo,
                Card::Sasori,  
                Card::Lobster, 
                Card::Steer, 
                Card::Centipede, 
                Card::Hornet, 
                Card::Nessie,
                Card::Serow,
                Card::Cat, 
                Card::Butterfly,
                Card::Moth,
            ],

        }
    }
}

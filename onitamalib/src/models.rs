use std::ops::{Add,Sub,Neg};
use std::fmt;
use enum_iterator::IntoEnumIterator;
use serde::{Serialize, Deserialize};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Player {
    Red,
    Blue,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Point {
    pub(crate) fn out_of_bounds(&self) -> bool {
        self.x < 0 || self.x > 4 || self.y < 0 || self.y > 4
    }
}

#[derive(Eq, PartialEq, Copy, Clone, IntoEnumIterator, Debug, Serialize, Deserialize)]
pub enum Card {
    Tiger,
    Dragon,
    Frog,
    Rabbit,
    Crab,
    Elephant,
    Goose,
    Rooster,
    Monkey,
    Mantis,
    Horse,
    Ox,
    Crane,
    Boar,
    Eel,
    Cobra,
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    pub blue_king: Point,
    pub blue_pawns: Vec<Point>,
    pub blue_hand: Vec<Card>,
    pub red_king: Point,
    pub red_pawns: Vec<Point>,
    pub red_hand: Vec<Card>,
    pub spare_card: Card,
    pub turn: Player,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Move {
    Move {
        card: Card,
        src: Point,
        dst: Point,
    },
    Null {
        card: Card,
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum GameState {
    Playing {
        board: Board,
    },
    Finished {
        winner: Player,
    },
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum GameSquare {
    RedKing,
    RedPawn,
    BlueKing,
    BluePawn,
    Empty,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "status")]
pub enum GameView {
    Playing {
        grid: [[GameSquare; 5]; 5],
        #[serde(rename = "redCards")]
        red_cards: Vec<CardDescription>,
        #[serde(rename = "blueCards")]
        blue_cards: Vec<CardDescription>,
        spare: CardDescription,
        turn: Player,
    },
    Finished {
        winner: Player,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardDescription {
    pub card: Card,
    pub moves: Vec<Point>,
}

impl From<Card> for CardDescription {
    fn from(card: Card) -> Self {
        let moves = card.moves();
        CardDescription { card, moves }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "status")]
pub enum GameError {
    Error {
        message: String,
    }
}

impl From<&GameState> for GameView {
    fn from(state: &GameState) -> Self {
        let to_card = |card: &Card| CardDescription::from(*card);
        match state {
            GameState::Playing { board } => Self::Playing {
                grid: board.to_grid(),
                red_cards: board.red_hand.iter().map(to_card).collect(),
                blue_cards: board.blue_hand.iter().map(to_card).collect(),
                spare: to_card(&board.spare_card),
                turn: board.turn,
            },
            GameState::Finished { winner } => Self::Finished { winner: *winner },
        }
    }
}

use std::fmt;
use std::ops::{Add, Neg, Sub};

use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};
use crate::AiAgent;

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

impl Player {
    pub fn invert(&self) -> Player {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        }
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
    pub fn out_of_bounds(&self) -> bool {
        self.x < 0 || self.x > 4 || self.y < 0 || self.y > 4
    }
    pub fn in_bounds(&self) -> bool {
        !self.out_of_bounds()
    }
    pub fn invert(&self) -> Point {
        Point {
            x: 4 - self.x,
            y: 4 - self.y,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum CardDirection {
    Right,
    Left,
    Balanced,
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
    Fox,
    Dog,
    Giraffe,
    Panda,
    Bear,
    Kirin,
    SeaSnake,
    Viper,
    Phoenix,
    Mouse,
    Rat,
    Turtle,
    Tanuki,
    Iguana,
    Sable,
    Otter,
    Goat,
    Sheep,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Board {
    pub blue_king: Point,
    pub blue_pawns: [Option<Point>; 4],
    pub blue_hand: [Card; 2],
    pub red_king: Point,
    pub red_pawns: [Option<Point>; 4],
    pub red_hand: [Card; 2],
    pub spare_card: Card,
    pub turn: Player,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Move {
    Move {
        card: Card,
        src: Point,
        dst: Point,
    },
    Discard {
        card: Card,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(tag = "status")]
pub enum GameState {
    Playing {
        board: Board,
    },
    Finished {
        board: Board,
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
        #[serde(rename = "canMove")]
        can_move: bool,
    },
    Finished {
        winner: Player,
        grid: [[GameSquare; 5]; 5],
        #[serde(rename = "redCards")]
        red_cards: Vec<CardDescription>,
        #[serde(rename = "blueCards")]
        blue_cards: Vec<CardDescription>,
        spare: CardDescription,
        turn: Player,
        #[serde(rename = "canMove")]
        can_move: bool,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardDescription {
    pub card: Card,
    pub moves: Vec<Point>,
    pub direction: CardDirection
}

impl From<Card> for CardDescription {
    fn from(card: Card) -> Self {
        let moves = card.moves();
        let direction = card.direction();
        CardDescription { card, moves, direction }
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
                can_move: board.can_move(),
            },
            GameState::Finished { winner, board } => Self::Finished {
                winner: *winner,
                grid: board.to_grid(),
                red_cards: board.red_hand.iter().map(to_card).collect(),
                blue_cards: board.blue_hand.iter().map(to_card).collect(),
                spare: to_card(&board.spare_card),
                turn: board.turn,
                can_move: true,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GameEvent {
    Start {
        against: String,
        #[serde(default)]
        training: bool,
    },
    End {
        against: String,
        winner: String,
        #[serde(default)]
        training: bool,
    },
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveRequest {
    pub state: GameState,
    pub agent: AiAgent,
}

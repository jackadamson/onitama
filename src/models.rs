use std::ops::{Add,Sub,Neg};
use std::fmt;
use enum_iterator::IntoEnumIterator;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Player {
    Red,
    Blue,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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

#[derive(Eq, PartialEq, Copy, Clone, IntoEnumIterator, Debug)]
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
#[derive(Clone, Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum GameState {
    Playing {
        board: Board,
    },
    Finished {
        winner: Player,
    },
}

#[derive(Copy, Clone)]
pub enum GameSquare {
    RedKing,
    RedPawn,
    BlueKing,
    BluePawn,
    Empty,
}

impl GameSquare {
    pub fn render(&self, callback: Callback<MouseEvent>) -> VNode {
        match self {
            GameSquare::RedKing => html!{
                <div onclick=callback class="cell red king">{"X"}</div>
            },
            GameSquare::RedPawn => html!{
                <div onclick=callback class="cell red pawn">{"o"}</div>
            },
            GameSquare::BlueKing => html!{
                <div onclick=callback class="cell blue king">{"X"}</div>
            },
            GameSquare::BluePawn => html!{
                <div onclick=callback class="cell blue pawn">{"o"}</div>
            },
            GameSquare::Empty => html!{
                <div onclick=callback class="cell empty"></div>
            },
        }
    }
}
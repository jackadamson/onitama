use core::iter;

use enum_iterator::IntoEnumIterator;
use rand::prelude::*;

use crate::models::{Board, Card, GameSquare, GameState, Move, Player, Point};

impl Board {
    pub fn try_move(self: &Board, game_move: Move) -> Result<GameState, String> {
        let (
            blue_king, blue_pawns, blue_hand, red_king, red_pawns, red_hand, spare_card, turn
        ) = match self {
            Board { blue_king, blue_pawns, blue_hand, red_king, red_pawns, red_hand, spare_card, turn } => {
                (blue_king, blue_pawns, blue_hand, red_king, red_pawns, red_hand, spare_card, turn)
            }
        };
        let (player_king, opponent_king) = match turn {
            Player::Red => (red_king, blue_king),
            Player::Blue => (blue_king, red_king),
        };
        let (player_pawns, opponent_pawns) = match turn {
            Player::Red => (red_pawns.iter(), blue_pawns.iter()),
            Player::Blue => (blue_pawns.iter(), red_pawns.iter()),
        };
        let player_pieces: Vec<&Point> = match turn {
            Player::Red => red_pawns
                .iter()
                .chain(iter::once(red_king))
                .collect(),
            Player::Blue => blue_pawns
                .iter()
                .chain(iter::once(blue_king))
                .collect(),
        };
        let (card, src, dst) = match game_move {
            Move::Move { card, src, dst } => (card, src, dst),
            Move::Forfeit => {
                return Ok(GameState::Finished { winner: turn.invert(), board: self.clone() })
            },
            Move::Discard { card } => {
                if self.can_move() {
                    return Err(format!("Valid moves exist"));
                }
                let player_hand: Vec<Card> = self
                    .player_hand()
                    .iter()
                    .map(|c| *c)
                    .filter(|c| *c != card)
                    .chain(iter::once(*spare_card))
                    .collect();
                let (red_hand, blue_hand) = match turn {
                    Player::Red => (player_hand, blue_hand.clone()),
                    Player::Blue => (red_hand.clone(), player_hand),
                };
                return Ok(GameState::Playing {
                    board: Board {
                        blue_king: *blue_king,
                        blue_pawns: blue_pawns.clone(),
                        blue_hand,
                        red_king: *red_king,
                        red_pawns: red_pawns.clone(),
                        red_hand,
                        spare_card: card,
                        turn: turn.invert(),
                    }
                });
            }
        };
        if !self.player_hand().contains(&card) {
            return Err("Card not in hand".to_string());
        }
        if !player_pieces.contains(&&src) {
            return Err("No piece at src".to_string());
        }
        if player_pieces.contains(&&dst) {
            return Err("Destination occupied by your piece".to_string());
        }
        if dst.out_of_bounds() {
            return Err("Destination is out of bounds".to_string());
        }
        let raw_delta = dst - src;
        let delta = match turn {
            Player::Red => raw_delta,
            Player::Blue => -raw_delta,
        };
        let moves = card.moves();
        if !moves.contains(&delta) {
            log::info!("Attempted {:?} with card {:?}", &delta, &card);
            return Err("Move not valid for card".to_string());
        }
        let goal_square = match turn {
            Player::Red => Point { x: 2, y: 0 },
            Player::Blue => Point { x: 2, y: 4 },
        };
        let moving_king = *player_king == src;
        let player_pawns = player_pawns.filter_map(
            |pawn| match *pawn == src {
                true => None,
                false => Some(pawn.clone()),
            }
        ).into_iter();
        let player_pawns: Vec<Point> = match moving_king {
            true => player_pawns.collect(),
            false => player_pawns.chain(iter::once(dst)).collect(),
        };
        let opponent_pawns: Vec<Point> = opponent_pawns.filter_map(
            |pawn| match *pawn == dst {
                true => None,
                false => Some(*pawn),
            }
        ).collect();
        let player_hand: Vec<Card> = self.player_hand()
            .iter()
            .map(|c| *c)
            .filter(|c| *c != card)
            .chain(iter::once(*spare_card))
            .collect();
        let player_king = match moving_king {
            true => dst,
            false => *player_king,
        };
        let board = match self.turn {
            Player::Red => Board {
                blue_king: *blue_king,
                blue_pawns: opponent_pawns,
                blue_hand: blue_hand.clone(),
                red_king: player_king,
                red_pawns: player_pawns,
                red_hand: player_hand,
                spare_card: card,
                turn: Player::Blue,
            },
            Player::Blue => Board {
                blue_king: player_king,
                blue_pawns: player_pawns,
                blue_hand: player_hand,
                red_king: *red_king,
                red_pawns: opponent_pawns,
                red_hand: red_hand.clone(),
                spare_card: card,
                turn: Player::Red,
            }
        };
        if dst == *opponent_king {
            return Ok(GameState::Finished { winner: *turn, board });
        }
        if moving_king && dst == goal_square {
            return Ok(GameState::Finished { winner: *turn, board });
        }
        return Ok(GameState::Playing { board });
    }
    pub fn new() -> Board {
        let mut rng = thread_rng();
        let mut cards: Vec<Card> = Card::into_enum_iter().collect();
        cards.shuffle(&mut rng);
        let mut cards = cards.into_iter();
        let pawn_xs: Vec<i8> = vec![0, 1, 3, 4];
        Board {
            blue_king: Point { x: 2, y: 0 },
            blue_pawns: pawn_xs
                .iter()
                .map(|x| Point { x: *x, y: 0 })
                .collect(),
            blue_hand: vec![cards.next().unwrap(), cards.next().unwrap()],
            red_king: Point { x: 2, y: 4 },
            red_pawns: pawn_xs
                .iter()
                .map(|x| Point { x: *x, y: 4 })
                .collect(),
            red_hand: vec![cards.next().unwrap(), cards.next().unwrap()],
            spare_card: cards.next().unwrap(),
            turn: Player::Red,
        }
    }
    pub fn to_grid(&self) -> [[GameSquare; 5]; 5] {
        let mut grid = [[GameSquare::Empty; 5]; 5];
        for Point { x, y } in self.blue_pawns.iter() {
            grid[*y as usize][*x as usize] = GameSquare::BluePawn;
        }
        for Point { x, y } in self.red_pawns.iter() {
            grid[*y as usize][*x as usize] = GameSquare::RedPawn;
        }
        let Point { x, y } = self.red_king;
        grid[y as usize][x as usize] = GameSquare::RedKing;
        let Point { x, y } = self.blue_king;
        grid[y as usize][x as usize] = GameSquare::BlueKing;
        return grid;
    }
    pub fn can_move(&self) -> bool {
        let player_pieces = self.player_pieces();
        for src in player_pieces.iter() {
            for card in self.player_hand() {
                for raw_delta in card.moves() {
                    let delta = match self.turn {
                        Player::Red => raw_delta,
                        Player::Blue => -raw_delta,
                    };
                    let dst = delta + *src;
                    if dst.in_bounds() && !player_pieces.contains(&&dst) {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState::Playing {
            board: Board::new(),
        }
    }
}

impl Board {
    pub fn player_hand(&self) -> &Vec<Card> {
        match self.turn {
            Player::Red => &self.red_hand,
            Player::Blue => &self.blue_hand,
        }
    }
    pub fn opponent_hand(&self) -> &Vec<Card> {
        match self.turn.invert() {
            Player::Red => &self.red_hand,
            Player::Blue => &self.blue_hand,
        }
    }
}


impl Board {
    pub fn player_pawns(&self) -> &Vec<Point> {
        match self.turn {
            Player::Red => &self.red_pawns,
            Player::Blue => &self.blue_pawns,
        }
    }
    pub fn opponent_pawns(&self) -> &Vec<Point> {
        match self.turn.invert() {
            Player::Red => &self.red_pawns,
            Player::Blue => &self.blue_pawns,
        }
    }
}

impl Board {
    pub fn player_king(&self) -> &Point {
        match self.turn {
            Player::Red => &self.red_king,
            Player::Blue => &self.blue_king,
        }
    }
    pub fn opponent_king(&self) -> &Point {
        match self.turn.invert() {
            Player::Red => &self.red_king,
            Player::Blue => &self.blue_king,
        }
    }
}

impl Board {
    pub fn player_pieces(&self) -> Vec<Point> {
        self.player_pawns()
            .iter()
            .chain(iter::once(self.player_king()))
            .map(|piece| *piece)
            .collect()
    }
    pub fn opponent_pieces(&self) -> Vec<Point> {
        self.opponent_pawns()
            .iter()
            .chain(iter::once(self.opponent_king()))
            .map(|piece| *piece)
            .collect()
    }
}

impl GameState {
    pub fn try_move(&self, game_move: Move) -> Result<GameState, String> {
        match self {
            GameState::Playing { board } => board.try_move(game_move),
            GameState::Finished { .. } => Err("Game already finished".to_string()),
        }
    }
}

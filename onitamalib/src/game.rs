use core::iter;
use rand::prelude::*;
use enum_iterator::IntoEnumIterator;
use crate::models::{Board, Card, GameState, Move, Player, Point, GameSquare};

impl Board {
    pub fn make_move(self: &Board, game_move: Move) -> Result<GameState, String> {
        let (
            blue_king, blue_pawns, blue_hand, red_king, red_pawns, red_hand, spare_card, turn
        ) = match self {
            Board { blue_king, blue_pawns, blue_hand, red_king, red_pawns, red_hand, spare_card, turn } => {
                (blue_king, blue_pawns, blue_hand, red_king, red_pawns, red_hand, spare_card, turn)
            }
        };
        let player_hand = match turn {
            Player::Red => red_hand,
            Player::Blue => blue_hand,
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
            Move::Null { card } => {
                for src in player_pieces.iter() {
                    for card in player_hand {
                        for raw_delta in card.moves() {
                            let delta = match turn {
                                Player::Red => raw_delta,
                                Player::Blue => -raw_delta,
                            };
                            let dst = delta + **src;
                            if !dst.out_of_bounds() && !player_pieces.contains(&&dst) {
                                log::error!("Valid card found: {:?} for piece at {:?}", card, src);
                                return Result::Err(format!("Valid card found: {:?} for piece at {:?}", card, src));
                            }
                        }
                    }
                }
                let next_turn = match turn {
                    Player::Red => Player::Blue,
                    Player::Blue => Player::Red,
                };
                let player_hand: Vec<Card> = player_hand
                    .iter()
                    .map(|c| *c)
                    .filter(|c| *c == card)
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
                        turn: next_turn,
                    }
                });
            }
        };
        if !player_hand.contains(&card) {
            return Result::Err("Card not in hand".to_string());
        }
        if !player_pieces.contains(&&src) {
            return Result::Err("No piece at src".to_string());
        }
        if player_pieces.contains(&&dst) {
            return Result::Err("Destination occupied by your piece".to_string());
        }
        if dst.out_of_bounds() {
            return Result::Err("Destination is out of bounds".to_string());
        }
        let raw_delta = dst - src;
        let delta = match turn {
            Player::Red => raw_delta,
            Player::Blue => -raw_delta,
        };
        let moves = card.moves();
        if !moves.contains(&delta) {
            let (x, y) = match delta { Point { x, y } => (x, y) };
            let msg = format!("Move not valid for card: {},{}", x, y);
            return Result::Err(msg);
        }
        if dst == *opponent_king {
            return Result::Ok(GameState::Finished { winner: *turn });
        }
        let goal_square = match turn {
            Player::Red => Point { x: 2, y: 0 },
            Player::Blue => Point { x: 2, y: 4 },
        };
        let moving_king = *player_king == src;
        if moving_king && dst == goal_square {
            return Result::Ok(GameState::Finished { winner: *turn });
        }
        let player_pawns = player_pawns.filter_map(
            |pawn| match *pawn == src {
                true => None,
                false => Some(pawn.clone()),
            }
        ).into_iter();
        log::info!("moving_king: {}", moving_king);
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
        let player_hand = match turn {
            Player::Red => red_hand.iter(),
            Player::Blue => blue_hand.iter(),
        };
        log::info!("Player hand before: {:?}", &player_hand);
        let player_hand: Vec<Card> = player_hand
            .map(|c| *c)
            .filter(|c| *c != card)
            .chain(iter::once(*spare_card))
            .collect();
        log::info!("Player hand after: {:?}", &player_hand);
        let player_king = match moving_king {
            true => dst,
            false => *player_king,
        };
        return match self.turn {
            Player::Red => Ok(GameState::Playing {
                board: Board {
                    blue_king: *blue_king,
                    blue_pawns: opponent_pawns,
                    blue_hand: blue_hand.clone(),
                    red_king: player_king,
                    red_pawns: player_pawns,
                    red_hand: player_hand,
                    spare_card: card,
                    turn: Player::Blue,
                }
            }),
            Player::Blue => Ok(GameState::Playing {
                board: Board {
                    blue_king: player_king,
                    blue_pawns: player_pawns,
                    blue_hand: player_hand,
                    red_king: *red_king,
                    red_pawns: opponent_pawns,
                    red_hand: red_hand.clone(),
                    spare_card: card,
                    turn: Player::Red,
                }
            }),
        };
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
            turn: Player::Red
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
}

impl GameState {
    pub fn new() -> GameState {
        GameState::Playing {
            board: Board::new(),
        }
    }
}

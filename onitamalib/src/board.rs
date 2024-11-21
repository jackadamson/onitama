use crate::CardSet;
use enum_iterator::IntoEnumIterator;
use rand::prelude::*;

use crate::models::{Board, Card, GameSquare, GameState, Move, Player, Point};

impl Board {
    pub fn try_move(&self, game_move: Move) -> Result<GameState, String> {
        let Board {
            wind_spirit,
            blue_king,
            blue_pawns,
            blue_hand,
            red_king,
            red_pawns,
            red_hand,
            spare_card,
            turn,
        } = self;

        let (player_king, opponent_king) = match turn {
            Player::Red => (red_king, blue_king),
            Player::Blue => (blue_king, red_king),
        };
        let player_pieces = self.player_pieces();
        let (card, src, dst) = match game_move {
            Move::Move { card, src, dst } => (card, src, dst),
            Move::Discard { card } => {
                if self.can_move() {
                    return Err("Valid moves exist".to_string());
                }
                let [c1, c2] = self.player_hand();
                let player_hand = [
                    if *c1 == card { *spare_card } else { *c1 },
                    if *c2 == card { *spare_card } else { *c2 },
                ];
                let (red_hand, blue_hand) = match turn {
                    Player::Red => (player_hand, *blue_hand),
                    Player::Blue => (*red_hand, player_hand),
                };
                return Ok(GameState::Playing {
                    board: Board {
                        wind_spirit: *wind_spirit,
                        blue_king: *blue_king,
                        blue_pawns: *blue_pawns,
                        blue_hand,
                        red_king: *red_king,
                        red_pawns: *red_pawns,
                        red_hand,
                        spare_card: card,
                        turn: turn.invert(),
                    },
                });
            }
        };
        if !self.player_hand().contains(&card) {
            return Err("Card not in hand".to_string());
        }
        if !player_pieces.contains(&Some(src)) {
            return Err("No piece at source".to_string());
        }

        // Determine if the moving piece is the Wind Spirit
        let move_wind_spirit = match wind_spirit {
            Some(wind_spirit_pos) => *wind_spirit_pos == src,
            None => false,
        };

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

        if player_pieces.contains(&Some(dst)) && !(move_wind_spirit && self.player_pawns().contains(&Some(dst))) {
            return Err("Destination occupied by your piece".to_string());
        }

        let goal_square = match turn {
            Player::Red => Point { x: 2, y: 0 },
            Player::Blue => Point { x: 2, y: 4 },
        };
        let moving_king = *player_king == src;

        // Update player's pawns
        let mut player_pawns = self.player_pawns();
        for pawn in player_pawns.iter_mut() {
            match pawn {
                None => {}
                Some(pawn_pos) if *pawn_pos == src => {
                    *pawn_pos = dst;
                }
                // If Wind Spirit moves to a player's pawn, swap positions
                Some(pawn_pos) if *pawn_pos == dst && move_wind_spirit => {
                    *pawn_pos = src;
                }
                _ => {}
            }
        }

        // Update opponent's pawns
        let mut opponent_pawns = self.opponent_pawns();
        for pawn in opponent_pawns.iter_mut() {
            match pawn {
                None => {}
                Some(pawn_pos) if *pawn_pos == dst => {
                    if move_wind_spirit {
                        *pawn_pos = src;
                    } else {
                        *pawn = None;
                    }
                }
                _ => {}
            }
        }

        let [c1, c2] = self.player_hand();
        let player_hand = [
            if *c1 == card { *spare_card } else { *c1 },
            if *c2 == card { *spare_card } else { *c2 },
        ];
        let player_king = if moving_king {
            dst
        } else {
            *player_king
        };

        // Prevent Wind Spirit from moving onto a Master
        if move_wind_spirit && (dst == *red_king || dst == *blue_king) {
            return Err("Wind Spirit cannot move onto a Master!".to_string());
        }

        // Update Wind Spirit position if it moved
        let wind_spirit = if move_wind_spirit {
            Some(dst)
        } else {
            *wind_spirit
        };

        let board = match self.turn {
            Player::Red => Board {
                wind_spirit,
                blue_king: *blue_king,
                blue_pawns: opponent_pawns,
                blue_hand: *blue_hand,
                red_king: player_king,
                red_pawns: player_pawns,
                red_hand: player_hand,
                spare_card: card,
                turn: Player::Blue,
            },
            Player::Blue => Board {
                wind_spirit,
                blue_king: player_king,
                blue_pawns: player_pawns,
                blue_hand: player_hand,
                red_king: *red_king,
                red_pawns: opponent_pawns,
                red_hand: *red_hand,
                spare_card: card,
                turn: Player::Red,
            },
        };
        if dst == *opponent_king {
            return Ok(GameState::Finished {
                winner: *turn,
                board,
            });
        }
        if moving_king && dst == goal_square {
            return Ok(GameState::Finished {
                winner: *turn,
                board,
            });
        }
        Ok(GameState::Playing { board })
    }

    fn new_from_cards_and_wind_spirit(cards: Vec<Card>, include_wind_spirit: bool) -> Board {
        let mut cards = cards.into_iter();
        let pawn_xs: [i8; 4] = [0, 1, 3, 4];
        Board {
            wind_spirit: if include_wind_spirit {
                Some(Point { x: 2, y: 2 })
            } else {
                None
            },
            blue_king: Point { x: 2, y: 0 },
            blue_pawns: pawn_xs.map(|x| Some(Point { x, y: 0 })),
            blue_hand: [cards.next().unwrap(), cards.next().unwrap()],
            red_king: Point { x: 2, y: 4 },
            red_pawns: pawn_xs.map(|x| Some(Point { x, y: 4 })),
            red_hand: [cards.next().unwrap(), cards.next().unwrap()],
            spare_card: cards.next().unwrap(),
            turn: Player::Red,
        }
    }

    pub fn new() -> Board {
        let mut rng = thread_rng();
        let mut cards: Vec<Card> = Card::into_enum_iter().collect();
        cards.shuffle(&mut rng);

        // Start a new game without the Wind Spirit
        Board::new_from_cards_and_wind_spirit(cards, false)
    }

    pub fn new_from_card_sets(card_sets: &Vec<CardSet>) -> Board {
        let mut rng = thread_rng();

        let include_wind_spirit = if card_sets.contains(&CardSet::WayOfTheWind) {
//            if force_wind_spirit {
//                true
//            } else {
                rng.gen_bool(0.25) // Include Wind Spirit 25%
//            }
        } else {
            false
        };
        let mut cards = Vec::new();
        for set in card_sets {
            // Only add "Way of the Wind" cards if Wind Spirit is included
            if *set == CardSet::WayOfTheWind && !include_wind_spirit {
                continue; // Skip cards from "Way of the Wind" set if Wind Spirit is not included
            }
            cards.extend(set.cards());
        }
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Board::new_from_cards_and_wind_spirit(cards, include_wind_spirit)
    }

    pub fn to_grid(&self) -> [[GameSquare; 5]; 5] {
        let mut grid = [[GameSquare::Empty; 5]; 5];
        for Point { x, y } in self.blue_pawns.iter().filter_map(|p| *p) {
            grid[y as usize][x as usize] = GameSquare::BluePawn;
        }
        for Point { x, y } in self.red_pawns.iter().filter_map(|p| *p) {
            grid[y as usize][x as usize] = GameSquare::RedPawn;
        }
        let Point { x, y } = self.red_king;
        grid[y as usize][x as usize] = GameSquare::RedKing;
        let Point { x, y } = self.blue_king;
        grid[y as usize][x as usize] = GameSquare::BlueKing;
        if let Some(Point { x, y }) = self.wind_spirit {
            grid[y as usize][x as usize] = GameSquare::WindSpirit;
        }
        grid
    }

    pub fn can_move(&self) -> bool {
        let player_pieces = self.player_pieces();
        for src in player_pieces.iter().filter_map(|&src| src) {
            for &card in self.player_hand() {
                for &raw_delta in card.moves().iter() {
                    let delta = match self.turn {
                        Player::Red => raw_delta,
                        Player::Blue => -raw_delta,
                    };
                    let dst = src + delta;
                    if dst.in_bounds() && !player_pieces.contains(&Some(dst)) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState::Playing {
            board: Board::new(),
        }
    }
    pub fn new_from_card_sets(card_sets: &Vec<CardSet>) -> GameState {
        GameState::Playing {
            board: Board::new_from_card_sets(card_sets),
        }
    }
}

impl Board {
    pub fn player_hand(&self) -> &[Card; 2] {
        match self.turn {
            Player::Red => &self.red_hand,
            Player::Blue => &self.blue_hand,
        }
    }
    pub fn opponent_hand(&self) -> &[Card; 2] {
        match self.turn.invert() {
            Player::Red => &self.red_hand,
            Player::Blue => &self.blue_hand,
        }
    }
}

impl Board {
    pub fn player_pawns(&self) -> [Option<Point>; 4] {
        match self.turn {
            Player::Red => self.red_pawns,
            Player::Blue => self.blue_pawns,
        }
    }
    pub fn opponent_pawns(&self) -> [Option<Point>; 4] {
        match self.turn.invert() {
            Player::Red => self.red_pawns,
            Player::Blue => self.blue_pawns,
        }
    }
}

impl Board {
    pub fn wind_spirit(&self) -> Option<Point> {
        self.wind_spirit
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
    pub fn player_pieces(&self) -> Vec<Option<Point>> {
        let mut pieces = vec![Some(*self.player_king())];
        pieces.extend(self.player_pawns().iter().copied());
        // Include Wind Spirit if it's present
        if let Some(wind_spirit_pos) = self.wind_spirit {
            pieces.push(Some(wind_spirit_pos));
        }
        pieces
    }
    pub fn opponent_pieces(&self) -> Vec<Option<Point>> {
        let mut pieces = vec![Some(*self.opponent_king())];
        pieces.extend(self.opponent_pawns().iter().copied());
        pieces
    }
}

impl GameState {
    pub fn finished(&self) -> bool {
        matches!(self, GameState::Finished { .. })
    }
    pub fn try_move(&self, game_move: Move) -> Result<GameState, String> {
        match self {
            GameState::Playing { board } => board.try_move(game_move),
            GameState::Finished { .. } => Err("Game already finished".to_string()),
        }
    }
}

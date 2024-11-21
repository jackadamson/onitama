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

        let mut player_pawns = self.player_pawns();
        for pawn in player_pawns.iter_mut() {
            match pawn {
                None => {}
                Some(pawn_pos) if *pawn_pos == src => {
                    *pawn_pos = dst;
                }
                Some(pawn_pos) if *pawn_pos == dst && move_wind_spirit => {
                    *pawn_pos = src;
                }
                _ => {}
            }
        }

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
        let player_king = if moving_king { dst } else { *player_king };

        if move_wind_spirit && (dst == *red_king || dst == *blue_king) {
            return Err("Wind Spirit cannot move onto a Master!".to_string());
        }

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
        if dst == *opponent_king || (moving_king && dst == goal_square) {
            return Ok(GameState::Finished {
                winner: *turn,
                board,
            });
        }
        Ok(GameState::Playing { board })
    }

    pub fn new_from_card_sets(card_sets: &Vec<CardSet>) -> Board {
        let mut rng = thread_rng();

        // Determine if the Wind Spirit should be included
        let include_wind_spirit = card_sets.contains(&CardSet::WayOfTheWind) && rng.gen_bool(0.25);

        let mut way_of_the_wind_cards = Vec::new();
        let mut other_cards = Vec::new();

        // Separate "Way of the Wind" cards from other cards
        for set in card_sets {
            if *set == CardSet::WayOfTheWind {
                way_of_the_wind_cards.extend(set.cards());
            } else {
                other_cards.extend(set.cards());
            }
        }

        // If Wind Spirit is not included, use only other cards
        if !include_wind_spirit {
            other_cards.shuffle(&mut rng);
            return Board::new_from_cards_and_wind_spirit(other_cards, include_wind_spirit);
        }

        // Determine the number of "Way of the Wind" cards to use
        let num_wind_cards = {
            let chance = rng.gen_range(0.0..1.0);
            if chance < 0.10 {
                0
            } else if chance < 0.25 {
                1
            } else if chance < 0.60 {
                2
            } else if chance < 0.75 {
                3
            } else if chance < 0.90 {
                4
            } else {
                5
            }
        };

        way_of_the_wind_cards.shuffle(&mut rng);
        other_cards.shuffle(&mut rng);

        // Declare the player hands and spare card without initializing them
        let player_hand_red: [Card; 2];
        let player_hand_blue: [Card; 2];
        let spare_card: Card;

        // Assign cards based on the determined number of "Way of the Wind" cards
        match num_wind_cards {
            0 => {
                player_hand_red = [other_cards.pop().unwrap(), other_cards.pop().unwrap()];
                player_hand_blue = [other_cards.pop().unwrap(), other_cards.pop().unwrap()];
                spare_card = other_cards.pop().unwrap();
            }
            1 => {
                player_hand_red = [other_cards.pop().unwrap(), other_cards.pop().unwrap()];
                player_hand_blue = [other_cards.pop().unwrap(), other_cards.pop().unwrap()];
                spare_card = way_of_the_wind_cards.pop().unwrap();
            }
            2 => {
                player_hand_red = [way_of_the_wind_cards.pop().unwrap(), other_cards.pop().unwrap()];
                player_hand_blue = [way_of_the_wind_cards.pop().unwrap(), other_cards.pop().unwrap()];
                spare_card = other_cards.pop().unwrap();
            }
            3 => {
                player_hand_red = [way_of_the_wind_cards.pop().unwrap(), other_cards.pop().unwrap()];
                player_hand_blue = [way_of_the_wind_cards.pop().unwrap(), other_cards.pop().unwrap()];
                spare_card = way_of_the_wind_cards.pop().unwrap();
            }
            4 => {
                player_hand_red = [way_of_the_wind_cards.pop().unwrap(), way_of_the_wind_cards.pop().unwrap()];
                player_hand_blue = [way_of_the_wind_cards.pop().unwrap(), way_of_the_wind_cards.pop().unwrap()];
                spare_card = other_cards.pop().unwrap();
            }
            5 => {
                player_hand_red = [way_of_the_wind_cards.pop().unwrap(), way_of_the_wind_cards.pop().unwrap()];
                player_hand_blue = [way_of_the_wind_cards.pop().unwrap(), way_of_the_wind_cards.pop().unwrap()];
                spare_card = way_of_the_wind_cards.pop().unwrap();
            }
            _ => unreachable!(),
        }

        // Create the board with the selected cards
        Board {
            wind_spirit: if include_wind_spirit {
                Some(Point { x: 2, y: 2 })
            } else {
                None
            },
            blue_king: Point { x: 2, y: 0 },
            blue_pawns: [Some(Point { x: 0, y: 0 }), Some(Point { x: 1, y: 0 }), Some(Point { x: 3, y: 0 }), Some(Point { x: 4, y: 0 })],
            blue_hand: player_hand_blue,
            red_king: Point { x: 2, y: 4 },
            red_pawns: [Some(Point { x: 0, y: 4 }), Some(Point { x: 1, y: 4 }), Some(Point { x: 3, y: 4 }), Some(Point { x: 4, y: 4 })],
            red_hand: player_hand_red,
            spare_card,
            turn: Player::Red,
        }
    }

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

    pub fn player_pieces(&self) -> Vec<Option<Point>> {
        let mut pieces = vec![Some(*self.player_king())];
        pieces.extend(self.player_pawns().iter().copied());
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

    pub fn wind_spirit(&self) -> Option<Point> {
        self.wind_spirit
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

        Board::new_from_cards_and_wind_spirit(cards, false)
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

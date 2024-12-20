use crate::CardSet;
use enum_iterator::IntoEnumIterator;
use rand::prelude::*;
use std::collections::HashSet;

use crate::models::{Board, Card, GameSquare, GameSettings, GameState, Move, Player, Point};

impl Board {
    pub fn try_move(&self, game_move: Move) -> Result<GameState, String> {
        // Destructure only what we actually need
        let Board {
            wind_spirit,
            blue_king,
            blue_hand,
            red_king,
            red_hand,
            spare_card,
            wind_move_pending,
            shadow_mode,
            turn,
            ..
        } = self;

        // If there's an wind move pending, route to try_wind_move
        if *wind_move_pending && !self.ninja_move_pending {
            return self.try_wind_move(game_move);
        }

        // Identify current and opponent kings
        let (player_king, opponent_king) = match turn {
            Player::Red => (red_king, blue_king),
            Player::Blue => (blue_king, red_king),
        };

        // Gather player's pieces (king + pawns + optional wind spirit)
        let player_pieces = self.player_pieces();

        // Parse the move
        let (card, src, dst, reveal_ninja) = match game_move {
            Move::Move { card, src, dst, reveal_ninja } => (card, src, dst, reveal_ninja),
            Move::Discard { card } => {
                if self.ninja_move_pending && self.can_move() {
                    if Some(card) != self.ninja_move_card {
                        return Err("Invalid card to discard during Ninja move phase.".to_string());
                    }

                    // Create a new board with ninja_move_pending set to false
                    let mut updated_board = self.clone();
                    updated_board.ninja_move_pending = false;

                    // If wind_move_pending is false, discard the card
                    if !updated_board.wind_move_pending {
                        return updated_board.discard_card(card);
                    }

                    // Otherwise, return the updated board without discarding
                    updated_board.ninja_move_card = None;
                    return Ok(GameState::Playing { board: updated_board });
                }

                // If not during Ninja move phase, ensure no moves exist
                if self.can_move() {
                    return Err("Valid moves exist".to_string());
                }

                return self.discard_card(card);
            }
        };

        // Validations
        if !self.player_hand().contains(&card) {
            return Err("Card not in hand".to_string());
        }
        if !player_pieces.contains(&Some(src)) {
            return Err("No piece at source".to_string());
        }

        // Check if this is a Way of the Shadow game and if the piece is a Ninja
        if *shadow_mode && self.is_ninja(src) && self.can_move() && !self.ninja_move_pending {
            return Err("You must move another piece before moving your Ninja!".to_string());
        }

        // Check if ninja_move_pending is true and the piece is not a Ninja
        if self.ninja_move_pending {
            if !self.is_ninja(src) {
                return Err("Only the Ninja can move now.".to_string());
            }
            if let Some(ninja_card) = self.ninja_move_card {
                if Some(card) != Some(ninja_card) {
                    return Err(format!("Must use the {} card for the Ninja move.", ninja_card));
                }
            } else {
                return Err("Ninja move card is not set during ninja_move_pending.".to_string());
            }
        }

        let move_wind_spirit = wind_spirit.map_or(false, |ws| ws == src);
        if move_wind_spirit && CardSet::WayOfTheWind.cards().contains(&card) {
            return Err("Wind Spirit cannot use a Way of the Wind card to move".to_string());
        }
        if out_of_bounds(dst) {
            return Err("Destination is out of bounds".to_string());
        }

        let raw_delta = dst - src;
        let delta = match turn {
            Player::Red => raw_delta,
            Player::Blue => -raw_delta,
        };

        let moving_king = *player_king == Some(src);
        let moves = card.moves(moving_king, false);
        if !moves.contains(&delta) {
            return Err("Move not valid for card".to_string());
        }

        if move_wind_spirit && (Some(dst) == *red_king || Some(dst) == *blue_king) {
            return Err("Wind Spirit cannot move onto a Master!".to_string());
        }

        // If a non-Wind Spirit piece tries to move onto your own piece, that's invalid
        if player_pieces.contains(&Some(dst))
            && !(move_wind_spirit
                && (self.player_pawns().contains(&Some(dst))
                    || self.player_ninjas()
                        .iter()
                        .filter_map(|ninja| ninja.map(|(point, _)| point))
                        .any(|point| point == dst)))
        {
            return Err("Destination occupied by your piece".to_string());
        }

        // Set opponent's Temple Arch to goal_square
        let goal_square = match turn {
            Player::Red => Point { x: 2, y: 0 },
            Player::Blue => Point { x: 2, y: 4 },
        };

        // Move/swap pieces
        let mut player_pawns = self.player_pawns();
        let mut opponent_pawns = self.opponent_pawns();
        let mut player_ninjas = self.player_ninjas();
        let mut opponent_ninjas = self.opponent_ninjas();
        move_or_swap_pieces(
            &mut player_pawns,
            &mut opponent_pawns,
            &mut player_ninjas,
            &mut opponent_ninjas,
            src,
            dst,
            move_wind_spirit,
            reveal_ninja,
        );

        let mut ninja_pending = self.ninja_move_pending;
        let mut ninja_card = self.ninja_move_card;

        if *shadow_mode && self.has_ninja() {
            if self.is_ninja(src) {
                // Reset ninja_move_pending and ninja_move_card if a Ninja is moved
                ninja_pending = false;
                ninja_card = None;
            } else {
                // Use enable_ninja_move to check and set pending Ninja move
                ninja_pending = self.enable_ninja_move(card, src, dst);
                ninja_card = if ninja_pending { Some(card) } else { None };
            }
        }


        // Check if we can enable wind move
        let wind_pending = self.enable_wind_move(card, src, dst);
        let wind_card = if wind_pending { Some(card) } else { None };

        // If no wind move, we replace the used card with the spare
        let player_hand = if !wind_pending && !ninja_pending {
            replace_card(self.player_hand(), card, *spare_card)
        } else {
            *self.player_hand()
        };

        // Update king position if it moved
        let new_king_pos = if moving_king { Some(dst) } else { *player_king };

        // Wind spirit might have moved
        let new_wind_spirit = if move_wind_spirit { Some(dst) } else { *wind_spirit };

        // Build the updated board
        let updated_board = match turn {
            Player::Red => Board {
                wind_spirit: new_wind_spirit,
                blue_king: *blue_king,
                blue_pawns: opponent_pawns,
                blue_ninjas: opponent_ninjas,
                blue_hand: *blue_hand,
                red_king: new_king_pos,
                red_pawns: player_pawns,
                red_ninjas: player_ninjas,
                red_hand: player_hand,
                spare_card: if wind_pending || ninja_pending { *spare_card } else { card },
                wind_move_pending: wind_pending,
                wind_move_card: wind_card,
                shadow_mode: *shadow_mode,
                ninja_move_pending: ninja_pending,
                ninja_move_card: ninja_card,
                turn: if wind_pending || ninja_pending { Player::Red } else { Player::Blue },
            },
            Player::Blue => Board {
                wind_spirit: new_wind_spirit,
                blue_king: new_king_pos,
                blue_pawns: player_pawns,
                blue_ninjas: player_ninjas,
                blue_hand: player_hand,
                red_king: *red_king,
                red_pawns: opponent_pawns,
                red_ninjas: opponent_ninjas,
                red_hand: *red_hand,
                spare_card: if wind_pending || ninja_pending { *spare_card } else { card },
                wind_move_pending: wind_pending,
                wind_move_card: wind_card,
                shadow_mode: *shadow_mode,
                ninja_move_pending: ninja_pending,
                ninja_move_card: ninja_card,
                turn: if wind_pending || ninja_pending { Player::Blue } else { Player::Red },
            },
        };

        // Check if this move finishes the game
        if Some(dst) == *opponent_king || (moving_king && dst == goal_square) || !updated_board.has_remaining_pieces(turn.invert()) {
            return Ok(GameState::Finished {
                winner: *turn,
                board: updated_board,
            });
        }

        Ok(GameState::Playing { board: updated_board })
    }

    fn try_wind_move(&self, game_move: Move) -> Result<GameState, String> {
         let Board {
            wind_spirit,
            blue_king,
            blue_hand,
            red_king,
            red_hand,
            spare_card,
            wind_move_card,
            turn,
            ..
        } = self;

        // If no valid moves exist for wind move, auto-discard
        if !self.can_move() {
            let card = wind_move_card.unwrap();
            let player_hand = replace_card(self.player_hand(), card, *spare_card);
            let (red_hand, blue_hand) = match turn {
                Player::Red => (player_hand, *blue_hand),
                Player::Blue => (*red_hand, player_hand),
            };
    
            return Ok(GameState::Playing {
                board: Board {
                    wind_spirit: *wind_spirit,
                    blue_king: *blue_king,
                    blue_pawns: self.blue_pawns,
                    blue_ninjas: self.blue_ninjas,
                    blue_hand,
                    red_king: *red_king,
                    red_pawns: self.red_pawns,
                    red_ninjas: self.red_ninjas,
                    red_hand,
                    spare_card: card,
                    wind_move_pending: false,
                    wind_move_card: None,
                    shadow_mode: self.shadow_mode,
                    ninja_move_pending: false,
                    ninja_move_card: None,
                    turn: turn.invert(),
                },
            });
        }
    
        let (card, src, dst, ..) = match game_move {
            Move::Move { card, src, dst, .. } => (card, src, dst),
            Move::Discard { card } => {
                let mut updated_board = self.clone();
                updated_board.wind_move_pending = false;
                return updated_board.try_move(Move::Discard { card });
            }
        };

        let wind_spirit_pos = match wind_spirit {
            Some(pos) => pos,
            None => return Err("Wind Spirit is missing!".to_string()),
        };
        if src != *wind_spirit_pos {
            return Err("You must move the Wind Spirit".to_string());
        }
        if card != wind_move_card.unwrap() {
            return Err(format!("Must use {} to move", wind_move_card.unwrap()));
        }
    
        let (player_king, _opponent_king) = match turn {
            Player::Red => (red_king, blue_king),
            Player::Blue => (blue_king, red_king),
        };
    
        if out_of_bounds(dst) {
            return Err("Destination is out of bounds".to_string());
        }

        let raw_delta = dst - src;
        let delta = match turn {
            Player::Red => raw_delta,
            Player::Blue => -raw_delta,
        };
    
        let moves = card.moves(false, true);
        if !moves.contains(&delta) {
            return Err("Move not valid for card".to_string());
        }
        if Some(dst) == *red_king || Some(dst) == *blue_king {
            return Err("Wind Spirit cannot move onto a Master!".to_string());
        }
    
        let player_hand = replace_card(self.player_hand(), card, *spare_card);

        let mut player_pawns = self.player_pawns();
        let mut opponent_pawns = self.opponent_pawns();
        let mut player_ninjas = self.player_ninjas();
        let mut opponent_ninjas = self.opponent_ninjas();
        move_or_swap_pieces(
            &mut player_pawns,
            &mut opponent_pawns,
            &mut player_ninjas,
            &mut opponent_ninjas,
            src,
            dst,
            true,
            false,
        );
    
        let wind_spirit = Some(dst);
        let player_king = *player_king;
  
        // Construct the updated board after the wind move
        let updated_board = match turn {
            Player::Red => Board {
                wind_spirit,
                blue_king: *blue_king,
                blue_pawns: opponent_pawns,
                blue_ninjas: opponent_ninjas,
                blue_hand: *blue_hand,
                red_king: player_king,
                red_pawns: player_pawns,
                red_ninjas: player_ninjas,
                red_hand: player_hand,
                spare_card: card,
                wind_move_pending: false,
                wind_move_card: None,
                shadow_mode: self.shadow_mode,
                ninja_move_pending: false,
                ninja_move_card: None,
                turn: Player::Blue,
            },
            Player::Blue => Board {
                wind_spirit,
                blue_king: player_king,
                blue_pawns: player_pawns,
                blue_ninjas: player_ninjas,
                blue_hand: player_hand,
                red_king: *red_king,
                red_pawns: opponent_pawns,
                red_ninjas: opponent_ninjas,
                red_hand: *red_hand,
                spare_card: card,
                wind_move_pending: false,
                wind_move_card: None,
                shadow_mode: self.shadow_mode,
                ninja_move_pending: false,
                ninja_move_card: None,
                turn: Player::Red,
            },
        };
    
        Ok(GameState::Playing { board: updated_board })
    }

    pub fn new_with_settings(settings: GameSettings) -> Board { 
        let mut rng = thread_rng();
        
        // Determine if the Light and Shadow expansion should be used
        let include_light_and_shadow = settings.force_light_and_shadow
            || (settings.enable_light_and_shadow && rng.gen_bool(0.05));

        let selected_mode = if include_light_and_shadow {
            settings.light_and_shadow_mode.clone().unwrap_or_else(|| {
                if rng.gen_bool(0.7) {
                    "Shadow".to_string()
                } else {
                    "Light".to_string()
                }
            })
        } else {
            "None".to_string() // Or handle the case where Light and Shadow is not included
        };

        // Decide if Wind Spirit is included
        let include_wind_spirit = !settings.disabled_card_sets.contains(&"WayOfTheWind".to_string())
            && (settings.force_wind_spirit_inclusion || rng.gen_bool(0.25));
  
        // Separate "Way of the Wind" cards
        let mut way_of_the_wind_cards = Vec::new();
        let mut other_cards = Vec::new();

        let disabled_card_sets: HashSet<CardSet> = settings
            .disabled_card_sets
            .iter()
            .filter_map(|s| std::str::FromStr::from_str(s).ok())
            .collect();

        for card_set in CardSet::into_enum_iter() {
            if !disabled_card_sets.contains(&card_set) {
                if card_set == CardSet::WayOfTheWind {
                    way_of_the_wind_cards.extend(card_set.cards());
                } else {
                    other_cards.extend(card_set.cards());
                }
            }
        }
    
        let num_wind_cards = if include_wind_spirit {
            settings.number_of_wind_cards.unwrap_or_else(|| {
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
            })
        } else {
            0
        };

        way_of_the_wind_cards.shuffle(&mut rng);
        other_cards.shuffle(&mut rng);

        // Distribute cards
        let (player_hand_red, player_hand_blue, spare_card) = match num_wind_cards {
            0 => {
                let red = [other_cards.pop().unwrap(), other_cards.pop().unwrap()];
                let blue = [other_cards.pop().unwrap(), other_cards.pop().unwrap()];
                let spare = other_cards.pop().unwrap();
                (red, blue, spare)
            }
            1 => {
                let red = [other_cards.pop().unwrap(), other_cards.pop().unwrap()];
                let blue = [other_cards.pop().unwrap(), other_cards.pop().unwrap()];
                let spare = way_of_the_wind_cards.pop().unwrap();
                (red, blue, spare)
            }
            2 => {
                let red = [way_of_the_wind_cards.pop().unwrap(), other_cards.pop().unwrap()];
                let blue = [way_of_the_wind_cards.pop().unwrap(), other_cards.pop().unwrap()];
                let spare = other_cards.pop().unwrap();
                (red, blue, spare)
            }
            3 => {
                let red = [way_of_the_wind_cards.pop().unwrap(), other_cards.pop().unwrap()];
                let blue = [way_of_the_wind_cards.pop().unwrap(), other_cards.pop().unwrap()];
                let spare = way_of_the_wind_cards.pop().unwrap();
                (red, blue, spare)
            }
            4 => {
                let red = [way_of_the_wind_cards.pop().unwrap(), way_of_the_wind_cards.pop().unwrap()];
                let blue = [way_of_the_wind_cards.pop().unwrap(), way_of_the_wind_cards.pop().unwrap()];
                let spare = other_cards.pop().unwrap();
                (red, blue, spare)
            }
            5 => {
                let red = [way_of_the_wind_cards.pop().unwrap(), way_of_the_wind_cards.pop().unwrap()];
                let blue = [way_of_the_wind_cards.pop().unwrap(), way_of_the_wind_cards.pop().unwrap()];
                let spare = way_of_the_wind_cards.pop().unwrap();
                (red, blue, spare)
            }
            _ => unreachable!(),
        };

        let mut board = Board {
            wind_spirit: if include_wind_spirit {
                Some(Point { x: 2, y: 2 })
            } else {
                None
            },
            blue_king: Some(Point { x: 2, y: 0 }),
            blue_pawns: [
                Some(Point { x: 0, y: 0 }),
                Some(Point { x: 1, y: 0 }),
                Some(Point { x: 3, y: 0 }),
                Some(Point { x: 4, y: 0 }),
            ],
            blue_ninjas: [None, None],
            blue_hand: player_hand_blue,
            red_king: Some(Point { x: 2, y: 4 }),
            red_pawns: [
                Some(Point { x: 0, y: 4 }),
                Some(Point { x: 1, y: 4 }),
                Some(Point { x: 3, y: 4 }),
                Some(Point { x: 4, y: 4 }),
            ],
            red_ninjas: [None, None],
            red_hand: player_hand_red,
            spare_card,
            wind_move_pending: false,
            wind_move_card: None,
            shadow_mode: false,
            ninja_move_pending: false,
            ninja_move_card: None,
            turn: Player::Red,
        };

        // Overwrite board for Light and Shadow modes
        if selected_mode == "Light" {
            board.blue_pawns = [None, None, None, None]; // Shadow Master has no Pawns
            board.blue_king = None; // Shadow Master has no King

            // Randomize Ninja placement until Player placement is implemented
            let mut x_positions = vec![0, 1, 2, 3, 4];
            x_positions.shuffle(&mut rng);

            board.blue_ninjas = [
                Some((Point { x: x_positions[0], y: 0 }, false)),
                Some((Point { x: x_positions[1], y: 0 }, false)),
            ];
        } else if selected_mode == "Shadow" {
            board.shadow_mode = true;    

            // Randomize Ninja placement until Player placement is implemented
            let red_ninja_x = if rng.gen_bool(0.5) { 1 } else { 3 };
            let blue_ninja_x = if rng.gen_bool(0.5) { 1 } else { 3 };

            board.red_pawns = [None, None, Some(Point { x: 0, y: 4 }), Some(Point { x: 4, y: 4 })];
            board.red_ninjas = [Some((Point { x: red_ninja_x, y: 4 }, false)), None]; // One Ninja for Red
            board.blue_pawns = [None, None, Some(Point { x: 0, y: 0 }), Some(Point { x: 4, y: 0 })];
            board.blue_ninjas = [Some((Point { x: blue_ninja_x, y: 0 }, false)), None]; // One Ninja for Blue
        }

        return board;

    }

    pub fn new() -> Board {
        let settings = GameSettings::default();
        Board::new_with_settings(settings)
    }

    pub fn can_move(&self) -> bool {
        // If a Ninja move is pending, only the Ninja can move
        if self.ninja_move_pending {
            if let Some(ninja_card) = self.ninja_move_card {
                let player_ninjas = self.player_ninjas();
                for (ninja_pos, _) in player_ninjas.iter().filter_map(|&ninja| ninja) {
                    for &raw_delta in ninja_card.moves(false, false).iter() {
                        let delta = match self.turn {
                            Player::Red => raw_delta,
                            Player::Blue => -raw_delta,
                        };
                        let dst = ninja_pos + delta;

                        if dst.in_bounds()
                            && !self.player_pieces().contains(&Some(dst))
                        {
                            return true;
                        }
                    }
                }
            }
            return false; // No valid moves for Ninjas
        }

        // If a wind move is pending, only the Wind Spirit can move using the wind_move_card
        if self.wind_move_pending {
            if let Some(wind_spirit_pos) = self.wind_spirit() {
                if let Some(wind_card) = self.wind_move_card {
                    for &raw_delta in wind_card.moves(false, true).iter() {
                        let delta = match self.turn {
                            Player::Red => raw_delta,
                            Player::Blue => -raw_delta,
                        };
                        let dst = wind_spirit_pos + delta;

                        if dst.in_bounds()
                            && ![self.red_king, self.blue_king].contains(&Some(dst))
                        {
                            return true;
                        }
                    }
                }
            }
            return false; // No valid moves for Wind Spirit
        }

        // Normal move check
        let player_pieces = self.player_pieces();
        let kings = [self.red_king, self.blue_king];

        for src in player_pieces.iter().filter_map(|&p| p) {
            for &card in self.player_hand() {
                let is_king = self.player_king() == Some(src);
                let is_wind_spirit = self.wind_spirit() == Some(src);
                let is_ninja = self.is_ninja(src);

                // Skip Ninja moves during shadow_mode
                if self.shadow_mode && is_ninja {
                    continue;
                }

                // Wind Spirit cannot use WayOfTheWind card for normal moves
                if is_wind_spirit && CardSet::WayOfTheWind.cards().contains(&card) {
                    continue;
                }

                for &raw_delta in card.moves(is_king, false).iter() {
                    let delta = match self.turn {
                        Player::Red => raw_delta,
                        Player::Blue => -raw_delta,
                    };
                    let dst = src + delta;

                    if dst.in_bounds() {
                        // Can't move onto the Wind Spirit
                        if let Some(ws) = self.wind_spirit() {
                            if dst == ws {
                                continue;
                            }
                        }
                        // Can't move onto your own piece, unless Wind Spirit is swapping
                        if !player_pieces.contains(&Some(dst)) || is_wind_spirit {
                            // Wind Spirit can't move onto a Master
                            if !(is_wind_spirit && kings.contains(&Some(dst))) {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false // No valid moves found
    }

    pub fn to_grid(&self) -> [[GameSquare; 5]; 5] {
        let mut grid = [[GameSquare::Empty; 5]; 5];
        for Point { x, y } in self.blue_pawns.iter().filter_map(|p| *p) {
            grid[y as usize][x as usize] = GameSquare::BluePawn;
        }
        for Point { x, y } in self.red_pawns.iter().filter_map(|p| *p) {
            grid[y as usize][x as usize] = GameSquare::RedPawn;
        }
        for ninja in self.blue_ninjas.iter().filter_map(|n| *n) {
            let Point { x, y } = ninja.0; // Extract Point
            let revealed = ninja.1;      // Extract revealed status
            grid[y as usize][x as usize] = GameSquare::BlueNinja { revealed };
        }
        for ninja in self.red_ninjas.iter().filter_map(|n| *n) {
            let Point { x, y } = ninja.0; // Extract Point
            let revealed = ninja.1;      // Extract revealed status
            grid[y as usize][x as usize] = GameSquare::RedNinja { revealed };
        }
        if let Some(Point { x, y }) = self.red_king {
            grid[y as usize][x as usize] = GameSquare::RedKing;
        }
        if let Some(Point { x, y }) = self.blue_king {
            grid[y as usize][x as usize] = GameSquare::BlueKing;
        }
        if let Some(Point { x, y }) = self.wind_spirit {
            grid[y as usize][x as usize] = GameSquare::WindSpirit;
        }
        grid
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

    pub fn player_ninjas(&self) -> [Option<(Point, bool)>; 2] {
        match self.turn {
            Player::Red => self.red_ninjas,
            Player::Blue => self.blue_ninjas,
        }
    }

    pub fn opponent_ninjas(&self) -> [Option<(Point, bool)>; 2] {
        match self.turn.invert() {
            Player::Red => self.red_ninjas,
            Player::Blue => self.blue_ninjas,
        }
    }

    pub fn is_ninja(&self, src: Point) -> bool {
        self.player_ninjas().iter().any(|ninja| ninja.map(|(point, _)| point) == Some(src))
    }

    pub fn has_ninja(&self) -> bool {
        self.player_ninjas().iter().any(|ninja| ninja.is_some())
    }

    pub fn player_king(&self) -> Option<Point> {
        match self.turn {
            Player::Red => self.red_king,
            Player::Blue => self.blue_king,
        }
    }

    pub fn opponent_king(&self) -> Option<Point> {
        match self.turn.invert() {
            Player::Red => self.red_king,
            Player::Blue => self.blue_king,
        }
    }

    pub fn player_pieces(&self) -> Vec<Option<Point>> {
        let mut pieces = vec![self.player_king()];
        pieces.extend(self.player_pawns().iter().copied());
        pieces.extend(self.player_ninjas().iter().map(|ninja| ninja.map(|(point, _)| point)));
        if let Some(ws) = self.wind_spirit {
            pieces.push(Some(ws));
        }
        pieces
    }

    pub fn opponent_pieces(&self) -> Vec<Option<Point>> {
        let mut pieces = vec![self.opponent_king()];
        pieces.extend(self.opponent_pawns().iter().copied());
        pieces.extend(self.opponent_ninjas().iter().map(|ninja| ninja.map(|(point, _)| point)));
        pieces
    }

    pub fn wind_spirit(&self) -> Option<Point> {
        self.wind_spirit
    }

    fn has_remaining_pieces(&self, player: Player) -> bool {
        match player {
            Player::Red => {
                self.red_king.is_some()
                    || self.red_pawns.iter().any(Option::is_some)
                    || self.red_ninjas.iter().any(Option::is_some)
            }
            Player::Blue => {
                self.blue_king.is_some()
                    || self.blue_pawns.iter().any(Option::is_some)
                    || self.blue_ninjas.iter().any(Option::is_some)
            }
        }
    }

    fn enable_wind_move(&self, card: Card, moved_src: Point, moved_dst: Point) -> bool {
        if !CardSet::WayOfTheWind.cards().contains(&card) {
            return false;
        }

        let mut temp_board = self.clone();
        for pawn in temp_board.player_pawns().iter_mut().chain(temp_board.opponent_pawns().iter_mut()) {
            if let Some(pos) = pawn {
                if *pos == moved_src {
                    *pos = moved_dst;
                }
            }
        }

        // Update the King if it moved
        if temp_board.player_king() == Some(moved_src) {
            if self.turn == Player::Red {
                temp_board.red_king = Some(moved_dst);
            } else {
                temp_board.blue_king = Some(moved_dst);
            }
        }

        temp_board.wind_move_pending = true;
        temp_board.wind_move_card = Some(card);
        temp_board.can_move()
    }

    fn enable_ninja_move(&self, card: Card, moved_src: Point, moved_dst: Point) -> bool {
    if !self.shadow_mode || !self.has_ninja() {
        return false; // Only enable Ninja move in Shadow mode with Ninjas present
    }

    // Clone the board to simulate the state after the move
    let mut temp_board = self.clone();
    for pawn in temp_board.player_pawns().iter_mut().chain(temp_board.opponent_pawns().iter_mut()) {
        if let Some(pos) = pawn {
            if *pos == moved_src {
                *pos = moved_dst; // Update the pawn's position
            }
        }
    }

    // Update the King if it moved
    if temp_board.player_king() == Some(moved_src) {
        if self.turn == Player::Red {
            temp_board.red_king = Some(moved_dst);
        } else {
            temp_board.blue_king = Some(moved_dst);
        }
    }

    // Temporarily set ninja_move_pending and ninja_move_card
    temp_board.ninja_move_pending = true;
    temp_board.ninja_move_card = Some(card);

    // Check if a Ninja can move with the given card
    temp_board.can_move()
}

    // Helper that discards a card
    fn discard_card(&self, card: Card) -> Result<GameState, String> {
        let player_hand = replace_card(self.player_hand(), card, self.spare_card);
        let (red_hand, blue_hand) = match self.turn {
            Player::Red => (player_hand, self.blue_hand),
            Player::Blue => (self.red_hand, player_hand),
        };

        Ok(GameState::Playing {
            board: Board {
                wind_spirit: self.wind_spirit,
                blue_king: self.blue_king,
                blue_pawns: self.blue_pawns,
                blue_ninjas: self.blue_ninjas,
                blue_hand,
                red_king: self.red_king,
                red_pawns: self.red_pawns,
                red_ninjas: self.red_ninjas,
                red_hand,
                spare_card: card,
                wind_move_pending: false,
                wind_move_card: None,
                shadow_mode: self.shadow_mode,
                ninja_move_pending: false,
                ninja_move_card: None,
                turn: self.turn.invert(),
            },
        })
    }
}

impl GameState {
    pub fn new() -> GameState {
        log::info!("GameState::new() called");
        GameState::Playing {
            board: Board::new_with_settings(GameSettings::default()),
        }
    }

    pub fn new_with_settings(settings: GameSettings) -> GameState {
        log::info!("GameState::new_with_settings() called with settings: {:?}", settings);
        GameState::Playing {
            board: Board::new_with_settings(settings),
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

fn replace_card(hand: &[Card; 2], used: Card, spare: Card) -> [Card; 2] {
    let [c1, c2] = hand;
    [
        if *c1 == used { spare } else { *c1 },
        if *c2 == used { spare } else { *c2 },
    ]
}

fn out_of_bounds(point: Point) -> bool {
    point.x < 0 || point.x > 4 || point.y < 0 || point.y > 4
}

fn move_or_swap_pieces(
    player_pawns: &mut [Option<Point>; 4],
    opponent_pawns: &mut [Option<Point>; 4],
    player_ninjas: &mut [Option<(Point, bool)>; 2],
    opponent_ninjas: &mut [Option<(Point, bool)>; 2],
    src: Point,
    dst: Point,
    wind_spirit_moving: bool,
    reveal_ninja:bool,
) {
    struct Occupant {
        is_player_side: bool,
        is_ninja: bool,
        index: usize,
        revealed: bool,
    }

    // Identify the occupant at `dst`
    let occupant = player_pawns
        .iter()
        .enumerate()
        .find(|(_, pawn)| pawn.map_or(false, |p| p == dst))
        .map(|(i, _)| Occupant {
            is_player_side: true,
            is_ninja: false,
            index: i,
            revealed: false,
        })
        .or_else(|| {
            opponent_pawns
                .iter()
                .enumerate()
                .find(|(_, pawn)| pawn.map_or(false, |p| p == dst))
                .map(|(i, _)| Occupant {
                    is_player_side: false,
                    is_ninja: false,
                    index: i,
                    revealed: false,
                })
        })
        .or_else(|| {
            player_ninjas
                .iter()
                .enumerate()
                .find(|(_, ninja)| ninja.map_or(false, |(pos, _)| pos == dst))
                .map(|(i, ninja)| Occupant {
                    is_player_side: true,
                    is_ninja: true,
                    index: i,
                    revealed: ninja.unwrap().1,
                })
        })
        .or_else(|| {
            opponent_ninjas
                .iter()
                .enumerate()
                .find(|(_, ninja)| ninja.map_or(false, |(pos, _)| pos == dst))
                .map(|(i, ninja)| Occupant {
                    is_player_side: false,
                    is_ninja: true,
                    index: i,
                    revealed: ninja.unwrap().1,
                })
        });

    // Extract mover information
    let mover_is_pawn = player_pawns.iter().position(|pawn| pawn.map_or(false, |p| p == src));
    let mover_is_ninja = player_ninjas
        .iter()
        .enumerate()
        .find(|(_, ninja)| ninja.map_or(false, |(pos, _)| pos == src))
        .map(|(i, ninja)| (i, ninja.unwrap().1 || reveal_ninja)); // Extract index and revealed state

    let mut captured_something = false;

    // Process interactions
    if let Some(occ) = occupant {
        if !occ.is_player_side {
            if wind_spirit_moving {
                if occ.is_ninja {
                    opponent_ninjas[occ.index] = Some((src, true)); // Reveal enemy ninja
                } else {
                    opponent_pawns[occ.index] = Some(src);
                }
            } else {
                let mover_is_hidden_ninja = mover_is_ninja.map_or(false, |(_, revealed)| !revealed);
                if occ.is_ninja {
                    if mover_is_hidden_ninja && !occ.revealed {
                        // Hidden ninja vs hidden ninja -> no capture or reveal
                    } else {
                        // Capture enemy ninja
                        opponent_ninjas[occ.index] = None;
                        captured_something = true;
                    }
                } else {
                    // Capture enemy pawn
                    opponent_pawns[occ.index] = None;
                    captured_something = true;
                }
            }
        } else if wind_spirit_moving {
            if occ.is_ninja {
                player_ninjas[occ.index] = Some((src, true)); // Reveal own ninja
            } else {
                player_pawns[occ.index] = Some(src);
            }
        }
    }

    // Always move the piece
    if let Some(i) = mover_is_pawn {
        player_pawns[i] = Some(dst);
    }
    if let Some((i, was_revealed)) = mover_is_ninja {
        let new_revealed_state = captured_something || wind_spirit_moving || was_revealed || reveal_ninja;
        player_ninjas[i] = Some((dst, new_revealed_state));

        // Explicitly hide the mover Ninja if it didn't capture or swap and wasn't revealed on purpose
        if !captured_something && !wind_spirit_moving && !reveal_ninja {
            player_ninjas[i] = Some((dst, false));
        }
    }

    // Hide ninjas that didnt perform a revealing action
    for ninja in player_ninjas.iter_mut() {
        if let Some((pos, rev)) = ninja {
            if *rev && *pos != dst && !captured_something && !wind_spirit_moving {
                *ninja = Some((*pos, false)); // Hide the ninja
            }
        }
    }
}

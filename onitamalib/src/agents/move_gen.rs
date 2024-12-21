use crate::models::{Board, Move, Player, Point, CardSet};
use rand::prelude::*;

impl Board {
pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let pieces = self.player_pieces();
        let wind_spirit_pos = self.wind_spirit();
        let kings: Vec<Point> = [self.red_king, self.blue_king]
            .iter()
            .filter_map(|&king| king)
            .collect();

        // Handle Ninja Move Pending
        if self.ninja_move_pending {
            if let Some(ninja_card) = self.ninja_move_card {
                for src in pieces.iter().filter_map(|p| *p) {
                    if !self.is_ninja(src) {
                        continue; // Only Ninja pieces can move
                    }
                    for offset in ninja_card.moves(false, false) {
                        let offset = match self.turn {
                            Player::Red => offset,
                            Player::Blue => -offset,
                        };
                        let dst = src + offset;

                        if dst.in_bounds() && !self.player_pieces().contains(&Some(dst)) {
                            moves.push(Move::Move {
                                card: ninja_card,
                                src,
                                dst,
                                reveal_ninja: false,
                            });
                        }
                    }
                }
            }
            return moves; // Only Ninja moves are valid during `ninja_move_pending`
        }

        // Handle Wind Spirit Moves
        if self.wind_move_pending {
            if let Some(wind_spirit_pos) = wind_spirit_pos {
                if let Some(wind_card) = self.wind_move_card {
                    for offset in wind_card.moves(false, true) {
                        let offset = match self.turn {
                            Player::Red => offset,
                            Player::Blue => -offset,
                        };
                        let dst = wind_spirit_pos + offset;

                        if dst.in_bounds()
                            && (!self.player_pieces().contains(&Some(dst)) || kings.contains(&dst))
                        {
                            // Prevent Wind Spirit from moving onto a King
                            if kings.contains(&dst) {
                                continue;
                            }

                            moves.push(Move::Move {
                                card: wind_card,
                                src: wind_spirit_pos,
                                dst,
                                reveal_ninja: false,
                            });
                        }
                    }
                }
            }
            return moves; // Only Wind Spirit moves are valid during `wind_move_pending`
        }

        // Generate Normal Moves
        for card in self.player_hand() {
            for src in pieces.iter().filter_map(|p| *p) {
                let is_wind_spirit = Some(src) == wind_spirit_pos;
                let is_king = self.player_king() == Some(src);
                let is_ninja = self.is_ninja(src);

                // Skip Ninja moves in shadow mode
                if self.shadow_mode && is_ninja {
                    continue; // Shadow mode: disallow Ninja moves
                }

                let cached_moves: Vec<_> = card.moves(is_king, false);
                for offset in cached_moves {
                    if is_wind_spirit && CardSet::WayOfTheWind.cards().contains(&card) {
                        continue; // Skip illegal moves for the Wind Spirit
                    }

                    let offset = match self.turn {
                        Player::Red => offset,
                        Player::Blue => -offset,
                    };
                    let dst = src + offset;

                    if dst.in_bounds() && (!pieces.contains(&Some(dst)) || is_wind_spirit) {
                        
                        if is_wind_spirit && kings.contains(&dst) {
                            continue;
                        }

                        // Prevent pieces from moving onto Wind Spirit
                        if let Some(wind_spirit_pos) = wind_spirit_pos {
                            if dst == wind_spirit_pos {
                                continue;
                            }
                        }

                        moves.push(Move::Move {
                            card: *card,
                            src,
                            dst,
                            reveal_ninja: false,
                        });
                    }
                }
            }
        }

        if !moves.is_empty() {
            let opponent_pieces = self.opponent_pieces();
            let key = |game_move: &Move| match game_move {
                Move::Move { dst, .. } => match opponent_pieces.contains(&Some(*dst)) {
                    true => 0, // Prioritize capturing moves
                    false => 1,
                },
                Move::Discard { .. } => 0,
            };
            moves.sort_by_cached_key(key);
            return moves;
        }

        // If no moves are possible, force a discard
        self.player_hand()
            .iter()
            .map(|&card| Move::Discard { card })
            .collect()
    }

    pub fn random_legal_move<R: Rng>(&self, rng: &mut R) -> Option<Move> {
        let mut moves = self.legal_moves();
    
        // Shuffle moves to randomize selection
        moves.shuffle(rng);
    
        // Validate each move using `try_move` before selecting
        for game_move in moves {
            if self.try_move(game_move).is_ok() {
                return Some(game_move); // Return the first valid move
            }
        }
    
        // Return None if no valid moves remain
        None
    }
    
}

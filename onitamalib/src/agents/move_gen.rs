use crate::models::{Board, Move, Player, CardSet};
use rand::prelude::*;

impl Board {
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let pieces = self.player_pieces();
        let wind_spirit_pos = self.wind_spirit();
        let red_king_pos = self.red_king;
        let blue_king_pos = self.blue_king;
        let opponent_kings = [red_king_pos, blue_king_pos];

        for src in pieces.iter().filter_map(|p| *p) {
            let is_wind_spirit = Some(src) == wind_spirit_pos;
            for &card in self.player_hand() {
                let is_king = src == *self.player_king();
                for &raw_delta in card.moves(is_king).iter() {
                    let offset = match self.turn {
                        Player::Red => raw_delta,
                        Player::Blue => -raw_delta,
                    };
                    let dst = src + offset;
                    if dst.in_bounds() && !pieces.contains(&Some(dst)) {
                        // Filter out moves where Wind Spirit uses a Way of the Wind card
                        if is_wind_spirit && CardSet::WayOfTheWind.cards().contains(&card) {
                            continue; // Skip this illegal move
                        }
                        // Prevent Wind Spirit from moving onto a King
                        if is_wind_spirit && opponent_kings.contains(&dst) {
                            continue; // Skip this move
                        }
                        moves.push(Move::Move {
                            card,
                            src,
                            dst,
                        });
                    }
                }
            }
        }
        if !moves.is_empty() {
            let opponent_pieces = self.opponent_pieces();
            let key = |game_move: &Move| match game_move {
                Move::Move { dst, .. } => match opponent_pieces.contains(&Some(*dst)) {
                    true => 0,
                    false => 1,
                },
                Move::Discard { .. } => 0,
            };
            moves.sort_by_cached_key(key);
            return moves;
        }
        // No moves, have to discard
        self
            .player_hand()
            .iter()
            .map(|&card| Move::Discard { card })
            .collect()
    }

    pub fn random_legal_move<R: Rng>(&self, rng: &mut R) -> Move {
        let mut moves = self.legal_moves();
        if !moves.is_empty() {
            moves.shuffle(rng);
            return moves[0];
        }
        // No moves, have to discard
        let cards = self.player_hand();
        Move::Discard { card: cards[0] }
    }
}

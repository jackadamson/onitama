use crate::models::{Board, Move, Player};
use rand::prelude::*;

impl Board {
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let pieces = self.player_pieces();
        for card in self.player_hand() {
            for src in self.player_pieces().iter().filter_map(|p| *p) {
                for offset in card.moves() {
                    let offset = match self.turn {
                        Player::Red => offset,
                        Player::Blue => -offset,
                    };
                    let dst = src + offset;
                    if dst.in_bounds() && !pieces.contains(&Some(dst)) {
                        moves.push(Move::Move {
                            card: *card,
                            src,
                            dst,
                        });
                    }
                }
            }
        }
        if moves.len() > 0 {
            return moves;
        }
        // No moves, have to discard
        return self.player_hand()
            .iter()
            .map(|card| Move::Discard { card: *card })
            .collect()
    }
    pub fn random_legal_move<R: Rng>(&self, rng: &mut R) -> Move {
        let mut cards = *self.player_hand();
        cards.shuffle(rng);
        let mut player_pieces = self.player_pieces();
        player_pieces.shuffle(rng);
        for card in cards {
            let mut moves = card.moves();
            moves.shuffle(rng);
            for src in player_pieces.iter().filter_map(|p| *p) {
                for offset in moves.iter() {
                    let offset = match self.turn {
                        Player::Red => *offset,
                        Player::Blue => -*offset,
                    };
                    let dst = src + offset;
                    if dst.in_bounds() && !player_pieces.contains(&Some(dst)) {
                        return Move::Move {
                            card,
                            src,
                            dst,
                        };
                    }
                }
            }
        }
        return Move::Discard {card: cards[0] }
    }
}

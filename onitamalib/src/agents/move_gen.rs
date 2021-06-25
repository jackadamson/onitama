use crate::models::{Board, Move, Player};

impl Board {
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let pieces = self.player_pieces();
        for card in self.player_hand() {
            for src in self.player_pieces() {
                for offset in card.moves() {
                    let offset = match self.turn {
                        Player::Red => offset,
                        Player::Blue => -offset,
                    };
                    let dst = src + offset;
                    if dst.in_bounds() && !pieces.contains(&dst) {
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
}

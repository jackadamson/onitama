use rand::prelude::*;
use crate::models::{Board, GameState, Point};

/// Checks if a given position is valid for a hidden Ninja
/// A position is valid if it's within bounds and not occupied by any piece.
pub fn is_valid_ninja_position(board: &Board, pos: Point) -> bool {
    pos.in_bounds()
        && !board.player_pieces().contains(&Some(pos))
        && !board.opponent_pieces().contains(&Some(pos))
}

/// Randomizes the positions of all hidden Ninjas during simulations.
/// If a Ninja is not revealed, it will be placed at a random valid position.
pub fn randomize_hidden_ninjas(state: GameState) -> GameState {
    let mut rng = thread_rng();
    if let GameState::Playing { mut board } = state {
        let current_board = board.clone();
        for ninja in board.blue_ninjas.iter_mut().chain(board.red_ninjas.iter_mut()) {
            if let Some((pos, revealed)) = ninja {
                if !*revealed {
                    *pos = generate_random_valid_position(&current_board, &mut rng);
                }
            }
        }
        GameState::Playing { board }
    } else {
        state
    }
}

/// Generates a random valid position for a hidden Ninja.
pub fn generate_random_valid_position<R: Rng>(board: &Board, rng: &mut R) -> Point {
    let mut candidates: Vec<Point> = vec![];
    for x in 0..5 {
        for y in 0..5 {
            let pos = Point { x, y };
            if is_valid_ninja_position(board, pos) {
                candidates.push(pos);
            }
        }
    }
    *candidates.choose(rng).unwrap()
}

/// Calculates all possible positions that a hidden Ninja could have moved to,
/// based on the opponent's cards and current Ninja positions.
pub fn possible_ninja_positions(board: &Board, current_positions: Vec<Point>) -> Vec<Point> {
    let mut possible_positions = vec![];

    for card in board.opponent_hand() {
        for src in current_positions.iter() {
            let moves = card.moves(false, false);

            for offset in moves {
                let dst = *src + offset;
                if dst.in_bounds() && is_valid_ninja_position(board, dst) {
                    possible_positions.push(dst);
                }
            }
        }
    }
    possible_positions
}

/// Replaces the positions of hidden ninjas with the provided potential positions.
pub fn randomize_ninjas_with_positions(state: &mut GameState, new_positions: &[Point]) {
    if let GameState::Playing { ref mut board } = state {
        let mut pos_iter = new_positions.iter();

        for ninja in board.blue_ninjas.iter_mut() {
            if let Some((ref mut pos, revealed)) = ninja {
                if !*revealed {
                    if let Some(&new_pos) = pos_iter.next() {
                        *pos = new_pos;
                    }
                }
            }
        }

        for ninja in board.red_ninjas.iter_mut() {
            if let Some((ref mut pos, revealed)) = ninja {
                if !*revealed {
                    if let Some(&new_pos) = pos_iter.next() {
                        *pos = new_pos;
                    }
                }
            }
        }
    }
}

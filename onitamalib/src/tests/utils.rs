use rand::prelude::*;
use enum_iterator::IntoEnumIterator;


use crate::{Board, Card, GameState, Player, Point};

const SAMPLES: usize = 100;
pub fn generate_test_states() -> Vec<GameState> {
    let mut states:  Vec<GameState> = vec![];
    let mut rng = SmallRng::seed_from_u64(0);
    while states.len() < SAMPLES {
        let mut state = GameState::new_rng(&mut rng);
        while let GameState::Playing { board } = state {
            states.push(state);
            let game_move = board.random_legal_move(&mut rng);
            state = state.try_move(game_move).expect("Generated illegal move");
        }
    }
    states.truncate(SAMPLES);
    return states;
}

impl GameState {
    pub fn new_rng<R: Rng>(rng: &mut R) -> GameState {
        GameState::Playing {
            board: Board::new_rng(rng),
        }
    }
}

impl Board {
    pub fn new_rng<R: Rng>(rng: &mut R) -> Board {
        let mut cards: Vec<Card> = Card::into_enum_iter().collect();
        cards.shuffle(rng);
        let mut cards = cards.into_iter();
        let pawn_xs: [i8; 4] = [0, 1, 3, 4];
        Board {
            blue_king: Point { x: 2, y: 0 },
            blue_pawns: pawn_xs
                .map(|x| Some(Point { x, y: 0 })),

            blue_hand: [cards.next().unwrap(), cards.next().unwrap()],
            red_king: Point { x: 2, y: 4 },
            red_pawns: pawn_xs
                .map(|x| Some(Point { x, y: 4 })),
            red_hand: [cards.next().unwrap(), cards.next().unwrap()],
            spare_card: cards.next().unwrap(),
            turn: Player::Red,
        }
    }
}

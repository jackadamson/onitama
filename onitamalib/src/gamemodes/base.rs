use crate::models::{GameState, Move, Player};

#[derive(Clone)]
pub struct Game {
    state: GameState,
    last_move: Option<Move>,
}

impl Game {
    pub fn new() -> Game {
        let game = Game {
            state: GameState::new(),
            last_move: None,
        };
        return game;
    }
}

impl Game {
    pub fn reset(&mut self) {
        self.state = GameState::new();
        self.last_move = None;
    }
}

impl Game {
    pub fn try_move(&mut self, game_move: Move) -> Result<(), String> {
        let board = match &self.state {
            GameState::Playing { board } => board,
            GameState::Finished { .. } => {
                return Err("Game Already Finished".to_string());
            },
        };
        self.state = board.try_move(game_move)?;
        self.last_move = Some(game_move);
        Ok(())
    }
    pub fn get_last_move(&self) -> Option<Move> {
        self.last_move
    }
    pub fn set_state(&mut self, state: GameState) {
        self.last_move = None;
        self.state = state;
    }
    pub fn get_state(&self) -> GameState {
        return self.state;
    }
    pub fn get_turn(&self) -> Option<Player> {
        match &self.state {
            GameState::Playing { board } => Some(board.turn),
            GameState::Finished { .. } => None
        }
    }
    pub fn is_finished(&self) -> bool {
        match self.state {
            GameState::Playing { .. } => false,
            GameState::Finished { .. } => true,
        }
    }
    pub fn get_winner(&self) -> Option<Player> {
        match self.state {
            GameState::Finished { winner, .. } => Some(winner),
            GameState::Playing { .. } => None,
        }
    }
}

use crate::models::{GameState, GameView, Move, Player};

pub struct Game {
    state: GameState,
}

impl Game {
    pub fn new() -> Game {
        let game = Game {
            state: GameState::new(),
        };
        return game;
    }
}

impl Game {
    pub fn reset(&mut self) {
        self.state = GameState::new();
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
        Ok(())
    }
    pub fn get_view(&self) -> GameView {
        GameView::from(&self.state)
    }
    pub fn set_state(&mut self, state: GameState) {
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
}

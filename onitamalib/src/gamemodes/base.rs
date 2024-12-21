use crate::models::{CardSet, GameState, Move, Player, GameSettings};
use enum_iterator::IntoEnumIterator;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Game {
    state: GameState,
    last_move: Option<Move>,
    settings: GameSettings, 
}

impl Game {
    /// Create a new game with default settings
    pub fn new() -> Game {
        let settings = GameSettings::default();
        Game::new_with_settings(settings)
    }

    /// Create a new game with custom settings
    pub fn new_with_settings(settings: GameSettings) -> Game {
        // Create a hash set for disabled card sets for easy lookup
        let disabled_card_sets_hash: HashSet<String> = settings
            .disabled_card_sets
            .iter()
            .cloned()
            .collect();
    
        // Filter card sets that are not disabled
        let card_sets: Vec<CardSet> = CardSet::into_enum_iter()
            .filter(|set| !disabled_card_sets_hash.contains(&set.to_string()))
            .collect();

        // Calculate the number of cards available in the enabled card sets
        let mut cards_enabled = 0;
        for set in &card_sets {
            cards_enabled += set.cards().len();
        }
    
        // If there are not enough cards, fallback to default game settings
        if cards_enabled < 5 {
            log::warn!("Not enough cards enabled for valid gameplay, falling back to default settings.");
            return Game::new(); // Fallback to default game
        }
    
        // Create the game state with settings
        let state = GameState::new_with_settings(settings.clone());
    
        // Create a game with the provided settings
        let game = Game {
            state,
            last_move: None,
            settings,
        };

        return game
    }
    

    /// Reset the game state to the initial configuration based on the current settings
    pub fn reset(&mut self) {
        self.state = GameState::new_with_settings(self.settings.clone());
        self.last_move = None;
    }

    pub fn try_move(&mut self, game_move: Move) -> Result<(), String> {
        let board = match &self.state {
            GameState::Playing { board } => board,
            GameState::Finished { .. } => {
                return Err("Game Already Finished".to_string());
            }
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
            GameState::Finished { .. } => None,
        }
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.state, GameState::Finished { .. })
    }

    pub fn get_winner(&self) -> Option<Player> {
        match self.state {
            GameState::Finished { winner, .. } => Some(winner),
            GameState::Playing { .. } => None,
        }
    }
}

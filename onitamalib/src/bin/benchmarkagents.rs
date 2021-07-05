use std::collections::HashMap;

use enum_iterator::IntoEnumIterator;
use indicatif::ProgressBar;
use instant::Duration;

use onitamalib::{AiAgent, GameState, Player};

const TURN_DURATION: Duration = Duration::from_millis(100);
struct Match {
    red: AiAgent,
    blue: AiAgent,
}
const MAX_TURNS: u64 = 1000;
impl Match {
    fn agent_from_player(&self, player: Player) -> AiAgent {
        match player {
            Player::Red => self.red,
            Player::Blue => self.blue,
        }
    }
    pub fn winner(&self) -> Option<AiAgent> {
        let mut state = GameState::new();
        let mut iteration = 0u64;
        loop {
            iteration += 1;
            if iteration > MAX_TURNS {
                println!("\nDeadlock between {:?} and {:?}\n\n", self.blue, self.red);
                return None;
            }
            let board = match state {
                GameState::Playing { board } => board,
                GameState::Finished { winner, .. } => {
                    return Some(self.agent_from_player(winner));
                },
            };
            let agent = self.agent_from_player(board.turn);
            let (game_move, _) = agent.play_move(&state, TURN_DURATION).unwrap();
            state = state.try_move(game_move).unwrap();
        }
    }
}
fn main() {
    let mut matches: Vec<Match> = vec![];
    for red in AiAgent::into_enum_iter() {
        for blue in AiAgent::into_enum_iter() {
            if red != blue {
                let ai_match = Match { blue, red };
                matches.push(ai_match);
            }
        }
    }
    let pb = ProgressBar::new(matches.len() as u64);

    pb.tick();
    let mut wins: HashMap<AiAgent, u64> = HashMap::new();
    for ai_match in matches.into_iter() {
        let winner = match ai_match.winner() {
            None => {
                continue;
            }
            Some(winner) => winner,
        };
        let current_wins = match wins.get(&winner) {
            None => 1,
            Some(current_wins) => *current_wins + 1,
        };
        wins.insert(winner, current_wins);
        pb.inc(1);
    }
    pb.finish();
    for (agent, won) in wins.iter() {
        println!("{:?} win {}", agent, won);
    }
}

use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use enum_iterator::IntoEnumIterator;
use indicatif::ProgressBar;
use instant::Duration;

use onitamalib::{AiAgent, GameState, Player};
use std::thread::JoinHandle;

const TURN_DURATION: Duration = Duration::from_millis(100);
const MATCH_REPEATS: u64 = 10;
struct Match {
    red: AiAgent,
    blue: AiAgent,
}
const MAX_TURNS: u64 = 250;
const PARALLELISM: u64 = 14;
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
                // println!("\nDeadlock between {:?} and {:?}\n\n", self.blue, self.red);
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
                for _ in 0..MATCH_REPEATS {
                    let ai_match = Match { blue, red };
                    matches.push(ai_match);
                }
            }
        }
    }
    let pb = ProgressBar::new(matches.len() as u64);
    let matches = Arc::new(Mutex::new(Box::new(matches)));
    let (tx,rx) = mpsc::channel();
    let handles: Vec<JoinHandle<()>> = (0..PARALLELISM).map(|idx| {
        let (matches, tx) = (Arc::clone(&matches), tx.clone());
        thread::spawn(move || {
            loop {
                // println!("{}: Getting lock", idx);
                let mut c_matches = matches.lock().unwrap();
                let ai_match = match c_matches.pop() {
                    None => { break; }
                    Some(ai_match) => ai_match,
                };
                // println!("{}: Got matches", idx);
                drop(c_matches);
                // println!("{}: Dropped", idx);
                let winner = ai_match.winner();
                // println!("{}: Determined winner", idx);
                tx.send(winner).unwrap();
                // println!("{}: Sent", idx);
            }
        })
    }).collect();
    drop(tx);
    pb.tick();
    let mut wins: HashMap<AiAgent, u64> = HashMap::new();
    for winner in rx.into_iter() {
        let winner = match winner {
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
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
    pb.finish();
    for (agent, won) in wins.iter() {
        println!("{:?} win {}", agent, won);
    }
}

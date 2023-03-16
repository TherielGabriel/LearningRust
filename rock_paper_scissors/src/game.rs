use rand::Rng;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum Move {
    Rock, Paper, Scissors, Invalid
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Rock => write!(f, "Rock"),
            Move::Paper => write!(f, "Paper"),
            Move::Scissors => write!(f, "Scissors"),
            _ => write!(f, "Invalid"),
        }
    }
}

pub struct Game {
    rounds: u8,
    move_map: HashMap<Move, Move>,
}

impl Game {
    pub fn new(rounds: u8) -> Self {
        let move_map = HashMap::from([
            (Move::Rock, Move::Scissors),
            (Move::Scissors, Move::Paper),
            (Move::Paper, Move::Rock),
        ]);
        Game{ 
            rounds,
            move_map
         }
    }

    pub fn play(&mut self, player_move: Move) {
        let game_move = self.gen_move();
        println!("Opponent played {game_move}");
        
        if self.move_map.get(&player_move) == Some(&game_move) {
            println!("Win!");
        } else if self.move_map.get(&game_move) == Some(&player_move) {
            println!("Lose!");
        } else {
            println!("Tie!");
        }
        self.rounds -= 1;
    }

    pub fn has_rounds(&self) -> bool {
        self.rounds > 0
    }

    fn gen_move(&self) -> Move {
        let num = rand::thread_rng().gen_range(1..=3);
        match num {
            1 => Move::Rock,
            2 => Move::Paper,
            3 => Move::Scissors,
            _ => Move::Paper
        }
    }
}
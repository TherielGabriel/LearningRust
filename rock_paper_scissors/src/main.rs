mod game;
use game::Game;
use std::io;

fn main() {
    let mut game = Game::new(3);
    println!("~Le Poggeurs RPS~");
    while game.has_rounds() { 
        println!("Rock[1] Paper[2] or Scissors[3]: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parses input to a u8
        let player_move = match guess.trim().parse::<u8>() {
            // If input is a u8, then it is returned as num
            Ok(num) => match num {
                1 => game::Move::Rock,
                2 => game::Move::Paper,
                3 => game::Move::Scissors,
                _ => game::Move::Invalid,
            },
            Err(_e) => game::Move::Invalid,
        };

        if player_move == game::Move::Invalid {
            println!("Invalid move.");
            continue;
        }

        game.play(player_move);
    }
}

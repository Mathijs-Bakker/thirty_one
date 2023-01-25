#![warn(clippy::all, clippy::pedantic)]
use std::io::Write;
use std::io::{stdin, stdout};

fn main() {
    let mut is_game_over = true;

    loop {
        start_new_game(&mut is_game_over);

        if is_game_over {
            println!("Game Over");
        }
    }
}

fn start_new_game(is_game_over: &mut bool) {
    if *is_game_over {
        let mut input = String::new();
        println!("Start? (y/n)");
        let _ = stdout().flush();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                if input == "y\n" {
                    *is_game_over = false;
                    println!("Start new game!");
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}

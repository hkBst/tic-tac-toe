//! Tic Tac Toe
//!
//!
//!
//!
//!
//!
//!
//!

use regex::Regex;
use std::io::Write;

use tic_tac_toe as ttt;

fn main() {
    let move_regex: regex::Regex = Regex::new(r"^\w*(\d)[ ,:;](\d)\w*\n").unwrap();
    println!("Welcome to Tic Tac Toe!");
    let mut game = ttt::TTT::new();
    loop {
        println!("{}", &game);
        match game.game_value() {
            ttt::GameValue::Unknown => {
                print!("Enter your move (row space column): ");
                std::io::stdout().flush().unwrap();
                let mut action = String::new();
                std::io::stdin()
                    .read_line(&mut action)
                    .expect("Failed to read line");
                if let Some(action) = move_regex.captures(&action) {
                    println!("{:?}'s move: {} {}", &game.player(), &action[1], &action[2]);
                    let field_name = ttt::FieldName {
                        x: action[1].parse::<usize>().unwrap() - 1,
                        y: action[2].parse::<usize>().unwrap() - 1,
                    };
                    if !game.is_valid_field_name(&field_name) {
                        println!("That square is not on the board.");
                        continue;
                    }
                    if !game.act(&ttt::Action { field_name }) {
                        println!(
                            "Your move is not valid, because that square is already occupied."
                        );
                        continue;
                    }
                } else {
                    println!("I did not understand your move.");
                    continue;
                }
            }
            ttt::GameValue::Won(winner) => {
                println!("The game ended in a win for {:?}!", winner);
                break;
            }
            ttt::GameValue::Draw => {
                println!("The game ended in a draw...");
                break;
            }
        }
    }
}

//! Tic Tac Toe
//!
//!
//!
//!
//!
//!
//!
//!

use std::io::Write;

use tic_tac_toe::{FieldName, GameValue, Hor, Vert, TTT};

fn main() {
    println!("Welcome to Tic Tac Toe!");
    let mut game = TTT::new();
    loop {
        println!("{}", &game);
        match game.game_value() {
            GameValue::Unknown => {
                print!("Enter your move (tl t tr, l m r, bl b br): ");
                std::io::stdout().flush().unwrap();
                let mut command = String::new();
                std::io::stdin()
                    .read_line(&mut command)
                    .expect("successfully read line");

                if let Some(action) = match command.trim() {
                    // top row
                    "tl" | "lt" => Some(FieldName {
                        v: Vert::Top,
                        h: Hor::Left,
                    }),
                    "tm" | "t" | "mt" => Some(FieldName {
                        v: Vert::Top,
                        h: Hor::Mid,
                    }),
                    "tr" | "rt" => Some(FieldName {
                        v: Vert::Top,
                        h: Hor::Right,
                    }),
                    // middle row
                    "ml" | "l" | "lm" => Some(FieldName {
                        v: Vert::Mid,
                        h: Hor::Left,
                    }),
                    "mm" | "m" => Some(FieldName {
                        v: Vert::Mid,
                        h: Hor::Mid,
                    }),
                    "mr" | "r" | "rm" => Some(FieldName {
                        v: Vert::Mid,
                        h: Hor::Right,
                    }),
                    // bottom row
                    "bl" | "lb" => Some(FieldName {
                        v: Vert::Bottom,
                        h: Hor::Left,
                    }),
                    "bm" | "b" | "mb" => Some(FieldName {
                        v: Vert::Bottom,
                        h: Hor::Mid,
                    }),
                    "br" | "rb" => Some(FieldName {
                        v: Vert::Bottom,
                        h: Hor::Right,
                    }),
                    _ => None,
                } {
                    println!("{:?}'s move: {}", game.player(), action);
                    if !game.act(action) {
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
            GameValue::Win(winner) => {
                println!("The game ended in a win for {winner:?}!");
                break;
            }
            GameValue::Draw => {
                println!("The game ended in a draw...");
                break;
            }
        }
    }
}

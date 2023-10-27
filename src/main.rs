//! A simple tic-tac-toe CLI

use std::io::Write; // for flush

use tic_tac_toe::{FieldName, Game, Hor, State, Vert};

static INSTRUCTIONS: &str = "\
Moves can be made using several methods:
 Top Mid Bot     Compass     Number pad
╭───┬───┬───╮ ╭───┬───┬───╮ ╭───┬───┬───╮
│tl │ t │ tr│ │nw │ n │ ne│ │ 7 │ 8 │ 9 │
├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤
│ l │ m │ r │ │ w │ c │ e │ │ 4 │ 5 │ 6 │
├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤
│bl │ b │ br│ │sw │ s │ se│ │ 1 │ 2 │ 3 │
╰───┴───┴───╯ ╰───┴───┴───╯ ╰───┴───┴───╯
";

fn main() {
    println!("Welcome to tic-tac-toe!\n{}", INSTRUCTIONS);
    let mut game = Game::new();
    loop {
        println!("{}", &game);

        match game.state() {
            State::Win(winner) => {
                println!("The game ended in a win for {winner}!");
                break;
            }
            State::Draw => {
                println!("The game ended in a draw...");
                break;
            }
            State::Play(side) => {
                print!("Player {side} to move: ");
                std::io::stdout().flush().unwrap();

                // read input
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("successfully read line");
                let command = input.trim(); // trim newline

                // determine action
                let action = match command {
                    // top row
                    "tl" | "nw" | "7" | "wn" | "lt" => FieldName {
                        v: Vert::Top,
                        h: Hor::Left,
                    },
                    "t" | "n" | "8" => FieldName {
                        v: Vert::Top,
                        h: Hor::Mid,
                    },
                    "tr" | "ne" | "9" | "en" | "rt" => FieldName {
                        v: Vert::Top,
                        h: Hor::Right,
                    },
                    // middle row
                    "l" | "w" | "4" => FieldName {
                        v: Vert::Mid,
                        h: Hor::Left,
                    },
                    "m" | "c" | "5" => FieldName {
                        v: Vert::Mid,
                        h: Hor::Mid,
                    },
                    "r" | "e" | "6" => FieldName {
                        v: Vert::Mid,
                        h: Hor::Right,
                    },
                    // bottom row
                    "bl" | "sw" | "1" | "ws" | "lb" => FieldName {
                        v: Vert::Bottom,
                        h: Hor::Left,
                    },
                    "b" | "s" | "2" => FieldName {
                        v: Vert::Bottom,
                        h: Hor::Mid,
                    },
                    "br" | "se" | "3" | "es" | "rb" => FieldName {
                        v: Vert::Bottom,
                        h: Hor::Right,
                    },
                    _ => {
                        println!("I did not understand your move.");
                        println!("{}", INSTRUCTIONS);
                        continue;
                    }
                };
                // then try to perform that action
                println!("{side} moves: {action}");
                if !game.act(action) {
                    println!("Your move is not valid, because that square is already occupied.");
                    continue;
                }
            }
        }
    }
}

//! Tic Tac Toe
//!
//! 
//!
//!
//!
//!
//!
//!

mod ttt {
    
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Black,
    White
}

impl Player {
    pub fn next(self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldState (pub Option<Player>);

impl std::fmt::Display for FieldState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            None => write!(f, "_")?,
            Some(Player::Black) => write!(f, "B")?,
            Some(Player::White) => write!(f, "W")?,
        }
        Ok(())
    }
}
    
pub struct FieldName {
    pub x : usize,
    pub y : usize,
}

pub struct Action {
    pub field_name : FieldName,
}

pub enum GameValue {
    Unknown,
    Draw,
    Won(Player),
}

#[derive(Debug)]
pub struct TTT {
    player : Player,
    board : [[FieldState; 3] ; 3] // 3x3 matrix
}

impl TTT {
    pub fn new() -> TTT {
        TTT {
            player : Player::White,
            board : [[FieldState(None) ; 3] ; 3]
        }
    }

    pub fn player(&self) -> Player {self.player}
    pub fn get(&self, name : &FieldName) -> &FieldState {
        &self.board[name.x][name.y]
    }
    
    pub fn game_value(&self) -> GameValue {
        fn make_win(a:(usize, usize), (bx, by):(usize, usize), (cx, cy):(usize, usize)) -> [FieldName; 3] {
            [FieldName{x:a.0, y:a.1}, FieldName{x:bx, y:by}, FieldName{x:cx, y:cy}]
        }
        // there are 8 ways to win
        let mut win_lines = vec!();
        for i in 0..2 {
            win_lines.push(make_win((0, i), (1, i), (2, i)));
            win_lines.push(make_win((i, 0), (i, 1), (i, 2)));
        }
        win_lines.push(make_win((0,0), (1,1), (2,2)));
        win_lines.push(make_win((0,2), (1,1), (2,0)));        

        for win in win_lines {
            if self.get(&win[0]) == self.get(&win[1]) && self.get(&win[0]) == self.get(&win[2]) && self.get(&win[0]).0.is_some() {
                return GameValue::Won(self.get(&win[0]).0.unwrap());
            }
        }
        // FIXME: detect Draw
        GameValue::Unknown
    }

    pub fn is_valid_field_name(&self, name : &FieldName) -> bool {
        name.x < self.board.len() && name.y < self.board[0].len()
    }
    
    pub fn is_valid_action(&self, action : &Action) -> bool {
        // an action is valid if it fills an empty field
        self.get_field(&action.field_name).0.is_none()
    }
    
    fn set_field_state(&mut self, name : &FieldName, state : FieldState) {
        self.board[name.x][name.y] = state;
    }

    fn get_field(&self, name : &FieldName) -> FieldState {
        self.board[name.x][name.y]
    }

    pub fn act(&mut self, action : &Action) -> bool {
        if self.is_valid_action(action) {
            self.set_field_state(&action.field_name, FieldState(Some(self.player)));
            self.player = self.player.next();
            true
        } else { false }        
    }
}

impl std::fmt::Display for TTT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "It is {:?}'s turn.", self.player)?;
        let width = 3;
        write!(f, "{:width$}", "", width = width)?;
        for col in 1..=self.board[0].len() {        
            write!(f, "{:^width$}", col, width = width)?
        }
        writeln!(f)?;
        for (row_num, row) in self.board.iter().enumerate() {
            write!(f, "{:^width$}", row_num+1, width = width)?;
            for square in row.iter() {
                write!(f, "{:^width$}", format!("{}", square), width = width)?
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}
} // close mod ttt

use regex::Regex;
use std::io::Write;

fn main() {
    let move_regex : regex::Regex = Regex::new(r"^\w*(\d)[ ,:;](\d)\w*\n").unwrap();
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
                    let field_name = ttt::FieldName{x: action[1].parse::<usize>().unwrap()-1, y: action[2].parse::<usize>().unwrap()-1};
                    if !game.is_valid_field_name(&field_name) {
                        println!("That square is not on the board.");
                        continue
                    }
                    if !game.act(&ttt::Action{field_name}) {
                        println!("Your move is not valid, because that square is already occupied.");
                        continue
                    }
                } else {
                    println!("I did not understand your move.");
                    continue
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

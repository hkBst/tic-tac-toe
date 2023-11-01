//! A simple and safe [tic-tac-toe](https://en.wikipedia.org/wiki/Tic-tac-toe) implementation.

use std::fmt::{Display, Formatter, Result as FmtResult};

/// sides of play
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    X,
    O,
}

impl Side {
    #[must_use]
    pub fn next(self) -> Side {
        match self {
            Side::X => Side::O,
            Side::O => Side::X,
        }
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Side::X => write!(f, "×")?, // or ❌
            Side::O => write!(f, "○")?, // or ⭕
        }
        Ok(())
    }
}

/// state of a field
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldState(pub Option<Side>);

impl FieldState {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

impl Display for FieldState {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self.0 {
            None => write!(f, " ")?,
            Some(p) => write!(f, "{p}")?,
        }
        Ok(())
    }
}

/// horizontal coordinates
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hor {
    Left,
    Mid,
    Right,
}

/// vertical coordinates
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Vert {
    Top,
    Mid,
    Bottom,
}

/// a coordinate designation for a field
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldName {
    pub v: Vert,
    pub h: Hor,
}

impl Display for FieldName {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self.v)?;
        write!(f, ", ")?;
        write!(f, "{:?}", self.h)?;
        Ok(())
    }
}

/// state of play
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Play(Side),
    Win(Side),
    Draw,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                State::Play(side) => format!("{side} to move... "),
                State::Win(side) => format!("{side} wins"),
                State::Draw => "It's a draw".to_string(),
            }
        )
    }
}

type Board = [[FieldState; 3]; 3];

/// board and state of play
#[derive(Clone, Debug)]
pub struct Game {
    state: State,
    board: Board,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    #[must_use]
    pub fn new() -> Game {
        Game {
            state: State::Play(Side::X),
            board: [[FieldState(None); 3]; 3],
        }
    }

    #[must_use]
    pub fn state(&self) -> State {
        self.state
    }

    #[must_use]
    pub fn playing(&self) -> bool {
        matches!(self.state, State::Play(_))
    }

    #[must_use]
    pub fn board(&self) -> &Board {
        &self.board
    }

    #[must_use]
    pub fn get(&self, name: FieldName) -> FieldState {
        self.board[name.v as usize][name.h as usize]
    }

    fn set(&mut self, name: FieldName, state: FieldState) {
        self.board[name.v as usize][name.h as usize] = state;
    }

    #[must_use]
    fn is_empty(&self, field_name: FieldName) -> bool {
        self.get(field_name).is_empty()
    }

    pub fn act(&mut self, field_name: FieldName) -> bool {
        match self.state {
            State::Play(side) if self.is_empty(field_name) => {
                self.set(field_name, FieldState(Some(side)));
                self.update_state(side);
                true
            }
            _ => false,
        }
    }

    fn update_state(&mut self, side: Side) {
        let hor = |v: Vert| {
            [
                FieldName { h: Hor::Left, v },
                FieldName { h: Hor::Mid, v },
                FieldName { h: Hor::Right, v },
            ]
        };
        let vert = |h: Hor| {
            [
                FieldName { h, v: Vert::Top },
                FieldName { h, v: Vert::Mid },
                FieldName { h, v: Vert::Bottom },
            ]
        };
        let win_lines: [[FieldName; 3]; 8] = [
            hor(Vert::Top),
            hor(Vert::Mid),
            hor(Vert::Bottom),
            vert(Hor::Left),
            vert(Hor::Mid),
            vert(Hor::Right),
            [
                FieldName {
                    h: Hor::Left,
                    v: Vert::Top,
                },
                FieldName {
                    h: Hor::Mid,
                    v: Vert::Mid,
                },
                FieldName {
                    h: Hor::Right,
                    v: Vert::Bottom,
                },
            ],
            [
                FieldName {
                    h: Hor::Right,
                    v: Vert::Top,
                },
                FieldName {
                    h: Hor::Mid,
                    v: Vert::Mid,
                },
                FieldName {
                    h: Hor::Left,
                    v: Vert::Bottom,
                },
            ],
        ];

        for [a, b, c] in win_lines {
            match self.get(a).0 {
                Some(s) if Some(s) == self.get(b).0 && Some(s) == self.get(c).0 => {
                    self.state = State::Win(s);
                    return;
                }
                _ => continue,
            }
        }

        // if there is a win_line with at most a single side, then a win is still possible
        for [a, b, c] in win_lines {
            match self.get(a).0 {
                Some(s) if Some(s.next()) != self.get(b).0 && Some(s.next()) != self.get(c).0 => {
                    self.state = State::Play(side.next());
                    return;
                }
                None => match self.get(b).0 {
                    Some(s) if Some(s.next()) != self.get(c).0 => {
                        self.state = State::Play(side.next());
                        return;
                    }
                    None => {
                        self.state = State::Play(side.next());
                        return;
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }

        self.state = State::Draw;
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let width = 3;
        for (border, fields) in std::iter::once("╭───┬───┬───╮")
            .chain(std::iter::repeat("├───┼───┼───┤"))
            .zip(self.board.iter().map(|row| {
                format!(
                    "│{:^width$}│{:^width$}│{:^width$}│",
                    format!("{}", row[0]),
                    format!("{}", row[1]),
                    format!("{}", row[2])
                )
            }))
        {
            writeln!(f, "{border}\n{fields}")?;
        }
        writeln!(f, "╰───┴───┴───╯")?;
        Ok(())
    }
}

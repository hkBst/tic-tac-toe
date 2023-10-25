#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    #[must_use]
    pub fn next(self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldState(pub Option<Player>);

impl FieldState {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hor {
    Left,
    Mid,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vert {
    Top,
    Mid,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldName {
    pub v: Vert,
    pub h: Hor,
}

impl std::fmt::Display for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.v)?;
        write!(f, ", ")?;
        write!(f, "{:?}", self.h)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameValue {
    Unknown,
    Draw,
    Win(Player),
}

#[derive(Debug)]
pub struct Game {
    player: Player,
    board: [[FieldState; 3]; 3],
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
            player: Player::White,
            board: [[FieldState(None); 3]; 3],
        }
    }

    #[must_use]
    pub fn player(&self) -> Player {
        self.player
    }

    #[must_use]
    pub fn get(&self, name: &FieldName) -> FieldState {
        self.board[name.v as usize][name.h as usize]
    }

    fn set(&mut self, name: FieldName, state: FieldState) {
        self.board[name.v as usize][name.h as usize] = state;
    }

    #[must_use]
    pub fn game_value(&self) -> GameValue {
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

        for [a, b, c] in &win_lines {
            match self.get(a).0 {
                Some(player) if Some(player) == self.get(b).0 && Some(player) == self.get(c).0 => {
                    return GameValue::Win(player);
                }
                _ => continue,
            }
        }

        // if there is a win_line with at most a single player, then a win is still possible
        for [a, b, c] in &win_lines {
            match self.get(a).0 {
                Some(player)
                    if Some(player.next()) != self.get(b).0
                        && Some(player.next()) != self.get(c).0 =>
                {
                    return GameValue::Unknown
                }
                None => match self.get(b).0 {
                    Some(player) if Some(player.next()) != self.get(c).0 => {
                        return GameValue::Unknown
                    }
                    None => return GameValue::Unknown,
                    _ => continue,
                },
                _ => continue,
            }
        }

        GameValue::Draw
    }

    #[must_use]
    pub fn is_valid_action(&self, field_name: FieldName) -> bool {
        // an action is valid if it fills an empty field
        self.get(&field_name).is_empty()
    }

    pub fn act(&mut self, field_name: FieldName) -> bool {
        if self.is_valid_action(field_name) {
            self.set(field_name, FieldState(Some(self.player)));
            self.player = self.player.next();
            true
        } else {
            false
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "It is {:?}'s turn.", self.player)?;
        let width = 3;
        writeln!(
            f,
            "{:width$}{l:^width$}{m:^width$}{r:^width$}",
            "",
            l = "l",
            m = "m",
            r = "r",
            width = width
        )?;
        for (symbol, row) in ['t', 'm', 'b'].iter().zip(self.board.iter()) {
            write!(f, "{symbol:^width$}")?;
            for field in row {
                write!(f, "{:^width$}", format!("{}", field), width = width)?;
            }
            writeln!(f)?;
        }
	Ok(())
    }
}

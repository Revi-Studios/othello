use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Black,
    White,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Black => write!(f, "Black"),
            Piece::White => write!(f, "White"),
        }
    }
}

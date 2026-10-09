use std::{fmt::Display, ops::Not};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
    Black,
    White,
}

impl Not for Piece {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Black => write!(f, "Black"),
            Piece::White => write!(f, "White"),
        }
    }
}

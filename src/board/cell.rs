use super::piece::Piece;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Filled(Piece),
    Emty,
}

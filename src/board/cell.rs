use super::piece::Piece;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Filled(Piece),
    Emty,
}

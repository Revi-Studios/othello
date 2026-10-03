use super::piece::Piece;

#[derive(Clone, Copy)]
pub enum Cell {
    Filled(Piece),
    Emty,
}

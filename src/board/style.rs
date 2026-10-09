use macroquad::color::{BLACK, Color, WHITE};

use crate::board::piece::Piece;

pub struct BoardStyle {
    pub x: f32,
    pub y: f32,

    pub color: Color,
    pub cell_width: f32,

    pub piece_radius: f32,
    pub piece_black_color: Color,
    pub piece_white_color: Color,

    pub line_thickness: f32,
    pub line_color: Color,

    pub render_suggest: bool,
    pub suggest_piece: Piece,

    pub suggest_color: Color,
    pub suggest_radius: f32,
}

impl Default for BoardStyle {
    fn default() -> Self {
        BoardStyle {
            x: 0.0,
            y: 0.0,

            color: Color::from_rgba(55, 126, 33, 255),

            piece_radius: 32.0,
            piece_black_color: BLACK,
            piece_white_color: WHITE,

            cell_width: 80.0,
            line_thickness: 2.0,
            line_color: BLACK,

            render_suggest: true,
            suggest_piece: Piece::Black,

            suggest_color: Color {
                r: 0.1,
                g: 0.1,
                b: 0.1,
                a: 0.5,
            },

            suggest_radius: 10.0,
        }
    }
}

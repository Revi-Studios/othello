use macroquad::color::{BLACK, Color};

pub struct BoardStyle {
    pub x: f32,
    pub y: f32,

    pub color: Color,
    pub cell_width: f32,

    pub piece_radius: f32,

    pub line_thickness: f32,
    pub line_color: Color,
}

impl Default for BoardStyle {
    fn default() -> Self {
        BoardStyle {
            x: 0.0,
            y: 0.0,

            color: Color::from_rgba(55, 126, 33, 255),

            piece_radius: 32.0,
            cell_width: 80.0,
            line_thickness: 2.0,
            line_color: BLACK,
        }
    }
}

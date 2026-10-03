pub mod cell;
pub mod piece;
pub mod style;

use crate::board::{cell::Cell, piece::Piece, style::BoardStyle};
use crate::render::{Render, RenderBox};
use macroquad::shapes::draw_rectangle;
use macroquad::{
    color::{BLACK, WHITE},
    shapes::{draw_circle, draw_line},
};

const WIDTH: u8 = 8;
const HEIGHT: u8 = 8;

pub struct Board {
    cells: [Cell; (WIDTH * HEIGHT) as usize],
    pub style: BoardStyle,
}

impl Board {
    pub fn new(style: BoardStyle) -> Board {
        Board {
            cells: [Cell::Emty; (WIDTH * HEIGHT) as usize],
            style: style,
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::Emty);
    }

    pub fn start_default(&mut self) {
        self.clear();

        self.set(3, 3, Piece::White);
        self.set(4, 3, Piece::Black);
        self.set(3, 4, Piece::Black);
        self.set(4, 4, Piece::White);
    }

    pub fn set(&mut self, x: u8, y: u8, piece: Piece) {
        let idx = (x + (y * WIDTH)) as usize;

        self.cells[idx] = Cell::Filled(piece);
    }
}

impl Render for Board {
    fn render(&self) {
        draw_rectangle(
            self.style.x,
            self.style.y,
            self.style.cell_width * WIDTH as f32,
            self.style.cell_width * HEIGHT as f32,
            self.style.color,
        );

        // Vertical Lines
        for i in 0..WIDTH + 1 {
            let x = i as f32 * self.style.cell_width + self.style.x;

            draw_line(
                x,
                self.style.y,
                x,
                self.style.y + HEIGHT as f32 * self.style.cell_width,
                self.style.line_thickness,
                self.style.line_color,
            );
        }
        // Horizontal Lines
        for i in 0..HEIGHT + 1 {
            let y = i as f32 * self.style.cell_width + self.style.y;

            draw_line(
                self.style.x,
                y,
                WIDTH as f32 * self.style.cell_width + self.style.x,
                y,
                self.style.line_thickness,
                self.style.line_color,
            );
        }
        for (x, y) in vec![(2.0 as f32, 2.0 as f32), (2.0, 6.0), (6.0, 2.0), (6.0, 6.0)] {
            draw_circle(
                self.style.cell_width * x + self.style.x,
                self.style.cell_width * y + self.style.y,
                self.style.line_thickness * 3.0,
                self.style.line_color,
            );
        }

        for idx in 0..self.cells.len() {
            match self.cells[idx] {
                Cell::Emty => continue,
                Cell::Filled(piece) => draw_piece(&self.style, piece, idx as u8),
            }
        }
    }

    fn render_box(&self) -> RenderBox {
        RenderBox {
            x: self.style.x,
            y: self.style.y,
            width: self.style.cell_width * WIDTH as f32 + self.style.line_thickness,
            height: self.style.cell_width * HEIGHT as f32 + self.style.line_thickness,
        }
    }
}

fn draw_piece(style: &BoardStyle, piece: Piece, idx: u8) {
    let color = match piece {
        Piece::Black => BLACK,
        Piece::White => WHITE,
    };

    draw_circle(
        style.cell_width * (idx % WIDTH) as f32 + style.cell_width / 2.0 + style.x,
        (idx / WIDTH) as f32 * style.cell_width + style.cell_width / 2.0 + style.y,
        style.piece_radius,
        color,
    )
}

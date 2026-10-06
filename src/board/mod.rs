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
    pub cells: [[Cell; HEIGHT as usize]; WIDTH as usize],
    pub style: BoardStyle,
}

impl Board {
    pub fn new(style: BoardStyle) -> Board {
        Board {
            cells: [[Cell::Emty; HEIGHT as usize]; WIDTH as usize],
            style: style,
        }
    }

    pub fn clear(&mut self) {
        self.cells.iter_mut().for_each(|c| c.fill(Cell::Emty));
    }

    pub fn start_default(&mut self) {
        self.clear();

        self.set(3, 3, Piece::White);
        self.set(4, 3, Piece::Black);
        self.set(3, 4, Piece::Black);
        self.set(4, 4, Piece::White);
    }

    pub fn set(&mut self, x: usize, y: usize, piece: Piece) {
        self.cells[x][y] = Cell::Filled(piece);
    }

    pub fn place(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        let Some(affected) = self.check(piece, x as i8, y as i8) else {
            return false;
        };

        if self.cells[x][y] != Cell::Emty {
            return false;
        }

        self.set(x, y, piece);

        for (r, c) in affected {
            self.cells[r][c] = Cell::Filled(piece);
        }

        true
    }

    /// Checks weather a piece can be place on spot (x, y). If it can, all affected piece positions get returned in `Vec<(x, y)`
    pub fn check(&self, piece: Piece, x: i8, y: i8) -> Option<Vec<(usize, usize)>> {
        const DIR: [(i8, i8); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let mut affected = Vec::with_capacity(10);

        for (dx, dy) in DIR {
            let mut found: Vec<(usize, usize)> = Vec::with_capacity(8);

            let mut lx = x;
            let mut ly = y;

            loop {
                lx += dx;
                ly += dy;

                if lx >= WIDTH as i8 || lx <= 0 || ly >= HEIGHT as i8 || ly <= 0 {
                    break;
                }

                // println!("({lx}, {ly})");

                match self.cells[lx as usize][ly as usize] {
                    Cell::Emty => break,
                    Cell::Filled(p) => {
                        if p == piece {
                            affected.append(&mut found);
                            break;
                        }
                        found.push((lx as usize, ly as usize));
                    }
                }
            }
        }

        println!("{:?}", affected);
        if affected.len() > 0 {
            Some(affected)
        } else {
            None
        }
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

        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                match self.cells[x][y] {
                    Cell::Emty => continue,
                    Cell::Filled(piece) => {
                        let color = match piece {
                            Piece::Black => BLACK,
                            Piece::White => WHITE,
                        };

                        draw_circle(
                            self.style.cell_width * x as f32
                                + self.style.cell_width / 2.0
                                + self.style.x,
                            self.style.cell_width * y as f32
                                + self.style.cell_width / 2.0
                                + self.style.y,
                            self.style.piece_radius,
                            color,
                        )
                    }
                }
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

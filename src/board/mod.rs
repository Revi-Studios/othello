pub mod cell;
pub mod piece;
pub mod style;

use crate::board::cell::Cell::Filled;
use crate::board::{cell::Cell, piece::Piece, style::BoardStyle};
use crate::render::{Render, RenderBox};
use macroquad::shapes::{draw_poly, draw_rectangle};
use macroquad::{
    color::{BLACK, WHITE},
    shapes::{draw_circle, draw_line},
};

const WIDTH: u8 = 8;
const HEIGHT: u8 = 8;

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub struct Board {
    pub cells: [[Cell; HEIGHT as usize]; WIDTH as usize],
    pub style: BoardStyle,

    last: Option<(usize, usize)>,
}

impl Board {
    pub fn new(style: BoardStyle) -> Board {
        Board {
            cells: [[Cell::Emty; HEIGHT as usize]; WIDTH as usize],
            style: style,

            last: None,
        }
    }

    pub fn clear(&mut self) {
        self.cells.iter_mut().for_each(|c| c.fill(Cell::Emty));
        self.last = None;
    }

    pub fn start_default(&mut self) {
        self.clear();

        self.set(3, 3, Filled(Piece::White));
        self.set(4, 3, Filled(Piece::Black));
        self.set(3, 4, Filled(Piece::Black));
        self.set(4, 4, Filled(Piece::White));
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[x][y] = cell;
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[x][y]
    }

    /// Returns true if it was valid and placed
    pub fn place(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        let Some(affected) = self.get_affected(piece, x as i8, y as i8) else {
            return false;
        };

        if self.cells[x][y] != Cell::Emty {
            return false;
        }

        self.set(x, y, Filled(piece));
        self.last = Some((x, y));

        for (r, c) in affected {
            self.cells[r][c] = Cell::Filled(piece);
        }

        true
    }

    /// Checks weather a piece can be place on spot (x, y). It dosn't save affected and will return as soon as it finds at least 1 affected. This should be used whenever you just want to know if a spot it a valid move.
    pub fn check(&self, piece: Piece, x: usize, y: usize) -> bool {
        for (dx, dy) in DIRECTIONS {
            let mut found_one = false;

            let mut lx = x as i8;
            let mut ly = y as i8;

            loop {
                lx += dx;
                ly += dy;

                if lx >= WIDTH as i8 || lx < 0 || ly >= HEIGHT as i8 || ly < 0 {
                    break;
                }

                match self.cells[lx as usize][ly as usize] {
                    Cell::Emty => break,
                    Cell::Filled(p) => {
                        if p == piece {
                            if found_one {
                                return true;
                            }
                            break;
                        }
                        if p != piece {
                            found_one = true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn has_moves(&self, piece: Piece) -> bool {
        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                match self.get(x, y) {
                    Cell::Filled(_) => continue,
                    Cell::Emty => {
                        if self.check(piece, x, y) {
                            return true;
                        }
                    }
                };
            }
        }
        false
    }

    pub fn get_affected(&self, piece: Piece, x: i8, y: i8) -> Option<Vec<(usize, usize)>> {
        let mut affected = Vec::with_capacity(10);

        for (dx, dy) in DIRECTIONS {
            let mut found: Vec<(usize, usize)> = Vec::with_capacity(8);

            let mut lx = x;
            let mut ly = y;

            loop {
                lx += dx;
                ly += dy;

                if lx >= WIDTH as i8 || lx < 0 || ly >= HEIGHT as i8 || ly < 0 {
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

    pub fn count(&self) -> (u8, u8) {
        let mut black = 0;
        let mut white = 0;
        for cell in self.cells.iter().flatten() {
            match cell {
                Cell::Filled(Piece::Black) => black += 1,
                Cell::Filled(Piece::White) => white += 1,
                _ => {}
            }
        }

        (black, white)
    }
}

impl Render for Board {
    fn render(&self) {
        // Board
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

        // Decoration points
        for (x, y) in vec![(2.0 as f32, 2.0 as f32), (2.0, 6.0), (6.0, 2.0), (6.0, 6.0)] {
            draw_circle(
                self.style.cell_width * x + self.style.x,
                self.style.cell_width * y + self.style.y,
                self.style.line_thickness * 3.0,
                self.style.line_color,
            );
        }

        // Pieces
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                let fade = match self.style.render_fade {
                    true => self.style.piece_fade,
                    false => 1.0,
                };

                match self.cells[x][y] {
                    Cell::Emty => {
                        if self.style.render_suggest && self.check(self.style.suggest_piece, x, y) {
                            draw_circle(
                                self.style.cell_width * x as f32
                                    + self.style.cell_width / 2.0
                                    + self.style.x,
                                self.style.cell_width * y as f32
                                    + self.style.cell_width / 2.0
                                    + self.style.y,
                                self.style.suggest_radius,
                                self.style
                                    .suggest_color
                                    .with_alpha(self.style.suggest_color.a * fade),
                            )
                        }
                    }
                    Cell::Filled(piece) => {
                        let color = match piece {
                            Piece::Black => BLACK,
                            Piece::White => WHITE,
                        };

                        draw_poly(
                            self.style.cell_width * x as f32
                                + self.style.cell_width / 2.0
                                + self.style.x,
                            self.style.cell_width * y as f32
                                + self.style.cell_width / 2.0
                                + self.style.y,
                            35,
                            self.style.piece_radius,
                            0.0,
                            color.with_alpha(fade),
                        )
                    }
                }
            }
        }
        if self.style.render_last && self.last.is_some() {
            let (x, y) = self.last.unwrap();

            draw_circle(
                self.style.cell_width * x as f32 + self.style.cell_width / 2.0 + self.style.x,
                self.style.cell_width * y as f32 + self.style.cell_width / 2.0 + self.style.y,
                self.style.last_radius,
                self.style.last_color,
            )
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

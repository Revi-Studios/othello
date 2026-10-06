#![allow(unused)]

use crate::{
    board::{self, Board, piece::Piece, style::BoardStyle},
    render::Render,
};

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Start,
    Paused,
    Waiting,
    End,
}

pub struct Game {
    pub board: Board,
    pub turn: Piece,
    state: State,
}

impl Game {
    pub fn new(board_style: BoardStyle) -> Game {
        Game {
            board: Board::new(board_style),
            turn: Piece::Black,
            state: State::Start,
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn start(&mut self) {
        self.board.start_default();

        // let starting_piece = if rand::random() {
        //     Piece::Black
        // } else {
        //     Piece::White
        // };

        // self.turn = starting_piece;

        self.turn = Piece::Black;

        self.state = State::Waiting;
    }

    pub fn end(&mut self) {
        self.state = State::End;
    }

    pub fn place(&mut self, mouse: (f32, f32)) {
        let (mut x, mut y) = mouse;

        let render_box = self.board.render_box();

        x -= render_box.x;
        y -= render_box.y;

        if !(x < render_box.op_x() && y < render_box.op_y()) {
            return;
        }

        println!("{x}, {y}");
        println!(
            "{}, {}",
            (x / self.board.style.cell_width),
            (y / self.board.style.cell_width)
        );
        println!(
            "{}, {}",
            (x / self.board.style.cell_width) as usize,
            (y / self.board.style.cell_width) as usize
        );

        if self.board.place(
            (x / self.board.style.cell_width) as usize,
            (y / self.board.style.cell_width) as usize,
            self.turn,
        ) {
            match self.turn {
                Piece::Black => self.turn = Piece::White,
                Piece::White => self.turn = Piece::Black,
            };
        }
        println!("{}", self.turn)
    }
}

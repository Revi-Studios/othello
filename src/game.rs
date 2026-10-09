#![allow(unused)]

use std::{cmp::Ordering, usize};

use crate::{
    board::{
        self, Board,
        cell::Cell::{self, Filled},
        piece::{self, Piece},
        style::BoardStyle,
    },
    game,
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
    pub state: State,
    history: Vec<(usize, usize, Piece)>,
    history_offset: usize,

    pub black_count: u8,
    pub white_count: u8,
}

impl Game {
    pub fn new(board_style: BoardStyle) -> Game {
        Game {
            board: Board::new(board_style),
            turn: Piece::Black,
            state: State::Start,
            history: Vec::with_capacity(64),
            history_offset: 0,
            black_count: 0,
            white_count: 0,
        }
    }

    pub fn start(&mut self) {
        self.board.start_default();
        self.history.clear();
        self.history_offset = 0;
        self.board.style.render_suggest = true;
        (self.black_count, self.white_count) = self.board.count();

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
        self.board.style.render_suggest = false;

        println!("{:?}", self.board.count());

        let (black, white) = self.board.count();

        match black.cmp(&white) {
            Ordering::Greater => println!("Black won! {}/{}", black, white),
            Ordering::Less => println!("White won! {}/{}", white, black),
            Ordering::Equal => println!("Tie! {}/{}", black, white),
        }
    }

    pub fn place(&mut self, mouse: (f32, f32)) {
        let (mut x, mut y) = mouse;

        let render_box = self.board.render_box();

        x -= render_box.x;
        y -= render_box.y;

        if !(x < render_box.op_x() && y < render_box.op_y()) {
            return;
        }

        let ux = (x / self.board.style.cell_width) as usize;
        let uy = (y / self.board.style.cell_width) as usize;

        if self.board.place(ux, uy, self.turn) {
            if self.history_offset != 0 {
                self.history
                    .truncate(self.history.len().saturating_sub(self.history_offset));
                self.history_offset = 0
            }
            self.history.push((ux, uy, self.turn));

            self.turn = !self.turn;
            (self.black_count, self.white_count) = self.board.count();

            if self.black_count + self.white_count == 64 {
                self.end();
            }
        }
    }

    pub fn undo(&mut self) {
        if !(self.history.len() > self.history_offset) {
            return;
        }

        self.history_offset += 1;

        self.turn = Piece::Black;
        self.board.start_default();
        for (x, y, piece) in self.history[0..self.history.len().saturating_sub(self.history_offset)]
            .iter()
            .copied()
        {
            println!("{x} {y} {piece}");
            self.board.place(x, y, piece);
            self.turn = !piece;
        }

        println!("undo: {}", self.history_offset);
        println!("undo: {:?}", self.history);
    }

    pub fn redo(&mut self) {
        if self.history_offset == 0 {
            return;
        }

        self.history_offset -= 1;
        let (x, y, piece) =
            self.history[self.history.len().saturating_sub(self.history_offset) - 1];

        self.board.place(x, y, piece);
        self.turn = !piece;

        println!("redo: {}", self.history_offset,);
        println!("redo: {:?}", self.history);
    }
}

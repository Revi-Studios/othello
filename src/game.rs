use crate::{
    board::{Board, piece::Piece, style::BoardStyle},
    render::Render,
};

pub struct Game {
    pub board: Board,
    pub turn: Piece,
    history: Vec<(usize, usize, Piece)>,
    history_offset: usize,

    pub black_count: u8,
    pub white_count: u8,
    pub suggestions: bool,

    pub started: bool,
    pub ended: bool,
}

impl Game {
    pub fn new(board_style: BoardStyle) -> Game {
        Game {
            board: Board::new(board_style),
            turn: Piece::Black,
            history: Vec::with_capacity(64),
            history_offset: 0,
            black_count: 0,
            white_count: 0,
            suggestions: true,
            started: false,
            ended: false,
        }
    }

    pub fn toggle_suggestions(&mut self) {
        self.suggestions = !self.suggestions;
        self.board.style.render_suggest = self.suggestions;
    }

    pub fn start(&mut self) {
        self.board.start_default();

        // Reset history
        self.history.clear();
        self.history_offset = 0;

        self.board.style.render_suggest = self.suggestions;
        (self.black_count, self.white_count) = self.board.count();

        self.ended = false;

        // let starting_piece = if rand::random() {
        //     Piece::Black
        // } else {
        //     Piece::White
        // };

        // self.turn = starting_piece;

        self.turn = Piece::Black;
    }

    pub fn end(&mut self) {
        self.board.style.render_suggest = false;
        self.started = false;
        self.ended = true;

        println!("{:?}", self.board.count());
    }

    pub fn place(&mut self, mouse: (f32, f32)) {
        let (mut x, mut y) = mouse;

        let render_box = self.board.render_box();

        x -= render_box.x;
        y -= render_box.y;

        if !(x < render_box.op_x() && y < render_box.op_y()) {
            return;
        }

        let ux = (x / (self.board.style.cell_width * self.board.style.scale)) as usize;
        let uy = (y / (self.board.style.cell_width * self.board.style.scale)) as usize;

        if self.board.place(ux, uy, self.turn) {
            if self.history_offset != 0 {
                self.history
                    .truncate(self.history.len().saturating_sub(self.history_offset));
                self.history_offset = 0
            }
            self.history.push((ux, uy, self.turn));

            (self.black_count, self.white_count) = self.board.count();

            if self.black_count + self.white_count == 64
                || self.black_count == 0
                || self.white_count == 0
            {
                self.end();
                return;
            }

            if self.board.has_moves(!self.turn) {
                self.turn = !self.turn;
            }
        }
        self.started = true
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
            if self.board.has_moves(!self.turn) {
                self.turn = !self.turn;
            }
        }

        println!("undo: {}", self.history_offset);
    }

    pub fn redo(&mut self) {
        if self.history_offset == 0 {
            return;
        }

        self.history_offset -= 1;
        let (x, y, piece) =
            self.history[self.history.len().saturating_sub(self.history_offset) - 1];

        self.board.place(x, y, piece);
        if self.board.has_moves(!self.turn) {
            self.turn = !self.turn;
        }

        println!("redo: {}", self.history_offset,);
    }
}

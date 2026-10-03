use board::Board;
use macroquad::{miniquad::window::quit, prelude::*, time, ui::root_ui};

use crate::{board::style::BoardStyle, render::Render};

mod board;
mod config;
mod render;

use config::window_conf;
#[macroquad::main(window_conf)]
async fn main() {
    let mut board = Board::new(BoardStyle {
        x: 20.0,
        y: 20.0,
        ..BoardStyle::default()
    });

    board.start_default();

    let board_box = board.render_box();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            quit();
        }

        {
            let h = screen_height();
            if h > board_box.height {
                board.style.y = h / 2.0 - board_box.height / 2.0
            }

            if root_ui().button(vec2(h - (h - board_box.op_x()) / 2.0, 200.0), "Start Game") {
                board.start_default();
            }
            if root_ui().button(vec2(h - (h - board_box.op_x()) / 2.0, 400.0), "End Game") {
                board.clear();
            }

            if is_mouse_button_pressed(MouseButton::Left) {
                todo!("Add piece placing")
            }
        }

        println!("updating {}", time::get_time());

        // Rendering

        clear_background(Color::from_rgba(55, 126, 33, 255));

        board.render();

        next_frame().await;
    }
}

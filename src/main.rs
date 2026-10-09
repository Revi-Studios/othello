use macroquad::{
    input::KeyCode::{Left, Right},
    miniquad::window::quit,
    prelude::*,
    ui::root_ui,
};

use crate::{
    board::style::BoardStyle,
    game::{Game, State},
    render::Render,
};

mod board;
mod config;
mod game;
mod render;

use config::window_conf;
#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new(BoardStyle {
        x: 20.0,
        y: 20.0,
        ..BoardStyle::default()
    });

    let board_box = game.board.render_box();
    loop {
        clear_background(Color::from_rgba(74, 58, 41, 250));

        if is_key_pressed(KeyCode::Escape) {
            quit();
        }

        {
            let h = screen_height();

            if h > board_box.height {
                game.board.style.y = h / 2.0 - board_box.height / 2.0
            }

            if root_ui().button(vec2(h - (h - board_box.op_x()) / 2.0, 200.0), "Start Game") {
                game.start();
            }
            if root_ui().button(vec2(h - (h - board_box.op_x()) / 2.0, 400.0), "End Game") {
                game.end();
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) && game.get_state() == State::Waiting {
            game.place(mouse_position());
        }

        if is_key_pressed(Left) && game.get_state() == State::Waiting {
            game.undo();
        }
        if is_key_pressed(Right) && game.get_state() == State::Waiting {
            game.redo();
        }

        draw_text(
            format!("Turn: {}", game.turn),
            board_box.op_x() + 10.0,
            60.0,
            20.0,
            WHITE,
        );

        // println!("updating {}", time::get_time());

        game.board.style.suggest_piece = game.turn;
        game.board.render();

        next_frame().await;
    }
}

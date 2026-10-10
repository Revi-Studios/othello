use macroquad::{
    input::KeyCode::{Left, Right},
    miniquad::window::quit,
    prelude::*,
    ui::root_ui,
};

use crate::{
    board::{piece::Piece, style::BoardStyle},
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

    let font = load_ttf_font("assets/fonts/JosefinSans-Regular.ttf")
        .await
        .expect("Faild to load font from file");

    loop {
        let h = screen_height();
        let w = screen_width();

        clear_background(Color::from_rgba(74, 58, 41, 250));

        if h > board_box.height {
            game.board.style.y = h / 2.0 - board_box.height / 2.0
        }

        if is_key_pressed(KeyCode::Escape) {
            quit();
        }

        if is_key_pressed(KeyCode::P) {
            game.toggle_suggestions();
            println!("{}", game.suggestions)
        }

        if is_mouse_button_pressed(MouseButton::Left) && game.state == State::Waiting {
            game.place(mouse_position());
        }

        if is_key_pressed(Left) && game.state == State::Waiting {
            game.undo();
        }

        if is_key_pressed(Right) && game.state == State::Waiting {
            game.redo();
        }

        {
            let m = w - (w - board_box.op_x()) / 2.0;

            // Black side
            draw_circle(
                m,
                40.0 + game.board.style.piece_radius,
                game.board.style.piece_radius,
                game.board.style.piece_black_color,
            );

            draw_text_ex(
                format!("{}", game.black_count),
                m - measure_text(format!("{}", game.black_count), Some(&font), 16, 1.0).width / 2.0,
                40.0 + game.board.style.piece_radius * 3.0,
                TextParams {
                    color: WHITE,
                    font: Some(&font),
                    font_size: 16,
                    ..Default::default()
                },
            );

            // White side
            draw_circle(
                m,
                h - (40.0 + game.board.style.piece_radius),
                game.board.style.piece_radius,
                game.board.style.piece_white_color,
            );

            draw_text_ex(
                format!("{}", game.white_count),
                m - measure_text(format!("{}", game.black_count), Some(&font), 16, 1.0).width / 2.0,
                h - (40.0 + game.board.style.piece_radius * 3.0),
                TextParams {
                    color: WHITE,
                    font: Some(&font),
                    font_size: 16,
                    ..Default::default()
                },
            );

            draw_circle(
                m,
                match game.turn {
                    Piece::Black => 80.0 + game.board.style.piece_radius * 3.0,
                    Piece::White => h - (80.0 + game.board.style.piece_radius * 3.0),
                },
                5.0,
                WHITE,
            );
        }

        //             if root_ui().button(vec2(h - (h - board_box.op_x()) / 2.0, 200.0), "Start Game") {
        //                 game.start();
        //             }
        //             if root_ui().button(vec2(h - (h - board_box.op_x()) / 2.0, 400.0), "End Game") {
        //                 game.end();
        //             }

        // println!("updating {}", time::get_time());

        game.board.style.suggest_piece = game.turn;
        game.board.render();

        if game.state == State::Start || game.state == State::Paused {
            draw_rectangle(
                0.0,
                0.0,
                w,
                h,
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.3,
                },
            );

            draw_text_ex(
                "Othello",
                w / 2.0 - measure_text("Othello", Some(&font), 50, 1.0).width / 2.0,
                200.0,
                TextParams {
                    color: WHITE,
                    font: Some(&font),
                    font_size: 50,
                    ..Default::default()
                },
            );

            if root_ui().button(vec2(w / 2.0 - 50.0, 250.0), "Start Game") {
                game.start();
                macroquad::miniquad::window::schedule_update();
            }
        }

        next_frame().await;
    }
}

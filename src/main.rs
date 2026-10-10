use macroquad::{
    input::KeyCode::{Left, Right},
    miniquad::window::quit,
    prelude::*,
    ui::root_ui,
};

use crate::{
    board::{piece::Piece, style::BoardStyle},
    game::Game,
    render::Render,
};

mod board;
mod config;
mod game;
mod render;

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Intro,
    Paused,
    Turn,
    End,
    Setting,
}

use config::window_conf;
#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new(BoardStyle {
        x: 20.0,
        y: 20.0,
        ..BoardStyle::default()
    });
    game.board.start_default();

    let mut state = State::Intro;

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
            state = match state {
                State::Paused => State::Turn,
                _ => match game.started {
                    true => State::Paused,
                    false => State::Intro,
                },
            }
        }

        if is_key_pressed(KeyCode::P) {
            game.toggle_suggestions();
            println!("{}", game.suggestions)
        }

        if is_mouse_button_pressed(MouseButton::Left) && state == State::Turn {
            game.place(mouse_position());
        }

        if is_key_pressed(Left) && state == State::Turn {
            game.undo();
        }

        if is_key_pressed(Right) && state == State::Turn {
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

        game.board.style.suggest_piece = game.turn;
        game.board.render();

        if state == State::Intro || state == State::Paused {
            // Darkened backgroudnd
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

            draw_text_ex(
                format!("v{}", env!("CARGO_PKG_VERSION")),
                w / 2.0 + measure_text("Othello", Some(&font), 50, 1.0).width / 2.0
                    - measure_text(
                        format!("v{}", env!("CARGO_PKG_VERSION")),
                        Some(&font),
                        20,
                        1.0,
                    )
                    .width,
                220.0,
                TextParams {
                    color: GRAY,
                    font: Some(&font),
                    font_size: 20,
                    ..Default::default()
                },
            );

            match state {
                State::Intro => {
                    if root_ui().button(vec2(w / 2.0 - 50.0, 250.0), "Start Game") {
                        game.start();
                        state = State::Turn;
                        macroquad::miniquad::window::schedule_update()
                    }

                    if root_ui().button(vec2(w / 2.0 - 50.0, 290.0), "Settings") {
                        state = State::Setting;
                        macroquad::miniquad::window::schedule_update();
                    }

                    if root_ui().button(vec2(w / 2.0 - 50.0, 330.0), "Quit Game") {
                        quit();
                    }
                }
                State::Paused => {
                    if root_ui().button(vec2(w / 2.0 - 50.0, 250.0), "Resume Game") {
                        state = State::Turn;
                        macroquad::miniquad::window::schedule_update()
                    }

                    if root_ui().button(vec2(w / 2.0 - 50.0, 290.0), "Restart Game") {
                        game.end();
                        game.start();
                        state = State::Turn;
                        macroquad::miniquad::window::schedule_update()
                    }

                    if root_ui().button(vec2(w / 2.0 - 50.0, 330.0), "Settings") {
                        state = State::Setting;
                        macroquad::miniquad::window::schedule_update();
                    }

                    if root_ui().button(vec2(w / 2.0 - 50.0, 370.0), "Quit Game") {
                        quit();
                    }
                }
                _ => {}
            };
        }

        if state == State::Setting {
            // Dark background
            draw_rectangle(
                0.0,
                0.0,
                w,
                h,
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.5,
                },
            );

            let m = w / 2.0;
            let s = m - 180.0;

            draw_text_ex(
                "Settings",
                m - measure_text("Othello", Some(&font), 50, 1.0).width / 2.0,
                110.0,
                TextParams {
                    color: WHITE,
                    font: Some(&font),
                    font_size: 30,
                    ..Default::default()
                },
            );

            draw_text_f("Info", s, 160.0, 15, &font);

            draw_multiline_text_ex(
                r"Keybinds:
 - Press Ecs to open pause/into menu
 - Press P to toggle suggestions
 - Press Arrow Left to undo
 - Press Arrow Right to redo

 Developed By: Revi Studios
 Github: https://github.com/Revi-Studios/
                ",
                s,
                180.0,
                Some(1.3),
                TextParams {
                    color: GRAY,
                    font: Some(&font),
                    font_size: 13,
                    ..Default::default()
                },
            );
        }

        next_frame().await;
    }
}

fn draw_text_f(text: &str, x: f32, y: f32, font_size: u16, font: &Font) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            color: WHITE,
            font: Some(font),
            font_size: font_size,
            ..Default::default()
        },
    );
}

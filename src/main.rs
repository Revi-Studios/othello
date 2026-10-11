#[cfg(not(target_arch = "wasm32"))]
use macroquad::miniquad::window::quit;

use macroquad::{
    input::KeyCode::{Left, Right},
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

const FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/JosefinSans-Regular.ttf");

use config::window_conf;
#[macroquad::main(window_conf)]
async fn main() {
    let margin = 20.0;

    let mut game = Game::new(BoardStyle {
        x: margin,
        y: margin,
        ..BoardStyle::default()
    });
    game.board.start_default();

    let mut state = State::Intro;

    let board_box = game.board.render_box();

    set_default_font(load_ttf_font_from_bytes(FONT_BYTES).expect("Faild to load font"));

    loop {
        let h = screen_height();
        let w = screen_width();

        clear_background(Color::from_rgba(74, 58, 41, 250));

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

        // Draw Points
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
                m - measure_text(format!("{}", game.black_count), None, 16, 1.0).width / 2.0,
                40.0 + game.board.style.piece_radius * 3.0,
                TextParams {
                    color: WHITE,
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
                m - measure_text(format!("{}", game.black_count), None, 16, 1.0).width / 2.0,
                h - (40.0 + game.board.style.piece_radius * 3.0),
                TextParams {
                    color: WHITE,
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

        //         let scale = (h - margin * 2.0)
        //             .min(w - (margin * 2.0 + game.board.style.piece_radius * 2.0))
        //             / board_box.width;
        //
        //         set_camera(&Camera2D {
        //             zoom: vec2(2.0 / w * scale, 2.0 / h * scale),
        //             target: vec2(w / (2.0 * scale), h / (2.0 * scale)),
        //             ..Default::default()
        //         });
        game.board.render();
        // set_default_camera();

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
                w / 2.0 - measure_text("Othello", None, 50, 1.0).width / 2.0,
                200.0,
                TextParams {
                    color: WHITE,
                    font_size: 50,
                    ..Default::default()
                },
            );

            draw_text_ex(
                format!("v{}", env!("CARGO_PKG_VERSION")),
                w / 2.0 + measure_text("Othello", None, 50, 1.0).width / 2.0
                    - measure_text(format!("v{}", env!("CARGO_PKG_VERSION")), None, 20, 1.0).width,
                220.0,
                TextParams {
                    color: GRAY,
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

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if root_ui().button(vec2(w / 2.0 - 50.0, 330.0), "Quit Game") {
                            quit();
                        }
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

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if root_ui().button(vec2(w / 2.0 - 50.0, 370.0), "Quit Game") {
                            quit();
                        }
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
                m - measure_text("Othello", None, 50, 1.0).width / 2.0,
                110.0,
                TextParams {
                    color: WHITE,
                    font_size: 30,
                    ..Default::default()
                },
            );

            draw_text_f("Info", s, 160.0, 15);

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
                    font_size: 13,
                    ..Default::default()
                },
            );
        }

        next_frame().await;
    }
}

fn draw_text_f(text: &str, x: f32, y: f32, font_size: u16) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            color: WHITE,
            font_size: font_size,
            ..Default::default()
        },
    );
}

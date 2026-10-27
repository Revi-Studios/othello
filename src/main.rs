use std::cmp::Ordering;

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

    let mut state = State::Intro;

    let mut game = Game::new(BoardStyle {
        x: margin,
        y: margin,
        ..BoardStyle::default()
    });

    set_default_font(load_ttf_font_from_bytes(FONT_BYTES).expect("Faild to load font"));
    game.board.start_default();

    loop {
        let h = screen_height();
        let w = screen_width();

        let vertical_mode = w >= h;

        let board_box = game.board.render_box();

        // game.board.style.scale = ((h - margin * 2.0) / game.board.default_render_box().width).min(
        //     (w - margin * 3.0 - game.board.style.piece_radius * 2.0)
        //         / game.board.default_render_box().height,
        // );

        game.board.style.scale = match vertical_mode {
            true => ((w - margin * 3.0 - game.board.style.piece_radius * 2.0)
                / game.board.default_render_box().width)
                .min((h - margin * 2.0) / game.board.default_render_box().height),
            false => ((h - margin * 3.0 - game.board.style.piece_radius * 2.0)
                / game.board.default_render_box().height)
                .min((w - margin * 2.0) / game.board.default_render_box().width),
        };

        clear_background(Color::from_rgba(74, 58, 41, 250));

        match state {
            State::Turn => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    game.place(mouse_position());
                    if game.ended {
                        game.board.style.render_fade = true;
                        state = State::End;
                    }
                }

                if is_key_pressed(KeyCode::Escape) {
                    state = match game.started {
                        true => State::Paused,
                        false => State::Intro,
                    }
                }

                if is_key_pressed(Left) {
                    game.undo();
                }

                if is_key_pressed(Right) {
                    game.redo();
                }
            }
            State::Intro => {
                if is_key_pressed(KeyCode::Enter) {
                    game.start();
                    state = State::Turn;
                }

                #[cfg(not(target_arch = "wasm32"))]
                if is_key_pressed(KeyCode::Escape) {
                    quit();
                }
            }
            State::Paused => {
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Escape) {
                    state = State::Turn;
                }
            }
            State::Setting => {
                if is_key_pressed(KeyCode::Escape) {
                    state = match game.started {
                        true => State::Paused,
                        false => State::Intro,
                    };
                    game.board.style.render_fade = false;
                }
            }
            State::End => {
                if is_key_pressed(KeyCode::Enter)
                    || is_key_pressed(KeyCode::Escape)
                    || is_mouse_button_pressed(MouseButton::Left)
                {
                    game.board.style.render_fade = false;
                    state = State::Intro;
                }
            }
        }

        if is_key_pressed(KeyCode::P) {
            game.toggle_suggestions();
        }

        // Draw Points
        match vertical_mode {
            true => {
                let m = w - (w - (board_box.op_x())) / 2.0;

                // Black side
                draw_poly(
                    m,
                    40.0 + game.board.style.piece_radius,
                    35,
                    game.board.style.piece_radius,
                    0.0,
                    game.board.style.piece_black_color,
                );

                draw_text(
                    format!("{}", game.black_count),
                    m - measure_text(format!("{}", game.black_count), None, 16, 1.0).width / 2.0,
                    40.0 + game.board.style.piece_radius * 3.0,
                    16.0,
                    WHITE,
                );

                // White side
                draw_poly(
                    m,
                    h - (40.0 + game.board.style.piece_radius),
                    35,
                    game.board.style.piece_radius,
                    0.0,
                    game.board.style.piece_white_color,
                );

                draw_text(
                    format!("{}", game.white_count),
                    m - measure_text(format!("{}", game.black_count), None, 16, 1.0).width / 2.0,
                    h - (40.0 + game.board.style.piece_radius * 3.0),
                    16.0,
                    WHITE,
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
            false => {
                let m = h - (h - (board_box.op_y())) / 2.0;

                // Black side
                draw_poly(
                    margin + game.board.style.piece_radius,
                    m,
                    35,
                    game.board.style.piece_radius,
                    0.0,
                    game.board.style.piece_black_color,
                );

                draw_text(
                    format!("{}", game.black_count),
                    margin + game.board.style.piece_radius * 3.0,
                    m - measure_text(format!("{}", game.black_count), None, 16, 1.0).width / 2.0,
                    16.0,
                    WHITE,
                );

                // White side
                draw_poly(
                    w - (margin + game.board.style.piece_radius),
                    m,
                    35,
                    game.board.style.piece_radius,
                    0.0,
                    game.board.style.piece_white_color,
                );

                draw_text(
                    format!("{}", game.white_count),
                    w - (40.0 + game.board.style.piece_radius * 3.0),
                    m - measure_text(format!("{}", game.black_count), None, 16, 1.0).width / 2.0,
                    16.0,
                    WHITE,
                );

                draw_circle(
                    match game.turn {
                        Piece::Black => 80.0 + game.board.style.piece_radius * 3.0,
                        Piece::White => w - (80.0 + game.board.style.piece_radius * 3.0),
                    },
                    m,
                    5.0,
                    WHITE,
                );
            }
        };

        game.board.style.suggest_piece = game.turn;

        // game.board.render();

        game.board.render();

        if state == State::Intro || state == State::Paused {
            background(Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.3,
            });

            // Title
            {
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
                        - measure_text(format!("v{}", env!("CARGO_PKG_VERSION")), None, 20, 1.0)
                            .width,
                    220.0,
                    TextParams {
                        color: GRAY,
                        font_size: 20,
                        ..Default::default()
                    },
                );
            }

            match state {
                State::Intro => {
                    if root_ui().button(vec2(w / 2.0 - 50.0, 250.0), "Start Game") {
                        game.start();
                        state = State::Turn;
                        macroquad::miniquad::window::schedule_update()
                    }

                    if root_ui().button(vec2(w / 2.0 - 50.0, 290.0), "Settings") {
                        state = State::Setting;
                        game.board.style.render_fade = true;
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
                        game.board.style.render_fade = true;
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
            background(Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.5,
            });

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

            draw_text("Info", s, 160.0, 15.0, WHITE);

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

        if state == State::End {
            background(Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.5,
            });

            let text = match game.black_count.cmp(&game.white_count) {
                Ordering::Greater => "Black won",
                Ordering::Less => "White won",
                Ordering::Equal => "Tie",
            };

            let s = w / 2.0;
            let r = game.board.style.piece_radius - 10.0;

            draw_text(
                text,
                w / 2.0 - measure_text(text, None, 30, 1.0).width / 2.0,
                100.0,
                30.0,
                WHITE,
            );

            draw_text("Well played!", s - 50.0, 135.0, 15.0, GRAY);

            let draw_point = |piece: Piece, y: f32| {
                draw_text(
                    piece.to_string() + ":",
                    s - r - 70.0,
                    y + r,
                    18.0,
                    Color {
                        r: 0.9,
                        g: 0.9,
                        b: 0.9,
                        a: 1.0,
                    },
                );

                draw_poly(
                    s,
                    y + r,
                    35,
                    r,
                    0.0,
                    match piece {
                        Piece::Black => game.board.style.piece_black_color,
                        Piece::White => game.board.style.piece_white_color,
                    },
                );

                draw_text(
                    format!(
                        "{}",
                        match piece {
                            Piece::Black => game.black_count,
                            Piece::White => game.white_count,
                        }
                    ),
                    s + r + 30.0,
                    y + r,
                    18.0,
                    WHITE,
                );
            };

            match game.black_count.cmp(&game.white_count) {
                Ordering::Greater | Ordering::Equal => {
                    draw_point(Piece::Black, 200.0);
                    draw_point(Piece::White, 275.0);
                }
                Ordering::Less => {
                    draw_point(Piece::White, 200.0);
                    draw_point(Piece::Black, 275.0);
                }
            };
        }

        next_frame().await;
    }
}

fn background(color: Color) {
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), color);
}

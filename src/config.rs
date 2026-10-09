use macroquad::conf::{Conf, UpdateTrigger};
use macroquad::miniquad;
use macroquad::prelude::KeyCode;

pub fn window_conf() -> Conf {
    Conf {
        miniquad_conf: miniquad::conf::Conf {
            platform: miniquad::conf::Platform {
                blocking_event_loop: true,
                ..Default::default()
            },
            window_title: "Othello".to_string(),
            window_height: 680,
            window_width: 820,
            icon: None,
            high_dpi: true,
            sample_count: 4,
            ..Default::default()
        },

        update_on: Some(UpdateTrigger {
            mouse_down: true,
            touch: true,
            specific_key: Some(vec![KeyCode::Escape, KeyCode::Left, KeyCode::Right]),
            ..Default::default()
        }),

        ..Conf::default()
    }
}

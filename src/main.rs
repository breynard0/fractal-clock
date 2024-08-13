use std::f32::consts::PI;

use chrono::{DateTime, Local, SubsecRound, Timelike};
use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        sample_count: 16,
        window_title: "Fractal Clock".to_string(),
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    loop {
        let time = chrono::Local::now();

        clear_background(BLACK);
        let min_dim = screen_width().min(screen_height());

        draw_clock(0.0, 0.0, min_dim / 6.0,  time, 15, 0.0);

        next_frame().await;
    }
}

fn draw_clock(x: f32, y: f32, r: f32, time: DateTime<Local>, n: i32, offset: f32) {
    let millis = time.timestamp_subsec_millis() as f32;
    let seconds = time.second() as f32;
    let minute = time.minute() as f32;

    // Seconds hand
    let s_theta = (seconds / 60.0 - 0.25) + (millis / (1000.0 * 60.0)) + offset;
    let s = draw_r_theta(x, y, r, s_theta);
    // Recursive ones off the seconds hand
    if n > 0 {
        draw_clock(s.0, s.1, r, time, n - 1, s_theta)
    }

    // Minute hand
    let m_theta = (minute / 60.0) + (seconds / (60.0 * 60.0)) + offset;
    draw_r_theta(x, y, r, m_theta);
}

fn draw_r_theta(x: f32, y: f32, r: f32, theta: f32) -> (f32, f32) {
    let end_pos = (
        (screen_width() / 2.0) + (r * (theta * 2.0 * PI).cos()),
        (screen_height() / 2.0) + (r * (theta * 2.0 * PI).sin()),
    );

    draw_line(x + (screen_width() / 2.0), y + (screen_height() / 2.0), end_pos.0, end_pos.1, 1.0, WHITE);

    end_pos
}

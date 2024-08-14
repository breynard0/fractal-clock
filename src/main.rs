use std::f32::consts::PI;

use chrono::{DateTime, Local, Timelike};
use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        sample_count: 6,
        window_title: "Fractal Clock".to_string(),
        high_dpi: true,
        ..Default::default()
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    thickness: f32,
    color: Color
}

const ITERATIONS: i32 = 12;

#[macroquad::main(conf)]
async fn main() {
    let mut fullscreen = false;

    loop {
        
        let time = chrono::Local::now();
        
        clear_background(BLACK);
        let min_dim = screen_width().min(screen_height());
        
        let mut lines = vec![];
        
        if is_key_pressed(KeyCode::Space) {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
        }

        draw_clock(
            screen_width() / 2.0,
            screen_height() / 2.0,
            min_dim / ITERATIONS as f32,
            time,
            ITERATIONS,
            ITERATIONS,
            0.0,
            &mut lines
        );

        for line in &lines {
            draw_line(line.x0, line.y0, line.x1, line.y1, line.thickness, line.color)
        }

        next_frame().await;
    }
}

fn draw_clock(
    x: f32,
    y: f32,
    r: f32,
    time: DateTime<Local>,
    n: i32,
    m: i32,
    offset: f32,
    lines: &mut Vec<Line>
) {
    let millis = time.timestamp_subsec_millis() as f32;
    let seconds = time.second() as f32;
    let minute = time.minute() as f32;

    let brightness = (1.0 + n as f32) / (1.0 + m as f32);
    let thickness = brightness * 3.0;

    // Seconds hand
    let s_theta = (seconds / 60.0 - 0.25) + (millis / (1000.0 * 60.0)) + offset;
    let s = draw_r_theta(x, y, r, s_theta, brightness, thickness, lines);
    // Recursive ones off the seconds hand
    if n > 0 {
        draw_clock(s.0, s.1, r, time, n - 1, m, s_theta, lines)
    }

    // Minute hand
    let m_theta = ((minute / 60.0) - 0.25)
        + (seconds / (60.0 * 60.0))
        + (millis / (1000.0 * 60.0 * 60.0))
        + offset;
    let z = draw_r_theta(x, y, r, m_theta, brightness, thickness, lines);
    if n > 0 {
        draw_clock(z.0, z.1, r, time, n - 1, m, m_theta, lines)
    }
}

fn draw_r_theta(
    x: f32,
    y: f32,
    r: f32,
    theta: f32,
    brightness: f32,
    thickness: f32,
    lines: &mut Vec<Line>
) -> (f32, f32) {
    let end_pos = (
        x + (r * (theta * 2.0 * PI).cos()),
        y + (r * (theta * 2.0 * PI).sin()),
    );

    // if on_screen((x, y)) || on_screen(end_pos) {
        lines.push(Line {
            x0: x,
            y0: y,
            x1: end_pos.0,
            y1: end_pos.1,
            thickness,
            color: Color::new(1.0, 1.0, 1.0, brightness),
        });
    // }

    end_pos
}
use macroquad::{
    color::{Color, WHITE},
    shapes::{draw_line, draw_rectangle},
};

use crate::constants::{S_COUNT, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn wrap(val: i32, max: i32) -> i32 {
    (val % max + max) % max
}

pub fn draw_board() {
    let horizontal_gap = WINDOW_WIDTH / S_COUNT;

    let vertical_gap = WINDOW_HEIGHT / S_COUNT;

    for i in 0..=S_COUNT {
        let x = i * horizontal_gap;
        let y = i * vertical_gap;
        draw_line(x as f32, WINDOW_HEIGHT as f32, x as f32, 0.0, 2.0, WHITE);
        draw_line(0.0, y as f32, WINDOW_WIDTH as f32, y as f32, 2.0, WHITE);
    }
}

pub fn draw_square(x: i32, y: i32, color: Color) {
    let x = x * (WINDOW_WIDTH / S_COUNT);
    let y = y * (WINDOW_HEIGHT / S_COUNT);

    draw_rectangle(
        x as f32,
        y as f32,
        (WINDOW_WIDTH / S_COUNT) as f32,
        (WINDOW_HEIGHT / S_COUNT) as f32,
        color,
    );
}

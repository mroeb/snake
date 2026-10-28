use macroquad::{
    color::*,
    shapes::{draw_line, draw_rectangle},
};

use super::*;

pub fn wrap(val: i32, max: i32) -> i32 {
    (val % max + max) % max
}

pub fn draw_board() {
    let horizontal_gap = constants::WINDOW_WIDTH / constants::S_COUNT;

    let vertical_gap = constants::WINDOW_HEIGHT / constants::S_COUNT;

    for i in 0..=constants::S_COUNT {
        let x = i * horizontal_gap;
        let y = i * vertical_gap;
        draw_line(
            x as f32,
            constants::WINDOW_HEIGHT as f32,
            x as f32,
            0.0,
            2.0,
            WHITE,
        );
        draw_line(
            0.0,
            y as f32,
            constants::WINDOW_WIDTH as f32,
            y as f32,
            2.0,
            WHITE,
        );
    }
}

pub fn draw_square(x: i32, y: i32, color: Color) {
    let x = x * (constants::WINDOW_WIDTH / constants::S_COUNT);
    let y = y * (constants::WINDOW_HEIGHT / constants::S_COUNT);

    draw_rectangle(
        x as f32,
        y as f32,
        (constants::WINDOW_WIDTH / constants::S_COUNT) as f32,
        (constants::WINDOW_HEIGHT / constants::S_COUNT) as f32,
        color,
    );
}

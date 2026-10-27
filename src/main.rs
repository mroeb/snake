use anyhow::Result;
use macroquad::{
    color::{BLACK, Color, GREEN, RED, WHITE},
    shapes::{draw_line, draw_rectangle},
    window::{Conf, clear_background, next_frame},
};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;
const S_COUNT: i32 = 16;

fn config() -> Conf {
    Conf {
        window_title: "snake".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

fn draw_board() {
    let horizontal_gap = WINDOW_WIDTH / S_COUNT;

    let vertical_gap = WINDOW_HEIGHT / S_COUNT;

    for i in 0..=S_COUNT {
        let x = i * horizontal_gap;
        let y = i * vertical_gap;
        draw_line(x as f32, WINDOW_HEIGHT as f32, x as f32, 0.0, 2.0, WHITE);
        draw_line(0.0, y as f32, WINDOW_WIDTH as f32, y as f32, 2.0, WHITE);
    }
}

fn draw_square(x: i32, y: i32, color: Color) {
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

#[macroquad::main(config)]
async fn main() -> Result<()> {
    loop {
        clear_background(BLACK);

        draw_board();

        draw_square(0, 0, GREEN);
        draw_square(3, 3, RED);

        next_frame().await
    }
}

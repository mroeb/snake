use std::collections::VecDeque;

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

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Snake {
    body: Vec<Pos>,
}

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
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

fn draw_snake(snake: &Snake) {
    for b in snake.body.iter() {
        let color = GREEN;

        draw_square(b.x, b.y, color);
    }
}

fn move_snake(snake: &mut Snake, direction: Orientation) {
    let top = snake.body.pop().unwrap_or(Pos::new(0, 0));

    match direction {
        Orientation::Up => {
            snake.body.push(Pos::new(top.x, top.y - 1));
        }
        Orientation::Down => {
            snake.body.push(Pos::new(top.x, top.y + 1));
        }
        Orientation::Left => {
            snake.body.push(Pos::new(top.x - 1, top.y));
        }
        Orientation::Right => {
            snake.body.push(Pos::new(top.x + 1, top.y));
        }
    }
}

#[macroquad::main(config)]
async fn main() -> Result<()> {
    let mut snake = Snake { body: Vec::new() };
    snake.body.push(Pos::new(3, 3));
    snake.body.push(Pos::new(2, 3));
    snake.body.push(Pos::new(1, 3));

    move_snake(&mut snake, Orientation::Right);
    move_snake(&mut snake, Orientation::Right);
    move_snake(&mut snake, Orientation::Right);

    loop {
        clear_background(BLACK);

        draw_board();

        draw_snake(&snake);

        next_frame().await
    }
}

use std::collections::VecDeque;

use anyhow::Result;
use macroquad::{
    color::{BLACK, Color, GREEN, RED, WHITE},
    input::{KeyCode, get_last_key_pressed, is_key_pressed},
    shapes::{draw_line, draw_rectangle},
    time::get_frame_time,
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
        high_dpi: true,
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
    body: std::collections::VecDeque<Pos>,
}

#[derive(Debug, PartialEq, Eq)]
struct Food {
    pos: Pos,
}

impl Food {
    fn spawn() -> Self {
        Self {
            pos: Pos::new(
                rand::random_range(0..S_COUNT),
                rand::random_range(0..S_COUNT),
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

fn wrap(val: i32, max: i32) -> i32 {
    (val % max + max) % max
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
    for b in snake.body.iter().enumerate() {
        let color = if b.0 == 0 { RED } else { GREEN };

        draw_square(b.1.x, b.1.y, color);
    }
}

fn move_snake(snake: &mut Snake, direction: &Orientation, has_eaten_food: bool) {
    let first = snake.body.front().expect("Body can't be empty");

    let new_head = match direction {
        Orientation::Up => Pos::new(first.x, wrap(first.y - 1, S_COUNT)),
        Orientation::Down => Pos::new(first.x, wrap(first.y + 1, S_COUNT)),
        Orientation::Left => Pos::new(wrap(first.x - 1, S_COUNT), first.y),
        Orientation::Right => Pos::new(wrap(first.x + 1, S_COUNT), first.y),
    };

    snake.body.push_front(new_head);

    if !has_eaten_food {
        snake.body.pop_back();
    }
}

#[macroquad::main(config)]
async fn main() -> Result<()> {
    let mut snake = Snake {
        body: VecDeque::new(),
    };
    snake.body.push_back(Pos::new(3, 3));
    snake.body.push_back(Pos::new(2, 3));
    snake.body.push_back(Pos::new(1, 3));

    let mut active_direction = Orientation::Right;

    for _ in 0..10 {
        move_snake(&mut snake, &active_direction, false);
    }

    let movement_interval = 0.15;
    let mut time_since_last_move = 0.0;

    let mut food = Food::spawn();

    loop {
        time_since_last_move += get_frame_time();

        if time_since_last_move >= movement_interval {
            let has_eaten_food =
                if snake.body.front().expect("Snake body should not be None") == &food.pos {
                    food = Food::spawn();

                    true
                } else {
                    false
                };
            move_snake(&mut snake, &active_direction, has_eaten_food);
            time_since_last_move = 0.0;
        }

        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::Up => {
                    if active_direction != Orientation::Down {
                        active_direction = Orientation::Up;
                    }
                }
                KeyCode::Down => {
                    if active_direction != Orientation::Up {
                        active_direction = Orientation::Down;
                    }
                }
                KeyCode::Left => {
                    if active_direction != Orientation::Right {
                        active_direction = Orientation::Left;
                    }
                }
                KeyCode::Right => {
                    if active_direction != Orientation::Left {
                        active_direction = Orientation::Right;
                    }
                }
                _ => {}
            }
        }

        clear_background(BLACK);

        draw_board();

        draw_square(food.pos.x, food.pos.y, GREEN);

        draw_snake(&snake);
        next_frame().await
    }
}

use crate::{constants, helper, orientation::Orientation, pos::Pos};

#[derive(Debug)]
pub struct Snake {
    pub body: std::collections::VecDeque<super::pos::Pos>,
    pub dir: Orientation,
}

impl Snake {
    pub fn draw(&self) {
        for b in self.body.iter().enumerate() {
            let color = if b.0 == 0 {
                macroquad::color::RED
            } else {
                macroquad::color::GREEN
            };

            helper::draw_square(b.1.x, b.1.y, color);
        }
    }

    pub fn move_next(&mut self, has_eaten_food: bool) {
        let first = self.body.front().expect("Body can't be empty");

        let new_head = match self.dir {
            Orientation::Up => Pos::new(first.x, helper::wrap(first.y - 1, constants::S_COUNT)),
            Orientation::Down => Pos::new(first.x, helper::wrap(first.y + 1, constants::S_COUNT)),
            Orientation::Left => Pos::new(helper::wrap(first.x - 1, constants::S_COUNT), first.y),
            Orientation::Right => Pos::new(helper::wrap(first.x + 1, constants::S_COUNT), first.y),
        };

        self.body.push_front(new_head);

        if !has_eaten_food {
            self.body.pop_back();
        }
    }
}

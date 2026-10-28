use crate::{constants::S_COUNT, pos::Pos};

#[derive(Debug, PartialEq, Eq)]
pub struct Food {
    pub pos: Pos,
}

impl Food {
    pub fn spawn() -> Self {
        Self {
            pos: Pos::new(
                rand::random_range(0..S_COUNT),
                rand::random_range(0..S_COUNT),
            ),
        }
    }
}

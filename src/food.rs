use crate::{constants, pos};

#[derive(Debug, PartialEq, Eq)]
pub struct Food {
    pub pos: super::pos::Pos,
}

impl Food {
    pub fn spawn() -> Self {
        Self {
            pos: pos::Pos::new(
                rand::random_range(0..constants::S_COUNT),
                rand::random_range(0..constants::S_COUNT),
            ),
        }
    }
}

use crate::domain::Id;

pub type UserId = u64;
pub type GameId = u64;

impl Id for u64 {
    fn new() -> u64 {
        0
    }
    fn inc(&mut self) {
        *self += 1;
    }
}


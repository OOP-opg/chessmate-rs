use crate::domain::{Game, Id, Wish};
use std::ops::Not;
use std::str::FromStr;

pub struct TttGame;

// impl TttGame {
// fn new() -> TttGame {
// TttGame {}
// }
// }

impl Game for TttGame {
    type Wish = TttWish;
}

impl Id for u64 {
    fn new() -> u64 {
        0
    }
    fn inc(&mut self) {
        *self += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TttSign {
    Xs,
    Os,
}

#[derive(Debug, Copy, Clone)]
pub struct TttWish {
    sign: TttSign,
}

impl Not for TttSign {
    type Output = TttSign;

    fn not(self) -> Self::Output {
        match self {
            TttSign::Xs => TttSign::Os,
            TttSign::Os => TttSign::Xs,
        }
    }
}

pub enum TttWishErr {
    InvalidWish,
}

impl Wish for TttWish {
    fn is_match(&self, other: &TttWish) -> bool {
        self.sign != other.sign
    }
}

impl FromStr for TttWish {
    type Err = TttWishErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Os" => Ok(TttWish { sign: TttSign::Os }),
            "Xs" => Ok(TttWish { sign: TttSign::Xs }),
            _ => Err(TttWishErr::InvalidWish),
        }
    }
}

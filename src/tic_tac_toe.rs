use crate::domain::{Game, Wish};
use std::ops::Not;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Pane {
    X,
    O,
    Empty,
}

pub struct TttGame {
    state: [Pane; 9] 
}

impl TttGame {
    const fn new() -> TttGame {
        TttGame { state: [Pane::Empty; 9] }
    }
}

impl Game for TttGame {
    type Wish = TttWish;
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

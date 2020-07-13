use std::ops::Not;
use std::str::FromStr;

use crate::common::core::UserId;
use crate::common::domain::{GameCore, /* Users,*/ Wish};

pub struct TttCore;

impl GameCore for TttCore {
    type Wish = TttWish;
    type Users = TttUsers;
}

pub struct TttUsers(pub UserId, pub UserId);
//impl Users for TttUsers {}

#[derive(Clone, Copy)]
pub enum Pane {
    X,
    O,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TttSign {
    Xs,
    Os,
}

#[derive(Debug, Copy, Clone)]
pub struct TttWish {
    pub sign: TttSign,
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

impl Wish for TttWish {}

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

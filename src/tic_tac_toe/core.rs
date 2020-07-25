use std::ops::Not;
use std::str::FromStr;
use std::fmt::{self, Display, Formatter};

use crate::common::core::UserId;
use crate::common::domain::{GameCore, /* Users,*/ Wish};
use crate::common::query_utils::parse_attrs;

pub struct TttCore;

impl GameCore for TttCore {
    type Wish = TttWish;
    type Users = TttUsers;
    type Action = TttAction;
    type ActionResult = TttActionResult;
}

#[derive(Debug)]
pub enum TttAction {
    Surrender,
    Draw,
    ApplyDraw,
    Move(TttMove)
}

impl Display for TttAction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let content = match self {
            Self::Surrender => "surrender".to_owned(),
            Self::Draw => "draw".to_owned(),
            Self::ApplyDraw => "apply_draw".to_owned(),
            Self::Move(ttt_move) => format!("{}", ttt_move),
        };
        write!(f, "{}", content)
    }
}

#[derive(Debug)]
pub struct TttMove { x: u8, y: u8 }

impl Display for TttMove {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{x},{y}", x=self.x, y=self.y)
    }
}


impl FromStr for TttAction {
    type Err = String;

    /// Parse to TttAction string like this:
    /// "move:0,1"
    /// "action:draw"
    /// "action:apply_draw"
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match parse_attrs(src, ':', 2) {
            Ok(attrs) => match attrs[0] {
                "action" => match attrs[1] {
                    "surrender" => Ok(TttAction::Surrender),
                    "draw" => Ok(TttAction::Draw),
                    "apply_draw" => Ok(TttAction::ApplyDraw),
                    _ => Err("invalid_action".to_owned())
                },
                "move" => match parse_attrs(attrs[1], ',', 2) {
                    Ok(pos) => match (pos[0].parse(), pos[1].parse()) {
                        (Ok(x), Ok(y)) => Ok(TttAction::Move( TttMove { x, y } )),
                        _ => Err("invalid_move".to_owned()),
                    }
                    _ => Err("invalid_move".to_owned())
                }
                _ => Err("invalid_action".to_owned())
            },
            Err(_) => Err("invalid_action".to_owned()),
        }
    }
}

#[derive(Debug)]
pub enum TttActionResult {
    ImpossibleAction,
    Action(TttAction),
}

impl Display for TttActionResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let content = match self {
            Self::ImpossibleAction => "impossible_action".to_owned(),
            Self::Action(action) => format!("{}", action).to_owned(),
        };
        write!(f, "{}", content)
    }
}

pub struct TttUsers(pub UserId, pub UserId);
//impl Users for TttUsers {}

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

use crate::domain::{Game, Id, Wish};
use std::ops::Not;
use std::str::FromStr;

pub struct ChessGame;

// impl TttGame {
// fn new() -> TttGame {
// TttGame {}
// }
// }

impl Game for ChessGame {
    type Wish = ChessWish;
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
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Copy, Clone)]
pub struct ChessWish {
    color: Color,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

pub enum ChessWishErr {
    InvalidWish,
}

impl Wish for ChessWish {
    fn is_match(&self, other: &ChessWish) -> bool {
        self.color != other.color
    }
}

impl FromStr for ChessWish {
    type Err = ChessWishErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "white" => Ok(ChessWish { color: Color::White }),
            "black" => Ok(ChessWish { color: Color::Black }),
            _ => Err(ChessWishErr::InvalidWish),
        }
    }
}

use crate::common::core::UserId;
use super::core::{TttMove, TttUsers, TttAction, TttActionResult, TttSign};

#[derive(Clone, Copy)]
enum Pane {
    X,
    O,
    Empty,
}

struct TttBoard {
    board: [Pane; 9]
}

fn get_pane(col: u8, row: u8) -> u8 {
    col * 3 + row
}

impl TttBoard {
    const fn new() -> Self {
        Self {
            board: [Pane::Empty; 9]
        }
    }
    fn make_move(&mut self, ttt_move: &TttMove, sign: TttSign) -> bool {
        let pane = get_pane(ttt_move.row, ttt_move.col);
        match self.board[pane as usize] {
            Pane::Empty => { 
                self.board[pane as usize] = if sign == TttSign::Xs {
                    Pane::X 
                } else {
                    Pane::O
                };
                true
            }
            _ => false
        }
    }
}

pub struct TttEngine {
    players: TttUsers,
    current_player: UserId,
    board: TttBoard,
}

impl TttEngine {
    pub const fn for_users(users: TttUsers) -> Self {
        Self { 
            current_player: users.0,
            players: users,
            board: TttBoard::new(),
        }
    }
    pub fn react(&mut self, user_id: UserId, action: TttAction) -> TttActionResult {
        if user_id != self.current_player {
            return TttActionResult::ImpossibleAction
        }
        let result = match action {
            TttAction::Move(ttt_move) => {
                let sign = if user_id == self.players.first() {
                    TttSign::Xs
                } else {
                    TttSign::Os
                };
                if self.board.make_move(&ttt_move, sign) {
                    TttActionResult::Action(TttAction::Move(ttt_move))
                } else {
                    TttActionResult::ImpossibleAction
                }
            }
            //FIXME: add possibility to win
            //FIXME: handle another action
            _ => unimplemented!(),
        };
        self.current_player = self.players.next(self.current_player);
        
        result
    }
}

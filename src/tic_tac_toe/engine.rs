use super::core::{TttAction, TttActionResult, TttMove, TttSign, TttUsers};
use crate::common::core::UserId;

#[derive(PartialEq, Clone, Copy)]
enum Pane {
    X,
    O,
    Empty,
}

const fn get_pane(col: u8, row: u8) -> u8 {
    col * 3 + row
}

trait Line {
    fn full_check(&self) -> Option<FullMatch>;
}

impl Line for [Pane; 3] {
    // Check that all panes is equal and inform about sign
    fn full_check(&self) -> Option<FullMatch> {
        let panes = self;
        if panes[0] == panes[1] && panes[1] == panes[2] {
            match panes[0] {
                Pane::X => Some(FullMatch::Xs),
                Pane::O => Some(FullMatch::Os),
                Pane::Empty => None,
            }
        } else {
            None
        }
    }
}

struct TttBoard {
    board: [Pane; 9],
}

impl TttBoard {
    const fn new() -> Self {
        Self {
            board: [Pane::Empty; 9],
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
            _ => false,
        }
    }

    fn full_board(&self) -> bool {
        let mut counter = 0;
        for pane in &self.board {
            match pane {
                Pane::X | Pane::O => counter += 1,
                Pane::Empty => {}
            }
        }

        if counter == 9 {
            true
        } else {
            false
        }
    }

    // poor man fmap for panes (because arr.iter().map().collect() is broken)
    // choose - function to get pane index by iteration number
    fn extract_triple(&self, choose: impl Fn(u8) -> u8) -> [Pane; 3] {
        let mut panes = [Pane::Empty; 3];
        for n in &[0, 1, 2] {
            panes[*n as usize] = self.board[choose(*n) as usize];
        }

        panes
    }

    fn detect_full_row(&self, row: u8) -> Option<FullMatch> {
        let panes = self.extract_triple(|col| get_pane(col, row));

        panes.full_check()
    }

    fn detect_full_column(&self, col: u8) -> Option<FullMatch> {
        let panes = self.extract_triple(|row| get_pane(col, row));

        panes.full_check()
    }

    fn detect_full_diagonal(&self, diagonal: Diagonal) -> Option<FullMatch> {
        let make_line = match diagonal {
            Diagonal::TopRightLeftBottom => |i| get_pane(i, i),
            Diagonal::BottomRightLeftTop => |i| get_pane(i, 2 - i),
        };
        let panes = self.extract_triple(make_line);

        panes.full_check()
    }

    fn detect_final(&self) -> Option<FinalResult> {
        let on_match = |matching| match matching {
            FullMatch::Xs => Some(FinalResult::XsWin),
            FullMatch::Os => Some(FinalResult::OsWin),
        };

        // try to find full row or column three times
        for move_try in &[0, 1, 2] {
            for method in &[Self::detect_full_row, Self::detect_full_column] {
                if let Some(matching) = method(self, *move_try) {
                    return on_match(matching);
                }
            }
        }

        // try to find full match on diagonal
        for diagonal in
            &[Diagonal::TopRightLeftBottom, Diagonal::BottomRightLeftTop]
        {
            if let Some(matching) = self.detect_full_diagonal(*diagonal) {
                return on_match(matching);
            }
        }

        // maybe it's over?
        if self.full_board() {
            return Some(FinalResult::Draw);
        } else {
            return None;
        }
    }
}

enum FinalResult {
    XsWin,
    OsWin,
    Draw,
}

enum FullMatch {
    Xs,
    Os,
}

#[derive(Copy, Clone)]
enum Diagonal {
    TopRightLeftBottom,
    BottomRightLeftTop,
}

enum TttEvent {
    WantDraw(UserId),
    Silence,
}

pub struct TttEngine {
    players: TttUsers,
    current_player: UserId,
    board: TttBoard,
    event: TttEvent,
}

impl TttEngine {
    pub const fn for_users(users: TttUsers) -> Self {
        Self {
            current_player: users.0,
            players: users,
            board: TttBoard::new(),
            event: TttEvent::Silence,
        }
    }

    pub fn react(
        &mut self,
        user_id: UserId,
        action: TttAction,
    ) -> TttActionResult {
        if user_id != self.current_player {
            return TttActionResult::ImpossibleAction;
        }
        let result = match action {
            TttAction::Move(ttt_move) => self.handle_move(user_id, ttt_move),
            TttAction::Surrender => self.surrender(user_id),
            TttAction::ProposeDraw => self.propose_draw(user_id),
            TttAction::ApplyDraw => self.apply_draw(user_id),
        };
        self.current_player = self.players.next(self.current_player);

        result
    }

    fn handle_move(
        &mut self,
        user_id: UserId,
        ttt_move: TttMove,
    ) -> TttActionResult {
        let sign = if user_id == self.players.first() {
            TttSign::Xs
        } else {
            TttSign::Os
        };
        if self.board.make_move(&ttt_move, sign) {
            if let Some(final_result) = self.board.detect_final() {
                // pass result of the game
                match final_result {
                    FinalResult::XsWin => {
                        TttActionResult::Win(self.players.first())
                    }
                    FinalResult::OsWin => {
                        TttActionResult::Win(self.players.second())
                    }
                    FinalResult::Draw => TttActionResult::Draw,
                }
            } else {
                // pass action to other
                TttActionResult::Action(TttAction::Move(ttt_move))
            }
        } else {
            TttActionResult::ImpossibleAction
        }
    }

    fn surrender(&self, user_id: UserId) -> TttActionResult {
        let winner = self.players.next(user_id);

        TttActionResult::Win(winner)
    }

    fn propose_draw(&mut self, user_id: UserId) -> TttActionResult {
        self.event = TttEvent::WantDraw(user_id);

        TttActionResult::Action(TttAction::ProposeDraw)
    }

    fn apply_draw(&self, user_id: UserId) -> TttActionResult {
        if let TttEvent::WantDraw(proposing) = self.event {
            if user_id == self.players.next(proposing) {
                TttActionResult::Draw
            } else {
                TttActionResult::ImpossibleAction
            }
        } else {
            TttActionResult::ImpossibleAction
        }
    }
}

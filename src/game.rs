use crate::engine;
pub type Replay = String;
pub type UserGameState = String;
pub use crate::engine::Color;
pub use crate::engine::Move;
use crate::gamepool::UserId;

pub enum UserActionType{
    Move,
    OfferDraw
}
pub struct UserAction{
    user: UserId,
    action_type: UserActionType,
    game_move: Option<Move>,
    draw_from: Option<UserId>,
}


pub struct Game {
    board_state: engine::BoardState,
    user1: UserId,
    user2: UserId,

}

pub enum DoActionError {
    WrongTurn,
    InvalidAction,

    InvalidUser,

}

impl Game {
    /// Provides user-format representation of game state
    /// # Returns
    /// * `UserGameState`

    pub fn get_state(&self) -> UserGameState {
        self.board_state.export_to_fen()

    }

    /// Performs game action
    /// # Arguments
    /// * `action` - user-format action
    /// # Returns
    /// * `true` - action is correct and ends the game
    /// * `false` - action is correct and doesn't end the game
    /// * `DoActionError` - action is incorrect see enum description for details
    pub fn do_action(&self, action: UserAction) -> Result<bool, DoActionError> {

        match action.action_type{
        UserActionType::Move => {
            if action.user != self.user1 && action.user != self.user2 {return Err(DoActionError::InvalidUser)};
            let mut cur_color: Color = if action.user == self.user1{
                Color::White
            }else{
                Color::Black
            };
            self.board_state.validate_move(action.game_move.unwrap(), cur_color);
        },
        UserActionType::OfferDraw => {
            return Ok(false)
        }
        }
        Ok(false)
    }

    pub fn new(user1: UserId, user2: UserId) -> Game {
        Game {
            board_state: engine::BoardState::new(),
            user1: user1,
            user2: user2
        }
    }
}

pub struct Ticket {
    side: Color,
}

impl Ticket {
    pub fn new(side: Color) -> Ticket{
        Ticket {
            side: side,
        }
    }
}


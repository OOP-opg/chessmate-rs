use crate::engine;

pub type UserAction = String;
pub type Replay = String;
pub type UserGameState = String;

pub struct Game {
    board_state: engine::B
}

pub enum DoActionError {
    WrongTurn,
    InvalidAction,
}

impl Game {
    /// Provides user-format representation of game state
    /// # Returns
    /// * `UserGameState`
    pub fn get_state(&self) -> &UserGameState {
        &self.state
    }

    /// Performs game action
    /// # Arguments
    /// * `action` - user-format action
    /// # Returns
    /// * `true` - action is correct and ends the game
    /// * `false` - action is correct and doesn't end the game
    /// * `DoActionError` - action is incorrect see enum description for details
    pub fn do_action(&self, action: UserAction) -> Result<bool, DoActionError> {
        print!("Action happened: {}", action);
        Ok(false)
    }

    pub fn new() -> Game {
        Game {
            state: "".to_owned(),
        }
    }
}

pub struct Ticket {
    side: String,
}

impl Ticket {
    pub fn new(/*TODO: add any parameters*/) -> Ticket{
        Ticket {
            
        }
    }
}

pub struct Engine {}

impl Engine { }

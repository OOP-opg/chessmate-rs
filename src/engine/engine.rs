pub type UserAction = String;
pub type Replay = String;
pub type UserGameState = String;

pub struct Game {
    state: String,
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
    /// * `bool` - true if action is correct and was applied to the game, false if incorrent and should be discarded
    pub fn do_action(&self, action: UserAction) -> Result<bool, DoActionError> {
        print!("Action happened: {}", action);
        Ok(true)
    }

    pub fn new() -> Game {
        Game {
            state: "".to_owned(),
        }
    }
}

pub struct Ticket {
    wish: String,
}

pub struct Engine {}

impl Engine {
    /// Determines whether ticket is valid or not
    /// # Arguments
    /// * `ticket` - ticket to check
    /// # Returns
    /// * `bool`
    fn is_ticket_valid(ticket: Ticket) -> bool {
        ticket.wish.len() > 0
    }
}

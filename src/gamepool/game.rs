use gamepool::ITicket;

pub trait IGameState {
    fn serialize(&self) -> String;
}

pub trait IGameAction {}

pub enum GameActionError {
    InvalidAction
}

pub trait IGameReplay {}

pub trait IGame {
    pub fn new() -> IGame;
    pub fn is_ticket_valid(ticket: ITicket) -> bool { 
        ticket.is_valid()
    }
    pub fn get_state() -> IGameState;
    pub fn do_action(action: IGameAction) -> Option<GameActionError>;
}

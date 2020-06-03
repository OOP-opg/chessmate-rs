mod server;
mod game;

use server::IUser;
use game::{IGame, IGameReplay, IGameAction};

pub trait IGameInfo {};

pub trait ITicket {
    pub fn is_valid(&self) -> bool;
}

pub trait ITicketStats {};

pub enum TicketError {
    TooMany,
    InvalidTicket,
    HaveTicket,
    AlreadyPlaying,
    NoTicket
}

pub enum GamePoolError {
    NotPlaying
}

pub trait IGamePool {
    pub fn is_playing(user: IUser) -> Option<IGameInfo>;
    pub fn have_ticket(user: IUser) -> Result<Option<ITicket>, TicketError::AlreadyPlaying>;
    pub fn get_tickets_stats() -> ITicketStats;
    pub fn add_ticket(user: IUser, ticket: ITicket) -> Option<TicketError>;
    pub fn remove_ticket(user: IUser) -> Option<TicketError::NoTicket>;
    pub fn change_ticket(user: IUser, ticket: ITicket) -> Option<TicketError>;
    pub fn do_game_action(user: IUser, IGameAction: IGameAction) -> Option<GamePoolError, TicketError>;
    pub fn end_game(game: IGame, replay: IGameReplay);
}
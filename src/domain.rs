use std::fmt::Debug;
use std::marker::Unpin;
use std::str::FromStr;

use crate::core::{GameId, UserId};

pub trait Wish: FromStr + Debug + Unpin + Clone + Send + 'static {
    fn is_match(&self, other: &Self) -> bool;
}

pub trait Id {
    fn new() -> Self;
    fn inc(&mut self);
}

pub trait GamePool<G: Game> {
    type Observers;
    fn new_game(&mut self, game_id: GameId, game: G, oberservers: Observer);
    fn handle_game_action(&mut self, game_id: GameId, action: Action, user_id: UserId);
}

pub trait Action: FromStr {
}

pub enum HandleActionError {
    WrongTurn,
    InvalidAction,
    InvalidUser,
}

    
pub trait Game {
    type Wish: Wish;
    type Action: Action;
    type Users;
    fn new(users: Users);
    fn handle_action(&mut self, action: Action, user_id: UserId) -> Result<(), HandleActionError>;
}

pub enum SetTicketError {
    DuplicateTicket,
}

pub trait AbstractLobby<W, O>: Unpin + 'static
where
    W: Wish,
    O: PairObserver,
{
    fn new() -> Self;
    fn add_ticket(
        &mut self,
        user: UserId,
        wish: W,
        observer: O,
    ) -> Result<(), SetTicketError>;
}

pub trait AbstractGamePool<G> {
}

pub trait PairObserver: Unpin + 'static {
    fn notify(&self, game: GameId);
}

pub trait PairReactor {
    fn wait(&self) -> Option<GameId>;
}

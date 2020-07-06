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

pub trait Game {
    type Wish: Wish;
}

pub enum SetTicketError {
    DuplicateTicket,
}

pub trait AbstractLobby<W, O>: Unpin + 'static
where
    I: Info,
    W: Wish,
    O: PairObserver,
{
    fn new() -> Self;
    fn add_ticket(
        &mut self,
        user: UserId,
        info: I,
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

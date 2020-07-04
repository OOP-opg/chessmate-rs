use std::fmt::Debug;
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

pub trait AbstractLobby<W, O>
where
    W: Wish,
    O: PairObserver,
{
    fn add_ticket(
        &mut self,
        user: UserId,
        wish: W,
        observer: O,
    ) -> Result<(), SetTicketError>;
}

pub trait PairObserver {
    fn notify(&self, game: GameId);
}

pub trait PairReactor {
    fn wait(&self) -> Option<GameId>;
}

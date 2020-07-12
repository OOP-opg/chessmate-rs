use super::core::{UserId, GameId};

use std::str::FromStr;

pub trait Id {
    fn new() -> Self;
    fn inc(&mut self);
}

pub trait Game<O: Observers>: Unpin + 'static {
    type Lobby: Lobby<Self::Wish, O::GameObserver> + Unpin + 'static;
    type Wish: Wish + Send + 'static;
}

pub trait Observers {
    type GameObserver: GameObserver;
}

pub trait GameObserver: Unpin + 'static {
    fn notify(&self, game_id: GameId);
}

pub trait Wish: FromStr {}

pub trait Lobby<W: Wish, O: GameObserver>: Default {
    fn add_ticket(
        &mut self,
        user_id: UserId,
        wish: W,
        observer: O,
    );
}

pub trait Action: FromStr {}

pub trait GamePool<A: Action>: Default {
    fn do_game_action(
        &mut self,
        game_id: GameId,
        user_id: UserId,
        action: A,
    );
}

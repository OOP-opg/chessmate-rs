use super::core::UserId;
use super::messages::NewGame;

use actix::Recipient;

use std::str::FromStr;

pub trait Id {
    fn new() -> Self;
    fn inc(&mut self);
}

pub trait Game: Unpin + 'static {
    type Lobby: Lobby<Self::Wish> + Unpin + 'static;
    //type GamePool;
    type Wish: Wish + Send + 'static;
    //type Action;
}

pub trait Wish: FromStr {}

pub trait Lobby<W: Wish>: Default {
    fn add_ticket(
        &mut self,
        user_id: UserId,
        wish: W,
        observer: Recipient<NewGame>,
    );
}

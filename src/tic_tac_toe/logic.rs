use crate::common::domain::{Game, GameObserver};
use std::marker::PhantomData;
use super::lobby::TttLobby;
use super::game::TttWish;

pub struct TttGame<O: GameObserver> {
    observer: PhantomData<O>,
}

impl<O: GameObserver> Game<O> for TttGame<O> {
    type Lobby = TttLobby<O>;
    type Wish = TttWish;
    //type GameObserver = TttGameObserver;
}

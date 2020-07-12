use crate::common::domain::{GameLogic, GameObserver};
use crate::common::communication::ActorObservers;
use std::marker::PhantomData;
use super::core::TttCore;
use super::lobby::{TttLobby, TttActorObservers};


pub struct TttGameLogic /* <O: GameObserver> */ {
    /* observer: PhantomData<O>, */
}

impl /* <O: GameObserver> */ GameLogic<TttCore, ActorObservers<TttCore>> for TttGameLogic {
    type Lobby = TttLobby;
    //type GameObserver = TttGameObserver;
}

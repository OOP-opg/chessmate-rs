use std::collections::HashMap;
use std::marker::PhantomData;

use super::engine::TttEngine;
use super::core::{TttCore, TttUsers, TttAction, TttActionResult};

use crate::common::core::{GameId, UserId};
use crate::common::domain::{GamePool, Observers, StartGameObserver, GameMoveObserver};

pub struct TttGamePool<O>
    where O: Observers<TttCore>,
          O::GameMoveObserver: GameMoveObserver<TttActionResult>,
          O::StartGameObserver: StartGameObserver<TttUsers> {
    games: HashMap<GameId, TttEngine>,
    observers: PhantomData<O>,
}

impl<O> Default for TttGamePool<O> 
    where O: Observers<TttCore>,
          O::GameMoveObserver: GameMoveObserver<TttActionResult>,
          O::StartGameObserver: StartGameObserver<TttUsers> {
    fn default() -> Self {
        Self {
            games: HashMap::new(),
            observers: PhantomData,
        }
    }
}

impl<O> GamePool<TttCore, O> for TttGamePool<O> 
    where O: Observers<TttCore>,
          O::GameMoveObserver: GameMoveObserver<TttActionResult>,
          O::StartGameObserver: StartGameObserver<TttUsers> {

    fn new_game(&mut self, game_id: GameId, users: TttUsers) {
        //TODO: implement new game
    }

    fn join_game(&mut self, game_id: GameId, user_id: UserId, observer: O::GameMoveObserver) {
        //TODO: implement registering user
    }

    fn do_game_action(&mut self, game_id: GameId, user_id: UserId, action: TttAction) {
        //TODO: implement handling action
    }
}

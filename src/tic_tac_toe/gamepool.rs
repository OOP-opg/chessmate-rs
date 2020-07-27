use std::collections::HashMap;

use super::core::{TttAction, TttActionResult, TttCore, TttUsers};
use super::engine::TttEngine;

use crate::common::core::{GameId, UserId};
use crate::common::domain::{
    GameMoveObserver, GamePool, Observers, StartGameObserver,
};

struct GameState<MO>
where
    MO: GameMoveObserver<TttActionResult>,
{
    is_ready: bool,
    engine: TttEngine,
    users: TttUsers,
    observers: HashMap<UserId, MO>,
}

impl<MO> GameState<MO>
where
    MO: GameMoveObserver<TttActionResult> + Sized,
{
    fn waiting(users: TttUsers) -> Self {
        Self {
            is_ready: false,
            engine: TttEngine::for_users(users.clone()),
            users,
            observers: HashMap::new(),
        }
    }

    fn update(&mut self) {
        if self.observers.values().count() == 2 {
            self.is_ready = true;
        }
    }
}

pub struct TttGamePool<O>
where
    O: Observers<TttCore>,
    O::GameMoveObserver: GameMoveObserver<TttActionResult>,
    O::StartGameObserver: StartGameObserver<TttUsers>,
{
    games: HashMap<GameId, GameState<O::GameMoveObserver>>,
}

impl<O> Default for TttGamePool<O>
where
    O: Observers<TttCore>,
    O::GameMoveObserver: GameMoveObserver<TttActionResult>,
    O::StartGameObserver: StartGameObserver<TttUsers>,
{
    fn default() -> Self {
        Self {
            games: HashMap::new(),
        }
    }
}

impl<O> GamePool<TttCore, O> for TttGamePool<O>
where
    O: Observers<TttCore>,
    O::GameMoveObserver: GameMoveObserver<TttActionResult>,
    O::StartGameObserver: StartGameObserver<TttUsers>,
{
    /// Register game and wait for players
    fn new_game(&mut self, game_id: GameId, users: TttUsers) {
        self.games.insert(game_id, GameState::waiting(users));
    }

    /// Register player
    fn enter_game(
        &mut self,
        game_id: GameId,
        user_id: UserId,
        observer: O::GameMoveObserver,
    ) {
        if let Some(game_state) = self.games.get_mut(&game_id) {
            if !game_state.users.contains(user_id) {
                //TODO: handle case if user is not in game users
                return;
            }
            if game_state.is_ready {
                //TODO: handle case if game is ready
                return;
            }
            if let None = game_state.observers.get(&user_id) {
                game_state.observers.insert(user_id, observer);
                game_state.update();
            } else {
                //TODO: handle case if observer is already registered, maybe just replace?
            }
        } else {
            //TODO: handle case if game is not registered
            log::error!(
                "attempt to call enter_game on game that is not registered"
            )
        }
    }

    fn do_game_action(
        &mut self,
        game_id: GameId,
        user_id: UserId,
        action: TttAction,
    ) {
        if let Some(game) = self.games.get_mut(&game_id) {
            //FIXME: notify observers
            let _ = game.engine.react(user_id, action);
        } else {
            //TODO: handle case if game is not registered
        }
    }
}

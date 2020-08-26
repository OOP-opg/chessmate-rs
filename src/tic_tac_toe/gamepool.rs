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
    engine: TttEngine,
    users: TttUsers,
    observers: [Option<MO>; 2],
}

impl<MO> GameState<MO>
where
    MO: GameMoveObserver<TttActionResult> + Sized,
{
    fn waiting(users: TttUsers) -> Self {
        Self {
            engine: TttEngine::for_users(users.clone()),
            users,
            observers: [None, None],
        }
    }

    fn add_user(&mut self, user_id: UserId, observer: MO) {
        if !self.users.contains(user_id) {
            log::error!(
                "attempt to call enter_game with user_id not from lobby"
            );
            return;
        }
        let user_index = if user_id == self.users.first() {
            0
        } else {
            1
        };
        if let None = self.observers[user_index] {
            self.observers[user_index] = Some(observer);
        } else {
            log::error!(
                "attempt to call enter_game with user already registered"
            )
        }
    }

    // if ready, notify all observers
    // if not, do nothing
    fn on_update(&self, game_id: GameId) {
        if self.observers.iter().all(Option::is_some) {
            self.observers.iter()
                .for_each(|o| if let Some(observer) = o {observer.start_fight(game_id)})
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
            game_state.add_user(user_id, observer);
            game_state.on_update(game_id);
        } else {
            // handle case if game is not registered
            log::error!(
                "attempt to call enter_game on game that is not registered"
            );
        }
    }

    fn do_game_action(
        &mut self,
        game_id: GameId,
        user_id: UserId,
        action: TttAction,
    ) {
        if let Some(game) = self.games.get_mut(&game_id) {
            if !game.observers.iter().all(Option::is_some) {
                log::error!("do_game_action called when game is not ready");
                return;
            }
            let action_result = game.engine.react(user_id, action);
            for observer in &game.observers {
                if let Some(observer) = observer {
                    observer.result_action(user_id, game_id, action_result)
                }
            }
        } else {
            //handle case if game is not registered
            log::error!(
                "attempt to call do_game_action on game that is not registered"
            );
        }
    }
}

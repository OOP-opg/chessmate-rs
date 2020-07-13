use super::core::{GameId, UserId};

use std::str::FromStr;

pub trait Id {
    fn new() -> Self;
    fn inc(&mut self);
}

pub trait GameCore: 'static {
    type Wish: Wish + Send + 'static;
    type Users: Send /*: Users */ ;
//    type Action: Send;
}

pub trait GameLogic<C: GameCore, O: Observers<C>>: Unpin + 'static {
    type Lobby: Lobby<C, O> + Unpin + 'static;
}

pub trait Observers<C: GameCore> {
    type GameObserver: GameObserver;
    type StartGameObserver: StartGameObserver<C::Users>;
}

pub trait StartGameObserver<US /*: Users*/ >: Unpin {
    fn start_game(&self, game_id: GameId, users: US);
}

//pub trait Users: Send {}

pub trait GameObserver: Unpin + 'static {
    fn notify(&self, game_id: GameId);
}

pub trait Wish: FromStr {}

pub trait Lobby<C: GameCore, O: Observers<C>> {
    fn with_communication(observer: O::StartGameObserver) -> Self;
    fn add_ticket(&mut self, user_id: UserId, wish: C::Wish, observer: O::GameObserver);
}

//pub trait Action: FromStr {}

pub trait GamePool<C: GameCore, O: Observers<C>>: Default {
    fn with_communication(observer: O::StartGameObserver) -> Self;
//    fn do_game_action(&mut self, game_id: GameId, user_id: UserId, action: C::Action);
}

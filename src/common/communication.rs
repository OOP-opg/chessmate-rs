use super::core::GameId;
use super::domain::{GameCore, GameObserver, Observers, StartGameObserver, /*Users*/};
use super::messages::{NewGame, StartGame};
use actix::Recipient;
use std::marker::PhantomData;

pub struct ActorObservers<GC: GameCore> {
    core: PhantomData<GC>,
}

impl<GC: GameCore> Observers<GC> for ActorObservers<GC> {
    type GameObserver = ActorGameObserver;
    type StartGameObserver = ActorStartGameObserver<GC::Users>;
}

pub struct ActorGameObserver(Recipient<NewGame>);

impl GameObserver for ActorGameObserver {
    fn notify(&self, game_id: GameId) {
        if let Err(e) = self.0.do_send(NewGame(game_id)) {
            log::error!("Error with notify about fined game game {}", e);
        }
    }
}

impl From<Recipient<NewGame>> for ActorGameObserver {
    fn from(src: Recipient<NewGame>) -> Self {
        ActorGameObserver(src)
    }
}

pub struct ActorStartGameObserver<US: Send /*: Users*/ >(Recipient<StartGame<US>>);

impl<US: Send /*: Users */> StartGameObserver<US> for ActorStartGameObserver<US> {
    fn start_game(&self, game_id: GameId, users: US) {
        let start_game_msg = StartGame { game_id, users };
        if let Err(e) = self.0.do_send(start_game_msg) {
            log::error!("Error with notify about game {}", e);
        }
    }
}

impl<US: Send /*: Users */> From<Recipient<StartGame<US>>> for ActorStartGameObserver<US> {
    fn from(src: Recipient<StartGame<US>>) -> Self {
        ActorStartGameObserver(src)
    }
}

use actix::Recipient;
use super::domain::GameObserver;
use super::core::GameId;
use super::messages::NewGame;

pub struct ActorGameObserver(Recipient<NewGame>);

impl From<Recipient<NewGame>> for ActorGameObserver {
    fn from(r: Recipient<NewGame>) -> Self {
        ActorGameObserver(r)
    }
}

impl GameObserver for ActorGameObserver {
    fn notify(&self, game_id: GameId) {
        self.0.do_send(NewGame(game_id));
    }
}

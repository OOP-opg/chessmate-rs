use actix::Recipient;

use crate::common::core::GameId;
use crate::common::messages::NewGame;
use crate::common::domain::GameObserver;

pub struct TttGameObserver(Recipient<NewGame>);

impl From<Recipient<NewGame>> for TttGameObserver {
    fn from(r: Recipient<NewGame>) -> Self {
        TttGameObserver(r)
    }
}

impl GameObserver for TttGameObserver {
    fn notify(&self, game_id: GameId) {
        self.0.do_send(NewGame(game_id));
    }
}

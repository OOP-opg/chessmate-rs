use super::domain::{Game, Lobby};
use super::communication::ActorGameObserver;
use super::messages::FindPair;

use actix::{Actor, Handler, Context};

pub struct GameServer<G: Game<ActorGameObserver>> {
    lobby: G::Lobby,
}

impl<G: Game<ActorGameObserver>> Default for  GameServer<G> {
    fn default() -> Self {
        GameServer {
            lobby: G::Lobby::default(),
        }
    }
}

impl<G: Game<ActorGameObserver>> Actor for GameServer<G> {
    type Context = Context<Self>;
}

impl<G: Game<ActorGameObserver>> Handler<FindPair<G::Wish>> for GameServer<G> {
    type Result = ();
    fn handle(&mut self, msg: FindPair<G::Wish>, _: &mut Context<Self>) {
        let observer = msg.addr.into();
        self.lobby.add_ticket(msg.user_id, msg.wish, observer);
    }
}


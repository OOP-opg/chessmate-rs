use super::domain::{Game, Lobby};
use super::messages::FindPair;

use actix::{Actor, Handler, Context};

pub struct GameServer<G: Game> {
    lobby: G::Lobby,
}

impl<G: Game> Default for  GameServer<G> {
    fn default() -> Self {
        GameServer {
            lobby: G::Lobby::default(),
        }
    }
}

impl<G: Game> Actor for GameServer<G> {
    type Context = Context<Self>;
}

impl<G: Game> Handler<FindPair<G::Wish>> for GameServer<G> {
    type Result = ();
    fn handle(&mut self, msg: FindPair<G::Wish>, _: &mut Context<Self>) {
        let observer = msg.addr.into();
        self.lobby.add_ticket(msg.user_id, msg.wish, observer);
    }
}


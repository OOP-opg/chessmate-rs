use crate::domain::{AbstractLobby, Wish};
use crate::lobby::Lobby;
use crate::observers::{FindPair, TicketObserver};

use actix::{Actor, Context, Handler};

//TODO: implement running gamepool

pub struct GameServer<W: Wish> {
    lobby: Lobby<W>,
}

impl<W: Wish> Default for GameServer<W> {
    fn default() -> GameServer<W> {
        GameServer {
            lobby: Lobby::<W>::new(),
        }
    }
}

impl<W: Wish> Handler<FindPair<W>> for GameServer<W> {
    type Result = ();
    fn handle(&mut self, msg: FindPair<W>, _: &mut Context<Self>) {
        let observer = TicketObserver { feedback: msg.addr };
        self.lobby.add_ticket(msg.user_id, msg.wish, observer);
    }
}

impl<W: Wish> Actor for GameServer<W> {
    type Context = Context<Self>;
}

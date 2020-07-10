use crate::domain::{AbstractLobby, Wish, PairObserver};
use crate::lobby::Lobby;
use crate::observers::{FindPair, TicketObserver};

use actix::{Actor, Context, Handler};

use std::marker::PhantomData;

//TODO: implement running gamepool

pub struct GameServer<W, L: AbstractLobby<W, TicketObserver>>
where W: Wish, {
    lobby: L,
    wish: PhantomData<W>,
}

impl<W, L> Default for GameServer<W, L>
where W: Wish,
      L: AbstractLobby<W, TicketObserver> {
    fn default() -> Self {
        GameServer {
            lobby: L::new(),
            wish: PhantomData,
        }
    }
}

impl<W, L> Handler<FindPair<W>> for GameServer<W, L> 
where W: Wish,
      L: AbstractLobby<W, TicketObserver> {
    type Result = ();
    fn handle(&mut self, msg: FindPair<W>, _: &mut Context<Self>) {
        let observer = TicketObserver { feedback: msg.addr };
        self.lobby.add_ticket(msg.user_id, msg.wish, observer);
    }
}

impl<W, L> Actor for GameServer<W, L> 
where W: Wish,
      L: AbstractLobby<W, TicketObserver> {
    type Context = Context<Self>;
}

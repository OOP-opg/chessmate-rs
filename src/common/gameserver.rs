use super::communication::ActorObservers;
use super::domain::{GameCore, GameLogic, Lobby};
use super::messages::FindPair;

use actix::{Actor, Context, Handler};

pub struct GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    lobby: GL::Lobby,
}

impl<GC, GL> Default for GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    fn default() -> Self {
        GameServer {
            lobby: GL::Lobby::default(),
        }
    }
}

impl<GC, GL> Actor for GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    type Context = Context<Self>;
}

impl<GC, GL> Handler<FindPair<GC::Wish>> for GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    type Result = ();
    fn handle(&mut self, msg: FindPair<GC::Wish>, _: &mut Context<Self>) {
        let observer = msg.addr.into();
        self.lobby.add_ticket(msg.user_id, msg.wish, observer);
    }
}

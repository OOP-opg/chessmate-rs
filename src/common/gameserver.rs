use super::communication::ActorObservers;
use super::domain::{GameCore, GameLogic, Lobby};
use super::messages::{FindPair, StartGame};

use actix::{Actor, AsyncContext, Context, Handler};

pub struct GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    lobby: Option<GL::Lobby>,
    gamepool: GL::GamePool,
}

impl<GC, GL> Default for GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    fn default() -> Self {
        GameServer {
            lobby: None,
            gamepool: GL::GamePool::default(),
        }
    }
}

impl<GC, GL> Actor for GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let observer = ctx.address().recipient().into();
        self.lobby.replace(GL::Lobby::with_communication(observer));
    }
}

impl<GC, GL> Handler<FindPair<GC::Wish>> for GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    type Result = ();
    fn handle(&mut self, msg: FindPair<GC::Wish>, _: &mut Context<Self>) {
        let observer = msg.addr.into();
        if let Some(lobby) = &mut self.lobby {
            lobby.add_ticket(msg.user_id, msg.wish, observer);
        } else {
            unreachable!();
        }
    }
}

impl<GC, GL> Handler<StartGame<GC::Users>> for GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    type Result = ();
    fn handle(&mut self, msg: StartGame<GC::Users>, _: &mut Context<Self>) {
        log::info!("New game {}", msg.game_id);
    }
}

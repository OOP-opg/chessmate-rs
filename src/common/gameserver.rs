use super::communication::ActorObservers;
use super::domain::{GameCore, GameLogic, Lobby};
use super::messages::{DoAction, FindPair, JoinToGame, StartGame};

use actix::{Actor, AsyncContext, Context, Handler};

pub struct GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    lobby: Option<GL::Lobby>,
    gamepool: GL::GamePool,
}

impl<GC, GL> GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    fn find_pair(&mut self, msg: FindPair<GC::Wish>) {
        let observer = msg.addr.into();
        if let Some(lobby) = &mut self.lobby {
            lobby.add_ticket(msg.user_id, msg.wish, observer);
        } else {
            unreachable!("Lobby must be initialized at this moment");
        }
    }

    fn start_game(&mut self, msg: StartGame<GC::Users>) {
        //TODO: starting game on event from lobby
        log::debug!("New game {}", msg.game_id);
        log::error!("UNIMPLEMENTED");
    }

    fn join_to_game(&mut self, msg: JoinToGame<GC::ActionResult>) {
        //TODO: join user to game on event from frontend
        log::debug!("User {} wants to join to {}", msg.user_id, msg.game_id);
        log::error!("UNIMPLEMENTED");
    }

    fn do_action(&mut self, msg: DoAction<GC::Action>) {
        //TODO: send player action to gamepool
        log::debug!("User {} wants do smth in {}", msg.user_id, msg.game_id);
        log::error!("UNIMPLEMENTED");
    }
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
        // set lobby with communication channel
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
        self.find_pair(msg);
    }
}

impl<GC, GL> Handler<StartGame<GC::Users>> for GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    type Result = ();
    fn handle(&mut self, msg: StartGame<GC::Users>, _: &mut Context<Self>) {
        self.start_game(msg);
    }
}

impl<GC, GL> Handler<JoinToGame<GC::ActionResult>> for GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    type Result = ();
    fn handle(&mut self, msg: JoinToGame<GC::ActionResult>, _: &mut Context<Self>) {
        self.join_to_game(msg);
    }
}

impl<GC, GL> Handler<DoAction<GC::Action>> for GameServer<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    type Result = ();
    fn handle(&mut self, msg: DoAction<GC::Action>, _: &mut Context<Self>) {
        self.do_action(msg);
    }
}

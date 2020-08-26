use super::communication::{ActorObservers, ActorGameMoveObserver};
use super::domain::{GameCore, GameLogic, Lobby, GamePool};
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
        let StartGame {game_id, users } = msg;
        log::debug!("New game: {}", game_id);
        self.gamepool.new_game(game_id, users);
    }

    fn join_to_game(&mut self, msg: JoinToGame<GC::ActionResult>) {
        let JoinToGame {
            user_id,
            game_id,
            action_recipient,
            game_recipient,
        } = msg;
        let move_observer = ActorGameMoveObserver {action_recipient, game_recipient};
        log::debug!("User {} wants to join to {}", msg.user_id, msg.game_id);
        self.gamepool.enter_game(game_id, user_id, move_observer);
    }

    fn do_action(&mut self, msg: DoAction<GC::Action>) {
        let DoAction { 
            game_id,
            user_id,
            action,
        } = msg; 
        log::debug!("User {} wants do {:?} in {}", msg.user_id, action, msg.game_id);
        self.gamepool.do_game_action(game_id, user_id, action);
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
    fn handle(
        &mut self,
        msg: JoinToGame<GC::ActionResult>,
        _: &mut Context<Self>,
    ) {
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

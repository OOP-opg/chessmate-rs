use std::fmt::{self, Display};

use actix::prelude::StreamHandler;
use actix::{Actor, Addr, AsyncContext, Handler};
use actix_web::web::{Data, Path, Payload};
use actix_web::{Error, HttpRequest, HttpResponse, ResponseError};
use actix_web_actors::ws;

use super::communication::ActorObservers;
use super::core::UserId;
use super::domain::{GameCore, GameLogic};
use super::gameserver::GameServer;
use super::messages::{DoAction, ActionOutcome, FindPair, NewGame, JoinToGame};
use super::query_utils::{parse_query, parse_attrs};

struct WsPlayerSession<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    server: Addr<GameServer<GC, GL>>,
    user_id: UserId,
}

impl<GC, GL> WsPlayerSession<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    fn find_pair(&self, wish: &str, ctx: &mut ws::WebsocketContext<Self>) {
        if let Ok(wish) = wish.parse() {
            let pair_request = FindPair {
                user_id: self.user_id,
                wish,
                addr: ctx.address().recipient(),
            };
            self.server.do_send(pair_request);
        }
        //TODO: handle invalid wish parsing
    }

    fn deliver_new_game(&self, msg: NewGame, ctx: &mut ws::WebsocketContext<Self>) {
        let game_id = msg.0;
        ctx.text(format!("/event/new_game/{}", game_id));
    }

    fn join_game(&self, game_id: &str, ctx: &mut ws::WebsocketContext<Self>) {
        log::debug!("Client wants to join to {}", game_id);
        if let Ok(game_id) = game_id.parse() {
            let join_game_request = JoinToGame {
                user_id: self.user_id,
                game_id,
                addr: ctx.address().recipient(),
            };
            self.server.do_send(join_game_request);
        }
        //TODO: handle invalid game_id parsing
    }

    fn make_action(&self, attrs: &str) {
        match parse_attrs(attrs, ':', 2) {
            Ok(attrs) => {
                let game_id = if let Ok(game_id) = attrs[0].parse() {
                    game_id
                } else {
                    //TODO: handle invalid game_id parsing
                    return
                };
                let action = if let Ok(action) = attrs[1].parse() {
                    action
                } else {
                    //TODO: handle invalid game_id parsing
                    return
                };
                let do_action = DoAction {
                    action,
                    user_id: self.user_id,
                    game_id,
                };
                self.server.do_send(do_action);
            },
            //TODO: handle different errors
            Err(_) => log::error!("Error during parsing attrs to action"),
        };
    }

    /// Notifies frontend about what's going on in the game
    fn deliver_action_outcome(
        &self,
        result: ActionOutcome<GC::ActionResult>,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let ActionOutcome { user_id, game_id, result } = result;
        log::debug!("GamePool responds with {:?}", result);
        ctx.text(format!("/event/action/{}/{}/{}", game_id, user_id, result));
    }
}

impl<GC, GL> Actor for WsPlayerSession<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    type Context = ws::WebsocketContext<Self>;
}

impl<GC, GL> Handler<NewGame> for WsPlayerSession<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    type Result = ();

    fn handle(&mut self, msg: NewGame, ctx: &mut ws::WebsocketContext<Self>) {
        self.deliver_new_game(msg, ctx);
    }
}

impl<GC, GL> Handler<ActionOutcome<GC::ActionResult>> for WsPlayerSession<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    type Result = ();

    fn handle(
        &mut self,
        msg: ActionOutcome<GC::ActionResult>,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        self.deliver_action_outcome(msg, ctx);
    }
}

impl<GC, GL> StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsPlayerSession<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        log::info!("websocket Message: {:?}", msg);

        if let Ok(message) = msg {
            if let ws::Message::Text(txt) = message {
                match parse_query(&txt) {
                    //TODO: decompose
                    Ok((cmd, attrs)) => match cmd {
                        "/find" => self.find_pair(attrs, ctx),
                        "/join" => self.join_game(attrs, ctx),
                        "/action" => self.make_action(attrs),
                        _ => ctx.text("/error/undefined_command"),
                    },
                    Err(e) => ctx.text(format!("/error/invalid_query:{:?}", e)),
                }
            } else {
                ctx.text("/error/unimplemented_transport_format");
            }
        } else {
            log::error!("Error during getting message from websocket");
        }
    }
}

#[derive(Debug)]
pub enum ReqError {
    WebSocketError(Error),
}

impl Display for ReqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ReqError {}

pub async fn new_session<GC, GL>(
    req: HttpRequest,
    stream: Payload,
    info: Path<UserId>,
    server: Data<Addr<GameServer<GC, GL>>>,
) -> Result<HttpResponse, ReqError>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    log::info!("Request: {:?}", info);

    let user_id = info.into_inner();
    let session = WsPlayerSession::<GC, GL> {
        server: server.get_ref().clone(),
        user_id,
    };
    ws::start(session, &req, stream).map_err(ReqError::WebSocketError)
}

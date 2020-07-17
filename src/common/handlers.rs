use std::fmt::{self, Display};

use actix::prelude::StreamHandler;
use actix::{Actor, Addr, AsyncContext, Handler};
use actix_web::web::{Data, Path, Payload};
use actix_web::{Error, HttpRequest, HttpResponse, ResponseError};
use actix_web_actors::ws;

use super::communication::ActorObservers;
use super::core::{UserId};
use super::domain::{GameCore, GameLogic};
use super::gameserver::GameServer;
use super::messages::{FindPair, NewGame};

struct WsPlayerSession<GC, GL>
where
    GC: GameCore,
    GL: GameLogic<GC, ActorObservers<GC>>,
{
    server: Addr<GameServer<GC, GL>>,
    user_id: UserId,
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
        let game_id = msg.0;
        ctx.text(format!("{}", game_id));
    }
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
    }

    fn join_game(&self, game_id: &str, _ctx: &mut ws::WebsocketContext<Self>) {
        log::debug!("Client wants to join to {}", game_id);
        log::error!("UNIMPLEMENTED");
    }

    fn make_action(&self, action: &str, _ctx: &mut ws::WebsocketContext<Self>) {
        //TODO: implement playing game
        log::debug!("Client wants to do {}", action);
        log::error!("UNIMPLEMENTED");
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
                let mut args = txt.splitn(2, '?');
                let cmd = args.next().unwrap();
                let attrs = args.next().unwrap();
                match cmd {
                    "/find" => self.find_pair(attrs, ctx),
                    "/action" => self.make_action(attrs, ctx),
                    "/join" => self.join_game(attrs, ctx),
                    _ => ctx.text("Henlo"),
                }
            } else {
                ctx.text("What are you doing");
            }
        } else {
            unimplemented!();
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

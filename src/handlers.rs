use std::fmt::{self, Display};

use actix::prelude::StreamHandler;
use actix::{Actor, Addr, AsyncContext, Handler};
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use actix_web_actors::ws;

use crate::core::UserId;
use crate::domain::{Game, Wish};
use crate::observers::{
    FindPair,
    NewGame,
};
use crate::runtime::GameServer;

struct WsPlayerSession<W: Wish> {
    server: Addr<GameServer<W>>,
    user_id: UserId,
    wish: W,
}

impl<W: Wish> Handler<NewGame> for WsPlayerSession<W> {
    type Result = ();

    fn handle(&mut self, msg: NewGame, ctx: &mut ws::WebsocketContext<Self>) {
        let game_id = msg.0;
        ctx.text(format!("{}", game_id));
    }
}

impl<W: Wish> WsPlayerSession<W> {
    fn find_pair(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let pair_request = FindPair {
            user_id: self.user_id,
            wish: self.wish.clone(),
            addr: ctx.address().recipient(),
        };
        self.server.do_send(pair_request);
    }
}

impl<W: Wish> Actor for WsPlayerSession<W> {
    type Context = ws::WebsocketContext<Self>;
}

impl<W: Wish> StreamHandler<Result<ws::Message, ws::ProtocolError>>
    for WsPlayerSession<W>
{
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        log::info!("websocket Message: {:?}", msg);
        match msg {
            Ok(message) => match message {
                ws::Message::Text(txt) => match txt.as_str() {
                    "/find" => self.find_pair(ctx),
                    //TODO: implement playing game
                    _ => ctx.text("Henlo"),
                },
                _ => ctx.text("What are you doing"),
            },
            Err(_) => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum ReqError {
    InvalidWish,
}

impl Display for ReqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ReqError {}

pub async fn new_game<G: Game>(
    req: HttpRequest,
    stream: web::Payload,
    info: web::Path<(UserId, String)>,
    server: web::Data<Addr<GameServer<G::Wish>>>,
) -> Result<HttpResponse, ReqError> {
    log::info!("request: {:?}", info);

    let user_id = info.0;
    //TODO: fuck this error handling, aaaah
    let wish = info.1.parse().map_err(|_| ReqError::InvalidWish)?;
    let session = WsPlayerSession {
        server: server.get_ref().clone(),
        user_id,
        wish,
    };
    ws::start(session, &req, stream).map_err(|_| ReqError::InvalidWish)
}

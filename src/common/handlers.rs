use std::fmt::{self, Display};

use actix::prelude::StreamHandler;
use actix::{Actor, Addr, AsyncContext, Handler};
use actix_web::{HttpRequest, HttpResponse, ResponseError, Error};
use actix_web::web::{Payload, Path, Data};
use actix_web_actors::ws;

use super::core::UserId;
use super::domain::Game;
use super::gameserver::GameServer;
use super::messages::{NewGame, FindPair};

struct WsPlayerSession<G: Game> {
    server: Addr<GameServer<G>>,
    user_id: UserId,
}

impl<G: Game> Actor for WsPlayerSession<G> {
    type Context = ws::WebsocketContext<Self>;
}

impl<G: Game> Handler<NewGame> for WsPlayerSession<G> {
    type Result = ();

    fn handle(&mut self, msg: NewGame, ctx: &mut ws::WebsocketContext<Self>) {
        let game_id = msg.0;
        ctx.text(format!("{}", game_id));
    }
}

impl<G: Game> WsPlayerSession<G> {
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
}


impl<G: Game> StreamHandler<Result<ws::Message, ws::ProtocolError>>
    for WsPlayerSession<G>
{
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {

        log::info!("websocket Message: {:?}", msg);

        if let Ok(message) = msg {
            match message {
                ws::Message::Text(txt) => {
                    let mut args = txt.splitn(2, "?"); 
                    let cmd = args.next().unwrap();
                    let attrs = args.next().unwrap(); 
                    match cmd {
                        "/find" => self.find_pair(attrs, ctx),
                        //TODO: implement playing game
                        _ => ctx.text("Henlo"),
                    }
                },
                _ => ctx.text("What are you doing"),
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

pub async fn new_session<G: Game>(
    req: HttpRequest,
    stream: Payload,
    info: Path<UserId>,
    server: Data<Addr<GameServer<G>>>,
) -> Result<HttpResponse, ReqError> {
    log::info!("Request: {:?}", info);

    let user_id = info.into_inner();
    let session = WsPlayerSession::<G> {
        server: server.get_ref().clone(),
        user_id,
    };
    ws::start(session, &req, stream).map_err(|e| ReqError::WebSocketError(e))
}

use std::fmt::{self, Display};
use std::marker::PhantomData;

use actix::prelude::StreamHandler;
use actix::{Actor, Addr, AsyncContext, Handler};
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use actix_web_actors::ws;

use crate::core::UserId;
use crate::lobby::Lobby;
use crate::domain::{Game, Wish};
use crate::observers::{
    TicketObserver,
    FindPair,
    NewGame,
};
use crate::runtime::GameServer;

struct WsPlayerSession<W: Wish> {
    server: Addr<GameServer<W, Lobby<W>>>,
    user_id: UserId,
    wish: PhantomData<W>,
}

impl<W: Wish> Handler<NewGame> for WsPlayerSession<W> {
    type Result = ();

    fn handle(&mut self, msg: NewGame, ctx: &mut ws::WebsocketContext<Self>) {
        let game_id = msg.0;
        ctx.text(format!("{}", game_id));
    }
}

impl<W: Wish> WsPlayerSession<W> {
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
    InvalidWish,
}

impl Display for ReqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ReqError {}

pub async fn new_session<G: Game>(
    req: HttpRequest,
    stream: web::Payload,
    info: web::Path<UserId>,
    server: web::Data<Addr<GameServer<G::Wish, Lobby<G::Wish>>>>,
) -> Result<HttpResponse, ReqError> {
    log::info!("request: {:?}", info);

    let user_id = info.into_inner();
    //TODO: fuck this error handling, aaaah
    let session = WsPlayerSession {
        server: server.get_ref().clone(),
        user_id,
        wish: PhantomData,
    };
    ws::start(session, &req, stream).map_err(|_| ReqError::InvalidWish)
}

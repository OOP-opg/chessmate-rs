use actix_files as fs;
use actix_web::{
    web,
    App,
    HttpServer,
};
use crossbeam_channel::unbounded;

mod engine;
mod game;
mod gamepool;
mod server;

use server::handlers::{new_game, index};
use server::pairing::{pairing_loop, Paired};

use crate::game::Color;
//use crate::game::Move;
use crate::gamepool::GameId;
use crate::gamepool::UserId;

type Id = u32;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let (sender_events, receiver_events) = unbounded::<(Color, UserId)>();
    let (sender_pairing, receiver_pairing) = unbounded::<Paired>();
    let front_events = web::Data::new(sender_events);
    let pairing_sender = web::Data::new(sender_pairing.clone());
    let pairing_events = web::Data::new(receiver_pairing);

    pairing_loop(receiver_events.clone(), sender_pairing.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(front_events.clone())
            .app_data(pairing_events.clone())
            .app_data(pairing_sender.clone())
            .service(index)
            .service(new_game)
            .service(fs::Files::new("/static", "./static"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

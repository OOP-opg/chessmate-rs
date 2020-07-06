use actix::Addr;
use actix_files as fs;
use actix_files::NamedFile;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};

use crate::handlers::new_session;
use crate::runtime::GameServer;

use crate::chess::{ChessGame, ChessWish};

use crate::lobby::Lobby;
use crate::observers::TicketObserver;

pub async fn run_server(
    listener: Addr<GameServer<ChessWish, Lobby<ChessWish>>>,
) -> std::io::Result<()> {
    env_logger::init();

    log::info!("starting server");
    let game_listener = web::Data::new(listener);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(game_listener.clone())
            .service(index)
            .service(
                web::resource("/api/chess/new_session/{user_id}")
                    .to(new_session::<ChessGame>),
            )
            .service(fs::Files::new("/static", "./static"))
    })
    .workers(1 as usize)
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

#[get("/")]
pub async fn index() -> impl Responder {
    log::info!("index");
    NamedFile::open("./index.html")
}

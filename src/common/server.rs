use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix::Actor;

use super::domain::Game;
use super::gameserver::GameServer;
use super::handlers::new_session;

pub async fn run_server<G: Game>(
    game_name: &'static str,
) -> std::io::Result<()> {
    env_logger::init();

    let game_server = GameServer::<G>::default().start();
    log::info!("starting server");
    let session_route = format!("/api/{game}/new_session/{{user_id}}", game=game_name); 
    HttpServer::new(move || {
        App::new()
            .data(game_server.clone())
            .wrap(Logger::default())
            .service(
                web::resource(&session_route)
                    .to(new_session::<G>),
            )
    })
    .workers(1 as usize)
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

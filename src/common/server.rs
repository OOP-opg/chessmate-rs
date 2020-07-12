use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix::Actor;

use super::domain::{GameCore, GameLogic};
use super::gameserver::GameServer;
use super::communication::ActorObservers;
use super::handlers::new_session;

pub async fn run_server<GC, GL>(
    game_name: &'static str,
) -> std::io::Result<()> 
where GC: GameCore,
      GL: GameLogic<GC, ActorObservers<GC>>{
    env_logger::init();

    let game_server = GameServer::<GC, GL>::default().start();
    log::info!("starting server");
    let session_route = format!("/api/{game}/new_session/{{user_id}}", game=game_name); 
    HttpServer::new(move || {
        App::new()
            .data(game_server.clone())
            .wrap(Logger::default())
            .service(
                web::resource(&session_route)
                    .to(new_session::<GC, GL>),
            )
    })
    .workers(1 as usize)
    .bind("127.0.0.1:5000")?
    .run()
    .await
}

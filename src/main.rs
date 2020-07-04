mod core;
mod domain;
mod handlers;
mod lobby;
mod observers;
mod runtime;
mod server;
mod chess;

use actix::Actor;
use chess::ChessWish;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let game_server = runtime::GameServer::<ChessWish>::default().start();

    server::run_server(game_server).await
}

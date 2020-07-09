mod core;
mod domain;
mod handlers;
mod lobby;
mod observers;
mod runtime;
mod server;
mod tic_tac_toe;
mod chess;

use actix::Actor;
use lobby::Lobby;
use tic_tac_toe::TttWish;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let game_server = runtime::GameServer::<TttWish, Lobby<TttWish>>::default().start();

    server::run_server(game_server).await
}

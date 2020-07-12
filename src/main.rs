mod tic_tac_toe;
mod common;

use tic_tac_toe::logic::TttGame;
use common::communication::ActorGameObserver;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
   common::server::run_server::<TttGame<ActorGameObserver>>("tic_tac_toe").await
}

mod tic_tac_toe;
mod common;

use tic_tac_toe::logic::TttGame;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
   common::server::run_server::<TttGame>("tic_tac_toe").await
}

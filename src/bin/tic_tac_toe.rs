use chessmate::tic_tac_toe::core::TttCore;
use chessmate::tic_tac_toe::logic::TttGameLogic;
use chessmate::common::server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server::run_server::<TttCore, TttGameLogic>("tic_tac_toe").await
}

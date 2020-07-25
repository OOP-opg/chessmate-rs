pub mod common;
mod tic_tac_toe;

use tic_tac_toe::core::TttCore;
use tic_tac_toe::logic::TttGameLogic;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
//    common::server::run_server::<TttCore, TttGameLogic>("tic_tac_toe").await
    Ok(())
}

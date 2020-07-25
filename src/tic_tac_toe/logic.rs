//use crate::common::communication::ActorObservers;
use crate::common::domain::{GameLogic, Observers, StartGameObserver, GameMoveObserver};

use super::core::{TttCore, TttUsers, TttActionResult};
use super::lobby::TttLobby;
use super::gamepool::TttGamePool;

pub struct TttGameLogic /* <O: GameObserver> */ {/* observer: PhantomData<O>, */}

impl<O> GameLogic<TttCore, O> for TttGameLogic 
    where O: Observers<TttCore> + 'static,
          O::GameMoveObserver: GameMoveObserver<TttActionResult>,
          O::StartGameObserver: StartGameObserver<TttUsers> {
    type Lobby = TttLobby<O>;
    type GamePool = TttGamePool<O>;
    //type GameObserver = TttGameObserver;
}

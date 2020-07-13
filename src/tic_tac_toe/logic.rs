use crate::common::communication::ActorObservers;
use crate::common::domain::{GameLogic, Observers, StartGameObserver};

use super::core::{TttCore, TttUsers};
use super::lobby::TttLobby;

pub struct TttGameLogic /* <O: GameObserver> */ {/* observer: PhantomData<O>, */}

impl<O: Observers<TttCore> + 'static> GameLogic<TttCore, O> for TttGameLogic 
    where O::StartGameObserver: StartGameObserver<TttUsers> {
    type Lobby = TttLobby<O>;
    //type GameObserver = TttGameObserver;
}


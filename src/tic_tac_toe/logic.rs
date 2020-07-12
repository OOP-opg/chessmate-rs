use crate::common::communication::ActorObservers;
use crate::common::domain::GameLogic;

use super::core::TttCore;
use super::lobby::TttLobby;

pub struct TttGameLogic /* <O: GameObserver> */ {/* observer: PhantomData<O>, */}

impl GameLogic<TttCore, ActorObservers<TttCore>> for TttGameLogic {
    type Lobby = TttLobby;
    //type GameObserver = TttGameObserver;
}

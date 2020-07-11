use crate::common::domain::Game;
use super::lobby::TttLobby;
use super::game::TttWish;
use super::communication::TttGameObserver;

pub struct TttGame;

impl Game for TttGame {
    type Lobby = TttLobby;
    type Wish = TttWish;
    type GameObserver = TttGameObserver;
}

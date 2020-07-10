use crate::common::domain::Game;
use super::lobby::TttLobby;
use super::game::TttWish;

pub struct TttGame;

impl Game for TttGame {
    type Lobby = TttLobby;
    type Wish = TttWish;
}

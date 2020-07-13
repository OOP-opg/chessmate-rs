use super::core::{GameId, UserId};
use super::domain::{Users, Wish};
use actix::{Message, Recipient};

#[derive(Message)]
#[rtype(result = "()")]
/*
 * Message returned from Lobby to Session actor, to notify about new game
 */
pub struct NewGame(pub GameId);

#[derive(Message)]
#[rtype(result = "()")]
/*
 * Message from Session actor to Lobby about ask to find new game
 */
pub struct FindPair<W: Wish> {
    pub user_id: UserId,
    pub wish: W,
    pub addr: Recipient<NewGame>,
}

#[derive(Message)]
#[rtype(result = "()")]
/*
 * Message from Lobby to GameServer actor, for starting new game
 */
pub struct StartGame<US /*: Users*/ > {
    pub users: US,
    pub game_id: GameId,
}

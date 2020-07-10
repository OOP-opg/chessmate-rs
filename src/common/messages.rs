use actix::{Message, Recipient};
use super::core::{GameId, UserId};
use super::domain::Wish;

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

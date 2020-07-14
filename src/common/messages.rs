use super::core::{GameId, UserId};
use super::domain::Wish;
use actix::{Message, Recipient};


/*
 * Message returned from Lobby to Session actor, to notify about new game
 */
#[derive(Message)]
#[rtype(result = "()")]
pub struct NewGame(pub GameId);


/*
 * Message from Session actor to Lobby (via GameServer) about ask to find new game
 */
#[derive(Message)]
#[rtype(result = "()")]
pub struct FindPair<W: Wish> {
    pub user_id: UserId,
    pub wish: W,
    pub addr: Recipient<NewGame>,
}


/*
 * Message from GamePool to Session actor about co-players move or invalid player move
 */
#[derive(Message)]
#[rtype(result = "()")]
pub struct ActionOutcome<R>(pub R);


/*
 * Message from Lobby to GameServer actor, for starting new game
 */
#[derive(Message)]
#[rtype(result = "()")]
pub struct StartGame<US /*: Users*/> {
    pub users: US,
    pub game_id: GameId,
}

/*
 * Message from Session to GamePool (via GameServer) about join to game
 */
//TODO: join message

/*
 * Message from Session to GamePool (via GameServer) about make action
 */
//TODO: action message 

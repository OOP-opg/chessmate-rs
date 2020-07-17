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
 * - user_id and game_id for identify user and game to join to
 * - addr specifies recipient which will be recieve ActionOutcome messages
 */
#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinToGame<R: Send> {
    pub user_id: UserId,
    pub game_id: GameId,
    pub addr: Recipient<ActionOutcome<R>>,
}

/*
 * Message from Session to GamePool (via GameServer) about make action
 */
#[derive(Message)]
#[rtype(result = "()")]
pub struct DoAction<A> {
    pub user_id: UserId,
    pub game_id: GameId,
    pub action: A,
}

/*
 * Message from GamePool to Session actor about co-players move or invalid player move
 * parametrised over MoveResult
 */
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ActionOutcome<R>(pub R);

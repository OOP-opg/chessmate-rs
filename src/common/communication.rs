use super::core::{GameId, UserId};
use super::domain::{
    GameCore, GameMoveObserver, GameObserver, Observers,
    StartGameObserver, /*Users*/
};
use super::messages::{ActionOutcome, NewGame, StartGame};
use actix::Recipient;
use std::marker::PhantomData;

pub struct ActorObservers<GC: GameCore> {
    core: PhantomData<GC>,
}

impl<GC: GameCore> Observers<GC> for ActorObservers<GC> {
    type GameObserver = ActorGameObserver;
    type GameMoveObserver = ActorGameMoveObserver<GC::ActionResult>;
    type StartGameObserver = ActorStartGameObserver<GC::Users>;
}

/*
 * Implementation for GameObserver with actix actors
 * Used by Lobby to notify player about available game
 *
 * ActorGameObserver is the structure with one field - recipient of messages
 * @notify(game_id) sends NewGame message to recipient
 * Also we are implementing From<Recipient> for better ergonomics
 */
pub struct ActorGameObserver(Recipient<NewGame>);

impl GameObserver for ActorGameObserver {
    fn notify(&self, game_id: GameId) {
        if let Err(e) = self.0.do_send(NewGame(game_id)) {
            log::error!("Error with notify about fined game game {}", e);
        }
    }
}

impl From<Recipient<NewGame>> for ActorGameObserver {
    fn from(src: Recipient<NewGame>) -> Self {
        ActorGameObserver(src)
    }
}

/*
 * Implementation for GameMoveObserver with actix actors
 * Used by GamePool to notify player about their own action or co-players action
 *
 * ActorGameMoveObserver is the structure with one field - recipient of messages
 * @result_action(game_id) sends ActionOutcome message to recipient
 * Also we are implementing From<Recipient> for better ergonomics
 */
pub struct ActorGameMoveObserver<R: Send + ToString>(
    Recipient<ActionOutcome<R>>,
);

impl<R: Send + ToString> GameMoveObserver<R> for ActorGameMoveObserver<R> {
    fn result_action(&self, user_id: UserId, game_id: GameId, result: R) {
        if let Err(e) = self.0.do_send(ActionOutcome {
            user_id,
            game_id,
            result,
        }) {
            log::error!("Error with send result of player action: {}", e);
        }
    }
}

/*
 * Implementation for StartGameObserver with actix actors
 * Used by Lobby to notify GameServer about new game
 *
 * ActorStartGameObserver is the structure with one field - recipient of messages
 * @start_game(game_id) sends StartGame message to recipient notifying about new game
 * Also we are implementing From<Recipient> for better ergonomics
 */
pub struct ActorStartGameObserver<US: Send /*: Users*/>(
    Recipient<StartGame<US>>,
);

impl<US: Send /*: Users */> StartGameObserver<US>
    for ActorStartGameObserver<US>
{
    fn start_game(&self, game_id: GameId, users: US) {
        let start_game_msg = StartGame { game_id, users };
        if let Err(e) = self.0.do_send(start_game_msg) {
            log::error!("Error with notify about game {}", e);
        }
    }
}

impl<US: Send /*: Users */> From<Recipient<StartGame<US>>>
    for ActorStartGameObserver<US>
{
    fn from(src: Recipient<StartGame<US>>) -> Self {
        ActorStartGameObserver(src)
    }
}

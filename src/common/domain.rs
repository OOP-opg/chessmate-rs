use super::core::{GameId, UserId};

use std::fmt::{Debug, Display};
use std::str::FromStr;

pub trait Id {
    fn new() -> Self;
    fn inc(&mut self);
}

pub trait GameCore: 'static {
    type Wish: Wish + Send + 'static;
    type Users: Send;
    type Action: Send + FromStr;
    type ActionResult: Send + Display + Debug;
}

pub trait Wish: FromStr {}

pub trait GameLogic<C: GameCore, O: Observers<C>>: Unpin + 'static {
    type Lobby: Lobby<C, O> + Unpin + 'static;
    type GamePool: GamePool<C, O> + Unpin + 'static;
}

/*
 * Meta trait for Observers
 */
pub trait Observers<C: GameCore> {
    type GameObserver: GameObserver;
    type GameMoveObserver: GameMoveObserver<C::ActionResult>;
    type StartGameObserver: StartGameObserver<C::Users>;
}

pub trait GameMoveObserver<R>: Unpin {
    fn result_action(&self, user: UserId, game: GameId, result: R);
    fn start_fight(&self, game: GameId);
}

pub trait GameObserver: Unpin + 'static {
    fn notify(&self, game_id: GameId);
}

pub trait StartGameObserver<US /*: Users*/ >: Unpin {
    fn start_game(&self, game_id: GameId, users: US);
}

/*
 * Lobby
 */
pub trait Lobby<C: GameCore, O: Observers<C>> {
    // in: gameserver -> lobby
    // out: @constructor
    //
    /// Create lobby and register observer to which send message about new game
    fn with_communication(observer: O::StartGameObserver) -> Self;

    // in: @wish => websocket -> gameserver -> lobby
    // out: @start_game => lobby -> gameserver
    // out: @new_game => lobby -> websocket
    //
    /// Register observer to which send message about available game
    fn add_ticket(
        &mut self,
        user_id: UserId,
        wish: C::Wish,
        observer: O::GameObserver,
    );
    /*
     * function to join game for existing ticket (without setting new ticket)
     * fn join_game(&mut self, user_id: UserId, owner_id: UserId, observer: O::GameObserver);
     */
}

/*
 * GamePool
 */
pub trait GamePool<C: GameCore, O: Observers<C>>: Default {
    // in: @start_game => lobby -> gameserver -> gamepool
    // out: nothing
    //
    /// Create new engine
    fn new_game(&mut self, game_id: GameId, users: C::Users);

    // in: @join_game => websocket -> gameserver -> gamepool
    // out: @fight => gamepool -> all registered websockets
    /// Register observer for game
    fn enter_game(
        &mut self,
        game_id: GameId,
        user_id: UserId,
        observer: O::GameMoveObserver,
    );

    // in: @action => websocket -> gameserver -> gamepool
    // out: @action_outcome => gamepool -> websocket (or websockets)
    //
    /// Send action to engine and notify all players about new action or return error
    /// to sender
    fn do_game_action(
        &mut self,
        game_id: GameId,
        user_id: UserId,
        action: C::Action,
    );
}

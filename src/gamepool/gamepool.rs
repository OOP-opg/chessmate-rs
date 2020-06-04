use crate::game;

use std::collections::HashMap;

pub type UserId = i32;
pub type GameId = i32;
pub type GameUsers = [UserId; 2];

pub struct GameInfo {
    game: engine::Game,
    pub users: GameUsers,
}

pub struct GamePool {
    playing_users: HashMap<UserId, GameId>,
    games: HashMap<GameId, GameInfo>,
    tickets: HashMap<UserId, engine::Ticket>,
}

/// Enum of errors that might occur when trying to add a ticket
/// * `TooMany` - will occur when limit of tickets in lobby is achieved
/// * `AlreadyPlaying` - will occur if specified user is already playing
pub enum SetTicketError {
    AlreadyPlaying,
    TooMany,
}

/// Enum of errors that might occur when trying to perform an action
/// * `NotPlaying` - will occur if specified user are not playing
/// * `BadGame` - will occur if specified game_id is not valid
/// * `BadUser` - will occur if specified user didn't play in this game
/// * `InvalidAction` - will occur if specified action is invalid in general or in current game conditions
pub enum DoGameActionError {
    NotPlaying,
    BadGame,
    BadUser,
    BadAction(engine::DoActionError),
}

impl GamePool {
    const MAX_LOBBY_SIZE: usize = 100;

    /// Returns game user is playing in, if so
    /// # Arguments
    /// * `user_id` - id of user you're looking for
    /// # Returns
    /// * `Some(GameId, GameInfo, UserGameState)` - game user is currently playing in, if any
    /// * `None` - if user is not currently playing in any game
    pub fn check_is_playing(
        &self,
        user_id: UserId,
    ) -> Option<(&GameId, &GameUsers, &engine::UserGameState)> {
        self.playing_users.get(&user_id).and_then(|game_id| {
            self.games
                .get(&game_id)
                .and_then(|game| Some((game_id, &game.users, game.game.get_state())))
        })
    }

    /// Returns ticket user have in lobby, if so
    /// # Arguments
    /// * `user_id` - id of user you're looking for
    /// # Returns
    /// * `Some(Ticket)` - ticket that user has, if any
    /// * `None` - if user didn't have any ticket
    pub fn check_have_ticket(&self, &user_id: &UserId) -> Option<&engine::Ticket> {
        self.tickets.get(&user_id)
    }

    /// Provides statistic of games currently played by users
    /// # Returns
    /// * `(i32,)` - count of games
    pub fn get_games_stats(&self) -> (usize,) {
        (self.games.len(),)
    }

    /// Provides statistic of tickets currently waiting in lobby
    /// # Returns
    /// * `(i32,)` - count of tickets
    pub fn get_tickets_stats(&self) -> (usize,) {
        (self.tickets.len(),)
    }

    /// Adds or replaces ticket of specified user to the lobby
    /// # Arguments
    /// * `user_id` - id of user that wants to set a ticket
    /// * `ticket` - ticket to be set
    /// # Returns
    /// * `()` - on success
    /// * `SetTicketError` - see enum definition for details
    pub fn set_ticket(
        &mut self,
        &user_id: &UserId,
        ticket: engine::Ticket,
    ) -> Result<(), SetTicketError> {
        if self.playing_users.contains_key(&user_id) {
            Err(SetTicketError::AlreadyPlaying)
        } else if self.tickets.len() >= GamePool::MAX_LOBBY_SIZE {
            Err(SetTicketError::TooMany)
        } else {
            // TODO: implement ticket checking
            self.tickets.insert(user_id, ticket);
            Ok(())
        }
    }

    /// Removes ticket (if any) of specified user from the lobby
    /// # Arguments
    /// * `user_id` - id of the user that wants to clear a ticket
    pub fn clear_ticket(&mut self, &user_id: &UserId) {
        self.tickets.remove(&user_id);
    }

    /// Performs game action in specified game by specified user
    /// # Arguments
    /// * `game_id` - id of the game in which user wants to perform an action
    /// * `user_id` - id of user which wants to perform an action
    /// * `action` - game action itself
    /// # Returns
    /// * `None` - on success, if action doesn't end the game
    /// * `GameInfo` - on success, if action ends the game, so you can disconnect users
    /// * `DoGameActionError` - see enum definition for details
    pub fn do_game_action(
        &self,
        &game_id: &GameId,
        &user_id: &UserId,
        action: engine::UserAction,
    ) -> Result<Option<&GameInfo>, DoGameActionError> {
        match self.games.get(&game_id) {
            Some(game_info) => {
                if game_info.users.contains(&user_id) {
                    match game_info.game.do_action(action) {
                        Ok(true) => {
                            game_info.user.iter().map(|user_id| {self.playing_users.remove(&user_id)});
                            self.games.remove(&game_id);
                            Ok(Some(game_info))
                        }
                        Ok(false) => Ok(None),
                        Err(err) => Err(Err(err)),
                    }
                } else {
                    if self.playing_users.contains_key(&user_id) {
                        Err(DoGameActionError::BadUser)
                    } else {
                        Err(DoGameActionError::NotPlaying)
                    }
                }
            }
            None => Err(Ok(DoGameActionError::BadGame)),
        }
    }
}

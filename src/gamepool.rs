use crate::game::{self, Ticket};

use std::collections::HashMap;

pub type UserId = i32;
pub type GameId = i32;
pub type GameUsers = [UserId; 2];

pub struct GameInfo {
    game: game::Game,
    pub users: GameUsers,
}

pub struct GamePool {
    playing_users: HashMap<UserId, GameId>,
    games: HashMap<GameId, GameInfo>,
    last_game_id: i32,
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
    BadAction(game::DoActionError),
}

impl GamePool {
    /// Returns game user is playing in, if so
    /// # Arguments
    /// * `user_id` - id of user you're looking for
    /// # Returns
    /// * `Some(GameId, GameInfo, UserGameState)` - game user is currently playing in, if any
    /// * `None` - if user is not currently playing in any game
    pub fn check_is_playing(
        &self,
        user_id: UserId,

    ) -> Option<(&GameId, &GameUsers, &game::UserGameState)> {
        self.playing_users.get(&user_id).and_then(|game_id| {
            self.games
                .get(&game_id)
                .and_then(|game| Some((game_id, &game.users, &game.game.get_state())))
        })
    }


    /// Provides statistic of games currently played by users
    /// # Returns
    /// * `(i32,)` - count of games
    pub fn get_games_stats(&self) -> (usize,) {
        (self.games.len(),)
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
        &mut self,
        game_id: GameId,
        user_id: UserId,
        action: game::UserAction,
    ) -> Result<Option<&GameInfo>, DoGameActionError> {
        let game_info_wrapped = self.games.get(&game_id);
        if game_info_wrapped.is_none(){
            return Err(DoGameActionError::BadGame);
        };
        let game_info = game_info_wrapped.unwrap();
        if game_info.users.contains(&user_id) {
            match game_info.game.do_action(action) {
                Ok(true) => {
                    //game_info.users.iter().map(|new_user_id| {self.playing_users.remove(new_user_id)});
                    //self.games.remove(&game_id);
                    Ok(Some(game_info))
                }
                Ok(false) => Ok(None),
                Err(err) => Err(DoGameActionError::BadAction(err)),
            }
        } else {
            if self.playing_users.contains_key(&user_id) {
                Err(DoGameActionError::BadUser)
            } else {
                Err(DoGameActionError::NotPlaying)
            }
        }
    }

    pub fn start_game(&mut self, users: [UserId; 2]) -> (GameId, GameUsers) {
        self.last_game_id += 1;
        let game_info = GameInfo {
            game: game::Game::new(users[0], users[1]),
            users: users.clone(),
        };
        self.games.insert(self.last_game_id, game_info);
        (self.last_game_id, users)
    }

    pub fn new() -> GamePool {
        GamePool {
            playing_users: HashMap::new(),
            games: HashMap::new(),
            last_game_id: 0,
        }
    }
}

pub struct Lobby {
    game_pool: GamePool,
    tickets: HashMap<UserId, game::Ticket>,
}

/// Enum of errors that might occur when trying to add a ticket
/// * `TooMany` - will occur when limit of tickets in lobby is achieved
/// * `AlreadyPlaying` - will occur if specified user is already playing
#[derive(Debug)]
pub enum SetTicketError {
    AlreadyPlaying,
    TooMany,
}

impl Lobby {
    const MAX_LOBBY_SIZE: usize = 100;
    /// Returns ticket user have in lobby, if so
    /// # Arguments
    /// * `user_id` - id of user you're looking for
    /// # Returns
    /// * `Some(Ticket)` - ticket that user has, if any
    /// * `None` - if user didn't have any ticket

    pub fn check_have_ticket(&self, &user_id: &UserId) -> Option<&game::Ticket> {
        self.tickets.get(&user_id)
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
    /// * `None` - on success, if no pair was found
    /// * `Some((GameId, Users))` - on success, if pair was found and game was formed
    /// * `SetTicketError` - see enum definition for details
    pub fn set_ticket(
        &mut self,
        user_id: UserId,
        ticket: Ticket,
    ) -> Result<Option<(GameId, GameUsers)>, SetTicketError> {
        if self.game_pool.playing_users.contains_key(&user_id) {
            Err(SetTicketError::AlreadyPlaying)
        } else if self.tickets.len() >= Lobby::MAX_LOBBY_SIZE {
            Err(SetTicketError::TooMany)
        } else {
            let mut tickets_keys_iter = self.tickets.keys();
            let mut new_user_id;
            while match tickets_keys_iter.next() {
                Some(new_ticket_iter) => {
                    new_user_id = *new_ticket_iter;
                    self.tickets[new_ticket_iter].side == ticket.side
                },
                None => {
                    self.tickets.insert(user_id, ticket);
                    return Ok(None)
                },
            } {};
            Ok(Some(self.game_pool.start_game([new_user_id, user_id])))
        }
    }

    /// Removes ticket (if any) of specified user from the lobby
    /// # Arguments
    /// * `user_id` - id of the user that wants to clear a ticket
    pub fn clear_ticket(&mut self, &user_id: &UserId) {
        self.tickets.remove(&user_id);
    }


    pub fn new(game_pool: GamePool) -> Lobby {
        Lobby {
            game_pool: game_pool,
            tickets: HashMap::new(),
        }
    }
}

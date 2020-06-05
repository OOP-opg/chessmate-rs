use crate::game;
use crate::gamepool;

pub struct Server {}

impl Server {


    /// Notifies user that another user in game had performed a game action
    /// # Arguments
    /// * `game_id` - id of game action in which action was performed
    /// * `user_id` - id of user that needs to be informed about this action
    /// * `action` - action itself in user format

    
    pub fn notify_game_action(
        &game_id: &gamepool::GameId,
        &user_id: &gamepool::UserId,
        game_action: game::UserAction,
    ) {
        print!(
            "In game '{}', user '{}' did action '{}'",
            &game_id, &user_id, game_action
        );
    }

}

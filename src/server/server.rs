use gamepool::IGameInfo;
use game::{IGameAction, IGameReplay};

pub trait IUser {}


pub trait IServer {
    pub fn notify_join(user: IUser, game_info: IGameInfo);
    pub fn notify_game_action(user: IUser, action: IGameAction);
    pub fn end_game(user: IUser, replay: IGameReplay);
}

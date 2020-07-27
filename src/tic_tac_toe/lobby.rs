use std::collections::HashMap;

use super::core::{TttActionResult, TttCore, TttUsers};
use crate::common::core::{GameId, UserId};
use crate::common::domain::{
    GameMoveObserver, GameObserver, Id, Lobby, Observers, StartGameObserver,
};

use super::core::TttWish;

struct TttInfo {
    rating: u64,
}

struct Ticket {
    wish: TttWish,
    info: TttInfo,
}

/*
pub struct TttActorObservers;
*/

/*
impl Observers<TttCore> for TttActorObservers {
    type GameObserver = ActorGameObserver;
    type StartGameObserver =  ActorStartGameObserver<TttUsers>;
}
*/

pub struct TttLobby<O: Observers<TttCore>>
/* */
where
    O: Observers<TttCore>,
    O::StartGameObserver: StartGameObserver<TttUsers>,
    O::GameMoveObserver: GameMoveObserver<TttActionResult>,
{
    communication: O::StartGameObserver,
    tickets: HashMap<UserId, (Ticket, /*ActorGameObserver*/ O::GameObserver)>,
    game_counter: GameId,
}

/*
impl<O: Observers<TttCore>> Default for TttLobby<O>
    where O::StartGameObserver: StartGameObserver<TttUsers> {
    fn default() -> Self {
        TttLobby {
            tickets: HashMap::new(),
            game_counter: GameId::new(),
        }
    }
}
*/

impl<O> Lobby<TttCore, O> for TttLobby<O>
where
    O: Observers<TttCore>,
    O::GameMoveObserver: GameMoveObserver<TttActionResult>,
    O::StartGameObserver: StartGameObserver<TttUsers>,
{
    fn with_communication(communication: O::StartGameObserver) -> Self {
        TttLobby {
            communication,
            tickets: HashMap::new(),
            game_counter: GameId::new(),
        }
    }
    fn add_ticket(
        &mut self,
        new_user: UserId,
        new_wish: TttWish,
        new_observer: O::GameObserver,
    ) {
        log::debug!("Got wish {:?} from {:?}", new_wish, new_user);

        let new_ticket = Ticket {
            wish: new_wish,
            info: TttInfo { rating: 1000 },
        };

        let mut paired = false;
        let mut paired_user = None;

        for ticket in &self.tickets {
            let (Ticket { wish, info: _ }, observer) = ticket.1;
            let user_id = *ticket.0;

            if new_user == user_id {
                continue;
            }
            if new_wish.sign != wish.sign {
                paired_user.replace(user_id);
                log::info!("Find pair for {} and {}", user_id, new_user);

                observer.notify(self.game_counter);
                new_observer.notify(self.game_counter);
                let users = TttUsers(user_id, new_user);
                self.communication.start_game(self.game_counter, users);

                self.game_counter.inc();
                paired = true;
                break;
            }
        }
        if paired {
            let _ = self.tickets.remove(&paired_user.unwrap());
        } else {
            let _ = self.tickets.insert(new_user, (new_ticket, new_observer));
        }
    }
}
/*
*/

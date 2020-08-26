use std::collections::HashMap;

use super::core::{TttActionResult, TttCore, TttUsers};
use crate::common::core::{GameId, UserId};
use crate::common::domain::{
    GameMoveObserver, GameObserver, Id, Lobby, Observers, StartGameObserver,
};

use super::core::TttWish;

#[derive(Copy, Clone)]
struct TttInfo {
    rating: u32,
}

impl TttInfo {
    fn is_match(&self, other: Self) -> bool {
        const RATING_DIFFERENCE_TRESHOLD: u32 = 20;
        let rating_difference = if self.rating < other.rating {
            self.rating - other.rating
        } else {
            other.rating - self.rating
        };
        
        rating_difference < RATING_DIFFERENCE_TRESHOLD
    }
}

struct Ticket {
    wish: TttWish,
    info: TttInfo,
}

pub struct TttLobby<O: Observers<TttCore>>
where
    O: Observers<TttCore>,
    O::StartGameObserver: StartGameObserver<TttUsers>,
    O::GameMoveObserver: GameMoveObserver<TttActionResult>,
{
    communication: O::StartGameObserver,
    tickets: HashMap<UserId, (Ticket, O::GameObserver)>,
    game_counter: GameId,
}

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
        new_user_id: UserId,
        new_wish: TttWish,
        new_observer: O::GameObserver,
    ) {
        log::debug!("Got wish {:?} from {:?}", new_wish, new_user_id);

        let new_ticket = Ticket {
            wish: new_wish,
            info: TttInfo { rating: 1000 },
        };

        let mut paired_user = None;

        for (user_id, ticket) in &self.tickets {
            let (Ticket { wish, info }, observer) = ticket;
            let user_id = *user_id;

            if new_user_id == user_id {
                // added ticket with user that already in tickets
                continue;
            }

            if (new_wish.sign != wish.sign) && new_ticket.info.is_match(*info) {
                // got match
                log::info!("Find pair for {} and {}", user_id, new_user_id);
                paired_user = Some(user_id);
                self.game_counter.inc();

                // notify observers
                observer.notify(self.game_counter);
                new_observer.notify(self.game_counter);

                //notify gameserver about new game
                let users = TttUsers(user_id, new_user_id);
                self.communication.start_game(self.game_counter, users);

                break;
            }
        }
        if let Some(paired_user) = paired_user {
            // because we have found user, we are remove our match
            // and don't add yourself
            let _ = self.tickets.remove(&paired_user);
        } else {
            // nothing is matched, so just add yourself and wait
            let _ = self.tickets.insert(new_user_id, (new_ticket, new_observer));
        }
    }
}

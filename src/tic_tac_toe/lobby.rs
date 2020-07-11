use std::collections::HashMap;

use crate::common::core::{GameId, UserId};
use crate::common::domain::{Lobby, Id, GameObserver};

use super::game::{TttWish};
use super::communication::TttGameObserver;


struct TttInfo {
    rating: u64,
}

struct Ticket {
    wish: TttWish,
    info: TttInfo,
}

pub struct TttLobby {
    tickets: HashMap<UserId, (Ticket, TttGameObserver)>,
    game_counter: GameId,
}

impl Default for TttLobby {
    fn default() -> Self {
        TttLobby {
            tickets: HashMap::new(),
            game_counter: GameId::new(),
        }
    }
}

impl Lobby<TttWish, TttGameObserver> for TttLobby {
    fn add_ticket(
        &mut self,
        new_user: UserId,
        new_wish: TttWish,
        new_observer: TttGameObserver,
    ) {
        log::debug!("Got wish {:?} from {:?}", new_wish, new_user);

        let new_ticket = Ticket {
            wish: new_wish,
            info: TttInfo { rating: 1000 },
        };

        let mut paired = false;
        let mut paired_user = None;

        for ticket in &self.tickets {
            let (Ticket {wish, info: _ }, observer) = ticket.1;
            let user_id = *ticket.0;

            if new_user == user_id {
                continue;
            }
            if new_wish.sign != wish.sign {
                paired_user.replace(user_id);
                log::info!("Find pair for {} and {}", user_id, new_user);
                observer.notify(self.game_counter);
                new_observer.notify(self.game_counter);
                self.game_counter.inc();
                paired = true;
                break;
            }
        };
        if paired {
            let _ = self.tickets.remove(&paired_user.unwrap());
        } else {
            let _ = self.tickets.insert(new_user, (new_ticket, new_observer));
        }
    }
}

use std::collections::HashMap;

use actix::Recipient;
use crate::common::messages::NewGame;
use crate::common::core::{GameId, UserId};
use crate::common::domain::{Lobby, Id};

use super::game::{TttWish};


struct TttInfo {
    rating: u64,
}

struct Ticket {
    wish: TttWish,
    info: TttInfo,
}

pub struct TttLobby {
    tickets: HashMap<UserId, (Ticket, Recipient<NewGame>)>,
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

impl Lobby<TttWish> for TttLobby {
    fn add_ticket(
        &mut self,
        user: UserId,
        new_wish: TttWish,
        new_observer: Recipient<NewGame>,
    ) {
        log::debug!("Got wish {:?} from {:?}", new_wish, user);

        let new_ticket = Ticket {
            wish: new_wish,
            info: TttInfo { rating: 1000 },
        };

        let mut paired = false;
        if self.tickets.contains_key(&user) {
            self.tickets.insert(user, (new_ticket, new_observer));
        }
        let mut paired_user = None;

        for ticket in &self.tickets {
            let (Ticket {wish, info: _ }, observer) = ticket.1;
            let user_id = ticket.0;

            if new_wish.sign != wish.sign {
                paired_user.replace(user_id);
                log::info!("Find pair for {} and {}", user_id, user);
                observer.do_send(NewGame(self.game_counter));
                new_observer.do_send(NewGame(self.game_counter));
                self.game_counter.inc();
                paired = true;
                break;
            }
        };
        if paired {
            let _ = self.tickets.remove(&paired_user.unwrap());
        } else {
            let _ = self.tickets.insert(user, (new_ticket, new_observer));
        }
    }
}

use std::collections::HashMap;

use crate::core::{GameId, UserId};
use crate::domain::{AbstractLobby, Id, PairObserver, SetTicketError, Wish};
use crate::observers::TicketObserver;

pub struct Lobby<W: Wish> {
    tickets: HashMap<UserId, (W, TicketObserver)>,
    game_counter: GameId,
}

impl<W: Wish> Lobby<W> {

}

impl<W> AbstractLobby<W, TicketObserver> for Lobby<W>
where
    W: Wish,
{
    fn new() -> Lobby<W> {
        Lobby {
            tickets: HashMap::new(),
            game_counter: GameId::new(),
        }
    }

    fn add_ticket(
        &mut self,
        user: UserId,
        new_wish: W,
        new_observer: TicketObserver,
    ) -> Result<(), SetTicketError> {
        log::debug!("Got wish {:?} from {:?}", new_wish, user);
        let mut paired = false;
        let mut paired_user = Option::None;
        for ticket in &self.tickets {
            let (wish, observer) = ticket.1;
            let user_id = *ticket.0;

            if wish.is_match(&new_wish) {
                paired = true;
                paired_user.replace(user_id);
                log::info!("Find pair for {} and {}", user_id, user);
                observer.notify(self.game_counter);
                new_observer.notify(self.game_counter);
                self.game_counter.inc();
                break;
            }
        }
        if paired {
            let _ = self.tickets.remove(&paired_user.unwrap());
            Ok(())
        } else {
            match self.tickets.insert(user, (new_wish, new_observer)) {
                None => Ok(()),
                Some(_) => Err(SetTicketError::DuplicateTicket),
            }
        }
    }
}

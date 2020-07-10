use crate::core::{GameId, UserId};
use crate::domain::{PairObserver, Wish};
use actix::{Message, Recipient};

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewGame(pub GameId);

#[derive(Message)]
#[rtype(result = "()")]
pub struct FindPair<W: Wish> {
    pub user_id: UserId,
    pub wish: W,
    pub addr: Recipient<NewGame>,
}

#[derive(Debug)]
pub struct TicketObserver {
    pub feedback: Recipient<NewGame>,
}

impl PairObserver for TicketObserver {
    fn notify(&self, game: GameId) {
        let _ = self.feedback.do_send(NewGame(game));
    }
}

use crossbeam_channel::{select, Receiver, Sender};

use crate::{UserId, GameId};
use crate::Color;
use crate::game::Ticket;
use crate::gamepool::{GamePool, Lobby};


#[derive(Debug)]
pub struct Paired {
    pub user_id: UserId,
    pub game_id: GameId,
}

pub fn pairing_loop(event_reciever: Receiver<(Color, UserId)>, pairing_sender: Sender<Paired>) {
    std::thread::spawn(move || {
        let gamepool = GamePool::new();
        let lobby = Lobby::new(gamepool);

        loop {
            select! {
                recv(event_reciever) -> event => {
                    println!("{:?} received", event);


                    let set_ticket_result = match event.unwrap() {
                        (Color::Black, id) => lobby.set_ticket(id, Ticket {side: Color::Black}),
                        (Color::White, id) => lobby.set_ticket(id, Ticket {side: Color::White}),
                    };

                    match set_ticket_result {
                        Ok(res) => { 
                            match res {
                                Some((gameid, users)) => {
                                    println!("Send pair: {:?}", pairing_sender.send(Paired { user_id: users[0], game_id: gameid }));
                                    println!("Send pair: {:?}", pairing_sender.send(Paired { user_id: users[1], game_id: gameid }));
                                }
                                None => { println!("Not found pair"); }
                            }
                        },
                        Err(err) => {
                            println!("Error {:?}", err);
                        }
                    }
                },
            }
        }
    });
}

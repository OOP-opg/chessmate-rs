use crossbeam_channel::{select, Receiver, Sender};

use crate::{UserId, GameId};
use crate::Color;
use crate::game::Ticket;
use crate::game::UserAction;
use crate::gamepool::{GamePool, Lobby, GameUsers};

pub struct EventAction {
    action: Action,
    user_id: UserId,
    game_id: GameId,
}

pub struct MoveResult {
    game_id: GameId,
    users: GameUsers,
}

pub fn actionew_loop(move_reciever: Receiver<EventAction>, move_runer_sender: Sender<MoveResult>) {
    std::thread::spawn(move || {
        let gamepool = GamePool::new();

        loop {
            select! {
                recv(event_reciever) -> event => {
                    println!("{:?} received", event);


                    let action = event.unwrap().action;
                    let game_id = event.unwrap().game_id;
                    let user_id = event.unwrap().user_id; 
                    let set_ticket_result = gamepool.do_game_action(game_id, user_id, action);

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

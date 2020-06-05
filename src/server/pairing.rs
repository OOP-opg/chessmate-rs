use crate::Paired;
use crate::Id;
use crate::Choice;
use crossbeam_channel::{select, Receiver, Sender};

pub fn pairing_loop(event_reciever: Receiver<(Choice, Id)>, pairing_sender: Sender<Paired>) {
    std::thread::spawn(move || {
        let (mut blacks, mut whites) = (vec![], vec![]);

        loop {
            select! {
                recv(event_reciever) -> event => {
                    println!("{:?} received", event);
                    match event.unwrap() {
                        (Choice::Black, id) => { blacks.push(id); },
                        (Choice::White, id) => { whites.push(id); },
                    };

                    if blacks.len() > 0 && whites.len() > 0  {
                        let black = Paired {
                            id: blacks.pop().unwrap()
                        };
                        let white = Paired {
                            id: whites.pop().unwrap()
                        };
                        println!("Send pair: {:?}", pairing_sender.send(black));
                        println!("Send pair: {:?}", pairing_sender.send(white));
                    }
                },
            }
        }
    });
}

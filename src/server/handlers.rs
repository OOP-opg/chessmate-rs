use actix_files::NamedFile;
use actix_web::{
    get,
    web,
   // HttpResponse,
    Responder,
};
use crossbeam_channel::{select, Receiver, Sender};

#[get("/api/new_game/{choice}/{id}")]
pub async fn new_game(
    front_events: web::Data<Sender<(Choice,Id)>>,
    pairing_events: web::Data<Receiver<Paired>>,
    pairing_sender: web::Data<Sender<Paired>>,
    info: web::Path<(String, Id)>,
) -> impl Responder {

    let choice_reqw = &info.0;
    let id_reqw = info.1;

    let choice: Choice = Choice::from_str(choice_reqw).unwrap();
    let event = front_events.send((choice, id_reqw));
    println!("Send new_game event: {:?}", event);

    loop {
        select! {
            recv(pairing_events) -> pair => {
                let recieved_pair = pair.unwrap(); 
                if recieved_pair.id == info.1 {
                    return format!("Found {:?}", recieved_pair);
                } else {
                    let miss = pairing_sender.send(recieved_pair);
                    println!("Return pair back, not mine: {:?}", miss);
                }
            },
        }
    }
}

#[get("/")]
pub async fn index() -> impl Responder {
    dbg!("index");
    NamedFile::open("./index.html")
}

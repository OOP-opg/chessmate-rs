use std::str::FromStr;

use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{
    get,
    web,
    App,
   // HttpResponse,
    HttpServer,
    Responder,
};
use crossbeam_channel::{select, unbounded, Receiver, Sender};

mod pairing;

#[derive(Debug)]
pub enum Choice {
    Black,
    White,
}

#[derive(Debug)]
pub enum ChoiceParseErr {
    InvalidChoice,
}

impl FromStr for Choice {
    type Err = ChoiceParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Black" | "black" => Ok(Choice::Black),
            "White" | "white" => Ok(Choice::White),
            _ => Err(ChoiceParseErr::InvalidChoice),
        }
    }
}

#[derive(Debug)]
pub struct Paired {
    id: Id,
}

type Id = u32;

#[get("/api/new_game/{choice}/{id}")]
async fn new_game(
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
async fn index() -> impl Responder {
    dbg!("index");
    NamedFile::open("./index.html")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let (sender_events, receiver_events) = unbounded::<(Choice, Id)>();
    let (sender_pairing, receiver_pairing) = unbounded::<Paired>();
    let front_events = web::Data::new(sender_events);
    let pairing_sender = web::Data::new(sender_pairing.clone());
    let pairing_events = web::Data::new(receiver_pairing);

    pairing::pairing_loop(receiver_events.clone(), sender_pairing.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(front_events.clone())
            .app_data(pairing_events.clone())
            .app_data(pairing_sender.clone())
            .service(index)
            .service(fs::Files::new("/static", "./static"))
            .service(new_game)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

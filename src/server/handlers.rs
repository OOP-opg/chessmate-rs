use std::str::FromStr;
use actix_files::NamedFile;
use actix_web::{
    get,
    web,
   // HttpResponse,
    Responder,
};
use crossbeam_channel::{select, Receiver, Sender};

//use crate::game::Move;
use crate::server::pairing::Paired;
use crate::gamepool::{GameId, UserId};
use crate::game::Color;

#[derive(Debug)]
pub enum ColorParseErr {
    InvalidColor,
}

impl FromStr for Color {
    type Err = ColorParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Black" | "black" => Ok(Color::Black),
            "White" | "white" => Ok(Color::White),
            _ => Err(ColorParseErr::InvalidColor),
        }
    }
}

#[get("/api/action/{gameid}/{userid}/{action}/{move_from}/{move_to}/{castling}")]
pub async fn action(
    info: web::Path::<(GameId, UserId, String, String, String, String, String)>
) -> impl Responder {
}

#[get("/api/new_game/{choice}/{id}")]
pub async fn new_game(
    front_events: web::Data<Sender<(Color,UserId)>>,
    pairing_events: web::Data<Receiver<Paired>>,
    pairing_sender: web::Data<Sender<Paired>>,
    info: web::Path<(String, UserId)>,
) -> impl Responder {

    let choice_reqw = &info.0;
    let id_reqw = info.1;

    let choice: Color = Color::from_str(choice_reqw).unwrap();
    let event = front_events.send((choice, id_reqw));
    println!("Send new_game event: {:?}", event);

    loop {
        select! {
            recv(pairing_events) -> pair => {
                let recieved_pair = pair.unwrap(); 
                if recieved_pair.user_id == id_reqw {
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

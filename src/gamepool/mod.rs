trait Ticket {
    // TODO : add user wishes
}

trait Game {}

trait Replay {
    fn export() -> String;
}

fn end_game<G: Game, R: Replay>(game: G, replay: R) {
    // TODO : remove game from active games
    // TODD : notify server
}

trait UserTicket {
    fn get_wish() -> String;
}

fn add_ticket<UT: UserTicket>(ticket: UT) {
    // TODO : validate with Danya's help
    // TODO : and add to tickets' list if valid
}

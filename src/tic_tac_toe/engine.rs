#[derive(Clone, Copy)]
enum Pane {
    X,
    O,
    Empty,
}

pub struct TttEngine {
    board: [Pane; 9],
}

impl TttEngine {
    const fn new() -> Self {
        Self { 
            board: [Pane::Empty; 9]
        }
    }
}

const START_POS: [u8; 64] = [114, 110, 98, 113, 107, 98, 110, 114, 
                                   112, 112, 112, 112, 112, 112, 112, 112, 
                                   46, 46, 46, 46, 46, 46, 46, 46, 
                                   46, 46, 46, 46, 46, 46, 46, 46, 
                                   46, 46, 46, 46, 46, 46, 46, 46, 
                                   46, 46, 46, 46, 46, 46, 46, 46, 
                                   80, 80, 80, 80, 80, 80, 80, 80, 
                                   82, 78, 66, 81, 75, 66, 78, 82];

const KING: [(i8, i8); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
const KNIGHT: [(i8, i8); 8] = [(-1, -2), (-1, 2), (1, 2), (1, -2), (2, 1), (2, -1), (-2, -1), (-2, 1)];
const PAWN_B: [(i8, i8); 2] = [(-1, 1),(1, 1)];
const PAWN_W: [(i8, i8); 2] = [(-1, -1),(1, -1)];


#[derive(Debug)]
enum Color{
    Black,
    White,
}

// TODO rewrite to bitfield
#[derive(Debug)]
struct CastlingRights{
    K: bool,
    Q: bool,
    k: bool,
    q: bool,
}


pub enum MoveType{
    Move,
    Castling,
    Surrender
}

pub struct Move{
    move_type: MoveType,
    move_squares: Option<([char; 2], [char; 2])>,
    castling: Option<(u8, u8)>
}

pub enum MoveResult{
    Valid,
    Invalid,
    WhiteWin,
    BlackWin
}

pub struct BoardState {
    board: [u8; 64],
    turn: Color,
    castling_rights: CastlingRights,
    en_passant: Option<u8>,
    halfmove_clock: u32,
    fullmoves: u32,
}

fn signum(num: i8) -> i8{
    if num > 0{
        1
    }else{
    if num < 0 { return -1 }
    0
    }
}

pub fn convert_square_to_u8(square: [char; 2]) -> u8{
    let letter = square[0];
    let number = square[1].to_digit(10).unwrap();
    let vertical = match letter{
        'a' | 'A' => 0,
        'b' | 'B' => 1,
        'c' | 'C' => 2,
        'd' | 'D' => 3,
        'e' | 'E' => 4,
        'f' | 'F' => 5,
        'g' | 'G' => 6,
        'h' | 'H' => 7,
        _ => panic!("En passant in FEN incorrect!")
    };
    ((8 - number) * 8 + vertical) as u8
}


pub fn convert_square_to_relative(square: [char; 2]) -> (u8, u8){
    let letter = square[0];
    let number = square[1].to_digit(10).unwrap();
    let vertical = match letter{
        'a' | 'A' => 0,
        'b' | 'B' => 1,
        'c' | 'C' => 2,
        'd' | 'D' => 3,
        'e' | 'E' => 4,
        'f' | 'F' => 5,
        'g' | 'G' => 6,
        'h' | 'H' => 7,
        _ => panic!("***wrong letter***")
    };
    (vertical, 8 - number as u8)
}

fn get_relative_coords(square: (i8, i8), x: i8, y: i8) -> Option<u8>{
    let square_x: i8 = square.0;
    let square_y: i8 = square.1;

    if square_x + x < 0 || square_x + x > 7 ||  square_y + y < 0 || square_y + y > 7{
        None
    }else{
        Some((y*7 + x) as u8)
    }
}


impl BoardState{
    pub fn new() -> BoardState{
        BoardState{
            board : START_POS,
            turn: Color::White,
            castling_rights: CastlingRights{K:true, Q:true, k:true, q:true},
            en_passant: None,
            halfmove_clock: 0,
            fullmoves: 1
        }
    }

    //get square biased by x, y relatively to given square
    fn get_relative(&self, square: u8, x: i8, y: i8) -> Option<char>{
        let square_x: i8 = (square % 8) as i8;
        let square_y: i8 = (square / 8) as i8;

        if square_x + x < 0 || square_x + x > 7 ||  square_y + y < 0 || square_y + y > 7{
            None
        }else{
            Some(self.board[((square_y + y) * 8 + square_x + x) as usize] as char)
        }
    }

    fn get_relatives(&self, square: u8, offsets: Vec<(i8, i8)>) -> Vec<Option<char>>{
        offsets.iter().map(|arg| self.get_relative(square, arg.0, arg.1)).collect::<Vec<Option<char>>>()
    }

    fn get_first_in_line(&self, square: u8, increment: (i8, i8)) -> Option<char>{
        let mut result = None;
        let mut next_square = self.get_relative(square, increment.0, increment.1);
        let mut counter: i8 = 1;
        while next_square.is_some(){
            match next_square.unwrap(){
                '.' => {next_square = self.get_relative(square, counter*increment.0, counter*increment.1); counter+=1;},
                x => return Some(x)
            }
        }
        result
    }
    
    fn check_diagonal_accessible(&self, square: u8, dest:(i8, i8)) -> bool{
        let square_x: i8 = (square % 8) as i8;
        let square_y: i8 = (square / 8) as i8;
        if (square_x - dest.0).abs() == (square_y - dest.1).abs(){
            let increment: (i8, i8) = (signum(dest.0 - square_x), signum(dest.1 - square_y));
            let mut next_square = self.get_relative(square, increment.0, increment.1);
            let mut counter: i8 = 1;
            while next_square.is_some(){
                if counter == (square_x - dest.0).abs(){return true;}
                match next_square.unwrap(){
                    '.' => {next_square = self.get_relative(square, increment.0*(counter+1), increment.1*(counter+1)); counter+=1;},
                    _ => {return false}
                }
            }
        }
        else{return false}
        false
    }

    fn check_straight_accessible(&self, square: u8, dest:(i8, i8)) -> bool{
        let square_x: i8 = (square % 8) as i8;
        let square_y: i8 = (square / 8) as i8;
        let increment: (i8, i8) = (signum(square_x - dest.0), signum(square_y - dest.1));
        let mut next_square = self.get_relative(square, increment.0, increment.1);
        let mut counter: i8 = 1;
        if square_x == dest.0 || square_y == dest.1{
            while next_square.is_some(){
                match next_square.unwrap(){
                    '.' => {next_square = self.get_relative(square, increment.0*counter, increment.1*counter); counter+=1},
                    x => {return false}
                }
            }
        }
        return false;
    }

    fn get_firsts_in_diagonals(&self, square: u8) -> [Option<char>; 4]{
        let result: [Option<char>; 4] = [
            self.get_first_in_line(square, (1, 1)),
            self.get_first_in_line(square, (1, -1)),
            self.get_first_in_line(square, (-1, 1)),
            self.get_first_in_line(square, (-1, -1))];
        result
    }

    fn get_firsts_in_straights(&self, square: u8) -> [Option<char>; 4]{
        let result: [Option<char>; 4] = [
                    self.get_first_in_line(square, (-1, 0)),
                    self.get_first_in_line(square, (1, 0)),
                    self.get_first_in_line(square, (0, 1)),
                    self.get_first_in_line(square, (0, -1))];
        result
    }

    fn check_if_square_under_attack(&self, square: u8, attacker: Color) -> bool{
        match attacker{
            Color::Black => {
                if self.get_relatives(square, KING.to_vec()).iter().any(|arg| *arg == Some('k')) {return true}
                if self.get_relatives(square, KNIGHT.to_vec()).iter().any(|arg| *arg == Some('n')) {return true}
                if self.get_relatives(square, PAWN_W.to_vec()).iter().any(|arg| *arg == Some('p')) {return true}
                if self.get_firsts_in_diagonals(square).iter().any(|arg| *arg == Some('q') || *arg == Some('b')) {return true}
                if self.get_firsts_in_straights(square).iter().any(|arg| *arg == Some('q') || *arg == Some('r')) {return true}
            }
            Color::White=>{
                if self.get_relatives(square, KING.to_vec()).iter().any(|arg| *arg == Some('K')) {return true}
                if self.get_relatives(square, KNIGHT.to_vec()).iter().any(|arg| *arg == Some('N')) {return true}
                if self.get_relatives(square, PAWN_B.to_vec()).iter().any(|arg| *arg == Some('P')) {return true}
                if self.get_firsts_in_diagonals(square).iter().any(|arg| *arg == Some('Q') || *arg == Some('B')) {return true}
                if self.get_firsts_in_straights(square).iter().any(|arg| *arg == Some('Q') || *arg == Some('R')) {return true}
            }
        }
        false
    }

    fn validate_move(&mut self, player_move: Move, player_color: Color) -> MoveResult{
        let mut result = MoveResult::Invalid;
        let handle_move = || -> MoveResult{
            result = MoveResult::Invalid;           
            let move_from = convert_square_to_u8(player_move.move_squares.unwrap().0);
            let move_to = convert_square_to_u8(player_move.move_squares.unwrap().1);
            let move_from_rel = convert_square_to_relative(player_move.move_squares.unwrap().0);
            let move_to_rel = convert_square_to_relative(player_move.move_squares.unwrap().1);
            let figure = self.board[move_from as usize] as char;
            let dest_figure = self.board[move_to as usize] as char;

            if figure == '.'{
                return MoveResult::Invalid
            }
            match player_color{
                Color::White => {
                    if figure.is_lowercase() {return MoveResult::Invalid}
                    if dest_figure.is_uppercase() {return MoveResult::Invalid}
                    match figure{
                        'P' => {              
                            result = MoveResult::Invalid;
                            if move_from_rel.1 - move_to_rel.1 == 1{
                                if move_from_rel.0 as i8 - move_to_rel.0 as i8 == 0 && self.get_relative(move_from, 0, -1).unwrap() == '.'{result = MoveResult::Valid}
                                if move_from_rel.0 as i8- move_to_rel.0 as i8 == -1 && (self.get_relative(move_from, 1, -1).unwrap() != '.') {result = MoveResult::Valid}
                                if move_from_rel.0 as i8- move_to_rel.0 as i8 == 1 && self.get_relative(move_from, -1, -1).unwrap() != '.'{result = MoveResult::Valid}
                            }
                            if move_from_rel.1 - move_to_rel.1 == 2{
                                if move_from_rel.0 as i8 - move_to_rel.0 as i8 == 0 && self.get_relative(move_from, 0, -2).unwrap() == '.' && self.get_relative(move_from, 0, -1).unwrap() == '.'{result = MoveResult::Valid;}
                            }
                        },
                        'N' => {
                            let temp: (i8, i8) = (move_from_rel.0 as i8 - move_to_rel.0 as i8, move_from_rel.1 as i8 - move_to_rel.1 as i8);
                            if KNIGHT.iter().any(|arg| *arg == temp){
                                result = MoveResult::Valid;
                            }
                        },
                        'K' => {
                            let temp: (i8, i8) = (move_from_rel.0 as i8 - move_to_rel.0 as i8, move_from_rel.1 as i8 - move_to_rel.1 as i8);
                            if KING.iter().any(|arg| *arg == temp){
                                if !self.check_if_square_under_attack(move_to, Color::White){
                                    result = MoveResult::Valid;
                                    self.castling_rights.K = false;
                                    self.castling_rights.Q = false;
                                }
                            }
                        },
                        'B' => {
                            if self.check_diagonal_accessible(move_from, (move_to_rel.0 as i8, move_to_rel.1 as i8)){
                                result = MoveResult::Valid;
                            }
                        },
                        'R' => {
                            if self.check_straight_accessible(move_from, (move_to_rel.0 as i8, move_to_rel.1 as i8)){
                                result = MoveResult::Valid;
                                if move_from == 63 && self.castling_rights.K  {self.castling_rights.K = false;}
                                if move_from == 56 && self.castling_rights.Q  {self.castling_rights.Q = false;}
                            }
                        },
                        'Q' => {
                            if self.check_straight_accessible(move_from, (move_to_rel.0 as i8, move_to_rel.1 as i8)) || self.check_diagonal_accessible(move_from, (move_to_rel.0 as i8, move_to_rel.1 as i8)){
                                result = MoveResult::Valid;
                            }
                        },
                        _=>panic!("Panic in white check")
                    };
                    let figure_from = self.board[move_from as usize];
                    let figure_to = self.board[move_to as usize];
                    self.board[move_to as usize] = figure_from;
                    self.board[move_from as usize] = b'.';
                    let mut king_pos: u8 = 0;
                    for (i, x) in self.board.iter().enumerate(){
                        if *x == b'K'{
                            king_pos = i as u8;
                        }
                    }
                    if self.check_if_square_under_attack(king_pos, Color::Black){
                        self.board[move_to as usize] = figure_to;
                        self.board[move_from as usize] = figure_from;
                        return MoveResult::Invalid;
                    }
                    let figure_from = self.board[move_from as usize];
                    let figure_to = self.board[move_to as usize];
                    self.board[move_to as usize] = figure_from;
                    self.board[move_from as usize] = b'.';
                    let mut king_pos: u8 = 0;
                    for (i, x) in self.board.iter().enumerate(){
                        if *x == b'K'{
                            king_pos = i as u8;
                        }
                    }
                    if self.check_if_square_under_attack(king_pos, Color::Black){
                        self.board[move_to as usize] = figure_to;
                        self.board[move_from as usize] = figure_from;
                        return MoveResult::Invalid;
                    }

                },
                Color::Black => {
                    if figure.is_uppercase() {return MoveResult::Invalid}
                    if dest_figure.is_lowercase() {return MoveResult::Invalid}
                    match figure{
                        'p' => {              
                            result = MoveResult::Invalid;
                            if  move_to_rel.1 - move_from_rel.1  == 1{
                                
                                if move_from_rel.0 as i8 - move_to_rel.0 as i8 == 0 && self.get_relative(move_from, 0, 1).unwrap() == '.'{result = MoveResult::Valid}
                                if move_from_rel.0 as i8- move_to_rel.0 as i8 == -1 && (self.get_relative(move_from, 1, 1).unwrap() != '.') {result = MoveResult::Valid}
                                if move_from_rel.0 as i8- move_to_rel.0 as i8 == 1 && self.get_relative(move_from, -1, 1).unwrap() != '.'{result = MoveResult::Valid}
                            }
                            if move_to_rel.1 - move_from_rel.1  == 2{
                               
                                if move_from_rel.0 as i8 - move_to_rel.0 as i8 == 0 && self.get_relative(move_from, 0, 2).unwrap() == '.' && self.get_relative(move_from, 0, 1).unwrap() == '.'{result = MoveResult::Valid}
                            }
                        },
                        'n' => {
                            let temp: (i8, i8) = (move_from_rel.0 as i8 - move_to_rel.0 as i8, move_from_rel.1 as i8 - move_to_rel.1 as i8);
                            if KNIGHT.iter().any(|arg| *arg == temp){
                                result = MoveResult::Valid;
                            }
                        },
                        'k' => {
                            let temp: (i8, i8) = (move_from_rel.0 as i8 - move_to_rel.0 as i8, move_from_rel.1 as i8 - move_to_rel.1 as i8);
                            if KING.iter().any(|arg| *arg == temp){
                                if !self.check_if_square_under_attack(move_to, Color::White){
                                    result = MoveResult::Valid;
                                    self.castling_rights.k = false;
                                    self.castling_rights.q = false;
                                }
                            }
                        },
                        'b' => {
                            if self.check_diagonal_accessible(move_from, (move_to_rel.0 as i8, move_to_rel.1 as i8)){
                                result = MoveResult::Valid;
                            }
                        },
                        'r' => {
                            if self.check_straight_accessible(move_from, (move_to_rel.0 as i8, move_to_rel.1 as i8)){
                                result = MoveResult::Valid;
                                if move_from == 63 && self.castling_rights.k  {self.castling_rights.k = false;}
                                if move_from == 56 && self.castling_rights.q  {self.castling_rights.q = false;}
                            }
                        },
                        'q' => {
                            if self.check_straight_accessible(move_from, (move_to_rel.0 as i8, move_to_rel.1 as i8)) || self.check_diagonal_accessible(move_from, (move_to_rel.0 as i8, move_to_rel.1 as i8)){
                                result = MoveResult::Valid;
                            }
                        },
                        _=>panic!("Panic in white check")
                    };
                    let figure_from = self.board[move_from as usize];
                    let figure_to = self.board[move_to as usize];
                    self.board[move_to as usize] = figure_from;
                    self.board[move_from as usize] = b'.';
                    let mut king_pos: u8 = 0;
                    for (i, x) in self.board.iter().enumerate(){
                        if *x == b'k'{
                            king_pos = i as u8;
                        }
                    }
                    if self.check_if_square_under_attack(king_pos, Color::White){
                        self.board[move_to as usize] = figure_to;
                        self.board[move_from as usize] = figure_from;
                        return MoveResult::Invalid;
                    }
                }
            };
            self.turn = match self.turn{
                Color::Black => Color::White,
                Color::White => Color::Black
            };

            let mut cur_king = 0;
            match self.turn{
                Color::Black=> {
                    for (i, x) in self.board.iter().enumerate(){
                        if *x == b'k'{
                            cur_king = i as u8;
                        }
                    }
                },
                Color::White=> {
                    for (i, x) in self.board.iter().enumerate(){
                        if *x == b'K'{
                            cur_king = i as u8;
                        }
                    }
                }
            };
            let opponent_color = match self.turn{
                Color::Black => Color::White,
                Color::White => Color::Black
            };

            if self.check_if_square_under_attack(cur_king, opponent_color){
                let mut the_end = true;
                for sq in KING.iter(){
                    if self.get_relative(cur_king, sq.0, sq.1) == Some('.'){
                        let king_x: i8 = (cur_king % 8) as i8;
                        let king_y: i8 = (cur_king / 8) as i8;
                        let temp2 = get_relative_coords((king_x, king_y), sq.0, sq.1);
                        if  temp2.is_some(){
                            match self.turn{
                                Color::Black => {
                                    if !self.check_if_square_under_attack(temp2.unwrap(), Color::White){
                                        the_end = false;
                                    }
                                },
                                Color::White => {
                                    if !self.check_if_square_under_attack(temp2.unwrap(), Color::White){
                                        the_end = false;
                                    }
                                }
                            };
                            
                        }
                    }
                }
                if the_end == true{
                    match self.turn{
                        Color::Black => return MoveResult::BlackWin,
                        Color::White => return MoveResult::WhiteWin,
                    };
                }
            }
            result
        };

        return match player_move.move_type{
            MoveType::Move => handle_move(),
            MoveType::Castling => {
                return MoveResult::Valid;
            },
            MoveType::Surrender => {
                match self.turn{
                    Color::Black => return MoveResult::WhiteWin,
                    Color::White => return MoveResult::BlackWin,
                }
            }
        }
    }
    
    //Forsyth–Edwards Notation (FEN) is a standard notation for describing a particular board position of a chess game.
    fn parse_fen(&mut self, fen_str: &str) -> Result<(), &'static str>{
        let fen_parts: Vec<&str> = fen_str.split_ascii_whitespace().collect();
        let mut ind = 0;
        for row in fen_parts[0].split("/"){
            for chr in row.chars(){
                if chr.is_digit(10){
                    for _ in 0..chr.to_digit(10).unwrap(){
                        self.board[ind] = '.' as u8;
                        ind += 1;
                    }
                }
                else{
                    self.board[ind] = chr as u8;
                    ind += 1;
                }
            }
        }

        if fen_parts[1].len() > 1 {
            return Err("Fen turn too long");
        }

        self.turn = match fen_parts[1].chars().next().unwrap(){
            'w' | 'W' => Color::White,
            'b' | 'B' => Color::Black,
            _  => panic!("Something wrong with turn recognizing while reading FEN")
        };


        self.castling_rights = CastlingRights{
            K: fen_parts[2].contains('K'),
            k: fen_parts[2].contains('k'),
            Q: fen_parts[2].contains('Q'),
            q: fen_parts[2].contains('q'),
        };

        if fen_parts[3] == "-" {
            self.en_passant = None;
        }else{
            let mut temp_iterator = fen_parts[3].chars();
            let move_from = temp_iterator.next().unwrap();
            let move_to = temp_iterator.next().unwrap();
            self.en_passant = Some(convert_square_to_u8([move_from, move_to]));
        }

        self.halfmove_clock = fen_parts[4].parse::<u32>().unwrap();
        self.fullmoves = fen_parts[5].parse::<u32>().unwrap();
        Ok(())
    }
}

/*
println!("Board: {}\nTurn: {:?}\n Castling rights: {:?}\nEn passant: {:?}\nHalfmoves: {}\nFullmoves: {}",
              std::str::from_utf8(&board_state.board).unwrap(),
              board_state.turn,
              board_state.castling_rights,
              board_state.en_passant,
              board_state.halfmove_clock,
              board_state.fullmoves);
*/

#[test]
    fn test_convert_square_to_u8() {
        assert_eq!(convert_square_to_u8(['a','8']), 0);
        assert_eq!(convert_square_to_u8(['h', '1']), 63);
        assert_eq!(convert_square_to_u8(['e', '4']), 36);
    }
#[test]
fn test_convert_square_to_relative() {
        assert_eq!(convert_square_to_relative(['a','8']), (0, 0));
        assert_eq!(convert_square_to_relative(['e','2']), (4, 6));
        assert_eq!(convert_square_to_relative(['h','1']), (7, 7));
        assert_eq!(convert_square_to_relative(['e','4']), (4, 4));
        assert_eq!(convert_square_to_relative(['c','7']), (2, 1));
    }
#[test]
    fn test_validate(){
        let mut board_state = BoardState::new();
        let mut mov = Move{
            move_type: MoveType::Move,
            move_squares: Some((['e', '3'], ['e','4'])),
            castling: None
        };
        let mut temp = match board_state.validate_move(mov, Color::White){
            MoveResult::Invalid => true,
            _ => false
        };
        assert_eq!(temp, true);
        let mut board_state = BoardState::new();

        let mut mov = Move{
            move_type: MoveType::Move,
            move_squares: Some((['e', '2'], ['e','4'])),
            castling: None
        };
        let mut temp = match board_state.validate_move(mov, Color::White){
            MoveResult::Valid => true,
            _ => false
        };
        assert_eq!(temp, true);
        let mut board_state = BoardState::new();
        let mut mov = Move{
            move_type: MoveType::Move,
            move_squares: Some((['e', '2'], ['e','5'])),
            castling: None
        };
        let mut temp = match board_state.validate_move(mov, Color::White){
            MoveResult::Invalid => true,
            _ => false
        };
        assert_eq!(temp, true);
        let mut board_state = BoardState::new();
        let mut mov = Move{
            move_type: MoveType::Move,
            move_squares: Some((['e', '2'], ['d','3'])),
            castling: None
        };
        let mut temp = match board_state.validate_move(mov, Color::White){
            MoveResult::Invalid => true,
            _ => false
        };
        assert_eq!(temp, true);
        let mut board_state = BoardState::new();
        let mut mov = Move{
            move_type: MoveType::Move,
            move_squares: Some((['f', '1'], ['f','5'])),
            castling: None
        };
        let mut temp = match board_state.validate_move(mov, Color::White){
            MoveResult::Invalid => true,
            _ => false
        };
        assert_eq!(temp, true);
        let mut board_state = BoardState::new();
        let mut mov = Move{
            move_type: MoveType::Move,
            move_squares: Some((['g', '1'], ['g','3'])),
            castling: None
        };
        let mut temp = match board_state.validate_move(mov, Color::White){
            MoveResult::Invalid => true,
            _ => false
        };
        assert_eq!(temp, true);
        let mut board_state = BoardState::new();
        let mut mov = Move{
            move_type: MoveType::Move,
            move_squares: Some((['g', '1'], ['f','3'])),
            castling: None
        };
        let mut temp = match board_state.validate_move(mov, Color::White){
            MoveResult::Valid => true,
            _ => false
        };
        assert_eq!(temp, true);
        
    }

#[test]
fn test_get_first_in_line(){
    let mut board_state = BoardState::new();
    let temp1 = board_state.get_first_in_line(convert_square_to_u8(['A', '4']), (1, -1));
    assert_eq!(temp1, Some('p'));
    let temp1 = board_state.get_first_in_line(convert_square_to_u8(['A', '4']), (1, 1));
    assert_eq!(temp1, Some('P'));
    let temp1 = board_state.get_first_in_line(convert_square_to_u8(['A', '4']), (-1, 0));
    assert_eq!(temp1, None);
}

#[test]
fn test_check_diagonal_accessible(){
    let mut board_state = BoardState::new();
    let temp1 = board_state.check_diagonal_accessible(convert_square_to_u8(['A', '4']), (3, 1));
    assert_eq!(temp1, true);
    let temp1 = board_state.check_diagonal_accessible(convert_square_to_u8(['A', '4']), (4, 0));
    assert_eq!(temp1, false);
    let temp1 = board_state.check_diagonal_accessible(convert_square_to_u8(['A', '4']), (2, 2));
    assert_eq!(temp1, true);
    let temp1 = board_state.check_diagonal_accessible(convert_square_to_u8(['A', '4']), (7, 7));
    assert_eq!(temp1, false);
    let temp1 = board_state.check_diagonal_accessible(convert_square_to_u8(['A', '4']), (1, 4));
    assert_eq!(temp1, false);
    let temp1 = board_state.check_diagonal_accessible(convert_square_to_u8(['f', '1']), (3, 5));
    assert_eq!(temp1, false);
}

#[test]
fn test_check_if_square_under_attack(){
    let mut board_state = BoardState::new();
    let temp1 = board_state.check_if_square_under_attack(convert_square_to_u8(['E', '2']), Color::White);
    assert_eq!(temp1, true);
    let temp1 = board_state.check_if_square_under_attack(convert_square_to_u8(['E', '2']), Color::Black);
    assert_eq!(temp1, false);
    let temp1 = board_state.check_if_square_under_attack(convert_square_to_u8(['a', '4']), Color::White);
    assert_eq!(temp1, false);
    let temp1 = board_state.check_if_square_under_attack(convert_square_to_u8(['a', '4']), Color::Black);
    assert_eq!(temp1, false);
    let temp1 = board_state.check_if_square_under_attack(convert_square_to_u8(['h', '7']), Color::White);
    assert_eq!(temp1, false);
    let temp1 = board_state.check_if_square_under_attack(convert_square_to_u8(['h', '7']), Color::Black);
    assert_eq!(temp1, true);
}
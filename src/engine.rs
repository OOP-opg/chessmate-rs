use Color::{Black, White};
use std::cmp::{Ordering, Ord};

const START_POS: [[char; 8]; 8] =  [['r', 'p', '.', '.', '.', '.', 'P', 'R'], 
                                    ['n', 'p', '.', '.', '.', '.', 'P', 'N'], 
                                    ['b', 'p', '.', '.', '.', '.', 'P', 'B'], 
                                    ['q', 'p', '.', '.', '.', '.', 'P', 'Q'], 
                                    ['k', 'p', '.', '.', '.', '.', 'P', 'K'], 
                                    ['b', 'p', '.', '.', '.', '.', 'P', 'B'], 
                                    ['n', 'p', '.', '.', '.', '.', 'P', 'N'], 
                                    ['r', 'p', '.', '.', '.', '.', 'P', 'R']];

const KING: [RelMov; 8] = [RelMov(-1, -1), RelMov(-1, 0), 
                           RelMov( -1,   1), RelMov(  0,   -1), 
                           RelMov(0, 1), RelMov(1, -1), 
                           RelMov(1, 0), RelMov(1, 1)];

const KNIGHT: [RelMov; 8] = [RelMov(-1, 2), RelMov(2, 1), 
                             RelMov(-1, -2), RelMov(2, -1), 
                             RelMov(1, 2), RelMov(-2, 1), 
                             RelMov(1, -2), RelMov(-2, -1)];


#[derive(Debug)]
pub enum Color{
    Black,
    White,
}
impl std::ops::Not for Color{
    type Output = Color;
    fn not(self) -> Self::Output{
        match self{
            White => Black,
            Black => White,
        }
    }
}
impl Copy for Color { }
impl Clone for Color{
    fn clone(&self) -> Color{
        *self
    }
}

//TODO replace with something smarter
#[derive(Debug)]
struct CastlingRights{
    wk: bool,
    wq: bool,
    bk: bool,
    bq: bool,
}

pub enum Castling{
    WK,
    WQ,
    BK,
    BQ,
}

#[derive(Copy, Clone, Debug)]
pub struct Square(u8, u8);
impl PartialEq for Square{
    fn eq(&self, other: &Self) -> bool{
        self.0 == other.0 && self.1 == other.1
    }
}

pub enum Move{
    Move(Square, Square),
    Castling(Castling),
    Surrender(Color),
}

#[derive(Debug)]
pub enum MoveResult{
    Valid,
    Invalid,
    WhiteWin,
    BlackWin
}


pub struct BoardState {
    board: [[char; 8]; 8], //where (0, 0) is top left corner or A8
    turn: Color,
    castling_rights: CastlingRights,
    en_passant: Option<Square>,
    halfmove_clock: u32,
    fullmoves: u32,
}

#[derive(Copy, Clone)]
pub struct RelMov(i8, i8);

impl PartialEq for RelMov{
    fn eq(&self, other: &Self) -> bool{
        self.0 == other.0 && self.1 == other.1
    }
}


pub enum ConvertStrToU8Error{
    LetterOutOfRange,
    ArgumentTooShort,
    ArgumentIsNotConvertibleToNumber,
}

fn is_col_uppercase(color: Color) -> bool{
    match color{
        White => true,
        Black => false,
    }
}

//Converts string representation like "e4" to coords; Coords (0, 0) is a8 
pub fn convert_str_to_u8(square: &str) -> Result<Square, ConvertStrToU8Error>{
    let mut arg = square.chars();

    let letter = match arg.next(){
        Some(x) => x,
        None => return Err(ConvertStrToU8Error::ArgumentTooShort)
    };

    let number: u8 = match arg.next(){
        Some(x) => match x.to_digit(10){
            Some(y) => y as u8,
            None => return Err(ConvertStrToU8Error::ArgumentIsNotConvertibleToNumber)
        },
        None => return Err(ConvertStrToU8Error::ArgumentTooShort),
    };

    let vertical = match letter{
        'a' | 'A' => 0,
        'b' | 'B' => 1,
        'c' | 'C' => 2,
        'd' | 'D' => 3,
        'e' | 'E' => 4,
        'f' | 'F' => 5,
        'g' | 'G' => 6,
        'h' | 'H' => 7,
        _ => return Err(ConvertStrToU8Error::LetterOutOfRange)
    };
    Ok(Square(vertical, 8 - number as u8))
}

enum ConvertSquareToStrError{
    IncorrectHorizontalInput,
    IncorrectVerticalInput,
}

//Converts coords like (3, 5) (Coords (0, 0) is a8 ) to string like "d3"
fn convert_to_text_notation(square: Square) -> Result<String, ConvertSquareToStrError>{
    let square_x = square.0;
    let square_y = square.1;
    let letter = match square_x{
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => return Err(ConvertSquareToStrError::IncorrectHorizontalInput)
    };
    if square_y > 7 {
        return Err(ConvertSquareToStrError::IncorrectVerticalInput)
    }
    Ok(format!("{}{}", letter, 8 - square_y))
}

//Basically adding coords with range check
fn get_relative_coords(square: Square, offset: RelMov) -> Option<Square>{
    let square_x: i8 = square.0 as i8;
    let square_y: i8 = square.1 as i8;

    if let (0..=7, 0..=7) = (square_x + offset.0, square_y + offset.1){
        Some(Square((square_x + offset.0) as u8, (square_y + offset.1) as u8))
    }else{
        None
    }
}

pub enum ParseFenError{
    InFenStringTurnInvalid,
    InFenStringBoardInvalid,
    InFenStringEnPasssantInvalid,
    InFenHalfmoveClockInvalid,
    InFenMoveclockInvalid
}

impl BoardState{
    pub fn new() -> BoardState{
        BoardState{
            board : START_POS,
            turn: White,
            castling_rights: CastlingRights{wk:true, wq:true, bk:true, bq:true},
            en_passant: None,
            halfmove_clock: 0,
            fullmoves: 1
        }
    }

    fn get(&self, square: Square) -> char{
        self.board[square.0 as usize][square.1 as usize]
    }

    fn put(&mut self, square: Square, figure: char){
        self.board[square.0 as usize][square.1 as usize] = figure;
    }

    //Same as get_relative_coords but returns figure instead of coords
    fn get_relative(&self, square: Square, mov: RelMov) -> Option<char>{
        let square_x = square.0 as i8;
        let square_y = square.1 as i8;

        if square_x + mov.0 < 0 || square_x + mov.0 > 7 ||  square_y + mov.1 < 0 || square_y + mov.1 > 7{
            None
        }else{
            Some(self.board[(square_x + mov.0) as usize][(square_y + mov.1) as usize])
        }
    }

    fn get_relative_pair(&self, square: Square, mov: RelMov) -> Option<(char, Square)>{
        let square_x = square.0 as i8;
        let square_y = square.1 as i8;

        if square_x + mov.0 < 0 || square_x + mov.0 > 7 ||  square_y + mov.1 < 0 || square_y + mov.1 > 7{
            None
        }else{
            Some((self.board[(square_x + mov.0) as usize][(square_y + mov.1) as usize], 
                  Square((square_x + mov.0) as u8, (square_y + mov.1) as u8)))
        }
    }

    fn get_relatives_pairs<'a, I>(&self, square: Square, movs: I) -> Vec<Option<(char, Square)>>
    where 
        I: IntoIterator<Item = &'a RelMov>
    {
        movs.into_iter().map(|x| self.get_relative_pair(square, *x)).collect::<Vec<Option<(char, Square)>>>()
    }

    //Deserialization: parses Forsyth–Edwards Notation into Boardstate
    pub fn parse_fen(&mut self, fen_str: &str) -> Result<(), ParseFenError>{
        let fen_parts: Vec<&str> = fen_str.split_ascii_whitespace().collect();
        for (y, row) in fen_parts[0].split("/").enumerate(){
            for (x, chr) in row.chars().enumerate(){
                if chr.is_digit(10){
                    //TODO find the best way to do this (unwrap, ?, match)?
                    let temp = match chr.to_digit(10){
                        Some(x) => x,
                        None => return Err(ParseFenError::InFenStringBoardInvalid)
                    };
                    if temp > 8 {return Err(ParseFenError::InFenStringBoardInvalid)}
                    for _ in 0..temp{ 
                        self.board[x][y] = '.';
                    }
                }
                else{
                    self.board[x][y] = chr;
                }
            }
        }

        if fen_parts[1].len() > 1 {
            return Err(ParseFenError::InFenStringTurnInvalid);
        }

        self.turn = match fen_parts[1].chars().next(){
            Some('w') | Some('W') => White,
            Some('b') | Some('B') => Black,
            _  => return Err(ParseFenError::InFenStringTurnInvalid)
        };


        self.castling_rights = CastlingRights{
            wk: fen_parts[2].contains('K'),
            bk: fen_parts[2].contains('k'),
            wq: fen_parts[2].contains('Q'),
            bq: fen_parts[2].contains('q'),
        };

        if fen_parts[3] == "-" {
            self.en_passant = None;
        }else{
            let en_passant = convert_str_to_u8(fen_parts[3]);
            if let Ok(x) = en_passant{
                self.en_passant = Some(x);
            }
            else{
                return Err(ParseFenError::InFenStringEnPasssantInvalid);
            }
        }


        if let Ok(x) = fen_parts[4].parse::<u32>(){
            self.halfmove_clock = x;
        }
        else{
            return Err(ParseFenError::InFenHalfmoveClockInvalid);
        }

        if let Ok(x) = fen_parts[5].parse::<u32>(){
            self.fullmoves = x;
        }
        else{
            return Err(ParseFenError::InFenMoveclockInvalid);
        }
        Ok(())
    }

    //TODO find out why std::chunks does not work
    fn chunks(&self) -> Vec<Vec<char>>{
        let mut result: Vec<Vec<char>> = std::vec::Vec::with_capacity(8);
        for i in 0..8{
            result[i] = std::vec::Vec::with_capacity(8);
            for j in 0..8{
                result[i][j] = self.board[i][j];
            }
        }
        result
    }

    pub fn export_to_fen(&self) -> String{
        let mut board_str = std::string::String::new();
        for row in &self.chunks(){
            let mut i = 0;
            while i <= 7 {
                let mut counter = 0;
                while row[i] == '.' && i <= 7 {
                    counter += 1;
                    i += 1
                }
                if counter > 0{
                    board_str.push_str(&*counter.to_string());
                }
                else{
                    board_str.push(row[i] as char);
                }
            }
            board_str.push('/');
        }

        board_str.push(' ');

        let color = match self.turn{
            Black => 'b',
            White => 'w',
        };
        board_str.push(color);

        board_str.push(' ');

        let mut any_castling = true;
        if self.castling_rights.wk {board_str.push('K'); any_castling = false;}
        if self.castling_rights.bk {board_str.push('k'); any_castling = false;}
        if self.castling_rights.wq {board_str.push('Q'); any_castling = false;}
        if self.castling_rights.bq {board_str.push('q'); any_castling = false;}
        if any_castling {board_str.push('-');}

        board_str.push(' ');

        match &self.en_passant{
            // due to way it is created inner state can not be invalid, or it would have returned error before
            Some(x) => board_str.push_str(&*convert_to_text_notation(*x).ok().unwrap()),
            None => board_str.push('-'),
        }

        board_str.push(' ');

        board_str.push_str(&*self.halfmove_clock.to_string());

        board_str.push(' ');

        board_str.push_str(&*self.fullmoves.to_string());
        return board_str
    } 

    fn are_same_color(&self, square1: Square, square2: Square) -> bool{
        self.get(square1).is_uppercase() == self.get(square2).is_uppercase()
    }

    pub fn validate_move(&mut self, player_move: Move, player_color: Color) -> MoveResult{
        match player_move{
            Move::Castling(x) => self.handle_castling(x, player_color),
            Move::Surrender(x) => self.handle_surrender(x),
            Move::Move(sqr1, sqr2) => self.handle_move(sqr1, sqr2, player_color),
        }
    }

    fn commit_castling(&mut self, castling: Castling){
        let horizontal: u8 = match castling{
            Castling::WK | Castling::WQ => 7, 
            Castling::BK | Castling::BQ => 0};
        let (rook_vertical, king_dir): (u8, i8) = match castling{
            Castling::WK | Castling::BK => (7,  1),
            Castling::WQ | Castling::BQ => (0, -1),};
        let king = self.get(Square(horizontal, rook_vertical));
        let rook = self.get(Square(horizontal, 4));
        self.put(Square((4 as i8 + king_dir*2) as u8, horizontal), king);
        self.put(Square((4 as i8 + king_dir) as u8, horizontal), rook);
        self.put(Square(horizontal, rook_vertical), '.');
        self.put(Square(4, horizontal), '.');
    }

    fn handle_castling(&mut self, castling: Castling, player_color: Color) -> MoveResult{
        match player_color{
            White => if let Castling::BK | Castling::BQ = castling{return MoveResult::Invalid},
            Black => if let Castling::WK | Castling::WQ = castling{return MoveResult::Invalid},
        }
        match castling{
            Castling::WK =>{
                if !self.castling_rights.wk{
                    return MoveResult::Invalid
                }else{
                    if  !self.get_all_possible_threats(Square(4, 7)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)) &&
                        !self.get_all_possible_threats(Square(5, 7)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)) &&
                        !self.get_all_possible_threats(Square(6, 7)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)){
                        self.commit_castling(castling);
                        return MoveResult::Valid
                    }else{
                        return MoveResult::Invalid
                    }
                }
            },
            Castling::WQ =>{
                if !self.castling_rights.wq{
                    return MoveResult::Invalid
                }else{
                    if  !self.get_all_possible_threats(Square(2, 7)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)) &&
                        !self.get_all_possible_threats(Square(3, 7)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)) &&
                        !self.get_all_possible_threats(Square(4, 7)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)){
                        self.commit_castling(castling);
                        return MoveResult::Valid
                    }else{
                        return MoveResult::Invalid
                    }
                }
            },
            Castling::BK =>{
                if !self.castling_rights.bk{
                    return MoveResult::Invalid
                }else{
                    if  !self.get_all_possible_threats(Square(4, 0)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)) &&
                        !self.get_all_possible_threats(Square(5, 0)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)) &&
                        !self.get_all_possible_threats(Square(6, 0)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)){
                        self.commit_castling(castling);
                        return MoveResult::Valid
                    }else{
                        return MoveResult::Invalid
                    }
                }
            },
            Castling::BQ =>{                
                if !self.castling_rights.bq{
                return MoveResult::Invalid
            }else{
                if  !self.get_all_possible_threats(Square(2, 0)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)) &&
                    !self.get_all_possible_threats(Square(3, 0)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)) &&
                    !self.get_all_possible_threats(Square(4, 0)).into_iter().any(|x| x.0.is_uppercase() != is_col_uppercase(player_color)){
                    self.commit_castling(castling);
                    return MoveResult::Valid
                }else{
                    return MoveResult::Invalid
                }
            }
            },
        }
    }

    fn handle_move(&mut self, move_from: Square, move_to: Square, player_color: Color) -> MoveResult{
        let figure_from: char = self.get(move_from);
        let figure_to: char = self.get(move_to);

        if figure_from == '.' {return MoveResult::Invalid}; //check if player moves a figure
        if figure_to != '.' && figure_from.is_uppercase() == figure_to.is_uppercase(){
            return MoveResult::Invalid; //check if player takes its own figure
        };
        println!("dsds");
        match player_color{//check if player moves figure that he owns
            White => {
                if figure_from.is_lowercase(){return MoveResult::Invalid};
            },
            Black => {
                if figure_from.is_uppercase(){return MoveResult::Invalid};
            }
        };
        
        let follows_rule: bool = match figure_from.to_uppercase().next().unwrap(){
            'P' => self.pawn_rule(move_from, move_to, player_color),
            'R' => self.rook_rule(move_from, move_to),
            'N' => self.knight_rule(move_from, move_to),
            'B' => self.bishop_rule(move_from, move_to),
            'Q' => self.queen_rule(move_from, move_to),
            'K' => self.king_rule(move_to, player_color),
            _ => unreachable!()
        };
        if follows_rule == false {
            return MoveResult::Invalid
        }else{
            if !self.check_if_safe_for_king(move_from, move_to, player_color){
                return MoveResult::Invalid;
            }
            else{
                self.put(move_to, figure_from);
                self.put(move_from, '.');
            }
        }
        return self.check_mate(!player_color);
    }

    fn handle_surrender(&mut self, player_color: Color) -> MoveResult{
        match player_color{
            White => MoveResult::BlackWin,
            Black => MoveResult::WhiteWin,
        }
    }

    fn pawn_rule(&self, move_from: Square, move_to: Square, player_color: Color) -> bool{
        self.pawn_possible_moves(move_from, player_color).contains(&move_to)
    }

    fn rook_rule(&self, move_from: Square, move_to: Square) -> bool{
        self.rook_possible_moves(move_from).contains(&move_to)
    }

    fn knight_rule(&self, move_from: Square, move_to: Square) -> bool{
        let temp: RelMov = RelMov(move_from.0 as i8 - move_to.0 as i8, move_from.1 as i8 - move_to.1 as i8);
        KNIGHT.iter().any(|arg| *arg == temp)
    }

    fn bishop_rule(&self, move_from: Square, move_to: Square) -> bool{
        self.bishop_possible_moves(move_from).iter().any(|x| *x == move_to)
    }

    fn queen_rule(&self, move_from: Square, move_to: Square) -> bool{
        self.queen_possible_moves(move_from).iter().any(|x| *x == move_to)
    }

    fn king_rule(&self, move_to: Square, player_color: Color) -> bool{
        let color_is_upper = match player_color{White => true, Black => false};
        !self.get_all_possible_threats(move_to).into_iter().any(|x| x.0.is_uppercase() != color_is_upper)
    }

    fn get_king(&self, player_color: Color) -> Square{
        let king = match player_color{White => 'K', Black => 'k'};
        for i in 0..8{
            for j in 0..8{
                if self.board[i][j] == king{
                    return Square(i as u8, j as u8);
                }
            }
        };
        unreachable!()
    }

    fn get_first_from_line(&self, start: Square, increment: RelMov) -> Option<(char, Square)>{
        if let Some(t) = get_relative_coords(start, increment){
            let figure = self.get(t);
            if figure == '.'{
                self.get_first_from_line(t, increment)
            }
            else{
                Some((figure, t))
            }
        }
        else{
            None
        }
    }

    fn get_squares_until(&self , start:Square, increment: RelMov) -> Vec<Square>{
        let mut result: Vec<Square> = std::vec::Vec::with_capacity(8);
        let mut counter = 1;
        while let Some(t) = get_relative_coords(start, RelMov(increment.0*counter, increment.1*counter)){
            let figure = self.get(t);
            if figure == '.'{
                result.push(t);
            }else{
                if self.get(start).is_uppercase() != figure.is_uppercase(){
                    result.push(t);
                }
                break;
            };
            counter += 1;
        }
        result
    }

    fn get_from_diagonals(&self, start: Square) -> Vec<Option<(char, Square)>>{
        let diags = [RelMov(1, 1), RelMov(1, -1), RelMov(-1, 1), RelMov(-1, -1)];
        return diags.iter().map(|x| self.get_first_from_line(start, *x)).collect::<Vec<Option<(char, Square)>>>();
    }

    fn get_from_straights(&self, start: Square) -> Vec<Option<(char, Square)>>{
        let diags = [RelMov(0, 1), RelMov(0, -1), RelMov(-1, 0), RelMov(1, 0)];
        return diags.iter().map(|x| self.get_first_from_line(start, *x)).collect::<Vec<Option<(char, Square)>>>();
    }

    fn get_all_possible_threats(&self, square: Square) -> Vec<(char, Square)>{
        let mut result: Vec<(char, Square)> = std::vec::Vec::with_capacity(16);
        let mut diags = self.get_from_diagonals(square)
                                .into_iter()
                                .filter_map(|x| x)
                                .collect::<Vec<(char, Square)>>();
        let mut straights = self.get_from_straights(square)
                                .into_iter()
                                .filter_map(|x| x)
                                .collect::<Vec<(char, Square)>>();
        let mut knight = self.get_relatives_pairs(square, KNIGHT.iter())
                                .into_iter()
                                .filter_map(|x| x)
                                .collect::<Vec<(char, Square)>>();
        let mut king = self.get_relatives_pairs(square, KING.iter())
                                .into_iter()
                                .filter_map(|x| x)
                                .collect::<Vec<(char, Square)>>();
        diags  =  diags.into_iter().filter(|x| {
                                    x.0.to_uppercase().next().unwrap() == 'Q' || 
                                    x.0.to_uppercase().next().unwrap() == 'B' || 
                                    (x.0.to_uppercase().next().unwrap() == 'P' && 
                                    ((x.1).1 as i8 - square.1 as i8).abs() == 1)})
                                        .collect::<Vec<(char, Square)>>();
        diags  =  diags.into_iter().filter(|x| {
                                    x.0.to_uppercase().next().unwrap() == 'Q' || 
                                    x.0.to_uppercase().next().unwrap() == 'R'})
                                        .collect::<Vec<(char, Square)>>();
        knight = knight.into_iter().filter(|x| x.0.to_uppercase().next().unwrap() == 'N')
                                                .collect::<Vec<(char, Square)>>();
        king   =   king.into_iter().filter(|x| x.0.to_uppercase().next().unwrap() == 'K')
                                                .collect::<Vec<(char, Square)>>();
        result.append(&mut diags); result.append(&mut straights); result.append(&mut knight); result.append(&mut king);
        result
    }

    fn check_if_safe_for_king(&mut self, move_from: Square, move_to: Square, player_color: Color) -> bool{
        let figure_from: char = self.get(move_from);
        let figure_to: char = self.get(move_to);
        self.put(move_to, figure_from);
        self.put(move_from, '.');
        let king_pos = self.get_king(player_color);
        let color_is_upper = match player_color{White => true, Black => false};
        let safe = !self.get_all_possible_threats(king_pos).into_iter().any(|x| x.0.is_uppercase() != color_is_upper);
        self.put(move_to, figure_to);
        self.put(move_from, figure_from);
        safe
    }

    fn check_mate(&mut self, player_to_check: Color) -> MoveResult{
        let moves = self.get_all_possible_moves(player_to_check);
        for mov in moves{
            if self.check_if_safe_for_king(mov.0, mov.1, player_to_check){
                return MoveResult::Valid
            }
        }
        match player_to_check{
            White => MoveResult::BlackWin,
            Black => MoveResult::WhiteWin,
        }
    }

    fn get_all_figures(&self, player_color: Color) -> Vec<(char, Square)>{
        let mut result = std::vec::Vec::<(char, Square)>::with_capacity(16);
        for i in 0..8{
            for j in 0..8{
                let square = Square(i, j);
                let figure = self.get(square);
                if figure != '.' && figure.is_uppercase() == is_col_uppercase(player_color){
                    result.push((figure, square));
                }
            }
        }
        result.shrink_to_fit();
        result
    }

    ///All possible moves except king
    fn get_all_possible_moves(&self, player_color: Color) -> Vec<(Square, Square)>{
        let mut result = std::vec::Vec::<(Square, Square)>::with_capacity(56);
        let figures: Vec<(char, Square)>   =  self.get_all_figures(player_color).iter()
                                                                                .map(|x| ((x.0).to_uppercase().next().unwrap(), x.1))
                                                                                .collect();
        for figure in figures{
            let pos_moves = match figure{
                ('P', x) => self.pawn_possible_moves(x, player_color),
                ('R', x) => self.rook_possible_moves(x),
                ('N', x) => self.knight_possible_moves(x),
                ('B', x) => self.bishop_possible_moves(x),
                ('Q', x) => self.queen_possible_moves(x),
                ('K', _) => std::vec::Vec::with_capacity(0),
                _ => unreachable!()
            };
            result.append(&mut pos_moves.into_iter().zip(std::iter::repeat(figure.1)).collect());
        }
        result.shrink_to_fit();
        result
    }

    fn pawn_possible_moves(&self, square: Square, player_color: Color) -> Vec<Square>{
        let pawn_dir = match player_color{White => -1, Black => 1};
        let mut result = std::vec::Vec::<Square>::with_capacity(4);
        if let Some(x) = get_relative_coords(square, RelMov(0, pawn_dir)){
            if self.get(x) == '.'{
                result.push(x)
            }
            if let Some(y) = get_relative_coords(square, RelMov(0, 2*pawn_dir)){
                if self.get(y) == '.' && square.1 == match player_color{White => 6, Black => 1}{
                    result.push(y)
                }
            }
        }
        if let Some(x) = get_relative_coords(square, RelMov(1, pawn_dir)){
            let figure_to = self.get(x);
            if figure_to != '.' && !self.are_same_color(square, x){
                result.push(x)
            }
        }
        if let Some(x) = get_relative_coords(square, RelMov(-1, pawn_dir)){
            let figure_to = self.get(x);
            if figure_to != '.' && !self.are_same_color(square, x){
                result.push(x)
            }
        }
        result.shrink_to_fit();
        result
    }

    fn knight_possible_moves(&self, square: Square) -> Vec<Square>{
        let knight_move = |x: RelMov| -> Option<Square>{
            if let Some(t) = get_relative_coords(square, x){
                if  self.get_relative(square, x).unwrap() == '.' || 
                    (self.get_relative(square, x).unwrap() != '.' && 
                    !self.are_same_color(square, t)){
                        Some(t)
                    }
                else{
                    None
                }
            }else{
                None
            }
        };
        KNIGHT.iter().map(|x| knight_move(*x)).filter_map(|x| x).collect()
    }

    fn bishop_possible_moves(&self, square: Square) -> Vec<Square>{
        let diagonals = [RelMov(1, 1), RelMov(1, -1), RelMov(-1, 1), RelMov(-1, -1)];
        let mut result = std::vec::Vec::<Square>::with_capacity(14);
        for diag in diagonals.iter(){
            result.append(&mut self.get_squares_until(square, *diag));
        }
        result.shrink_to_fit();
        result
    }

    fn rook_possible_moves(&self, square: Square) -> Vec<Square>{
        let straights = [RelMov(0, 1), RelMov(0, -1), RelMov(1, 0), RelMov(-1, 0)];
        let mut result = std::vec::Vec::<Square>::with_capacity(14);
        for diag in straights.iter(){
            result.append(&mut self.get_squares_until(square, *diag));
        }
        result.shrink_to_fit();
        result
    }

    fn queen_possible_moves(&self, square: Square) -> Vec<Square>{
        let mut result = std::vec::Vec::<Square>::with_capacity(28);
        result.append(&mut self.bishop_possible_moves(square));
        result.append(&mut self.rook_possible_moves(square));
        result.shrink_to_fit();
        result
    }
}

///TEMPORARY
impl std::fmt::Display for BoardState{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "\n  ")?;
        for i in 0..8{
            write!(f, "{}", (65 + i as u8)as char)?;
        }
        write!(f, "\n")?;
        for i in 0..8{
            write!(f, "{} ", 8 - i)?;
            for j in 0..8{
                write!(f, "{}", self.get(Square(j, i)))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

use std::process::Command;
use std::io::{self, Read};
fn main(){
    let mut board = BoardState::new();
    let mut color: Color = White;
    println!("{}", board);
    loop{
        let mut buffer = std::string::String::new();
        io::stdin().read_line(&mut buffer);
        let mut itr = buffer.split_ascii_whitespace();
        let from = convert_str_to_u8(itr.next().unwrap()).ok().unwrap();
        let to = convert_str_to_u8(itr.next().unwrap()).ok().unwrap();
        let result = board.validate_move(Move::Move(from, to), color);
        if let MoveResult::Valid = result{
            color = !color;  
        }
        println!("{}\n{:?}\n{} {} : {} {}\n{} {}", board, 
                                                    result, 
                                                    from.0, 
                                                    from.1, 
                                                    to.0, 
                                                    to.1, 
                                                    board.get(from), 
                                                    board.get(to));
    }
}

#[test]
    fn test_convert_str_to_u8() {
        let temp = convert_str_to_u8("a8").ok().unwrap();
        assert_eq!((temp.0, temp.1), (0, 0));
        let temp = convert_str_to_u8("h1").ok().unwrap();
        assert_eq!((temp.0, temp.1), (7, 7));
        let temp = convert_str_to_u8("e2").ok().unwrap();
        assert_eq!((temp.0, temp.1), (4, 6));
        let temp = convert_str_to_u8("e4").ok().unwrap();
        assert_eq!((temp.0, temp.1), (4, 4));
        let temp = convert_str_to_u8("c7").ok().unwrap();
        assert_eq!((temp.0, temp.1), (2, 1));
    }
#[test]
fn test_convert_to_text_notation(){
    assert_eq!(convert_to_text_notation(Square(0, 0)).ok().unwrap(), "a8");
    assert_eq!(convert_to_text_notation(Square(7, 7)).ok().unwrap(), "h1");
    assert_eq!(convert_to_text_notation(Square(4, 4)).ok().unwrap(), "e4");
}

#[test]
fn test_get_relative_coords(){
    let board = BoardState::new();
    assert_eq!(get_relative_coords(Square(4, 6), RelMov(0, -1)), Some(Square(4, 5)));
}
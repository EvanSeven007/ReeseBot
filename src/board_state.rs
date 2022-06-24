use crate::square::*;
use crate::piece::*;
use crate::color::*;
use crate::chess_move::*;
use crate::move_gen::*;
use num::{abs};

#[derive(Clone, Copy)]
pub struct CastleRights {
    pub can_castle_white_kingside: bool,
    pub can_castle_white_queenside: bool,
    pub can_castle_black_kingside: bool, 
    pub can_castle_black_queenside: bool,
}
/* A board is a 8x8 array of squares */
#[derive(Clone, Copy)]
pub struct BoardState {
    pub squares: [[Square; 10]; 10],
    pub active_color: Color, 
    pub castle_rights: CastleRights,
    pub en_passant: Option<Position>,
}

impl BoardState { 
    /* Creates a board state from a FEN string */
    pub fn new(fen: &str) -> Result<BoardState, &str> {
        //Creating an 8x8 array of uninitialized arrays
        let mut squares = [[Square {piece: None, color: (Color::White) }; 10]; 10]; //Setting to white and then updating later
        //Assigning colors, but not charged
        for index in 1..9 {
            for inner_index in 1..9 {
                squares[index][inner_index].color = BoardState::get_color(&index, &inner_index);
            }
        }

        let fen = fen.to_string();

        let fen_strings: Vec<&str> = fen.split(' ').collect();
        if fen_strings.len() != 6 {
            return Err("Invalid fen string!");
        }

        let position_str: Vec<&str> = fen_strings[0].split('/').collect();
        let mut col: usize;
        let mut row_string: &str; //String that stores the current row info
        
        for row in 0..8 {
            row_string = position_str[row];
            col = 1;
            for fen_entry in row_string.chars() {
                if fen_entry.is_digit(10) {
                    col += fen_entry.to_digit(10).unwrap() as usize;
                } else {
                    squares[row + 1][col].piece = BoardState::parse_fen_entry(&fen_entry).unwrap();
                    col += 1;
                }
            }
        }

        let active_color = match fen_strings[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("fen string color malformed!"),
        };

        let mut can_castle_white_kingside = false;
        let mut can_castle_white_queenside = false;
        let mut can_castle_black_kingside = false;
        let mut can_castle_black_queenside = false;
        
        /* This is unforgiveably stupid */
        for fen_entry in fen_strings[2].chars() {
            match fen_entry {
                'K' => can_castle_white_kingside = true,
                'Q' => can_castle_white_queenside = true,
                'k' => can_castle_black_kingside = true,
                'q' => can_castle_black_queenside = true,
                '-' => {},
                _ => panic!("fen string castling info is malformed!"),
            }
        }

        let castle_rights: CastleRights = CastleRights {
            can_castle_white_kingside,
            can_castle_white_queenside,
            can_castle_black_kingside,
            can_castle_black_queenside
        };

        //Variables for enpassant goodness
        let en_passant: Option<Position>;
        let x: usize;
        let y: usize;
        if fen_strings[3].len() == 1 && fen_strings[3] == "-" {
            en_passant = None;
        } else if fen_strings[3].len() == 2 { 
            /* Parse enpassant string */
            let en_passant_string: Vec<char> = fen_strings[3].chars().collect();
            match en_passant_string[0] {
                'a' => x = 1,
                'b' => x = 2,
                'c' => x = 3,
                'd' => x = 4,
                'e' => x = 5,
                'f' => x = 6,
                'g' => x = 7,
                'h' => x = 8,
                _ => panic!("fen string enpassant malformed!"),
            };
            y = (9 - en_passant_string[1]
                .to_digit(10)
                .unwrap_or_else(|| panic!("fen string enpassant malformed!"))) as usize;

            if !(1..9).contains(&y) {
                return Err("fen string enpassant malformed!")
            }
            en_passant = Some(Position{ x , y }.swap()); //Accounting for how we index array
        
        } else {
            return Err("fen string enpassant malformed!")
        }

        Ok(BoardState { squares, active_color, castle_rights, en_passant})
    }


    fn parse_fen_entry(entry: &char) -> Result<Option<Piece>, &str> {
        match entry {
            'r' => Ok(Some(Piece {piece_type: PieceType::Rook, color: Color::Black})),
            'n' => Ok(Some(Piece {piece_type: PieceType::Knight, color: Color::Black})),
            'b' => Ok(Some(Piece {piece_type: PieceType::Bishop, color: Color::Black})),
            'q' => Ok(Some(Piece {piece_type: PieceType::Queen, color: Color::Black})),
            'k' => Ok(Some(Piece {piece_type: PieceType::King, color: Color::Black})),
            'p' => Ok(Some(Piece {piece_type: PieceType::Pawn, color: Color::Black})),
            'R' => Ok(Some(Piece {piece_type: PieceType::Rook, color: Color::White})),
            'N' => Ok(Some(Piece {piece_type: PieceType::Knight, color: Color::White})),
            'B' => Ok(Some(Piece {piece_type: PieceType::Bishop, color: Color::White})),
            'Q' => Ok(Some(Piece {piece_type: PieceType::Queen, color: Color::White})),
            'K' => Ok(Some(Piece {piece_type: PieceType::King, color: Color::White})),
            'P' => Ok(Some(Piece {piece_type: PieceType::Pawn, color: Color::White})),
            _ => Err("Not a valid fen piece!"),
        }
    }
    

    /* Gets the color of a board from its coordinates */
    fn get_color(val1: &usize, val2: &usize) -> Color {
        if (val1 > &8 || val2 > &8) {
            panic!("Not a valid coordinate {} {}", val1, val2);
        }

        let val1_is_odd: bool = val1 % 2 == 0;
        let val2_is_odd: bool = val2 % 2 == 0;

        if (val1_is_odd && val2_is_odd) || (!val1_is_odd && !val2_is_odd) {
            Color::White
        } else {
            Color::Black
        }
    }

    /* Updates a board state given a move, which was already been prechecked to be valid */
    pub fn make_move(&mut self, current_move: &Move) {
        let move_type: &MoveType = &current_move.move_type;
        self.en_passant = None; //Reseting en_passant square to None after every move, this will be updated later depending on move
        match move_type {
            MoveType::standard(val) => {
                self.squares[val.before.x][val.before.y].piece = None;
                self.squares[val.after.x][val.after.y].piece = Some(val.piece_moved);
                //Setting enpassant 
                if val.piece_moved.piece_type == PieceType::Pawn && abs(val.after.x as i8 - val.before.x as i8) == 2 {
                    self.en_passant = Some(val.after);
                }
            },

            MoveType::castle(val) => {
                let y_positions: Vec<usize>;
                if val.is_kingside  {
                    y_positions = vec![5,8,7,6];
                }
                else {
                    y_positions = vec![5,1,3,4];
                }

                self.squares[8][y_positions[0]].piece = None;
                self.squares[8][y_positions[1]].piece = None;
                self.squares[8][y_positions[2]].piece = Some(Piece {piece_type: PieceType::King, color: self.active_color });
                self.squares[8][y_positions[3]].piece = Some(Piece {piece_type: PieceType::Rook, color: self.active_color });

            },
            MoveType::promotion(val) => {
                self.squares[val.before.x][val.before.y].piece = None;
                self.squares[val.after.x][val.after.y].piece = Some(val.promote_to);
            }
            MoveType::enPassant(val) => {
                self.squares[val.before.x][val.before.y].piece = None;
                self.squares[val.after.x][val.after.y].piece = Some(Piece{piece_type: PieceType::Pawn, color: self.active_color});
                self.squares[val.en_passant.x][val.en_passant.y].piece = None;
            }
        }

        //Changing color
        match self.active_color {
            Color::Black => self.active_color = Color::White,
            Color::White => self.active_color = Color::Black, 
        };
    }
    
    /* Checks if the king is in check given a certain position 
        Returns True if the king of active color is in check, false otherwise
    */
    pub fn is_in_check(self: BoardState) -> bool {
        false //Need refactor
        
        /* 
        * Looking for check on the diagonals 
        */
        //Checking pawn moves
        /*
        let board = self;
        let mut king_pos_opt: Option<Position> = None;
        let king_pos: Position;
        for i in 1..9 {
            for j in 1..9 {
                if board.squares[i][j].is_occupied() {
                    let p = board.squares[i][j].piece.unwrap();
                    if p.piece_type == PieceType::King && p.color == board.active_color {
                        king_pos_opt= Some(Position{x: i, y: j});
                        break;
                    }
                }
            }
        }

        king_pos = king_pos_opt.expect("Could not find the king!");

        let pawn_square_right: Position;
        let pawn_square_left: Position;
        match board.active_color {
            Color::White => {
                pawn_square_right = Position{x: king_pos.x - 1 , y: king_pos.y - 1};
                pawn_square_left = Position{x: king_pos.x - 1 , y: king_pos.y + 1};
            },
            Color::Black => {
                pawn_square_right = Position{x: king_pos.x + 1 , y: king_pos.y - 1};
                pawn_square_left = Position{x: king_pos.x + 1 , y: king_pos.y + 1};
            },
        }
        
        /* Checking for checks by pawns */
        if pawn_square_right.is_valid_position() {
            if board.squares[pawn_square_right.x][pawn_square_right.y].is_occupied() {
                let p = board.squares[pawn_square_right.x][pawn_square_right.y].piece.unwrap();
                if p.piece_type == PieceType::Pawn && p.color != board.active_color {
                    println!("In check by pawn");
                    return true;
                }
            }
        }

        if pawn_square_left.is_valid_position() {
            if board.squares[pawn_square_left.x][pawn_square_left.y].is_occupied() {
                let p = board.squares[pawn_square_left.x][pawn_square_left.y].piece.unwrap();
                if p.piece_type == PieceType::Pawn && p.color != board.active_color {
                    println!("in check by pawn");
                    return true;
                }
            }
        }

        /* Checking rank checks */
        let mut candidates = generate_rook_moves_helper(&board, &king_pos, board.active_color);
        for mv in candidates {
            if board.squares[mv.x][mv.y].is_occupied() {
                let p = board.squares[mv.x][mv.y].piece.unwrap();
                if p.color != board.active_color {
                    match p.piece_type {
                        PieceType::Rook => {
                            println!("In check by rook");
                            return true
                        },
                        PieceType::Queen => {
                            println!("In check by queen");
                            return true
                        },
                        _ => {},
                    }
                }
            }
        }

        /* Checking check by diagonals */
        candidates = generate_bishop_moves_helper(&board, &king_pos, board.active_color);
        for mv in candidates {
            if board.squares[mv.x][mv.y].is_occupied() {
                let p = board.squares[mv.x][mv.y].piece.unwrap();
                if p.color != board.active_color {
                    match p.piece_type {
                        PieceType::Bishop => {
                            println!("In check by bishop");
                            return true
                        },
                        PieceType::Queen => {
                            println!("In check by queen");
                            return true
                        },
                        _ => {},
                    }
                }
            }
        }

        /* Looking for checks by knight */
        candidates =  generate_knight_moves_helper(&king_pos);
        for mv in candidates {
            if !mv.is_valid_position() {
                continue;
            }
            if board.squares[mv.x][mv.y].is_occupied() {
                let p = board.squares[mv.x][mv.y].piece.unwrap();
                if p.piece_type == PieceType::Knight && p.color != board.active_color {
                    println!("In check by knight");
                    return true;
                }
            }
        }

        /* Looking for checks by king */
        candidates = generate_king_moves_helper(&board, &king_pos);

        for mv in candidates {
            if !mv.is_valid_position() {
                continue;
            }

            if board.squares[mv.x][mv.y].is_occupied() {
                let p = board.squares[mv.x][mv.y].piece.unwrap();
                if p.piece_type == PieceType::King && p.color != board.active_color {
                    println!("In check by King");
                    return true;
                }
            }
        }


        return false;
        */
    }

    pub fn print_board(&self) {
        for index in 1..9 {
            print!("[{}]", index);
            for inner_index in 1..9 {
                print!("{}", self.squares[index][inner_index].symbol());
            }
            print!("\n");
        }
        println!("   [1][2][3][4][5][6][7][8]");
    }
}

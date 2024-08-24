/* This crate encapsualtes a board state for a chess game */
use crate::square::*;
use crate::piece::*;
use crate::color::*;
use crate::chess_move::*;
use crate::move_gen::{knight_positions, king_positions};
use num::abs;

///Bools that describe which side can castle at any given point in time
#[derive(Clone, Copy)]
pub struct CastleRights {
    pub can_castle_white_kingside: bool,
    pub can_castle_white_queenside: bool,
    pub can_castle_black_kingside: bool, 
    pub can_castle_black_queenside: bool,
}
///A boardstate is a 12x12 filled with Piece Structs. Active color is the color whose turn it is to play. en_passant is the position of a pawn that just moved up two squares. 
#[derive(Clone, Copy)]
pub struct BoardState {
    pub squares: [[Square; 12]; 12],
    pub active_color: Color, 
    pub castle_rights: CastleRights,
    pub en_passant: Option<Position>,
    pub last_move: Option<Move>
}

impl BoardState { 
    /* Creates a board state from a FEN string */
    pub fn new(fen: &str) -> Result<BoardState, &str> {

        //Creating an 12x12 array of uninitialized arrays
        //The chess board will sit in the center, with two squares of "boundary" around them. This is so we don't have to deal with out of array errors later on
        let mut squares = [[Square {piece: None, color: (Color::White) }; 12]; 12]; //Setting to white and then updating later
        //Assigning colors, but not charged
        for index in 2..10 {
            for inner_index in 2..10 {
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
            col = 2;
            for fen_entry in row_string.chars() {
                if fen_entry.is_digit(10) {
                    col += fen_entry.to_digit(10).unwrap() as usize;
                } else {
                    squares[row + 2][col].piece = BoardState::parse_fen_entry(&fen_entry).unwrap();
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

            if !(1..=8).contains(&y) {
                return Err("fen string enpassant malformed!")
            }
            en_passant = Some(Position{ row: x, col: y }.swap()); //Accounting for how we index array
        
        } else {
            return Err("fen string enpassant malformed!")
        }


        Ok(BoardState { squares, active_color, castle_rights, en_passant, last_move: None})
    }

    //Creates a Piece from a fen string representation of said piece
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
        if !(2..=9).contains(val1) || !(2..=9).contains(val2) {
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
        self.last_move = Some(*current_move);
        let move_type: &MoveType = &current_move.move_type;
        self.en_passant = None; //Reseting en_passant square to None after every move, this will be updated later depending on move
        
        match move_type {
            MoveType::Standard(val) => {
                //Moving the piece
                self.squares[val.before.row][val.before.col].piece = None;
                self.squares[val.after.row][val.after.col].piece = Some(val.piece_moved);

                //Setting enpassant if we moved a pawn 
                match val.piece_moved.piece_type {
                    PieceType::Pawn => {
                        if abs(val.after.row as i8 - val.before.row as i8) == 2 {
                            self.en_passant = Some(val.after);
                        }
                    }, 
                    //removing Castling Rights if we move the king
                    PieceType::King => {
                        match val.piece_moved.color {
                            Color::White => {
                                self.castle_rights.can_castle_white_kingside = false;
                                self.castle_rights.can_castle_white_queenside = false;
                            }, 
                            Color::Black => {
                                self.castle_rights.can_castle_black_kingside = false;
                                self.castle_rights.can_castle_black_queenside = false;
                            }
                        }
                    },
                    //removing castling rights if we move the rook
                    PieceType::Rook => {
                        match val.piece_moved.color {
                            Color::White => {
                                //Were the rooks on default positions?
                                match val.before {
                                    Position{row: 9, col: 2} => self.castle_rights.can_castle_white_queenside = false,
                                    Position{row: 9, col: 9} => self.castle_rights.can_castle_white_kingside = false,
                                    _ => {}
                                }
                            }, 
                            Color::Black => {
                                match val.before {
                                    //Were the rooks on default positions?
                                    Position{row: 2, col: 2} => self.castle_rights.can_castle_black_queenside = false,
                                    Position{row: 2, col: 9} => self.castle_rights.can_castle_black_kingside = false,
                                    _ => {}
                                }
                            }
                        }
                    },
                    _ => {}
                }

                //Removing castling rights if a rook is captured
                if let Some(piece) = current_move.piece_captured {
                    if piece.piece_type == PieceType::Rook {
                        match val.after {
                            //Default positions of the rooks
                            Position{row: 2, col: 2} => self.castle_rights.can_castle_black_queenside = false,
                            Position{row: 2, col: 9} => self.castle_rights.can_castle_black_kingside = false,
                            Position{row: 9, col: 2} => self.castle_rights.can_castle_white_queenside = false,
                            Position{row: 9, col: 9} => self.castle_rights.can_castle_white_kingside = false,
                            _ => {}
                            } 
                        }
                    }
            },

            MoveType::Castle(val) => {
                let x_position;
                let y_positions: Vec<usize>;
                if val.is_kingside  {
                    y_positions = vec![6,9,8,7];
                }
                else {
                    y_positions = vec![6,2,4,5];
                }
                //Set castling rights here
                match val.color {
                    Color::White => {
                        x_position = 9; //First row
                        self.castle_rights.can_castle_white_kingside = false;
                        self.castle_rights.can_castle_white_queenside = false;
                    }
                    Color::Black => {
                        x_position = 2; //Last row
                        self.castle_rights.can_castle_black_kingside = false;
                        self.castle_rights.can_castle_black_queenside = false;
                    }
                }
                self.squares[x_position][y_positions[0]].piece = None;
                self.squares[x_position][y_positions[1]].piece = None;
                self.squares[x_position][y_positions[2]].piece = Some(Piece {piece_type: PieceType::King, color: self.active_color });
                self.squares[x_position][y_positions[3]].piece = Some(Piece {piece_type: PieceType::Rook, color: self.active_color });

            },
            MoveType::Promotion(val) => {
                self.squares[val.before.row][val.before.col].piece = None;
                self.squares[val.after.row][val.after.col].piece = Some(val.promote_to);
            }
            MoveType::EnPassant(val) => {
                self.squares[val.before.row][val.before.col].piece = None;
                self.squares[val.after.row][val.after.col].piece = Some(Piece{piece_type: PieceType::Pawn, color: self.active_color});
                self.squares[val.en_passant_pos.row][val.en_passant_pos.col].piece = None;
            }
        }

        //Changing color
        match self.active_color {
            Color::Black => self.active_color = Color::White,
            Color::White => self.active_color = Color::Black, 
        };
    }
    
    /* 
    * Checks whether or not a given square is under attack from enemy pieces. 
    * Passed_king_pos is the position from which it will check if it is under attack, if this is None it will find the king manually
    * This can be used for more generally just checking if a king is under attack, for example in determining if castling is possible
    */
    pub fn is_in_check(self: BoardState, color: Color, passed_king_pos: Option<Position>) -> bool {
        //White makes move -> black is active color, check if whtie is in check
        //Finding the king
        let king_pos: Position;
        
        //Finding the king if there is no passed Value
        if let Some(val) = passed_king_pos {
            king_pos = val;
        } else {
            let mut king_pos_opt: Option<Position> = None;
            for x in 2..10 {
                for y in 2..10 {
                    if let Some(piece) = self.squares[x][y].piece {
                        if piece.piece_type == PieceType::King && piece.color == color {
                            king_pos_opt = Some(Position{row: x, col: y});
                        }
                    }
                }
            }
            king_pos = king_pos_opt.expect("Could not find the king!"); //might change this to true?
        }

        //Checking by rook/Queen
        let mut next_pos = king_pos.clone();
        for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            while next_pos.next_position(&dir).is_valid_position() {
                next_pos = next_pos.next_position(&dir);
                if let Some(piece) = self.squares[next_pos.row][next_pos.col].piece {
                    if piece.color != color && (piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen) {
                        return true;
                    }
                    break;
                }
            }
            next_pos = king_pos.clone();
        }


        //Checking by bishop/Queen
        let mut next_pos = king_pos.clone();
        for dir in [Direction::UpRight, Direction::DownRight, Direction::UpLeft, Direction::DownLeft] {
            while next_pos.next_position(&dir).is_valid_position() {
                next_pos = next_pos.next_position(&dir);
                if let Some(piece) = self.squares[next_pos.row][next_pos.col].piece{
                    if piece.color != color && (piece.piece_type == PieceType::Bishop || piece.piece_type == PieceType::Queen) {
                        return true;
                    }
                    break;   
                }
            }
            next_pos = king_pos.clone();
        }

        //Checking for pawn 
        let square_right;
        let square_left;
        match color {
            Color::White => {
                square_right = self.squares[king_pos.up().right().row][king_pos.up().right().col];
                square_left = self.squares[king_pos.up().left().row][king_pos.up().left().col];
            },
            Color::Black => {
                square_right = self.squares[king_pos.down().right().row][king_pos.up().right().col];
                square_left = self.squares[king_pos.down().left().row][king_pos.down().left().col];
            }
        }

        for square in vec![square_right, square_left] {
            if let Some(piece) = square.piece {
                if piece.piece_type == PieceType::Pawn && piece.color != color {
                    return true;
                }
            }
        }

        //Checking for Knight
        for pos in knight_positions(king_pos) {
            if let Some(piece) = self.squares[pos.row][pos.col].piece {
                if piece.piece_type == PieceType::Knight && piece.color != color {
                    return true;
                }
            }
        }

        //Checking for king
        for pos in king_positions(king_pos) {
            if let Some(piece) = self.squares[pos.row][pos.col].piece {
                if piece.piece_type == PieceType::King && piece.color != color {
                    return true;
                }
            }
        }

        false
    }

    //Prints the board to the screen
    pub fn print_board(self) {
        for index in 2..10 {
            print!("[{}]", 10 - index);
            for inner_index in 2..10{
                print!("{}", self.squares[index][inner_index].symbol());
            }
            print!("\n");
        }
        println!("   [a][b][c][d][e][f][g][h]");
    }
}

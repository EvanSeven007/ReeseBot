use crate::square::*;
use crate::piece::*;
use crate::color::*;
use crate::chess_move::*;
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
    pub squares: [[Square; 12]; 12],
    pub active_color: Color, 
    pub castle_rights: CastleRights,
    pub en_passant: Option<Position>,
}

impl BoardState { 
    /* Creates a board state from a FEN string */
    pub fn new(fen: &str) -> Result<BoardState, &str> {
        //Creating an 12x12 array of uninitialized arrays
        //The chess board will sit in the center, with two squares of "boundary" around them. This is so we don't have to deal with out of array errors later on
        let mut squares = [[Square {piece: None, color: (Color::White) }; 12]; 12]; //Setting to white and then updating later
        //Assigning colors, but not charged
        for index in 2..10 {
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
        if !(1..=9).contains(val1) || !(1..=9).contains(val2) {
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
            MoveType::Standard(val) => {
                self.squares[val.before.x][val.before.y].piece = None;
                self.squares[val.after.x][val.after.y].piece = Some(val.piece_moved);
                //Setting enpassant 
                if val.piece_moved.piece_type == PieceType::Pawn && abs(val.after.x as i8 - val.before.x as i8) == 2 {
                    self.en_passant = Some(val.after);
                }
                match val.piece_moved.piece_type {
                    //Setting enPassant
                    PieceType::Pawn => {
                        if abs(val.after.x as i8 - val.before.x as i8) == 2 {
                            self.en_passant = Some(val.after);
                        }
                    }, 
                    //removing Castling Rights
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
                    //removing castling rights
                    PieceType::Rook => {
                        match val.piece_moved.color {
                            Color::White => {
                                //Were the rooks on default positions?
                                match val.before {
                                    Position{x: 8, y: 1} => self.castle_rights.can_castle_white_queenside = false,
                                    Position{x: 8, y: 8} => self.castle_rights.can_castle_white_kingside = false,
                                    _ => {}
                                }
                            }, 
                            Color::Black => {
                                match val.before {
                                    //Were the rooks on default positions?
                                    Position{x: 1, y: 1} => self.castle_rights.can_castle_black_queenside = false,
                                    Position{x: 1, y: 8} => self.castle_rights.can_castle_black_kingside = false,
                                    _ => {}
                                }
                            }
                        }
                    },
                    _ => {}
                }
            },

            MoveType::Castle(val) => {
                let y_positions: Vec<usize>;
                if val.is_kingside  {
                    y_positions = vec![5,8,7,6];
                }
                else {
                    y_positions = vec![5,1,3,4];
                }
                //Set castling rights here
                match val.color {
                    Color::White => {
                        self.castle_rights.can_castle_white_kingside = false;
                        self.castle_rights.can_castle_white_queenside = false;
                    }
                    Color::Black => {
                        self.castle_rights.can_castle_black_kingside = false;
                        self.castle_rights.can_castle_black_queenside = false;
                    }
                }
                self.squares[8][y_positions[0]].piece = None;
                self.squares[8][y_positions[1]].piece = None;
                self.squares[8][y_positions[2]].piece = Some(Piece {piece_type: PieceType::King, color: self.active_color });
                self.squares[8][y_positions[3]].piece = Some(Piece {piece_type: PieceType::Rook, color: self.active_color });

            },
            MoveType::Promotion(val) => {
                self.squares[val.before.x][val.before.y].piece = None;
                self.squares[val.after.x][val.after.y].piece = Some(val.promote_to);
            }
            MoveType::EnPassant(val) => {
                self.squares[val.before.x][val.before.y].piece = None;
                self.squares[val.after.x][val.after.y].piece = Some(Piece{piece_type: PieceType::Pawn, color: self.active_color});
                self.squares[val.en_passant_pos.x][val.en_passant_pos.y].piece = None;
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
        //White makes move -> black is active color, check if whtie is in check
        //Finding the king
        let king_pos: Position;
        let mut king_pos_opt: Option<Position> = None;

        for x in 2..10 {
            for y in 2..10 {
                if let Some(piece) = self.squares[x][y].piece {
                    if piece.piece_type == PieceType::King && piece.color == self.active_color.opposite() {
                        king_pos_opt = Some(Position{x, y});
                    }
                }
            }
        }

        king_pos = king_pos_opt.expect("Could not find the king!");

        //Checking by rook/Queen
        let mut next_pos = king_pos.clone();
        for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            while next_pos.next_position(&dir).is_valid_position() {
                next_pos = next_pos.next_position(&dir);
                if let Some(piece) = self.squares[next_pos.x][next_pos.y].piece {
                    if piece.color == self.active_color && (piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen) {
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
                if let Some(piece) = self.squares[next_pos.x][next_pos.y].piece{
                    if piece.color == self.active_color && (piece.piece_type == PieceType::Bishop || piece.piece_type == PieceType::Queen) {
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
        match self.active_color.opposite() {
            Color::White => {
                square_right = self.squares[king_pos.up().right().x][king_pos.up().right().y];
                square_left = self.squares[king_pos.up().left().x][king_pos.up().left().y];
            },
            Color::Black => {
                square_right = self.squares[king_pos.down().right().x][king_pos.up().right().y];
                square_left = self.squares[king_pos.down().left().x][king_pos.down().left().y];
            }
        }
        for square in vec![square_right, square_left] {
            if let Some(piece) = square.piece {
                if piece.piece_type == PieceType::Pawn && piece.color == self.active_color {
                    return true;
                }
            }
        }

        //Checking for knight
        let possible_knight_positions: Vec<Position> = vec![
            king_pos.up().up().right(),
            king_pos.up().up().left(),
            king_pos.down().down().right(),
            king_pos.down().down().left(),
            king_pos.left().left().up(),
            king_pos.left().left().down(),
            king_pos.right().right().up(),
            king_pos.right().right().down(),
        ];

        for pos in possible_knight_positions {
            if let Some(piece) = self.squares[pos.x][pos.y].piece {
                if piece.piece_type == PieceType::Knight && piece.color == self.active_color {
                    return true;
                }
            }
        }

        //Checking for king checks
        let possible_king_positions: Vec<Position> = vec![
            king_pos.up(),
            king_pos.down(),
            king_pos.left(),
            king_pos.right(),
            king_pos.up().right(),
            king_pos.up().left(),
            king_pos.down().left(),
            king_pos.down().right(),
        ];

        for pos in possible_king_positions {
            if let Some(piece) = self.squares[pos.x][pos.y].piece {
                if piece.piece_type == PieceType::King && piece.color == self.active_color {
                    return true;
                }
            }
        }

        false
    }

    pub fn print_board(&self) {
        for index in 2..10 {
            print!("[{}]", index);
            for inner_index in 2..10 {
                print!("{}", self.squares[index][inner_index].symbol());
            }
            print!("\n");
        }
        println!("   [1][2][3][4][5][6][7][8]");
    }

    pub fn switch_color(mut self) {
        self.active_color = self.active_color.opposite();
    }
}

use crate::square::*;
use crate::piece::*;
use crate::color::*;
use crate::chess_move::*;

/* A board is a 8x8 array of squares */
pub struct BoardState {
    pub squares: [[Square; 8]; 8],
    pub active_color: Color, 
    pub can_castle_white_kingside: bool,
    pub can_castle_white_queenside: bool,
    pub can_castle_black_kingside: bool, 
    pub can_castle_black_queenside: bool,
    pub en_passant: Option<Position>,
    //Todo halfmove
    //Todo full move
    
}

impl BoardState {
    /* Creating an empty board */

    /* Creates a board state from a FEN string */
    pub fn new(fen: &str) -> Result<BoardState, &str> {
        //Creating an 8x8 array of uninitialized arrays
        let mut squares = [[Square {piece: None, color: (Color::White) }; 8]; 8]; //Setting to white and then updating later
        //Assigning colors, but not charged
        for index in 0..8 {
            for inner_index in 0..8 {
                squares[index][inner_index].color = BoardState::get_color(&index, &inner_index);
            }
        }

        let mut fen = fen.to_string();
        //Stupid way to get rid of new line characters
        if fen.ends_with('\n') {
            fen.pop();
            if fen.ends_with('\r') {
                fen.pop();
            }
        }

        let fen_strings: Vec<&str> = fen.split(' ').collect();
        if fen_strings.len() != 6 {
            return Err("Invalid fen string!");
        }

        let position_str: Vec<&str> = fen_strings[0].split('/').collect();
        let mut col: usize;
        let mut row_string: &str; //String that stores the current row info
        for row in 0..8 {
            row_string = position_str[row];
            col = 0;
            for fen_entry in row_string.chars() {
                if fen_entry.is_digit(10) {
                    col += fen_entry.to_digit(10).unwrap() as usize;
                } else {
                    squares[row][col].piece = BoardState::parse_fen_entry(&fen_entry).unwrap();
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
        //Variables for enpassant goodness
        let mut en_passant: Option<Position>;
        let x: usize;
        let y: usize;
        if fen_strings[3].len() == 1 && fen_strings[3] == "-" {
            en_passant = None;
        } else if fen_strings[3].len() == 2 { 
            let en_passant_string: Vec<char> = fen_strings[3].chars().collect();
            match en_passant_string[0] {
                'a' => x = 0,
                'b' => x = 1,
                'c' => x = 2,
                'd' => x = 3,
                'e' => x = 4,
                'f' => x = 5,
                'g' => x = 6,
                'h' => x = 7,
                _ => panic!("fen string enpassant malformed!"),
            };
            /* This is just atrocious Evan, fix this with an unwrap or else*/
            if !en_passant_string[1].is_digit(10) {
                return Err("fen string enpassant malformed!")
            }
            y = (en_passant_string[1].to_digit(10).unwrap() - 1) as usize; /* Potential off by one bug */
            if y > 7 {
                return Err("fen string enpassant malformed!")
            }
            en_passant = Some(Position{ x , y });
        } else {
            return Err("fen string enpassant malformed!")
        }

        Ok(BoardState { squares, active_color, can_castle_white_kingside, can_castle_white_queenside, can_castle_black_kingside, can_castle_black_queenside, en_passant })
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
        match val1 {
            0 | 2 | 4 | 6 => {
                match val2 {
                    1 | 3 | 5 | 7 => Color::White,
                    0 | 2 | 4 | 6 => Color::Black,
                    _ => panic!("not a valid coordinate! {} {}", val1, val2),
                }
            }
            _ => {
                match val2 {
                    1 | 3 | 5 | 7 => Color::Black,
                    0 | 2 | 4 | 6 => Color::White,
                    _ => panic!("not a valid coordinate! {} {}", val1 + 1, val2 + 1),
                }
            }
        }
    }

    /* Updates a board state given a move, which was already been prechecked to be valid */
    pub fn make_move(&mut self, current_move: Move) {
        let move_type: MoveType = current_move.move_type;
        match move_type {
            /* How to get value of enumeration? */
            MoveType::standard(val) => {
                self.squares[val.before.x][val.before.y].piece = None;
                self.squares[val.after.x][val.after.y].piece = Some(val.piece_moved);

            },

            MoveType::castle(val) => {
                if val.is_kingside  {
                    /* This is redundant */ 
                    match self.active_color {
                        Color::White => {
                            println!("got here 1");
                            self.squares[7][4].piece = None;
                            self.squares[7][7].piece = None;
                            self.squares[7][6].piece = Some(Piece {piece_type: PieceType::King, color: Color::White });
                            self.squares[7][5].piece = Some(Piece {piece_type: PieceType::Rook, color: Color:: White });
                        }, 
                        Color::Black => {
                            println!("got here 2");
                            self.squares[0][4].piece = None;
                            self.squares[0][7].piece = None;
                            self.squares[0][6].piece = Some(Piece {piece_type: PieceType::King, color: Color::Black });
                            self.squares[0][5].piece = Some(Piece {piece_type: PieceType::Rook, color: Color:: Black });

                        },
                    }
                } else {
                    match self.active_color {
                        Color::White => {
                            self.squares[7][4].piece = None;
                            self.squares[7][0].piece = None;
                            self.squares[7][2].piece = Some(Piece {piece_type: PieceType::King, color: Color::White });
                            self.squares[7][3].piece = Some(Piece {piece_type: PieceType::Rook, color: Color:: White });
                        }, 
                        Color::Black => {
                            self.squares[0][4].piece = None;
                            self.squares[0][0].piece = None;
                            self.squares[0][2].piece = Some(Piece {piece_type: PieceType::King, color: Color::Black });
                            self.squares[0][3].piece = Some(Piece {piece_type: PieceType::Rook, color: Color:: Black });

                        },
                    }
                }
            },
            MoveType::promotion(val) => {
                self.squares[val.before.x][val.before.y].piece = None;
                self.squares[val.after.x][val.after.y].piece = Some(val.promote_to);
            }
        }

        //Changing color
        match self.active_color {
            Color::Black => self.active_color = Color::White,
            Color::White => self.active_color = Color::Black, 
        };
    }

    pub fn print_board(&self) {
        for index in 0..8 {
            print!("[{}]", index);
            for inner_index in 0..8 {
                print!("{}", self.squares[index][inner_index].symbol());
            }
            print!("\n");
        }
        println!("   [0][1][2][3][4][5][6][7]");
    }
}

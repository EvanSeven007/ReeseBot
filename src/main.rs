use colored::*;

/* Enumeration for a color for squares and pieces */
#[derive(Clone, Copy)]
enum Color {
    White,
    Black,
    Undef, //Place holder value, used for instantiation
}
/* Simple 1 - 1 map function from each type of color to a corresponding string */ 
fn color_to_string(color: Color) -> String {
    match color {
        Color::White => String::from("white"), 
        Color::Black => String::from("blue"), //We are using blue until we graduate from a CLI program
        Color::Undef => String::from("red"), //If anything is red, something went wrong
    }
}

/* Enumeration for a piece type */ 
#[derive(Clone, Copy)]
pub enum PieceType {
    King, 
    Queen, 
    Bishop,
    Knight,
    Rook,
    Pawn,
    None, //Placeholder, used for instantiation
}

/* A piece consists of a type and a color */
#[derive(Clone, Copy)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

/* A square consists of a color and a piece on that square */
#[derive(Clone, Copy)]
struct Square {
    piece: Option<Piece>,
    color: Color, 
}

/* Squares start with nothing on it and a piece on that square */ 
impl Square {
    /* Simple constructor for a new square */
    pub fn new(square_piece: Option<Piece>, square_color: Color) -> Square {
        Square {piece: square_piece, color: square_color}
    }

    /* Creates a string representation of the square */
    fn symbol(&self) -> String {
        let square_color = match self.color {
            Color::White => String::from("white"),
            Color::Black => String::from("blue"),
            Color::Undef => String::from("red"),
        };

        //Variables to hold the piece type and pice color
        let square_piece_type: PieceType;
        let piece_color: String; //String for now due to being a CLI program
        match self.piece {
            Some(_) => {
                square_piece_type = self.piece.unwrap().piece_type;
                piece_color = color_to_string(self.piece.unwrap().color);
            },
            None => {
                square_piece_type = PieceType::None;
                piece_color = color_to_string(Color::Undef);
            },
        };

        let (left_bracket, right_bracket) = ("[".color(square_color.clone()), "]".color(square_color)); //This is unelegant, but it works
        match square_piece_type {
            PieceType::King   => format!("{}{}{}", left_bracket, "♚".color(piece_color), right_bracket),
            PieceType::Queen  => format!("{}{}{}", left_bracket, "♛".color(piece_color), right_bracket),
            PieceType::Rook   => format!("{}{}{}", left_bracket, "♜".color(piece_color), right_bracket),
            PieceType::Bishop => format!("{}{}{}", left_bracket, "♝".color(piece_color), right_bracket),
            PieceType::Knight => format!("{}{}{}", left_bracket, "♞".color(piece_color), right_bracket),
            PieceType::Pawn  => format!("{}{}{}", left_bracket, "♟︎".color(piece_color), right_bracket),
            PieceType::None => format!("{}{}{}", left_bracket, " ", right_bracket),
        }
    }

}

struct position {
    x: usize,
    y: usize,
}
/* A board is a 8x8 array of squares */
struct BoardState {
    squares: [[Square; 8]; 8],
    active_color: Color, 
    can_castle_white_kingside: bool,
    can_castle_white_queenside: bool,
    can_castle_black_kingside: bool, 
    can_castle_black_queenside: bool,
    en_passant: Option<position>,
    //Todo halfmove
    //Todo full move
    
}

impl BoardState {
    /* Creating an empty board */

    /* Creates a board state from a FEN string */
    fn new(fen: &str) -> Result<BoardState, &str> {
        //Creating an 8x8 array of uninitialized arrays
        let mut squares = [[Square {piece: None, color: (Color::Undef) }; 8]; 8];
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
        if(fen_strings.len() != 6) {
            return Err("Invalid fen string!");
        }

        println!("{}", fen_strings[0]);
        let position_str: Vec<&str> = fen_strings[0].split('/').collect();
        let mut col: usize;
        let mut row_string: &str; //String that stores the current row info
        for row in 0..8 {
            row_string = position_str[row];
            col = 0;
            for fen_entry in row_string.chars() {
                if fen_entry.is_digit(10) {
                    col += (fen_entry.to_digit(10).unwrap() as usize);
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
        let mut en_passant: Option<position>;
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
            /* This is just atrocious Evan, fix this */
            if(!en_passant_string[1].is_digit(10)){
                return Err("fen string enpassant malformed!")
            }
            y = (en_passant_string[1].to_digit(10).unwrap() - 1) as usize; /* Potential off by one bug */
            if y > 7 {
                return Err("fen string enpassant malformed!")
            }
            en_passant = Some(position{ x, y });
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
                    _ => Color::Black,
                }
            }
            _ => {
                match val2 {
                    1 | 3 | 5 | 7 => Color::Black,
                    _ => Color::White,
                }
            }
        }
    }

    fn print_board(&self) {
        for index in 0..8 {
            print!("[{}]", index + 1);
            for inner_index in 0..8 {
                print!("{}", self.squares[index][inner_index].symbol());
            }
            print!("\n");
        }
        println!("   [a][b][c][d][e][f][g][h]");
    }
}


fn main() {
    let board_state = "1R3b1r/p1ppkpp1/2n4n/4p3/p1b1P1B1/NpB2N2/PPP1rPPP/3QK2R b - - 0 1";
    let board = BoardState::new(board_state);
    match board {
        Ok(_) => board.unwrap().print_board(),
        Err(e) => println!("{}", e),
    }
    
    //TODO
    //Make a move parser
    //takes in a string and returns a Result<Move>

    //Make a move struct
    //This will consist of fields about a given move
    //Move types can be: moving one piece from one square to another, 
        //Subset of these are captures
    //castles
    //enpassant
    //All moves can be checks
    
    //Move logic will ensure that the move is valid
    //That is, the move is possible (i.e. not allowing a rook to skip over squares)
    //Ensuring that a capture is indeed a capture (i.e. if the user writes exd5 then the program will ensure that a pawn exists that can take a pawn on d5)
    //Ensuring the move is legal (not leaving a king in check)
    //Probably many more edge cases
    //
}
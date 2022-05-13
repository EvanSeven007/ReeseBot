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

/* A board is a 8x8 array of squares */
struct Board {
    squares: [[Square; 8]; 8]
}

impl Board {
    /* Creating an empty board */

    /* GOAL: TODO change this to support FEN NOTATION */ 
    fn new() -> Board {
        //Creating an 8x8 array of uninitialized arrays
        let mut squares = [[Square {piece: None, color: (Color::Undef) }; 8]; 8]; 

        /* All of the below code initialized a basic 8x8 board. It is very inelegant, and
        the need for FEN notation is apparent */
        let mut index1: u16;
        let mut index2: u16;
        for index in 0..8 {
            //Getting the color of a piece if one exists on this square 
            let piece_color = match index { 
                0 | 1 => Color::White,
                6 | 7 => Color::Black,
                _ => Color::Undef, //Risky
            };

            match index {
                //Populating first and back row
                0 | 7 => {
                    for inner_index in 0..8 {
                        /* This is dumb */
                        index1 = index.clone() as u16;
                        index2 = inner_index.clone() as u16;
                        let color = Board::get_color(index1, index2);
                        let piece = match inner_index {
                            0 | 7 => Piece{piece_type: PieceType::Rook, color: piece_color},
                            1 | 6 => Piece{piece_type: PieceType::Knight, color: piece_color},
                            2 | 5 => Piece{piece_type: PieceType::Bishop, color: piece_color},
                            3 => Piece{piece_type: PieceType::Queen, color: piece_color},
                            4 => Piece{piece_type: PieceType::King, color: piece_color},
                            _ => {panic!("Not a valid piece type")}
                        };
                        squares[index][inner_index] = Square{ piece: Some(piece), color};
                    }
                }, 
                //Matching pawn rows
                1 | 6 => { 
                    for inner_index in 0..8 {
                        /* This is dumb */
                        let piece = Piece{piece_type: PieceType::Pawn, color: piece_color};
                        index1 = index.clone() as u16;
                        index2 = inner_index.clone() as u16;
                        let color = Board::get_color(index1, index2);
                        squares[index][inner_index] = Square { piece: Some(piece), color};
                    }
                },
                _ => {
                    for inner_index in 0..8 {
                        /* This is dumb */
                        index1 = index.clone() as u16;
                        index2 = inner_index.clone() as u16;
                        squares[index][inner_index] = Square {piece : None, color: Board::get_color(index1, index2)};
                    }
                }
            }
        }
        Board { squares }
    }
    
    /* Gets the color of a board from its coordinates */
    fn get_color(val1: u16, val2: u16) -> Color {
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
        for index in (0..8).rev() {
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
    let board = Board::new();
    //Add loop here, take in a move as input
    //If the move is invalid, keep taking the move in
    //Assess the move (is the game over or not?)
    //Print the board after the move, 
    //Loop
    board.print_board();
    //board.move(coord1, coord2)
    //Have to make exceptions for castling 0 - 0 and pawn promotion e4 - e5Q
    //Need to make functoin that checks if a king is in check given a board state and color of the king
    //This is so we can do stuff like (the only valid moves are the one that get you out of checkl)
}
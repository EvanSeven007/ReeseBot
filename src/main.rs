mod piece;
use colored::*;

/* Enumeration for a color for squares and pieces */
#[derive(Clone, Copy)]
enum Color {
    White,
    Black,
    Undef, /* This is dumb */
}

fn color_to_string(color: Color) -> String {
    match color {
        Color::White => String::from("white"),
        Color::Black => String::from("blue"),
        Undef => String::from("red"), //NEED TO DO BETTER ERROR HANDLING
    }
}

/* Enumeration for a piece type */ 
#[derive(Clone, Copy)]
pub enum piece_type {
    King, 
    Queen, 
    Bishop,
    Knight,
    Rook,
    Pawn,
    None,
}

/* A piece consists of a type and a color */
#[derive(Clone, Copy)]
pub struct Piece {
    piece_type: piece_type,
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
    pub fn new(square_piece: Option<Piece>, square_color: Color) -> Square {
        Square {piece: square_piece, color: square_color}
    }

    fn symbol(&self) -> String {
        /* This is dumb */
        let mut color_str: String; //Change to String later maybe?
        match self.color {
            Color::White => color_str = String::from("white"),
            _ => color_str = String::from("blue"), //FIX THIS, IT CAN BE NONE!!!!!
        }
        /* This is dumb */
        let mut piece_type;
        let mut piece_color; 
        match self.piece {
            Some(x) => {
                piece_type = self.piece.unwrap().piece_type;
                piece_color = color_to_string(self.piece.unwrap().color);
            },
            None => {
                piece_type = piece_type::None;
                piece_color = color_to_string(Color::Undef);
            },
        };
        /* This is dumb */
        let leftBracket = "[".color(color_str.clone());
        let rightBracket = "]".color(color_str);
        match piece_type {
            piece_type::King   => format!("{}{}{}", leftBracket, "K".color(piece_color), rightBracket),
            piece_type::Queen  => format!("{}{}{}", leftBracket, "Q".color(piece_color), rightBracket),
            piece_type::Rook   => format!("{}{}{}", leftBracket, "R".color(piece_color), rightBracket),
            piece_type::Bishop => format!("{}{}{}", leftBracket, "B".color(piece_color), rightBracket),
            piece_type::Knight => format!("{}{}{}", leftBracket, "N".color(piece_color), rightBracket),
            piece_type::Pawn  => format!("{}{}{}", leftBracket, "p".color(piece_color), rightBracket),
            piece_type::None => format!("{}{}{}", leftBracket, " ", rightBracket),
        }
    }

}

struct Board {
    squares: [[Square; 8]; 8]
}

impl Board {
    fn new() -> Board {
        let mut squares = [[Square {piece: None, color: (Color::Undef) }; 8]; 8]; //Maybe change this?
        /* This is dumb *//* This is dumb *//* This is dumb *//* This is dumb *//* This is dumb */
        let mut index1: u16;
        let mut index2: u16;

        for index in 0..8 {
            let mut piece_color: Color = Color::Undef; /* This is dumb */
            match &index { /* This is dumb */
                0 | 1 => piece_color = Color::White,
                6 | 7 => piece_color = Color::Black,
                _ => {}
            }

            match index {
                //Populating first and back row
                0 | 7 => {
                    for innerIndex in 0..8 {
                        /* This is dumb */
                        index1 = index.clone() as u16;
                        index2 = innerIndex.clone() as u16;
                        let color = Board::getColor(&index1, &index2);
                        let piece = match innerIndex {
                            0 | 7 => Piece{piece_type: piece_type::Rook, color: piece_color},
                            1 | 6 => Piece{piece_type: piece_type::Knight, color: piece_color},
                            2 | 5 => Piece{piece_type: piece_type::Bishop, color: piece_color},
                            3 => Piece{piece_type: piece_type::Queen, color: piece_color},
                            4 => Piece{piece_type: piece_type::King, color: piece_color},
                            _ => {panic!("Not a valid piece type")}
                        };
                        squares[index][innerIndex] = Square{ piece: Some(piece), color};
                    }
                }, 
                //Matching pawn rows
                1 | 6 => { 
                    for innerIndex in 0..8 {
                        /* This is dumb */
                        let piece = Piece{piece_type: piece_type::Pawn, color: piece_color};
                        index1 = index.clone() as u16;
                        index2 = innerIndex.clone() as u16;
                        let color = Board::getColor(&index1, &index2);
                        squares[index][innerIndex] = Square { piece: Some(piece), color};
                    }
                },
                _ => {
                    for innerIndex in 0..8 {
                        /* This is dumb */
                        index1 = index.clone() as u16;
                        index2 = innerIndex.clone() as u16;
                        squares[index][innerIndex] = Square {piece : None, color: Board::getColor(&index1, &index2)};
                    }
                }
            }
        }
        Board { squares }
    }
    
    fn getColor(val1: &u16, val2: &u16) -> Color {
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

    fn printBoard(&self) {
        for index in (0..8).rev() {
            print!("[{}]", index + 1);
            for innerIndex in 0..8 {
                print!("{}", self.squares[index][innerIndex].symbol());
            }
            print!("\n");
        }
        println!("   [a][b][c][d][e][f][g][h]");
    }
}

fn main() {
    let board = Board::new();
    board.printBoard();
}
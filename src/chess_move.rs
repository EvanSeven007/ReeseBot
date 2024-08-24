use crate::piece::{Piece, PieceType};
use crate::color::Color;

/* Position of a square on the board */
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

/* Direction to move */
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

/* All moves are of one of three types */ 
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MoveType {
    Standard(StandardMove), //move a piece from one square to another
    Castle(CastleMove), //Castling 
    Promotion(PromotionMove), //upgrade pawn by getting to the back row
    EnPassant(EnPassantMove),
}

/* Standard moves involve normal captures and enpassants */ 
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct StandardMove { //enpassant is in this?
    pub before: Position, 
    pub after: Position, 
    pub piece_moved: Piece, 
}

/* Castles are either king or queenside */
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct CastleMove {
    pub is_kingside: bool, //Else queenside
    pub color: Color, 
}

/* Promoting a pawn */
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct PromotionMove {
    pub before: Position,
    pub after: Position,
    pub promote_to: Piece,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct EnPassantMove {
    pub before: Position,
    pub after: Position,
    pub en_passant_pos: Position, //Square of captured piece
}

/* A general move */
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub move_type: MoveType,
    pub piece_captured: Option<Piece>, 
}

impl Move {
    //I will implement Display later, but for now I can't be asked
    pub fn to_string(self) -> String {
        match self.move_type {
            MoveType::Standard(val) => {
                return format!("{}{}", val.before.to_string(), val.after.to_string());
            },
            MoveType::Promotion(val) => {
                return format!("{}{}", val.before.to_string(), val.after.to_string());
            },
            MoveType::EnPassant(val) => {
                return format!("{}{}", val.before.to_string(), val.after.to_string());
            },
            MoveType::Castle(val) => {
                match val.is_kingside {
                    true => return String::from("0-0"),
                    false => return String::from("0-0-0"),
                }
            }
        }
    }
}

//Struct that encapsulates the numerical position on a chessboard 
impl Position {
    pub fn swap(self) -> Position {
        Position {row: self.col, col: self.row}
    }

    pub fn is_valid_position(self) -> bool {
        self.row >= 2 && self.row <= 9 && self.col >= 2 && self.col <= 9
    }

    //Returns the square to the right from the perspective of white
    pub fn right(self) -> Position {
        Position{row: self.row, col: self.col + 1}
    }
    //Returns the square to the left from the perspective of white
    pub fn left(self) -> Position {
        Position{row: self.row, col: self.col - 1}
    }
    //Returns the square to the top from the perspective of white
    pub fn up(self) -> Position {
        Position{row: self.row - 1, col: self.col}
    }
    //Returns the square to the bottom from the perspective of white
    pub fn down(self) -> Position {
        Position{row: self.row + 1, col: self.col}
    } 

    //Takes in a directiona and returns the square in that position
    pub fn next_position(self, dir: &Direction) -> Position {
        match dir {
            Direction::Right => {
                self.right()
            },
            Direction::Left => {
                self.left()
            },
            Direction::Up => {
                self.up()
            },
            Direction::Down => {
                self.down()
            },
            Direction::UpRight => {
                self.up().right()
            },
            Direction::UpLeft => {
                self.up().left()
            },
            Direction::DownRight => {
                self.down().right()
            },
            Direction::DownLeft => {
                self.down().left()
            },
        }
    }

    //Returns the position in standard notation (i.e. a3, d4, e5)
    pub fn to_string(self) -> String {
        let start: String;
        match self.col {
            2 => start = String::from("a"),
            3 => start = String::from("b"),
            4 => start = String::from("c"),
            5 => start = String::from("d"),
            6 => start = String::from("e"),
            7 => start = String::from("f"),
            8 => start = String::from("g"),
            9 => start = String::from("h"),
            _ => start = String::from("Not a valid Position!!"),
        }
        let end = (10 - self.row).to_string();

        format!("{}{}", start, end)
    }

    pub fn from_string<'a>(position: String) -> Result<Position, &'a str> {
        let position_stripped: Vec<&str> = position.split_whitespace().collect();
        if position_stripped.len() > 1 && position_stripped[1] != " " {
            return Err("Not a valid position -");
        }
        if !(position_stripped[0].len() == 2 || position_stripped[0].len() == 4) {
            return Err("Not a valid position");
        }
        let row: usize;
        let col;
        let row_res = position_stripped[0].chars().nth(1).unwrap().to_digit(10);
        if let Some(val) = row_res {
            row = (10 - val) as usize;
        } else {
            return Err("cannot parse position");
        }

        match position_stripped[0].chars().nth(0).unwrap() {
            'a' | 'A' => col = 2,
            'b' | 'B' => col = 3,
            'c' | 'C' => col = 4,
            'd' | 'D' => col = 5,
            'e' | 'E' => col = 6,
            'f' | 'F' => col = 7,
            'g' | 'G' => col = 8,
            'h' | 'H' => col = 9,
            _ => return Err("Index out of bounds"),
        }

        Ok(Position{row, col})
    }
}

/* Constructor functions for each of the basic moves */
pub fn standard(before: Position, after: Position, piece_moved: Piece, piece_captured: Option<Piece>) -> Move {
    let move_type: MoveType = MoveType::Standard(StandardMove{before, after, piece_moved});
    
    Move {
        move_type,
        piece_captured,
    }
}

pub fn castle(is_kingside: bool, color: Color) -> Move {
    let move_type: MoveType = MoveType::Castle(CastleMove{is_kingside, color});

    Move {
        move_type,
        piece_captured: None,
    }
}

pub fn promotion(before: Position, after: Position, promote_to: Piece, piece_captured: Option<Piece>) -> Move {
    assert!(promote_to.piece_type != PieceType::King);
    let move_type: MoveType = MoveType::Promotion(PromotionMove{before, after, promote_to});
    
    Move {
        move_type,
        piece_captured,
    }
}

//Could be done better
//Enpassant is position of captured pawn
pub fn en_passant(before: Position, after: Position, en_passant_pos: Position, piece_captured: Option<Piece>) -> Move{
    let move_type: MoveType = MoveType::EnPassant(EnPassantMove{before, after, en_passant_pos});

    Move {
        move_type,
        piece_captured,
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_1() {
        let res = Position::from_string(String::from("e1"));
        match res {
            Ok(val) => assert_eq!(Position{row: 9, col: 6}, val),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_2() {
        let res = Position::from_string(String::from("h5"));
        match res {
            Ok(val) => assert_eq!(Position{row: 5, col: 9}, val),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_3() {
        let res = Position::from_string(String::from("C7"));
        match res {
            Ok(val) => assert_eq!(Position{row: 3, col: 4}, val),
            Err(e) => panic!("{}", e),
        }
    }
    #[test]
    fn test_4() {
        let res = Position::from_string(String::from("CC7"));
        match res {
            Ok(val) => panic!("Shoudl have returned Error"),
            Err(e) => {}
        }
    }

    #[test]
    fn test_5() {
        let res = Position::from_string(String::from("C7 "));
        match res {
            Ok(val) => assert_eq!(Position{row: 3, col: 4}, val),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_6() {
        let res = Position::from_string(String::from("C7 E4"));
        match res {
            Ok(val) => panic!("Shoudl have returned Error"),
            Err(e) => {}
        }
    }
}
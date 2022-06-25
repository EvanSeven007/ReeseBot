use crate::piece::{Piece};
use crate::board_state::{BoardState};
use crate::color::{Color};
/* Position in on a board */ 
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

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

///Struct that encapsulates the numerical position on a chessboard 
impl Position {
    pub fn swap(&self) -> Position {
        Position {x: self.y, y: self.x}
    }

    pub fn is_valid_position(&self) -> bool {
        self.x >= 2 && self.x <= 9 && self.y >= 2 && self.y <= 9
    }

    pub fn right(&self) -> Position {
        Position{x: self.x, y: self.y + 1}
    }

    pub fn left(&self) -> Position {
        Position{x: self.x, y: self.y - 1}
    }

    pub fn up(&self) -> Position {
        Position{x: self.x - 1, y: self.y}
    }

    pub fn down(&self) -> Position {
        Position{x: self.x + 1, y: self.y}
    } 

    pub fn next_position(&self, dir: &Direction) -> Position {
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
}

/* Refactor this later */
pub enum MoveValue {
    StandardMove,
    CastleMove,
    PromotionMove,
    EnPassantMove,
}
/* All moves are of one of three types */ 
pub enum MoveType {
    standard(StandardMove), //move a piece from one square to another
    castle(CastleMove), //Castling 
    promotion(PromotionMove), //upgrade pawn by getting to the back row
    enPassant(EnPassantMove),

}

/* Standard moves involve normal captures and enpassants */ 
pub struct StandardMove { //enpassant is in this?
    pub before: Position, 
    pub after: Position, 
    pub piece_moved: Piece, 
}

/* Castles are either king or queenside */
pub struct CastleMove {
    pub is_kingside: bool, //Else queenside
    pub color: Color, 
}

/* Promoting a pawn */
pub struct PromotionMove {
    pub before: Position,
    pub after: Position,
    pub promote_to: Piece,
}

pub struct EnPassantMove {
    pub before: Position,
    pub after: Position,
    pub en_passant: Position, //Square of captured piece
}

/* A general move */ 
pub struct Move {
    pub move_type: MoveType,
    pub piece_captured: Option<Piece>, 
}

/* Constructor functions for each of the basic moves */
pub fn standard(before: Position, after: Position, piece_moved: Piece, piece_captured: Option<Piece>) -> Move {
    let move_type: MoveType = MoveType::standard(StandardMove{before, after, piece_moved});
    
    Move {
        move_type,
        piece_captured,
    }
}

pub fn castle(is_kingside: bool, color: Color) -> Move {
    let move_type: MoveType = MoveType::castle(CastleMove{is_kingside, color});

    Move {
        move_type,
        piece_captured: None,
    }
}

pub fn promotion(before: Position, after: Position, promote_to: Piece, piece_captured: Option<Piece>) -> Move {
    let move_type: MoveType = MoveType::promotion(PromotionMove{before, after, promote_to});
    
    Move {
        move_type,
        piece_captured,
    }
}

//Could be done better
//Enpassant is position of captured pawn
pub fn enPassant(before: Position, after: Position, en_passant: Position, piece_captured: Option<Piece>) -> Move{
    let move_type: MoveType = MoveType::enPassant(EnPassantMove{before, after, en_passant});

    Move {
        move_type,
        piece_captured,
    }
}
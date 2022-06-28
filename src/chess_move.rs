use crate::piece::{Piece, PieceType};
use crate::color::{Color};
/* Position in on a board */ 
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
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

/* All moves are of one of three types */ 
#[derive(Clone, Copy)]
pub enum MoveType {
    Standard(StandardMove), //move a piece from one square to another
    Castle(CastleMove), //Castling 
    Promotion(PromotionMove), //upgrade pawn by getting to the back row
    EnPassant(EnPassantMove),

}

/* Standard moves involve normal captures and enpassants */ 
#[derive(Clone, Copy)]
pub struct StandardMove { //enpassant is in this?
    pub before: Position, 
    pub after: Position, 
    pub piece_moved: Piece, 
}

/* Castles are either king or queenside */
#[derive(Clone, Copy)]
pub struct CastleMove {
    pub is_kingside: bool, //Else queenside
    pub color: Color, 
}

/* Promoting a pawn */
#[derive(Clone, Copy)]
pub struct PromotionMove {
    pub before: Position,
    pub after: Position,
    pub promote_to: Piece,
}

#[derive(Clone, Copy)]
pub struct EnPassantMove {
    pub before: Position,
    pub after: Position,
    pub en_passant_pos: Position, //Square of captured piece
}

/* A general move */
#[derive(Clone, Copy)] 
pub struct Move {
    pub move_type: MoveType,
    pub piece_captured: Option<Piece>, 
}

///Struct that encapsulates the numerical position on a chessboard 
impl Position {
    pub fn swap(&self) -> Position {
        Position {row: self.col, col: self.row}
    }

    pub fn is_valid_position(&self) -> bool {
        self.row >= 2 && self.row <= 9 && self.col >= 2 && self.col <= 9
    }

    pub fn right(&self) -> Position {
        Position{row: self.row, col: self.col + 1}
    }

    pub fn left(&self) -> Position {
        Position{row: self.row, col: self.col - 1}
    }

    pub fn up(&self) -> Position {
        Position{row: self.row - 1, col: self.col}
    }

    pub fn down(&self) -> Position {
        Position{row: self.row + 1, col: self.col}
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

use crate::piece::{Piece};
/* Position in on a board */ 
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn swap(&self) -> Position {
        Position {x: self.y, y: self.x}
    }

    pub fn is_valid_position(&self) -> bool {
        return self.x >= 1 && self.x <= 8 && self.y >= 1 && self.y <= 8;
    }
}

/* All moves are of one of three types */ 
pub enum MoveType {
    standard(StandardMove), //move a piece from one square to another
    castle(CastleMove), //Castling 
    promotion(PromotionMove), //upgrade pawn by getting to the back row
}

/* Standard moves involve normal captures and enpassants */ 
pub struct StandardMove { //enpassant is in this?
    pub before: Position, 
    pub after: Position, 
    pub piece_moved: Piece, 
    pub is_enpassant: bool,
}

/* Castles are either king or queenside */
pub struct CastleMove {
    pub is_kingside: bool, //Else queenside
}

/* Promoting a pawn */
pub struct PromotionMove {
    pub before: Position,
    pub after: Position,
    pub promote_to: Piece,
}

/* A move */ 
pub struct Move {
    pub move_type: MoveType,
    pub piece_captured: Option<Piece>, 
}
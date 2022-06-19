use crate::piece::{Piece};
use crate::board_state::{BoardState};
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

pub fn castle(is_kingside: bool) -> Move {
    let move_type: MoveType = MoveType::castle(CastleMove{is_kingside});

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
pub fn enPassant(before: Position, after: Position, en_passant: Position, piece_captured: Option<Piece>) -> Move{
    let move_type: MoveType = MoveType::enPassant(EnPassantMove{before, after, en_passant});

    Move {
        move_type,
        piece_captured,
    }
}
/*
impl Move {
    /* takes in a string and gives a corresponding move for it, and then checks if that move is valid */
    /* If the move is valid, it makes the move on the board, otherwise it prompts for a new move */
    pub fn parse_move(board: &mut BoardState, input: &str) {
        let mut parsed_move: Move;
        match input {
            "O-O" => parsed_move = Move {
                move_type: MoveType::castle{
                    is_kingside: true
                }, 
                piece_captured: None
            },

            _ => {
                println!("Malformed Move: Please Enter Another");
            }
        }
    }
}
*/
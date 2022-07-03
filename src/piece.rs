use crate::color::{Color};

/* Enumeration for a piece type */ 
#[derive(PartialEq, Eq, Clone, Copy)]
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
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn worth(self) -> i32 {
        let mult: i32;
        
        match self.color {
            Color::White => mult = 1,
            Color::Black => mult = -1,
        }

        match self.piece_type {
            PieceType::King => mult * 90,
            PieceType::Queen => mult * 9,
            PieceType::Rook => mult * 5,
            PieceType::Bishop | PieceType::Knight => mult * 3,
            PieceType::Pawn => mult * 1,
            PieceType::None => 0,
        }
    }
}
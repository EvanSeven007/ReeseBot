use core::fmt;

use crate::color::{Color};

/* Enumeration for a piece type */ 
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
    pub fn worth(self) -> f32 {
        let mult: f32;
        
        match self.color {
            Color::White => mult = 1.0,
            Color::Black => mult = -1.0,
        }

        match self.piece_type {
            PieceType::King => mult * 200.0,
            PieceType::Queen => mult * 9.0,
            PieceType::Rook => mult * 5.0,
            PieceType::Bishop | PieceType::Knight => mult * 3.0,
            PieceType::Pawn => mult * 1.0,
            PieceType::None => 0.0,
        }
    }
}
impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_type_s: &str = match self.piece_type {
            PieceType::King => "King",
            PieceType::Queen => "Queen",
            PieceType::Bishop => "Bishop",
            PieceType::Knight => "Knight",
            PieceType::Rook => "Rook",
            PieceType::Pawn => "Pawn",
            PieceType::None => "None",
        };
        f.debug_struct("Piece")
            .field("piece_type", &piece_type_s)
            .field("color", &self.color)
            .finish()
    }
}
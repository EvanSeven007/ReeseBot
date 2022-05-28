use crate::color;
/* Enumeration for a piece type */ 
#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub piece_type: PieceType,
    pub color: color::Color,
}
pub mod piece {
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
}
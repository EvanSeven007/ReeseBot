use crate::color::Color;
use crate::piece::{Piece, PieceType};
use colored::*;

/* A square consists of a color and a piece on that square */
#[derive(Clone, Copy)]
pub struct Square {
    pub piece: Option<Piece>,
    pub color: Color,
}

/* Squares start with nothing on it and a piece on that square */
impl Square {
    pub fn is_occupied(&self) -> bool {
        match self.piece {
            Some(_) => true,
            None => false,
        }
    }

    /* Creates a string representation of the square */
    pub fn symbol(&self) -> String {
        let square_color = match self.color {
            Color::White => String::from("white"),
            Color::Black => String::from("blue"),
        };

        //Variables to hold the piece type and pice color
        let square_piece_type: PieceType;
        let piece_color: String; //String for now due to being a CLI program
        match self.piece {
            Some(_) => {
                square_piece_type = self.piece.unwrap().piece_type;
                piece_color = self.piece.unwrap().color.color_to_string();
            }
            None => {
                square_piece_type = PieceType::None;
                piece_color = String::new();
            }
        };

        //Display
        let (left_bracket, right_bracket) =
            ("[".color(square_color.clone()), "]".color(square_color)); //This is unelegant, but it works
        match square_piece_type {
            PieceType::King => format!(
                "{}{}{}",
                left_bracket,
                "♚".color(piece_color),
                right_bracket
            ),
            PieceType::Queen => format!(
                "{}{}{}",
                left_bracket,
                "♛".color(piece_color),
                right_bracket
            ),
            PieceType::Rook => format!(
                "{}{}{}",
                left_bracket,
                "♜".color(piece_color),
                right_bracket
            ),
            PieceType::Bishop => format!(
                "{}{}{}",
                left_bracket,
                "♝".color(piece_color),
                right_bracket
            ),
            PieceType::Knight => format!(
                "{}{}{}",
                left_bracket,
                "♞".color(piece_color),
                right_bracket
            ),
            PieceType::Pawn => format!(
                "{}{}{}",
                left_bracket,
                "♟︎".color(piece_color),
                right_bracket
            ),
            PieceType::None => format!("{}{}{}", left_bracket, " ", right_bracket),
        }
    }
}

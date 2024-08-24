use log::debug;

use crate::chess_move::{Move, MoveType, Position, castle};
use crate::piece::PieceType;
use crate::board_state::BoardState;
use std::process::exit;

/**
 * Tries to parse a move from user-input and returns the corresponding Move Struct if it is a valid move, otherwise returns an error message
 * move_string: The string that the user has inputted
 * board: The current board state
 * moves: The list of all possible moves that the player can make
 */
pub fn parse_move<'a>(move_string: String, board: &BoardState, moves: Vec<Move>) -> Result<Move, &'a str> { 
    let move_string = move_string.trim();
    let strings: Vec<&str> = move_string.split_whitespace().collect(); 
    debug!("Parsed move string: {:?}", strings);
    debug!("Length = {}", moves.len());
    match strings[0].to_lowercase().as_str() {
        "resign" => {
            println!("{} has won by resignation", board.active_color.opposite().color_to_string());
            exit(0);
        },
        "move" => {
            if strings.len() == 2 {
                //Handling castle moves
                match strings[1] {
                    "0-0" | "O-O" => {
                        let castle_move = castle(true, board.active_color.clone());
                        if moves.contains(&castle_move) {
                            return Ok(castle_move);
                        } else {
                            return Err("Cannot castle kingside");
                        }
                    },
                    "0-0-0" | "O-O-O" => {
                        let castle_move = castle(false, board.active_color.clone());
                        if moves.contains(&castle_move) {
                            return Ok(castle_move);
                        } else {
                            return Err("Cannot castle Queenside");
                        }
                    },
                    _ => return Err("Castle Malformed"),
                }
            }
            if strings.len() != 3 {
                return Err("Not a valid command, move string is of incorrect length (3). Please re-enter");
            }

            let before_move = strings[1].to_string();
            let after_move = strings[2].to_string();
            let first_pos = Position::from_string(before_move)?;
            let second_pos = Position::from_string(after_move)?;
            // Promotion move
            if strings[2].len() == 4 { 
                if strings[2].chars().nth(2).unwrap().to_string() == String::from("=") {
                    let promote_to;
                    match strings[2].chars().nth(3).unwrap() {
                        'Q' => promote_to = PieceType::Queen,
                        'K' => promote_to = PieceType::King,
                        'R' => promote_to = PieceType::Rook,
                        'B' => promote_to = PieceType::Bishop,
                        _ => return Err("no a valid command. Could not parse positions in promotions. Please re-enter"),
                    }
                    
                    for mv in moves {
                        if let MoveType::Promotion(val) = mv.move_type {
                            if val.before == first_pos && val.after == second_pos && val.promote_to.piece_type == promote_to {
                                return Ok(mv);
                            }
                        }
                    }
                }
            } else if strings[2].len() == 2 {
                for mv in moves {
                    if let MoveType::Standard(val) = mv.move_type {
                        if val.before == first_pos && val.after == second_pos {
                            return Ok(mv);
                        }
                    }

                    if let MoveType::EnPassant(val) = mv.move_type {
                        if val.before == first_pos && val.after == second_pos {
                            return Ok(mv);
                        }
                    }
                }
            }
        },
        _ => {return Err("Not a valid command, did not get anywhere. Please re-enter");}
    }

    return Err("Not a valid command. Either your move was incorrectly formatted or it is an invalid move. Please re-enter");
}
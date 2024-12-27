use log::{debug, info};

use crate::board_state::{self, BoardState};
use crate::chess_move::{castle, Move, MoveType, Position};
use crate::color::Color;
use crate::move_gen::gen_all_moves;
use crate::piece::{Piece, PieceType};
use std::process::exit;

#[derive(Debug)]
struct MoveMetadata<'a> {
    piece: Option<PieceType>,
    from_square_y: Option<usize>,
    to_square_x: Option<usize>,
    to_square_y: Option<usize>,
    promotion: Option<PieceType>,
    is_from_populated: bool,
    is_capture: Option<bool>,
    captured_piece: Option<Piece>,
    piece_rank: Option<usize>,
    just_saw_promotion_char: bool,
    is_check: bool,
    is_checkmate: bool,
    move_string: &'a str
}
/**
 * Tries to parse a move from user-input and returns the corresponding Move Struct if it is a valid move, otherwise returns an error message
 * move_string: The string that the user has inputted
 * board: The current board state
 * moves: The list of all possible moves that the player can make
 */
pub fn parse_move<'a>(
    move_string: &'a str,
    board: &'a BoardState,
) -> Result<Move, &'a str> {
    let move_metadata = parse_fen(move_string)?;
    return validate_move(&move_metadata, board, gen_all_moves(board, board.active_color))
}

/**
 * Parses a FEN move string and tries to create a Move struct from it. 
 * board: The current board state
 * moves: The set of all possible moves that the player can make
 */
fn parse_fen<'a>(
    move_string: &'a str,
) -> Result<MoveMetadata<'a>, &'a str> {
    let trimmed_string = move_string.trim();
    let parsed_move: Move;

    if trimmed_string.to_lowercase() == "resign" {
        info!("Thanks for playing!");
        exit(0);
    }
    let mut move_info = MoveMetadata {
        piece: None,
        from_square_y: None,
        to_square_x: None,
        to_square_y: None,
        promotion: None,
        is_from_populated: false,
        is_capture: None,
        captured_piece: None,
        piece_rank: None,
        just_saw_promotion_char: false,
        is_check: false,
        is_checkmate: false,
        move_string: trimmed_string
    };

    for ch in trimmed_string.chars() {
        if move_info.piece.is_none() {
            let parsed_piece = char_to_piece_type(ch)
                .or_else(|_|  Err("Parse error: Invalid Piece type."))?;
            move_info.piece = Some(parsed_piece);
            // if the string is not a capture then this is just a naked pawn move (i.e. e4)
            if move_info.piece == Some(PieceType::Pawn) && !trimmed_string.contains("x") {
                move_info.to_square_x = Some(char_to_board_index(ch).unwrap());
            }
            continue;
        }

        match ch {
            'a'..='h' => {
                handle_lowercase_char(&mut move_info, ch);
            }
            '1'..='8' => {
                handle_numeric_char(&mut move_info, ch);
            }
            'x' => {
                handle_capture(&mut move_info);
            }
            '=' => {
                handle_promotion(&mut move_info);
            }
            'Q' | 'B' | 'N' | 'R' => {
                handle_piece_char(&mut move_info, ch);
            }
            '+' => {
                move_info.is_check = true;
            }
            '#' => {
                move_info.is_checkmate = true;
            }
            _ => return Err("Parse error: Encountered an unexpected character."),
        }
    }

    // Look through the set of possible moves and see we can find one that matches our parsed move
    // No matter what we know that we'll have the piece type and the destination square
    if move_info.piece.is_none() {
        return Err("Parse error: Missing piece field.");
    }
    if move_info.to_square_x.is_none() {
        return Err("Parse error: Missing x coord of square to move to!");
    }
    if move_info.to_square_y.is_none() {
        return Err("Parse error: Missing y coord of square to move to!");
    }

    return Ok(move_info);
}

fn parse_uci<'a>(
    move_string: String,
    board: &BoardState,
    moves: Vec<Move>,
) -> Result<Move, &'a str> {
    todo!();
}

pub fn validate_move<'a>(move_info: &MoveMetadata<'a>, board: &'a BoardState, moves: Vec<Move>) -> Result<Move, &'a str> {
    // If we captured something, we need to find out what it was
    let move_type: MoveType;
    let after_pos = Position {
        row: move_info.to_square_y.unwrap(),
        col: move_info.to_square_x.unwrap(),
    };
                    let mut captured_piece: Option<Piece> = None;

                    // if trimmed_string == "0-0" || trimmed_string == "0-0-0" {
                    //     return Ok(castle(trimmed_string == "0-0"));
                    // }

                    if move_info.is_capture == Some(true) {
                        captured_piece = board.get_piece(after_pos);
                        if captured_piece.is_none() {
                            return Err("Parse error: No piece to capture at destination square.");
                        }
                    }


                        // Filter out all invalid moves. This isn't a direct search as its possible for the user to specify a move that could be made by multiple pieces. (i.e. two knights that can move to the same square)
                        let valid_moves: Vec<Move> = moves
                        .into_iter()
                        // Filter out all non-matching moves
                        .filter(|mv: &Move| {
                            // Either both have to be None or the same thing
                            if mv.piece_captured != captured_piece {
                                return false;
                            }
                            match mv.move_type {
                                MoveType::Standard(standard) => {
                                    standard.piece_moved.piece_type == move_info.piece.unwrap()
                                    && standard.after == after_pos
                                    && move_info.promotion.is_none()
                                    && move_info.piece_rank == move_info.from_square_y
                                }
                                MoveType::Promotion(promo_move) => {
                                    if move_info.promotion.is_none() {
                                        return false;
                                    }

                                    promo_move.promote_to.piece_type == move_info.promotion.unwrap()
                                    && promo_move.after == after_pos
                                }
                                MoveType::EnPassant(enpassant) => {
                                    move_info.piece == Some(PieceType::Pawn)      
                                    && move_info.is_capture == Some(true)
                                    && captured_piece.unwrap().piece_type == PieceType::Pawn          
                                    && enpassant.after == after_pos
                                }
                                _ => false,
                            }
                        })
                        .collect();

                    if valid_moves.len() == 1 {
                        return Ok(valid_moves[0]);
                    }
                    if valid_moves.len() > 1 {
                        return select_correct_move(move_info, valid_moves); 
                    }
                    return Err("Could not find a valid move that matches your input!");
}

fn handle_lowercase_char(move_info: &mut MoveMetadata, ch: char) -> Result<(), &'static str> {
    if move_info.to_square_x.is_some() {
        return Err(
            "Parse error: Encountered an unexpected character when trying to find the 'to' square.",
        );
    }
    move_info.to_square_x = Some(char_to_board_index(ch).unwrap());
    Ok(())
}

fn handle_numeric_char(move_info: &mut MoveMetadata, ch: char) -> Result<(), &'static str> {
    // for pawn moves, a number is either the new rank or the rank of the piece that the pawn captures
    if move_info.piece == Some(PieceType::Pawn) {
        if move_info.to_square_y.is_some() {
            return Err("Parse error: Encountered an unexpected number while parsing a pawn move.");
        }
        move_info.to_square_y = Some(char_to_board_index(ch).unwrap());
        return Ok(());
    }
    // Is this rank metadata?
    if move_info.to_square_x.is_none() {
        move_info.piece_rank = Some(char_to_board_index(ch).unwrap());
        return Ok(());
    }
    move_info.to_square_y = Some(char_to_board_index(ch).unwrap());
    Ok(())
}

fn handle_capture(move_info: &mut MoveMetadata) -> Result<(), &'static str> {
    if move_info.is_capture.is_some() {
        return Err("Parse error: Encountered an unexpected 'x'");
    }
    move_info.is_capture = Some(true);
    Ok(())
}

fn handle_promotion(move_info: &mut MoveMetadata) -> Result<(), &'static str> {
    if move_info.just_saw_promotion_char {
        return Err("Parse error: Encountered an unexpected '='");
    }
    move_info.just_saw_promotion_char = true;
    Ok(())
}

fn handle_piece_char(move_info: &mut MoveMetadata, ch: char) -> Result<(), &'static str> {
    if move_info.promotion.is_some() {
        return Err("Parse error: Encountered an unexpected character while parsing promotion.");
    }
    move_info.promotion = Some(char_to_piece_type(ch)?);
    move_info.just_saw_promotion_char = false;
    Ok(())
}

fn char_to_board_index(ch: char) -> Result<usize, &'static str> {
    match ch {
        'a' | '8' => Ok(2),
        'b' | '7' => Ok(3),
        'c' | '6' => Ok(4),
        'd' | '5' => Ok(5),
        'e' | '4' => Ok(6),
        'f' | '3' => Ok(7),
        'g' | '2' => Ok(8),
        'h' | '1' => Ok(9),
        _ => return Err("Invalid character in move string"),
    }
}

fn char_to_piece_type(ch: char) -> Result<PieceType, &'static str> {
    match ch {
        'K' => Ok(PieceType::King),
        'Q' => Ok(PieceType::Queen),
        'B' => Ok(PieceType::Bishop),
        'N' => Ok(PieceType::Knight),
        'R' => Ok(PieceType::Rook),
        'a'..='h' => Ok(PieceType::Pawn),
        _ => return Err("Invalid character in move string"),
    }
}

fn handle_castle_move(
    trimmed_string: &str,
    color: Color,
    moves: Vec<Move>,
) -> Result<Move, &'static str> {
    let parsed_move = castle(trimmed_string == "0-0", color);
    if moves.contains(&parsed_move) {
        return Ok(parsed_move);
    } else {
        return Err("Cannot castle kingside");
    }
}

fn select_correct_move(move_info: &MoveMetadata, valid_moves: Vec<Move>) -> Result<Move, &'static str> {
    if move_info.piece_rank.is_none() {
        return Err("Ambiguous move: Multiple pieces of the same type can move to the same square. Please specify the rank of the piece you want to move.");
    }
    for mv in valid_moves {
        if let rank = move_info.piece_rank.unwrap() {
            match mv.move_type {
                MoveType::Standard(standard) => {
                    if standard.before.row == rank {
                        return Ok(mv);
                    }
                }
                MoveType::Promotion(promo_move) => {
                    if promo_move.before.row == rank {
                        return Ok(mv);
                    }
                },
                MoveType::EnPassant(enpassant) => {
                    if enpassant.before.row == rank {
                        return Ok(mv);
                    }
                },
                _ => continue,
            }
        }
    }
    
    return Err("Could not figure out which piece to move amongst several options. Please specify the rank of the rank of the piece you want to move.");
}

mod tests {
    use super::*;
    use crate::move_gen::gen_all_moves;

    #[test]
    fn test_standard_position_white_moves() {
        let board_state =
            BoardState::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let moves = gen_all_moves(&board_state, Color::White);

        let moves_string: Vec<String> = vec![
            String::from("a3"),
            String::from("a4"),
            String::from("b3"),
            String::from("b4"),
            String::from("c3"),
            String::from("c4"),
            String::from("d3"),
            String::from("d4"),
            String::from("e3"),
            String::from("e4"),
            String::from("f3"),
            String::from("f4"),
            String::from("g3"),
            String::from("g4"),
            String::from("h3"),
            String::from("h4"),
            String::from("Na3"),
            String::from("Nc3"),
            String::from("Nf3"),
            String::from("Nh3"),
        ];

        for mv_string in moves_string {
            let result = parse_move(&mv_string, &board_state);
            if result.is_err() {
                println!(
                    "Error: {} when trying to parse move: {}",
                    result.unwrap_err(),
                    mv_string
                );
            }
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_standard_position_black_moves() {
        let board_state =
            BoardState::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
        let moves = gen_all_moves(&board_state, Color::Black);

        let moves_string: Vec<String> = vec![
            String::from("a5"),
            String::from("a6"),
            String::from("b5"),
            String::from("b6"),
            String::from("c5"),
            String::from("c6"),
            String::from("d5"),
            String::from("d6"),
            String::from("e5"),
            String::from("e6"),
            String::from("f5"),
            String::from("f6"),
            String::from("g5"),
            String::from("g6"),
            String::from("h5"),
            String::from("h6"),
            String::from("Na6"),
            String::from("Nc6"),
            String::from("Nf6"),
            String::from("Nh6"),
        ];

        for mv_string in moves_string {
            let result = parse_move(&mv_string, &board_state);
            if result.is_err() {
                println!(
                    "Error: {} when trying to parse move: {}",
                    result.unwrap_err(),
                    mv_string
                );
            }
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_promotion() {
        let board_state = BoardState::new("8/3P4/8/8/8/8/8/1k2K3 w - - 0 1").unwrap();
        let moves = gen_all_moves(&board_state, Color::White);

        let moves_string: Vec<String> = vec![
            String::from("d8=Q"),
            String::from("d8=B"),
            String::from("d8=R"),
            String::from("d8=N"),
        ];

        for mv_string in moves_string {
            let result = parse_move(&mv_string, &board_state);
            if result.is_err() {
                println!(
                    "Error: {} when trying to parse move: {}",
                    result.unwrap_err(),
                    mv_string
                );
            }
            assert!(result.is_ok());
        }

        // Testing that invalid promotions are caught
        assert!(parse_move(&String::from("d8=K"), &board_state).is_err());
    }

    #[test]
    fn test_enpassant() {
        // todo!();
    }
}

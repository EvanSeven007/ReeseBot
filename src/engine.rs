use crate::board_state::BoardState;
use crate::chess_move::{Move, Position};
use crate::move_gen::{king_positions, gen_all_moves};
use crate::piece::{PieceType};
use crate::color::{Color};
use std::f32;

pub struct SearchResult {
    pub score: f32,
    pub move_found: Option<Move>,
}

//Returns a simple evaluation of the board state
fn evaluate(board: &BoardState) -> f32 {
    let mut eval: f32 = 0.0;
    //We will be taking account of pawn mobility, so we will update these every time we hit a pawn. 
    let mut pawn_pos: Position; //position of a pawn
    let mut behind: Position; //position behind pawn_pos
    let mut in_front: Position; //position in front of pawn_pos
    let mut mult: f32; //Multiplier (-1 or 1) depending on black/white
    for row in 2..10 {
        for col in 2..10 {
            let square = board.squares[row][col];
            if let Some(piece) = square.piece {
                eval += piece.worth();
                if piece.piece_type == PieceType::Pawn {
                    pawn_pos = Position{row, col};
                    match piece.color {
                        Color::White => {
                            behind = pawn_pos.down();
                            in_front = pawn_pos.up();
                            mult = -1.0;
                        },
                        Color::Black => {
                            behind = pawn_pos.up();
                            in_front = pawn_pos.down();
                            mult = 1.0;
                        },
                    }

                    //Doubled pawns

                    if let Some(behind_piece) = board.squares[behind.row][behind.col].piece {
                        if behind_piece.piece_type == PieceType::Pawn && behind_piece.color == piece.color {
                            eval += mult;
                        }
                    }

                    //Blocked pawns
                    if let Some(in_front_piece) = board.squares[in_front.row][in_front.col].piece {
                        if in_front_piece.piece_type == PieceType::Pawn && in_front_piece.color != piece.color {
                            eval += mult * 0.5;
                        }
                    }

                    //Isolated pawns
                    let mut is_isolated = true;
                    for pos in king_positions(pawn_pos) {
                        if let Some(neighbor) = board.squares[pos.row][pos.col].piece {
                            if neighbor.piece_type == PieceType::Pawn && neighbor.color == piece.color {
                                is_isolated = false;
                            }
                        }
                    }
                    if is_isolated {
                        eval += mult * 0.5;
                    }
                }
            }
        }
    }

    //Mobility
    let mut board_copy = board.clone();
    board_copy.active_color = Color::White;
    eval += 0.1 * (gen_all_moves(&board_copy, Color::White).len() as f32);
    board_copy.active_color = Color::Black;
    eval -= 0.1 * (gen_all_moves(&board_copy, Color::Black).len() as f32);

    eval 
}

fn quiesce(mut alpha: f32, beta: f32, board: &BoardState) -> f32 {
    let init_eval: f32 = evaluate(board);
    
    if init_eval >= beta {
        return beta;
    }
    if alpha < init_eval {
        alpha = init_eval;
    }

    let mut score: f32;
    let mut board_copy;
    let active_color = board.active_color;
    for mv in gen_all_moves(board, active_color) {
        match mv.piece_captured {
            Some(_) => {
                board_copy = board.clone();
                board_copy.make_move(&mv);
                score = -1.0 * quiesce(-1.0 * beta, -1.0 * alpha, &board_copy);
                if score >= beta {
                    return beta;
                }

                if score > alpha {
                    alpha = score;
                }
            },
            None => {}
        }
    }

    return alpha;
}

fn negamax(mut alpha: f32, beta: f32, depth_left: i32, board: &BoardState) -> f32 {
    let active_color = board.active_color;
    let moves = gen_all_moves(board, active_color);
    let mut score: f32;

    if moves.len() == 0 { //game over
        let active_color = board.active_color;
        if board.is_in_check(active_color, None) {
            return f32::MIN; //Game over by checkmate
        } else {
            return 0.0; //draw
        }
    }

    if depth_left == 0 {
        return quiesce(alpha, beta, board);
    }

    for mv in moves {
        let mut board_copy = board.clone();
        board_copy.make_move(&mv);
        score = -1.0 * negamax(-1.0 * beta, -1.0 * alpha, depth_left - 1, &board_copy);
        if score >= beta {
            return beta; //fail hard beta cutofff
        }

        if score > alpha {
            alpha = score;
        }
    }
    return alpha; 
}

pub fn find_move(board: &BoardState, depth: i32) -> SearchResult {
    let mut result = SearchResult{score: f32::MIN, move_found: None};
    let mut score;
    let active_color = board.active_color;
    for mv in gen_all_moves(board, active_color) {
        let mut board_copy = board.clone();
        board_copy.make_move(&mv);
        score = negamax(f32::MIN, f32::MAX, depth, &board_copy);
        if score > result.score {
            result.score = score;
            result.move_found = Some(mv);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_1() {
        let board_state_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - - -";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;
    
        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(evaluate(&board), 0.0);
    }
}
use crate::board_state::BoardState;
use crate::chess_move::{Move, MoveType};
use crate::evaluation::evaluate;
use crate::move_gen::gen_all_moves;
use std::cmp::{max, min};
use std::i32;
use std::time::Instant;
use simple_logger::SimpleLogger;
use log::{info, debug};

/* Everything drawn from https://www.chessprogramming.org/Main_Page */
/* Search struct idea drawm from https://github.com/MitchelPaulin/Walleye/blob/main/src/engine.rs */

const MATE_VALUE:i32 = 1000000000; //evaluation of a board state in mate
pub const MAX_DEPTH: u16 = 25;
pub const ARRAY_SIZE: usize =  ((MAX_DEPTH * MAX_DEPTH + MAX_DEPTH)/2 + 1) as usize;
type MoveList = [Option<Move>; ARRAY_SIZE];

pub struct Search {
    pub nodes_searched: u32, 
    pub pv_moves: MoveList,
    pub current_line: MoveList, //current line being searched
}

pub struct SearchResult {
    pub score: i32,
    pub move_found: Option<Move>,
}

impl Search {
    pub fn new() -> Search {
        Search {
            nodes_searched: 0,
            pv_moves: [None; ARRAY_SIZE],
            current_line: [None; ARRAY_SIZE],
        }
    }

    pub fn increment_nodes_searched(&mut self) {
        self.nodes_searched += 1;
    }

    pub fn insert_into_current_line(&mut self, ply: i32, mv: &Move) {
        self.current_line[ply as usize] = Some(*mv);
    }

    pub fn set_principle_variation(&mut self) {
        self.pv_moves = self.current_line.clone();
    }

    pub fn reset_search(&mut self) {
        self.nodes_searched = 0;
        self.current_line = [None; ARRAY_SIZE];
    }
}

fn quiesce(mut alpha: i32, mut beta: i32, search: &mut Search, board: &BoardState) -> i32 {
    let init_eval: i32 = evaluate(board);
    
    search.increment_nodes_searched();

    if init_eval >= beta {
        return beta;
    }
    if alpha < init_eval {
        alpha = init_eval;
    }

    let mut score: i32;
    let mut board_copy;
    let active_color = board.active_color;
    for mv in gen_all_moves(board, active_color) {
        match mv.piece_captured {
            Some(_) => {
                board_copy = board.clone();
                board_copy.make_move(&mv);
                score = -1 * quiesce(-1 * beta, -1 * alpha, search, &board_copy);
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

fn alpha_beta(mut alpha: i32, mut beta: i32, mut depth: u16, search: &mut Search, ply: i32, board: &BoardState) -> i32 {
    search.increment_nodes_searched();
    let ply_index: usize = ply as usize;

    if depth == 0 {
        let active_color = board.active_color;
        if board.is_in_check(active_color, None) {
            depth += 1;
        } else {
            return quiesce(alpha, beta, search, board);
        }
    }

    alpha = max(alpha, ply - MATE_VALUE);
    beta = min(beta,  MATE_VALUE - ply);

    if alpha >= beta {
        return alpha;
    }

    let active_color = board.active_color;
    let mut moves = gen_all_moves(board, active_color);

    //Game over
    if moves.len() == 0 {
        if board.is_in_check(active_color, None) {
            return -1 * (MATE_VALUE - ply); //Checkmate
        } else {
            return 0; //Draw by stalemate
        }
    }

    let mut potential_best_score: i32 = i32::MIN;
    let mut other_moves: Vec<Move> = Vec::new();
    //Calculating on principal variation first
    for mv in &mut moves {
        if board.last_move == search.pv_moves[ply_index] {
            let mut board_copy = board.clone();
            board_copy.make_move(mv);
            search.insert_into_current_line(ply, mv); //Ply or ply + 1?
            potential_best_score = -alpha_beta(-1 * beta, -1 * alpha, depth - 1, search, ply + 1, &board_copy);
            if potential_best_score > alpha {
                if potential_best_score >= beta {
                    return potential_best_score
                }
                search.set_principle_variation();
                alpha = potential_best_score;
            }
        } else {
            other_moves.push(*mv);
        }
    }
    
    for mv in &mut other_moves {
        search.insert_into_current_line(ply, mv);
        let mut board_copy = board.clone();
        board_copy.make_move(mv);
        let mut score = -alpha_beta(-1 * beta, -1 * alpha, depth - 1, search, ply + 1, &board_copy);
        if -1 * score > potential_best_score {
            if score >= beta {
                return score;
            }
            search.set_principle_variation();
            potential_best_score = score;
        }

    }

    potential_best_score
}

pub fn calculate_best_move(board: &BoardState, time_to_think: u64) -> SearchResult {
    let mut result = SearchResult{score: i32::MIN, move_found: None};
    let mut depth = 2;
    let mut ply = 0;
    let mut search = Search::new();

    let active_color = board.active_color;
    let mut moves = gen_all_moves(board, active_color);
    let mut alpha: i32 = -100000;
    let mut beta: i32 = 100000;
    let start = Instant::now();
    let mut nodes_searched = 0;
    while depth < MAX_DEPTH {
        nodes_searched += search.nodes_searched;
        // debug!("Trying Depth: {}, Nodes Searched: {}", depth, nodes_searched);
        search.reset_search();

        for mv in &moves {
            //Ending after two minutes seconds
            if start.elapsed().as_secs() > time_to_think {
                match result.move_found {
                    Some(_) => return result,
                    None => {
                        println!("no movef found!");
                        result.move_found = Some(moves[0]);
                        return result;
                    }
                }
            }
            let mut board_copy = board.clone();
            board_copy.make_move(mv);
            //-1 or positive one??
            let eval = -1 * alpha_beta(-1 * beta, -1 * alpha, depth - 1, &mut search, ply + 1, &board_copy);
    
            search.insert_into_current_line(ply, mv);
            if eval > alpha {
                if eval > result.score {
                    result.move_found = Some(*mv);
                    result.score = eval;
                }
                alpha = eval;
                search.set_principle_variation();
            }
        }

        depth += 1;
    }
    //
    result
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::{board_state, chess_move::MoveType, piece::{self, PieceType}};

    use super::*;

    #[test]
    fn sanity_check() {
        let board_state_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - - -";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;
    
        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

    }

    #[test]
    fn sanity_check2() {
        let board_state_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - - -";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;
    
        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

    }

    #[test]
    fn takes_queen() {
        let board_state_fen: &str = "k7/6q1/5P2/8/8/8/8/K7 w - - 0 1";
        let mut board_state: BoardState= BoardState::new(board_state_fen).unwrap_or_else(|e| panic!("Error creating board state"));
        let best_move = calculate_best_move(&board_state, 5).move_found.unwrap();
        board_state.print_board();
        board_state.make_move(&best_move);
        board_state.print_board();

        match best_move.piece_captured {
            Some(piece) => {
                // Handle the case when a piece is captured
                if (piece.piece_type == PieceType::Queen) {
                    assert!(true);
                } else {
                    panic!("Queen should have been captured!");
                }
            }
            None => {
                panic!("Something should have been captured.")
            }
        }
    }

    #[test]
    fn takes_queen_over_rook() {
        let board_state_fen: &str = "k7/4r1q1/5P2/8/8/8/8/K7 w - - 0 1";
        let mut board_state: BoardState= BoardState::new(board_state_fen).unwrap_or_else(|e| panic!("Error creating board state"));
        let best_move = calculate_best_move(&board_state, 5).move_found.unwrap();
        println!("{:?}", best_move);
        board_state.print_board();
        board_state.make_move(&best_move);
        board_state.print_board();


        match best_move.piece_captured {
            Some(piece) => {
                // Handle the case when a piece is captured
                if (piece.piece_type == PieceType::Queen) {
                    assert!(true);
                } else {
                    panic!("Queen should have been captured!");
                }
            }
            None => {
                panic!("Something should have been captured.")
            }
        }
    }

    #[test]
    fn finds_mate() {
        let board_state_fen: &str = "rnbqkbnr/ppppp2p/5p2/6p1/3PP3/8/PPP2PPP/RNBQKBNR w KQkq d4 0 1";
        let mut board_state: BoardState= BoardState::new(board_state_fen).unwrap_or_else(|e| panic!("Error creating board state"));
        let best_move = calculate_best_move(&board_state, 5).move_found.unwrap();
        println!("{:?}", best_move);
        board_state.print_board();
        board_state.make_move(&best_move);
        board_state.print_board();


        match best_move.move_type {
            MoveType::Standard(std_move) => {
                if (std_move.piece_moved.piece_type != PieceType::Queen) {
                    panic!("Queen should have been moved!");
                } else {
                    assert!(std_move.after.row == 5 && std_move.after.col == 9);
                }
            }
            _ => {
                panic!("Should have moved Queen in for Checkmate")
            }
        }
    }


}
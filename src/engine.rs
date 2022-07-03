use crate::board_state::BoardState;
use crate::chess_move::{Move};
pub struct SearchResult {
    pub score: i32,
    pub move_found: Option<Move>,
}

fn evaluate(board: &BoardState) -> i32 {
    42
}

fn quiesce(alpha: i32, beta: i32, board: &BoardState) -> i32 {
    42
}

fn alphaBeta(alpha: i32, beta: i32, depthleft: i32, board: &BoardState) -> i32 {
    42
}

pub fn find_move(board: &BoardState, depth: i32) -> SearchResult {
    let mut result = SearchResult{score: -10000, move_found: None};
    result 
}
use crate::board_state::*;
use crate::chess_move::*;
use crate::move_gen::{find_pieces, gen_all_moves};
use crate::color::{Color};
use std::cmp::{min, max};
use std::i32;
use std::i64;
use std::i128;
use std::process::exit;

pub struct minimax_res {
    pub score: i32,
    pub curr_move: Move,
}

fn evaluate(board: &BoardState) -> i32 {
    let mut eval = 0;
    for row in 2..=9 {
        for col in 2..=10 {
            if let Some(piece) = board.squares[row][col].piece {
                eval += piece.worth();
            }
        }
    }

    eval
}

fn minimax(depth: i32, board: &BoardState, isMaximizing: bool) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let mut best_score: i32; 
    let mut curr_score: i32;
    let mut best_move: Move;
    let mut copy = board.clone();
    let active_color = copy.active_color;
    let moves = gen_all_moves(&mut copy, active_color);
    if isMaximizing {
        best_score = std::i32::MIN;
        for mv in moves {
            let mut board_clone = board.clone();
            board_clone.make_move(&mv);
            best_score = max(best_score, minimax(depth - 1, &board_clone, !isMaximizing));
        }
    } else {
        best_score = std::i32::MAX;
        for mv in moves {
            let mut board_clone = board.clone();
            board_clone.make_move(&mv);
            best_score = min(best_score, minimax(depth - 1, &board_clone, isMaximizing));
        }
    }

    best_score
}

//Huge Bugs here
pub fn find_move(depth: i32, board: &mut BoardState) -> Move {
    let is_maximizing: bool;
    let mut best_score: i32= std::i32::MAX;
    let mult: i32;
    //Figuring out which to maximize
    match board.active_color {
        Color::White => {
            is_maximizing = true;
            mult = 1;
        },
        Color::Black => {
            is_maximizing = false;
            mult = -1;
        }
    }

    let active_color = board.active_color;
    let moves = gen_all_moves(board, active_color);
    if moves.len() == 0 {
        stop_game(board);
    }

    let mut best_move = moves[0]; 
    for mv in moves {
        let mut board_copy = board.clone();
        board_copy.make_move(&mv);
        let minimax_result = minimax(depth - 1, &board_copy, !is_maximizing);
        if minimax_result >= best_score {
            best_move = mv;
            best_score = minimax_result;
        }
    }

    best_move
}

fn stop_game(board: &mut BoardState) {
    if !board.is_in_check(board.active_color, None){
        match board.active_color {
            Color::White => board.status = BoardStatus::BlackWin,
            Color::Black => board.status = BoardStatus::WhiteWin,
        }
        println!("GAME OVER BY CHECKMATE: {} has defeated {}", board.active_color.opposite().color_to_string(), board.active_color.color_to_string());
    } else {
        board.status = BoardStatus::Draw;
        println!("Game over by Stalemate!");
    }
    exit(1);
}
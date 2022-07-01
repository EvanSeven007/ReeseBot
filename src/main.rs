mod piece;
mod color;
mod square;
mod chess_move;
mod board_state;
mod move_gen;
mod minimax;

use board_state::BoardState;
use rand::seq::SliceRandom;
use crate::minimax::{find_move};
use crate::move_gen::gen_all_moves;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let board_state_fen = "r3k2r/p1p1qpb1/bn1ppnp1/3PN3/1p2P3/P1N2Q1p/1PPBBPPP/R4K1R w kq - - -";
    let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
    let mut board: BoardState;

    match board_state {
        Ok(_) => board = board_state.unwrap(),
        Err(e) => panic!("Error: {}", e),
    }

    let active_color = board.active_color;
    board.print_board();
    let moves = gen_all_moves(&mut board, active_color);
    let mut num_moves = 0;
    println!("{}", moves.len());
    for mv in moves {
        let mut board_copy = board.clone();
        board_copy.make_move(&mv);
        let num_moves_curr = count_moves(1, &board_copy);
        num_moves += num_moves_curr;
        println!("{} {}", mv.to_string(), num_moves_curr);
        //println!("king_w: {}, queen_w: {}, king_b: {}, queen_b: {}", board_copy.castle_rights.can_castle_white_kingside, board_copy.castle_rights.can_castle_white_queenside, board_copy.castle_rights.can_castle_black_kingside, board_copy.castle_rights.can_castle_black_queenside);
        //board_copy.print_board();
    }
    println!("{}", num_moves);
}


fn count_moves(depth: u8, board: &BoardState) -> i64 {
    if depth == 0 {
        return 1;
    }

    let moves = gen_all_moves(board, board.active_color);
    let mut num_positions: i64 = 0;
    
    for mv in moves {
        let board_copy = &mut board.clone();
        board_copy.make_move(&mv);
        num_positions += count_moves(depth - 1, board_copy);
    }

    num_positions
}

//Tests
#[cfg(test)]
mod tests {
    use super::*;


    #[test] //Making sure the number of moves is correct
    fn move_test_standard_pos() {
        let board_state_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3 25";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;
        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(count_moves(0, &mut board), 1);
        assert_eq!(count_moves(1, &mut board), 20);
        assert_eq!(count_moves(2, &mut board), 400);
        assert_eq!(count_moves(3, &mut board), 8902);
        //assert_eq!(count_moves(4, &mut board), 197281);
        //assert_eq!(count_moves(5, &mut board), 4865609);
    }

    #[test] //Making sure the number of moves is correct
    fn move_test_second_pos() {
        let board_state_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - - -";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;
        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(count_moves(0, &mut board), 1);
        assert_eq!(count_moves(1, &mut board), 48);
        assert_eq!(count_moves(2, &mut board), 2039);
        assert_eq!(count_moves(3, &mut board), 97862);
        assert_eq!(count_moves(4, &mut board), 4085603);
    }

    #[test] //Making sure the number of moves is correct
    fn move_test_third_pos() {
        let board_state_fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - - -";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;

        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(count_moves(0, &mut board), 1);
        assert_eq!(count_moves(1, &mut board), 14);
        assert_eq!(count_moves(2, &mut board), 191);
        assert_eq!(count_moves(3, &mut board), 2812);
        assert_eq!(count_moves(4, &mut board), 43238);
    }

    //Making sure that the correct board state is reflected 
    //
}
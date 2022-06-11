mod piece;
mod color;
mod square;
mod chess_move;
mod board_state;
use piece::{PieceType, Piece};
use color::Color;
use square::Square;
use chess_move::*;
use board_state::BoardState;


fn main() {
    let board_state_fen = "r4nk1/4Rp1p/1p4p1/pP1N4/3P2P1/3r4/5P1P/R5K1 b - - 3 25";
    let mut board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
    let mut board: BoardState;

    match board_state {
        Ok(_) => board = board_state.unwrap(),
        Err(e) => panic!("Error: {}", e),
    }
    
    println!("STARTING::::::");
    let moves = board.gen_all_moves();
    for mv in moves {
        let mut cl = board.clone();
        cl.make_move(&mv);
        cl.print_board();
    }
}
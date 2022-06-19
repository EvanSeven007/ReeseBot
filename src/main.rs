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
    let board_state_fen = "k7/5P2/8/8/8/8/8/K7 w - - 3 25";
    let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
    let board: BoardState;

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
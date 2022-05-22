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
    let board_state_fen = "8/8/8/8/8/qr6/Kq6/8 w KQkq - 0 1";
    let mut board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
    let mut board: BoardState;

    match board_state {
        Ok(_) => board = board_state.unwrap(),
        Err(e) => panic!("Error: {}", e),
    }

    board.print_board();
    println!("STARTING::::::");
    let moves = board.gen_all_moves();
    for mv in moves {
        let mut cl = board.clone();
        cl.make_move(mv);
        println!("============================");
        cl.print_board();
        println!("============================");
    }
}
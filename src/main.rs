mod piece;
mod color;
mod square;
mod chess_move;
mod board_state;
mod move_gen;
use piece::{PieceType, Piece};
use board_state::BoardState;
use rand::seq::SliceRandom;
use std::io;

fn main() {
    let board_state_fen = "rnbqkbnr/pppppppp/8/8/8/8/QQQQQQQQ/QQQQKQQQ w KQkq - 3 25"; 
    let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
    let mut board: BoardState;

    match board_state {
        Ok(_) => board = board_state.unwrap(),
        Err(e) => panic!("Error: {}", e),
    }

    println!("STARTING::::::");
    let mut input = String::new();
    let mut moves;
    board.print_board();
    //Dumb loop for testing purposes
    loop {
        io::stdin().read_line(&mut input).expect("failed to readline");
        match input.as_str().trim() {
            "" => {
                moves = move_gen::gen_all_moves(&board);
                board.make_move(moves.choose(&mut rand::thread_rng()).unwrap());
                board.print_board();
            }
            _ => {
                break;
            }
        }
        input = String::new();
    }
}
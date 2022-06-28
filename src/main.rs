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

fn main() {
    let board_state_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3 25"; 
    let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
    let mut board: BoardState;

    match board_state {
        Ok(_) => board = board_state.unwrap(),
        Err(e) => panic!("Error: {}", e),
    }

    println!("STARTING::::::");

    //nonce
    //Play a random game
    let mut move_count: u64 = 0;
    print!("\x1B[2J\x1B[1;1H");
    board.print_board();
    let DEPTH: i32 = 4;
    loop {
        let best_move = find_move(DEPTH, &mut board);
        board.make_move(&best_move);
        print!("\x1B[2J\x1B[1;1H"); //clear screen
        board.print_board();
   }
}
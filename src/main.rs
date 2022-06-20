#![warn(clippy::pedantic)]

mod board_state;
mod chess_move;
mod color;
mod move_gen;
mod piece;
mod square;
use board_state::BoardState;

fn main() {
    let board_state_fen = "8/k7/3r4/8/8/3R4/8/K7 w - - 3 25";
    let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
    let board: BoardState;

    if let Err(e) = board_state {
        panic!("Error: {}", e)
    } else {
        board = board_state.unwrap();
    };

    println!("STARTING::::::");
    let moves = move_gen::gen_all_moves(&board);
    for mv in moves {
        let mut cl = board;
        cl.make_move(&mv);
        cl.print_board();
    }
}

mod piece;
mod color;
mod square;
mod chess_move;
mod board_state;
mod move_gen;
use piece::{PieceType, Piece};
use board_state::BoardState;


fn main() {
    let board_state_fen = "8/k7/3r4/8/8/3R4/8/K7 w - - 3 25";
    let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
    let board: BoardState;

    match board_state {
        Ok(_) => board = board_state.unwrap(),
        Err(e) => panic!("Error: {}", e),
    }
    
    
    println!("STARTING::::::");
    let moves = move_gen::gen_all_moves(&board);
    for mv in moves {
        let mut cl = board.clone();
        cl.make_move(&mv);
        cl.print_board();
    }
}
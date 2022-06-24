mod piece;
mod color;
mod square;
mod chess_move;
mod board_state;
mod move_gen;
use piece::{PieceType, Piece};
use board_state::BoardState;


fn main() {
    let board_state_fen = "3k4/8/8/5pP1/8/8/r7/R3K3 w Q f5 3 25"; //Still recognizing as Kinside castle
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
        if let Some(pos) = cl.en_passant {
            println!("{} {}", pos.x, pos.y);
        }
    }
}
mod piece;
mod color;
mod square;
mod chess_move;
mod board_state;
mod move_gen;
use board_state::BoardState;
use rand::seq::SliceRandom;

fn main() {
    let board_state_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3 25"; 
    let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
    let mut board: BoardState;

    match board_state {
        Ok(_) => board = board_state.unwrap(),
        Err(e) => panic!("Error: {}", e),
    }

    println!("STARTING::::::");
    let mut moves;

    //Play a random game
    let mut move_count: u64 = 0;
    loop {
        moves = move_gen::gen_all_moves(&board);
        board.make_move(moves.choose(&mut rand::thread_rng()).unwrap());
        move_count += 1;
        println!("move count: {}", move_count);
        board.print_board();
    }
}
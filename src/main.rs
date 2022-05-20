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
    let board_state = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board: Result<BoardState, &str> = BoardState::new(board_state);

    match board {
        Ok(_) => board.unwrap().print_board(),
        Err(e) => println!("Error: {}", e),
    }


    //Next idea is to make a function that generates all possible moves from a given position
        //This is computationally not as bad as it seems, as the number of possible moves from a given position is almost always < 100
    //Then, take in a move parser that simply takes in a move and tries to find a corresponding move in the move_set that corresponds to it,
        //as in the user writes "Nf5" and then it creates a Knight move move, and then tries to find the corresponding move the move set and use board.make_move()
    //If none exist or error with prompting => ask the user for another input
    //Finally, we will do some testing with a couple of full games. 

    //Then, we do some refactoring to get rid of inevitable redudancy

    //Then, it's algorithm time!!
}
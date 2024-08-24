mod piece;
mod color;
mod square;
mod chess_move;
mod board_state;
mod move_gen;
mod engine;
mod move_parser;
mod evaluation;

use board_state::BoardState;
use crate::move_gen::gen_all_moves;
use crate::move_parser::parse_move;
use crate::color::Color;
use crate::engine::find_move;
use simple_logger::SimpleLogger;
use log::{info};
use std::env;

fn main() {
    SimpleLogger::new().without_timestamps().init().unwrap();
    info!("Hello! I am Reese Bot, a chess playing program created by Evan Stegall (https://github.com/EvanSeven007)");
    info!("To play, you can either use the MOVE or RESIGN command");
    info!("To move a piece from point A to point B, use\nMOVE before after where before, after are squares in algebraic notation (i.e. e4, d4)");
    info!("To castle, use MOVE 0-0 or MOVE 0-0-0 for king/queen side castle respecively");
    info!("For pawn promotions, simply type MOVE before after=(Q, B, R, N) where Q = Queen, B = Bishop, R = Rook, N = Knight");
    info!("To resign the game, type RESIGN");
    info!("");
    let board_state_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - - -";
    let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
    let mut board: BoardState;

    match board_state {
        Ok(_) => board = board_state.unwrap(),
        Err(e) => panic!("Error: {}", e),
    }

    board.print_board();
    let mut moves; 
    loop {
        let mut input = String::new();
        moves = gen_all_moves(&board, Color::White);
        if moves.len() == 0 {
            if board.is_in_check(Color::White, None) {
                info!("Black has won the game");
            } else {
                info!("Game over by draw");
            }
            break;
        }

        match board.active_color {
            Color::White => {
                println!("Please enter a move: ");
                std::io::stdin().read_line(&mut input).unwrap();
                match parse_move(input, &board, moves) {
                    Ok(mv) => {
                        board.make_move(&mv);
                        board.print_board();
                    },
                    Err(e) => {
                        info!("Error: {}", e);
                    }
                }
            },
            Color::Black => {
                let result = find_move(&board);
                if let Some(mv) = result.move_found {
                    board.make_move(&mv);
                    board.print_board();
                } else { //Black has no moves
                    if board.is_in_check(Color::Black, None) {
                        info!("White has won the game");
                    } else {
                        info!("Game over by draw");
                    }
                    break;
                }
            }
        }
    }
}

//MOVE, RESIGN

//Perft Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn count_moves(depth: u16, board: &BoardState) -> i64 {
        if depth == 0 {
            return 1;
        }
    
        let moves = gen_all_moves(board, board.active_color);
        let mut num_positions: i64 = 0;
        
        for mv in moves {
            let board_copy = &mut board.clone();
            board_copy.make_move(&mv);
            num_positions += count_moves(depth - 1, board_copy);
        }
    
        num_positions
    }

    #[test] //Making sure the number of moves is correct
    fn move_test_standard_pos() {
        let board_state_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3 25";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;
        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(count_moves(0, &mut board), 1);
        assert_eq!(count_moves(1, &mut board), 20);
        assert_eq!(count_moves(2, &mut board), 400);
        assert_eq!(count_moves(3, &mut board), 8902);
        assert_eq!(count_moves(4, &mut board), 197281);
        assert_eq!(count_moves(5, &mut board), 4865609);
    }

    #[test] //Making sure the number of moves is correct
    fn move_test_second_pos() {
        let board_state_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - - -";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;
        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(count_moves(0, &mut board), 1);
        assert_eq!(count_moves(1, &mut board), 48);
        assert_eq!(count_moves(2, &mut board), 2039);
        assert_eq!(count_moves(3, &mut board), 97862);
        assert_eq!(count_moves(4, &mut board), 4085603);
    }

    #[test] //Making sure the number of moves is correct
    fn move_test_third_pos() {
        let board_state_fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - - -";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;

        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(count_moves(0, &mut board), 1);
        assert_eq!(count_moves(1, &mut board), 14);
        assert_eq!(count_moves(2, &mut board), 191);
        assert_eq!(count_moves(3, &mut board), 2812);
        assert_eq!(count_moves(4, &mut board), 43238);
    }

    #[test] //Making sure the number of moves is correct
    fn move_test_fourth_pos() {
        let board_state_fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;

        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(count_moves(0, &mut board), 1);
        assert_eq!(count_moves(1, &mut board), 6);
        assert_eq!(count_moves(2, &mut board), 264);
        assert_eq!(count_moves(3, &mut board), 9467);
        assert_eq!(count_moves(4, &mut board), 422333);
        assert_eq!(count_moves(5, &mut board), 15833292);
    }

    #[test] //Making sure the number of moves is correct
    fn move_test_fifth_pos() {
        let board_state_fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;

        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(count_moves(0, &mut board), 1);
        assert_eq!(count_moves(1, &mut board), 44);
        assert_eq!(count_moves(2, &mut board), 1486);
        assert_eq!(count_moves(3, &mut board), 62379);
        assert_eq!(count_moves(4, &mut board), 2103487);
    }

    #[test] //Making sure the number of moves is correct
    fn move_test_sixth_pos() {
        let board_state_fen = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
        let board_state: Result<BoardState, &str> = BoardState::new(board_state_fen);
        let mut board: BoardState;

        match board_state {
            Ok(_) => board = board_state.unwrap(),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(count_moves(0, &mut board), 1);
        assert_eq!(count_moves(1, &mut board), 46);
        assert_eq!(count_moves(2, &mut board), 2079);
        assert_eq!(count_moves(3, &mut board), 89890);
        assert_eq!(count_moves(4, &mut board), 3894594);
    }

    //Making sure that the correct board state is reflected 
    //
}
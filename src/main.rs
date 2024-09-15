#![allow(warnings)]

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
use crate::engine::calculate_best_move;
use simple_logger::SimpleLogger;
use log::{info, error};
use std::env;

fn main() {
    SimpleLogger::new().without_timestamps().init().unwrap();
    info!("Hello! I am Reese Bot, a CLI based (for now) chess engine.");
    info!("To play, simply type your move in standard fen string notation.");
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
                        clear_screen();
                        board.make_move(&mv);
                        board.print_board();
                    },
                    Err(e) => {
                        error!("Error: {}", e);
                    }
                }
            },
            Color::Black => {
                println!("Thinking...");
                let result = calculate_best_move(&board, 10);
                if let Some(mv) = result.move_found {
                    clear_screen();
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

fn clear_screen() {
    print!("{}[2J", 27 as u8 as char);
}

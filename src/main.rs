#![allow(warnings)]

mod board_state;
mod chess_move;
mod color;
mod engine;
mod evaluation;
mod move_gen;
mod move_parser;
mod piece;
mod square;

use crate::color::Color;
use crate::engine::calculate_best_move;
use crate::move_gen::gen_all_moves;
use crate::move_parser::parse_move;
use board_state::BoardState;
use clap::{ArgAction, Parser};
use log::{error, info};
use move_parser::validate_move;
use simple_logger::SimpleLogger;
use std::env;

const DEFAULT_BOARD_STATE: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - - -";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Desired starting board state denoted in fen notation
    #[arg(short, long, default_value = DEFAULT_BOARD_STATE)]
    fen: String,

    /// Number in seconds allocated to engine to think
    #[arg(short, long, default_value_t = 10)]
    time_to_think: u64,

    /// Enables the engine mode
    #[arg(long, action = ArgAction::SetTrue)]
    engine_mode: bool,
}

fn main() {
    SimpleLogger::new().without_timestamps().init().unwrap();
    info!("Hello! I am Reese Bot, a CLI based (for now) chess engine.");
    info!("To play, simply type your move in standard fen string notation.");
    info!("");
    let args = Args::parse();

    play_game(&args.fen, args.time_to_think, args.engine_mode);
}

fn play_game(board_state_fen: &str, time_to_think: u64, engine_mode: bool) {
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
                match parse_move(&input, &board, engine_mode) {
                    Ok(mv) => {
                        clear_screen();
                        board.make_move(&mv);
                        board.print_board();
                    }
                    Err(e) => {
                        error!("Received Invalid Move: {}", e);
                    }
                }
            }
            Color::Black => {
                println!("Thinking...");
                let result = calculate_best_move(&board, time_to_think);
                if let Some(mv) = result.move_found {
                    clear_screen();
                    board.make_move(&mv);
                    board.print_board();
                } else {
                    //Black has no moves
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

// Weird hack but it works
fn clear_screen() {
    print!("{}[2J", 27 as u8 as char);
}

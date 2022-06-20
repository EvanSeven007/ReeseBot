use crate::board_state::BoardState;
use crate::chess_move::Position;
use crate::chess_move::{castle, en_passant, promotion, standard, Move, MoveValue};
use crate::color::Color;
use crate::piece::{Piece, PieceType};
use std::collections::HashSet;
use std::process::exit;

pub fn gen_all_moves(board: &BoardState) -> Vec<Move> {
    /* Storing the positions of the white and black pieces */
    let mut white_pieces_pos: HashSet<Position> = HashSet::new();
    let mut black_pieces_pos: HashSet<Position> = HashSet::new();
    let mut opt_king_pos: Option<Position> = None;

    //TODO Make this a function
    for x in 1..9 {
        for y in 1..9 {
            let curr_piece: Option<Piece> = board.squares[x][y].piece;
            match curr_piece {
                Some(val) => {
                    match val.color {
                        Color::White => {
                            white_pieces_pos.insert(Position { x, y });
                        }
                        Color::Black => {
                            black_pieces_pos.insert(Position { x, y });
                        }
                    }

                    //Found the king
                    if val.piece_type == PieceType::King && val.color == board.active_color {
                        opt_king_pos = Some(Position { x, y });
                    }
                }
                None => {}
            }
        }
    }

    let king_pos: Position = opt_king_pos.expect("Could not find the king!");

    /* Current set is the one we are on */
    let curr_set: HashSet<Position>;
    if let Color::White = board.active_color {
        curr_set = white_pieces_pos;
    } else {
        curr_set = black_pieces_pos;
    }

    let mut move_set: Vec<Move> = Vec::new(); /* change this to a set later */

    let mut curr_piece: Piece;
    let mut before: Position;
    let mut after: Position;
    for pos in &curr_set {
        curr_piece = board.squares[pos.x][pos.y].piece.unwrap(); //Guarantted to not be None

        match curr_piece.piece_type {
            /*
             * Pawn Moves
             */
            PieceType::Pawn => {
                let (right_up, left_up, oneup, twoup, en_passant_left, en_passant_right) =
                    generate_pawn_moves_helper(pos, curr_piece.color);
                let (first_move, is_promotion) = generate_pawn_permissions(pos, curr_piece.color);

                if !is_promotion {
                    // Capturing a piece but not a promotion

                    // Generating moves for 3 pawn moves
                    move_set.extend(generate_moves_from_pawn(
                        board,
                        curr_piece,
                        *pos,
                        right_up,
                        left_up,
                        oneup,
                        twoup,
                        en_passant_right,
                        en_passant_left,
                    ));

                    //Checking enpassant
                    match board.en_passant {
                        Some(val) => {
                            let side_right = Position {
                                x: pos.x,
                                y: pos.y - 1,
                            };
                            let side_left = Position {
                                x: pos.x,
                                y: pos.y + 1,
                            };
                            if side_right == val {
                                move_set.push(en_passant(
                                    *pos,
                                    en_passant_right,
                                    val,
                                    Some(Piece {
                                        piece_type: PieceType::Pawn,
                                        color: board.active_color.opposite(),
                                    }),
                                ));
                            } else if side_left == val {
                                move_set.push(en_passant(
                                    *pos,
                                    en_passant_left,
                                    val,
                                    Some(Piece {
                                        piece_type: PieceType::Pawn,
                                        color: board.active_color.opposite(),
                                    }),
                                ));
                            }
                        }
                        None => {}
                    }
                } else {
                    //Generating all promotion moves
                    move_set.extend(generate_moves_from_piece(
                        board,
                        &MoveValue::PromotionMove,
                        curr_piece,
                        *pos,
                        vec![right_up, left_up, oneup],
                        None,
                    ));
                }
            }
            //Make a fn for this
            PieceType::King => {
                let possible_king_positions: Vec<Position> = generate_king_moves_helper(pos);
                move_set.extend(generate_moves_from_piece(
                    board,
                    &MoveValue::StandardMove,
                    curr_piece,
                    *pos,
                    possible_king_positions,
                    None,
                ));
            }
            PieceType::Knight => {
                let possible_knight_positions: Vec<Position> = generate_knight_moves_helper(pos);
                move_set.extend(generate_moves_from_piece(
                    board,
                    &MoveValue::StandardMove,
                    curr_piece,
                    *pos,
                    possible_knight_positions,
                    None,
                ));
            }

            PieceType::Rook => {
                //Look vertical and horizontal until you hit a piece
                //For four loops from 0..8, each stopping one a certain position
                //Store all possible positions, then add moves
                //Consider making a "generate rook moves"
                let possible_rook_positions: Vec<Position> =
                    generate_rook_moves_helper(board, pos, board.active_color);
                move_set.extend(generate_moves_from_piece(
                    board,
                    &MoveValue::StandardMove,
                    curr_piece,
                    *pos,
                    possible_rook_positions,
                    None,
                ));
            }
            PieceType::Bishop => {
                //Look diagonally, for four loops
                //"Generate bishop moves"
                let possible_bishop_positions: Vec<Position> =
                    generate_bishop_moves_helper(board, pos, board.active_color);
                move_set.extend(generate_moves_from_piece(
                    board,
                    &MoveValue::StandardMove,
                    curr_piece,
                    *pos,
                    possible_bishop_positions,
                    None,
                ));
            }
            PieceType::Queen => {
                //Look diagonally, vertically, and horizontally
                //copy the loops from above
                //Vec.push(generatebishop moves, generate rook moves, )
                let possible_queen_positions: Vec<Position> =
                    generate_rook_moves_helper(board, pos, board.active_color);
                move_set.extend(generate_moves_from_piece(
                    board,
                    &MoveValue::StandardMove,
                    curr_piece,
                    *pos,
                    possible_queen_positions,
                    None,
                ));
            }
            PieceType::None => {}
        }
    }

    // Find the king in every call to, fix this with a do, undo move pattern
    let mut legal_moves: Vec<Move> = Vec::new();
    for mv in move_set {
        let mut board_copy: BoardState = *board;
        board_copy.make_move(&mv);
        board_copy.active_color = board_copy.active_color.opposite();
        // Why does this work?
        if !board_copy.is_in_check() {
            legal_moves.push(mv);
        }
    }

    if legal_moves.is_empty() {
        if board.is_in_check() {
            println!(
                "GAME OVER BY CHECKMATE: {} has defeated {}",
                board.active_color.opposite().color_to_string(),
                board.active_color.color_to_string()
            );
        } else {
            println!("Game over by Stalemate!");
        }
        exit(1);
    }
    legal_moves
}

pub fn generate_moves_from_pawn(
    board: &BoardState,
    curr_piece: Piece,
    curr_piece_position: Position,
    right_up: Position,
    left_up: Position,
    one_up: Position,
    two_up: Position, //Has a value if two_up is not occupied
    right: Position,
    left: Position,
) -> Vec<Move> {
    let mut pawn_moves: Vec<Move> = Vec::new();

    if !board.squares[one_up.x][one_up.y].is_occupied() {
        pawn_moves.push(standard(curr_piece_position, one_up, curr_piece, None));
    }

    if !board.squares[two_up.x][two_up.y].is_occupied() {
        pawn_moves.push(standard(curr_piece_position, two_up, curr_piece, None));
    }

    //Capturing left and right
    for pos in &[left_up, right_up] {
        if board.squares[pos.x][pos.y].is_occupied() {
            let p = board.squares[pos.x][pos.y].piece.unwrap();
            if p.color == curr_piece.color.opposite() {
                pawn_moves.push(standard(curr_piece_position, *pos, curr_piece, Some(p)));
            }
        }
    }

    match board.en_passant {
        Some(val) => {
            for pos in &[left, right] {
                if *pos == val {
                    pawn_moves.push(en_passant(
                        curr_piece_position,
                        val,
                        val,
                        Some(Piece {
                            piece_type: PieceType::Pawn,
                            color: curr_piece.color.opposite(),
                        }),
                    ));
                }
            }
        }
        None => {}
    }

    pawn_moves
}
/* Generates moves from a given piece and related info */
pub fn generate_moves_from_piece(
    board: &BoardState,
    move_type: &MoveValue,
    curr_piece: Piece,
    curr_piece_position: Position,
    possible_positions: Vec<Position>,
    castle_bool: Option<bool>,
) -> Vec<Move> {
    let mut generated_moves: Vec<Move> = Vec::new();
    let is_stoppable: bool = (curr_piece.piece_type == PieceType::Rook)
        || (curr_piece.piece_type == PieceType::Bishop)
        || (curr_piece.piece_type == PieceType::Queen);

    for position in possible_positions {
        if !position.is_valid_position() {
            continue;
        }

        match move_type {
            MoveValue::StandardMove => {
                match board.squares[position.x][position.y].piece {
                    Some(val) => {
                        if val.color == board.active_color.opposite() {
                            generated_moves.push(standard(
                                curr_piece_position,
                                position,
                                curr_piece,
                                Some(val),
                            ));
                        }
                        if is_stoppable {
                            continue; //Break?
                        }
                    }
                    None => generated_moves.push(standard(
                        curr_piece_position,
                        position,
                        curr_piece,
                        None,
                    )),
                }
            }
            MoveValue::CastleMove => {
                generated_moves.push(castle(castle_bool.unwrap()));
            }
            MoveValue::PromotionMove => match board.squares[position.x][position.y].piece {
                Some(val) => {
                    if val.color == board.active_color.opposite() {
                        generated_moves.push(promotion(
                            curr_piece_position,
                            position,
                            Piece {
                                piece_type: PieceType::Queen,
                                color: board.active_color,
                            },
                            Some(val),
                        ));
                    }
                }
                None => {
                    generated_moves.push(promotion(
                        curr_piece_position,
                        position,
                        Piece {
                            piece_type: PieceType::Queen,
                            color: board.active_color,
                        },
                        None,
                    ));
                }
            },
            MoveValue::EnPassantMove => {
                generated_moves.push(en_passant(
                    curr_piece_position,
                    position,
                    board.en_passant.unwrap(),
                    Some(Piece {
                        piece_type: PieceType::Pawn,
                        color: board.active_color.opposite(),
                    }),
                ));
            }
        }
    }

    generated_moves
}
/* Generate pawn moves for a given square/color */
pub fn generate_pawn_moves_helper(
    pos: &Position,
    color: Color,
) -> (Position, Position, Position, Position, Position, Position) {
    let right: Position;
    let left: Position;
    let oneup: Position;
    let twoup: Position;
    let en_passant_left: Position;
    let en_passant_right: Position;
    /* Are we on the first pawn move? are we on a promotion? */
    /*
    let first_move: bool;
    let is_promotion: bool;
    */

    /* Going forwards or backwards depending on piece color */
    match color {
        Color::White => {
            right = Position {
                x: pos.x - 1,
                y: pos.y + 1,
            };
            left = Position {
                x: pos.x - 1,
                y: pos.y - 1,
            };
            oneup = Position {
                x: pos.x - 1,
                y: pos.y,
            };
            twoup = Position {
                x: pos.x - 2,
                y: pos.y,
            };
            en_passant_left = Position {
                x: pos.x - 1,
                y: pos.y + 1,
            };
            en_passant_right = Position {
                x: pos.x - 1,
                y: pos.y - 1,
            };
            //Figure these out
            //first_move = pos.x == 7;
            //is_promotion = pos.x == 2;
        }
        Color::Black => {
            right = Position {
                x: pos.x - 1,
                y: pos.y + 1,
            };
            left = Position {
                x: pos.x - 1,
                y: pos.y - 1,
            };
            oneup = Position {
                x: pos.x + 1,
                y: pos.y,
            };
            twoup = Position {
                x: pos.x + 2,
                y: pos.y,
            };
            en_passant_left = Position {
                x: pos.x + 1,
                y: pos.y + 1,
            };
            en_passant_right = Position {
                x: pos.x + 1,
                y: pos.y - 1,
            };
            //Figure these out
            //first_move = pos.x == 2;
            //is_promotion = pos.x - 1 == 7;
        }
    }

    (right, left, oneup, twoup, en_passant_left, en_passant_right)
}

/* Generates two bools to record whether a pawn move is the first move by that pawn or if it is a promotion move */
pub fn generate_pawn_permissions(pos: &Position, color: Color) -> (bool, bool) {
    let first_move: bool;
    let is_promotion: bool;

    match color {
        Color::White => {
            first_move = pos.x == 7;
            is_promotion = pos.x == 2;
        }
        Color::Black => {
            first_move = pos.x == 2;
            is_promotion = pos.x == 7; //POSSIBLE BUG SOURCE
        }
    }

    (first_move, is_promotion)
}

// Generates possible king_moves given a square
pub fn generate_king_moves_helper(pos: &Position) -> Vec<Position> {
    vec![
        Position {
            x: pos.x - 1,
            y: pos.y - 1,
        },
        Position {
            x: pos.x - 1,
            y: pos.y,
        },
        Position {
            x: pos.x - 1,
            y: pos.y + 1,
        },
        Position {
            x: pos.x,
            y: pos.y - 1,
        },
        Position {
            x: pos.x,
            y: pos.y + 1,
        },
        Position {
            x: pos.x + 1,
            y: pos.y - 1,
        },
        Position {
            x: pos.x + 1,
            y: pos.y,
        },
        Position {
            x: pos.x + 1,
            y: pos.y + 1,
        },
    ]
    .into_iter()
    .filter(Position::is_valid_position)
    .collect()
}

// Given a position, this function generates a set of positions for that knight
pub fn generate_knight_moves_helper(pos: &Position) -> Vec<Position> {
    let mut possible_knight_positions: Vec<Position> = Vec::new();
    for u in 0..3 {
        for v in 0..3 {
            if u != v && u != 0 && v != 0 {
                possible_knight_positions.push(Position {
                    x: pos.x + u,
                    y: pos.y + v,
                });
                if pos.x >= u && pos.y >= v {
                    possible_knight_positions.push(Position {
                        x: pos.x - u,
                        y: pos.y - v,
                    });
                    possible_knight_positions.push(Position {
                        x: pos.x - u,
                        y: pos.y + v,
                    });
                    possible_knight_positions.push(Position {
                        x: pos.x + u,
                        y: pos.y - v,
                    });
                } else if pos.x >= u && pos.y < v {
                    possible_knight_positions.push(Position {
                        x: pos.x - u,
                        y: pos.y + v,
                    });
                } else if pos.x < u && pos.y >= v {
                    possible_knight_positions.push(Position {
                        x: pos.x + u,
                        y: pos.y - v,
                    });
                }
            }
        }
    }

    possible_knight_positions
        .into_iter()
        .filter(Position::is_valid_position)
        .collect()
}

/* Given a position on the board and a color, this function generates a set of squares
a rook of that color placed on that position could move to */
pub fn generate_rook_moves_helper(
    board: &BoardState,
    pos: &Position,
    color: Color,
) -> Vec<Position> {
    let mut rook_positions: Vec<Position> = Vec::new();
    let mut curr_pos: Position;

    // Looking horizontally
    for index in 1..8 {
        curr_pos = Position {
            x: pos.x,
            y: pos.y + index,
        };
        if !curr_pos.is_valid_position() {
            break;
        }

        let square = board.squares[curr_pos.x][curr_pos.y];

        if !square.is_occupied() {
            rook_positions.push(curr_pos);
        }

        if let Some(piece) = square.piece {
            if piece.color == color.opposite() {
                rook_positions.push(curr_pos);
            }
        }
    }

    for index in 1..8 {
        curr_pos = Position {
            x: pos.x,
            y: pos.y - index,
        };
        if !curr_pos.is_valid_position() {
            break;
        }

        let square = board.squares[curr_pos.x][curr_pos.y];

        if !square.is_occupied() {
            rook_positions.push(curr_pos);
        }

        if let Some(piece) = square.piece {
            if piece.color == color.opposite() {
                rook_positions.push(curr_pos);
            }
        }
    }

    // Looking vertically
    for index in 1..8 {
        curr_pos = Position {
            x: pos.x + index,
            y: pos.y,
        };
        if !curr_pos.is_valid_position() {
            break;
        }

        let square = board.squares[curr_pos.x][curr_pos.y];

        if !square.is_occupied() {
            rook_positions.push(curr_pos);
        }

        if let Some(piece) = square.piece {
            if piece.color == color.opposite() {
                rook_positions.push(curr_pos);
            }
        }
    }
    for index in 1..8 {
        curr_pos = Position {
            x: pos.x - index,
            y: pos.y,
        };
        if !curr_pos.is_valid_position() {
            break;
        }

        let square = board.squares[curr_pos.x][curr_pos.y];

        if !square.is_occupied() {
            rook_positions.push(curr_pos);
        }

        if let Some(piece) = square.piece {
            if piece.color == color.opposite() {
                rook_positions.push(curr_pos);
            }
        }
    }

    rook_positions
}

// Given a position on the board and a color, this fucntion generates a set of squares
// a bishop of that color placed on that position could move to
pub fn generate_bishop_moves_helper(
    board: &BoardState,
    pos: &Position,
    color: Color,
) -> Vec<Position> {
    let mut bishop_positions: Vec<Position> = Vec::new();
    let mut curr_pos: Position;
    /* each for loop corresponds to a diagonal */
    for index in 1..8 {
        curr_pos = Position {
            x: pos.x + index,
            y: pos.y + index,
        };
        if !curr_pos.is_valid_position() {
            break;
        }

        let square = board.squares[curr_pos.x][curr_pos.y];

        if !square.is_occupied() {
            bishop_positions.push(curr_pos);
        }

        if let Some(piece) = square.piece {
            if piece.color == color.opposite() {
                bishop_positions.push(curr_pos);
            }
        }
    }

    for index in 1..8 {
        curr_pos = Position {
            x: pos.x + index,
            y: pos.y - index,
        };
        if !curr_pos.is_valid_position() {
            break;
        }
        let square = board.squares[curr_pos.x][curr_pos.y];

        if !square.is_occupied() {
            bishop_positions.push(curr_pos);
        }

        if let Some(piece) = square.piece {
            if piece.color == color.opposite() {
                bishop_positions.push(curr_pos);
            }
        }
    }

    for index in 1..8 {
        curr_pos = Position {
            x: pos.x - index,
            y: pos.y + index,
        };
        if !curr_pos.is_valid_position() {
            break;
        }

        let square = board.squares[curr_pos.x][curr_pos.y];

        if !square.is_occupied() {
            bishop_positions.push(curr_pos);
        }

        if let Some(piece) = square.piece {
            if piece.color == color.opposite() {
                bishop_positions.push(curr_pos);
            }
        }
    }

    for index in 1..8 {
        curr_pos = Position {
            x: pos.x - index,
            y: pos.y - index,
        };
        if !curr_pos.is_valid_position() {
            break;
        }

        let square = board.squares[curr_pos.x][curr_pos.y];

        if !square.is_occupied() {
            bishop_positions.push(curr_pos);
        }

        if let Some(piece) = square.piece {
            if piece.color == color.opposite() {
                bishop_positions.push(curr_pos);
            }
        }
    }

    bishop_positions
}

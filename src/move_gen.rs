use crate::piece::*;
use crate::color::*;
use crate::chess_move::*;
use crate::board_state::*;
use std::collections::HashSet;
use std::process::exit;

//Generates all valid moves for a color from a given board
pub fn gen_all_moves(board: &BoardState, color: Color) -> Vec<Move> {
    /* Storing the positions of the white and black pieces */
    let (curr_set, other_set, king_pos) = find_pieces(board, color);

    if curr_set.len() + other_set.len() == 2 {
        return Vec::new(); //Draw 
    }
    
    let mut move_set: Vec<Move> = Vec::new();
    let mut curr_piece: Piece;

    //Iterating over every piece and generating moves 
    for pos in curr_set {
        curr_piece = board.squares[pos.row][pos.col].piece.unwrap(); 
        
        match curr_piece.piece_type {
            PieceType::Pawn => {
                move_set.extend(generate_pawn_moves(board, curr_piece, &pos));
            },
            PieceType::King => {
                let king_moves = filter_moves(king_positions(pos), &board, &curr_piece, pos);

                //Castle moves
                move_set.extend(generate_castle_moves(&king_pos, &board.castle_rights, curr_piece.color, &board));
                move_set.extend(king_moves);
            },
            PieceType::Knight => {
                let knight_moves = filter_moves(knight_positions(pos), &board, &curr_piece, pos);

                move_set.extend(knight_moves);
            },

            PieceType::Rook => {
                    let mut rook_positions: Vec<Move> = Vec::new();
                    /* Looking horizontally */
                    for dir in vec![Direction::Right, Direction::Left, Direction::Up, Direction::Down] {
                        rook_positions.extend(move_in_direction(pos, &dir, curr_piece.clone(), board));
                    }

                    move_set.extend(rook_positions);
            },
            PieceType::Bishop => {
                let mut bishop_positions: Vec<Move> = Vec::new();
                /*Looking diagonally */
                for dir in vec![Direction::UpRight, Direction::UpLeft, Direction::DownRight, Direction::DownLeft] {
                    bishop_positions.extend(move_in_direction(pos, &dir, curr_piece.clone(), board));
                }

                move_set.extend(bishop_positions);
            },
            PieceType::Queen => {
                let mut queen_positions: Vec<Move> = Vec::new();
                /*Looking horizontally and diagonally */
                for dir in vec![Direction::Right, Direction::Left, Direction::Up, Direction::Down,
                    Direction::UpRight, Direction::UpLeft, Direction::DownRight, Direction::DownLeft] {
                    queen_positions.extend(move_in_direction(pos, &dir, curr_piece.clone(), board));
                }

                move_set.extend(queen_positions);
            },
            PieceType::None => {},
        }
    }
    
    //Every move is valid if it doesn't leave your king in check after
    let mut legal_moves: Vec<Move> = Vec::new();
    for mv in move_set {
        let mut board_copy: BoardState = board.clone();
        board_copy.make_move(&mv); 
        if !board_copy.is_in_check(color, None) {
            legal_moves.push(mv);
        }
    }

    legal_moves
}

fn filter_moves(positions: Vec<Position>, board: &BoardState, curr_piece: &Piece, pos: Position) -> Vec<Move> {
    positions
    .into_iter()
    .filter(|val| -> bool {
        val.is_valid_position() && ( //Must be a valid square
            //Square isn't occupied
            !board.squares[val.row][val.col].is_occupied() || 
            //Square is occupied, but we can capture it
            board.squares[val.row][val.col].piece.unwrap().color == board.active_color.opposite() 
        )
    })
    .map(|val| -> Move {
            if let Some(piece_captured) = board.squares[val.row][val.col].piece {
                standard(pos, val, curr_piece.clone(), Some(piece_captured)) //Capture
            } else {
                standard(pos, val, curr_piece.clone(), None) //No capture
            }
        }
    )
    .collect()
}
pub fn find_pieces(board: &BoardState, color: Color) -> (HashSet<Position>, HashSet<Position>, Position){
    /* Storing the positions of the white and black pieces */
    let mut curr_pieces: HashSet<Position> = HashSet::new();
    let mut other_pieces: HashSet<Position> = HashSet::new();
    let mut opt_king_pos: Option<Position> = None;
    
    for x in 2..=10 {
        for y in 2..=10 {
            if let Some(curr_piece) = board.squares[x][y].piece {
                if curr_piece.color == color {
                    curr_pieces.insert(Position{row: x, col: y});
                    if curr_piece.piece_type == PieceType::King {
                        opt_king_pos = Some(Position{row: x, col: y});
                    }
                } else {
                    other_pieces.insert(Position{row: x, col: y});
                }
            }
        }
    }

    let king_pos: Position = opt_king_pos.expect("Could not find the king!");

    (curr_pieces, other_pieces, king_pos)

}

pub fn generate_castle_moves(king_pos: &Position, castle_rights: &CastleRights, color: Color, board: &BoardState) -> Vec<Move> {
    let mut castle_moves: Vec<Move> = Vec::new();
    let mut king_side: bool = true;
    let mut queen_side: bool = true;
    let mut king_side_squares = Vec::new();
    let mut queen_side_squares = Vec::new();

    if board.is_in_check(color, Some(*king_pos)) {
        return vec![];
    }

    if castle_rights.can_castle_white_kingside || castle_rights.can_castle_black_kingside { 
        king_side_squares = vec![
            king_pos.right(),
            king_pos.right().right(),
        ];
    }

    if castle_rights.can_castle_white_queenside || castle_rights.can_castle_black_queenside { 
        queen_side_squares = vec![
            king_pos.left(),
            king_pos.left().left(),
            king_pos.left().left().left(),
        ];
    }
    //Problem is that we can't castle in
    match color.clone() {
        Color::White => {
            if castle_rights.can_castle_white_kingside {
                for pos in king_side_squares {
                    let square = board.squares[pos.row][pos.col];
                    if square.is_occupied() || board.is_in_check(Color::White, Some(pos)) {
                        king_side = false;
                    }
                }
                if king_side {
                    castle_moves.push(castle(true, color));
                }
            }
            //No castle into check
            if castle_rights.can_castle_white_queenside {
                for pos in queen_side_squares {
                    let square = board.squares[pos.row][pos.col];
                    if square.is_occupied() {
                        queen_side = false;
                    }
                    if pos != king_pos.left().left().left() {
                        if board.is_in_check(Color::White, Some(pos)) {
                            queen_side = false;
                        }
                    }
                }

                if queen_side {
                    castle_moves.push(castle(false, color));
                }
            }
        },
        Color::Black => {
            if castle_rights.can_castle_black_kingside {
                for pos in king_side_squares {
                    let square = board.squares[pos.row][pos.col];
                    if square.is_occupied() || board.is_in_check(Color::Black, Some(pos)) {
                        king_side = false;
                    }
                }
                if king_side {
                    castle_moves.push(castle(true, color));
                }
            }
            if castle_rights.can_castle_black_queenside {
                for pos in queen_side_squares {
                    let square = board.squares[pos.row][pos.col];
                    if square.is_occupied() {
                        queen_side = false;
                    }

                    if pos != king_pos.left().left().left() {
                        if board.is_in_check(Color::Black, Some(pos)) {
                            queen_side = false;
                        }
                    }
                }

                if queen_side {
                    castle_moves.push(castle(false, color));
                }
            }
        }
    }

    castle_moves
}

pub fn generate_pawn_moves(board: &BoardState, curr_pawn: Piece, pos: &Position) -> Vec<Move> {
    let mut pawn_moves: Vec<Move> = Vec::new();
    let (first_move, is_promotion) = generate_pawn_permissions(pos, &curr_pawn.color);
    let (right_up, left_up, one_up, two_up, en_passant_left, en_passant_right) = generate_pawn_moves_helper(pos, &curr_pawn.color);
    
    //Generating all non promotion moves
    if !is_promotion {
        //moving one up
        if !board.squares[one_up.row][one_up.col].is_occupied() {
            pawn_moves.push(standard(*pos, one_up, curr_pawn.clone(), None));
        }
        //moving two up
        if first_move && !board.squares[two_up.row][two_up.col].is_occupied() && !board.squares[one_up.row][one_up.col].is_occupied() {
            pawn_moves.push(standard(*pos, two_up, curr_pawn.clone(), None));
        }
        //Captures
        for potential_capture in vec![right_up, left_up] {
            if let Some(capture) = board.squares[potential_capture.row][potential_capture.col].piece {
                if capture.color == curr_pawn.color.opposite() {
                    pawn_moves.push(standard(*pos, potential_capture, curr_pawn.clone(), Some(capture)));
                }
            }
        }

        //Capturing en passant moves
        if let Some(en_passant_pos) = board.en_passant {
            let captured = Some(Piece{piece_type: PieceType::Pawn, color: curr_pawn.color.opposite()});
            if en_passant_pos == pos.left() {
                pawn_moves.push(en_passant(*pos, en_passant_left, en_passant_pos, captured));;
            } else if en_passant_pos == pos.right() {
                pawn_moves.push(en_passant(*pos, en_passant_right, en_passant_pos, captured));
            }
        }

    } else {
        for promote_to in vec![PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
            let new_piece = Piece{piece_type: promote_to, color: curr_pawn.color};
            if !board.squares[one_up.row][one_up.col].is_occupied() {
                pawn_moves.push(promotion(*pos, one_up, new_piece, None));
            }

            for potential_capture in vec![right_up, left_up] {
                if let Some(capture) = board.squares[potential_capture.row][potential_capture.col].piece {
                    if capture.color == curr_pawn.color.opposite() {
                        pawn_moves.push(promotion(*pos, potential_capture, new_piece, Some(capture)));
                    }
                }
            }
        }
    }

    pawn_moves
}

/* Generate pawn move positions for a given square/color */
pub fn generate_pawn_moves_helper(pos: &Position, color: &Color) -> 
(
    Position,
    Position,
    Position,
    Position,
    Position,
    Position
) {
    let right_up: Position;
    let left_up: Position;
    let oneup: Position;
    let twoup: Position;
    let en_passant_left: Position;
    let en_passant_right: Position; 

    /* Going forwards or backwards depending on piece color */ 
    match color {
        Color::White => {
            right_up = pos.up().right();
            left_up = pos.up().left();
            oneup = pos.up();
            twoup = pos.up().up();
            en_passant_left = pos.up().left();
            en_passant_right = pos.up().right();
        },
        Color::Black => {
            right_up = pos.down().right();
            left_up = pos.down().left();
            oneup = pos.down();
            twoup = pos.down().down();
            en_passant_left = pos.down().left();
            en_passant_right = pos.down().right();
        }
    }

    (right_up, left_up, oneup, twoup, en_passant_left, en_passant_right)
}

/* Generates two bools to record whether a pawn move is the first move by that pawn or if it is a promotion move */
pub fn generate_pawn_permissions(pos: &Position, color: &Color) -> (bool, bool) {
    let first_move: bool;
    let is_promotion: bool;

    match color {
        Color::White => {
            first_move = pos.row == 8;
            is_promotion = pos.row == 3;
        }
        Color::Black => {
            first_move = pos.row == 3;
            is_promotion = pos.row == 8; 
        }
    }

    (first_move, is_promotion)
}

/* Moves continually in a specified direction and stops when it either reaches the end of the board or hits another piece */
pub fn move_in_direction(pos: Position, dir: &Direction, piece: Piece, board: &BoardState) -> Vec<Move> {
    assert!(piece.piece_type == PieceType::Rook ||
            piece.piece_type == PieceType::Bishop ||
            piece.piece_type == PieceType::Queen
    );

    let mut valid_moves: Vec<Move> = Vec::new();
    let mut next_pos = pos.next_position(dir);

    while !board.squares[next_pos.row][next_pos.col].is_occupied() && next_pos.is_valid_position() {
        valid_moves.push(standard(pos, next_pos, piece, None));
        next_pos = next_pos.next_position(dir);
    }

    if let Some(piece_captured) = board.squares[next_pos.row][next_pos.col].piece {
        if piece_captured.color == piece.color.opposite() {
            valid_moves.push(standard(pos, next_pos, piece, Some(piece_captured)));
        }
    }

    valid_moves
}

//Generates all positions a knight could move to from a given position
pub fn knight_positions(pos: Position) -> Vec<Position> {
    vec![
        pos.up().up().right(),
        pos.up().up().left(),
        pos.down().down().right(),
        pos.down().down().left(),
        pos.left().left().up(),
        pos.left().left().down(),
        pos.right().right().up(),
        pos.right().right().down(),
    ]
}

//Generates all positions a king could move to from a given position
pub fn king_positions(pos: Position) -> Vec<Position> {
    vec![
        pos.up().left(),
        pos.up(),
        pos.up().right(),
        pos.left(),
        pos.right(),
        pos.down().left(),
        pos.down(),
        pos.down().right()
    ]
}
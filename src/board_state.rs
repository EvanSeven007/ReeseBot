use crate::square::*;
use crate::piece::*;
use crate::color::*;
use crate::chess_move::*;
use std::collections::HashSet;
use std::process::exit;

#[derive(Clone, Copy)]
pub struct CastleRights {
    pub can_castle_white_kingside: bool,
    pub can_castle_white_queenside: bool,
    pub can_castle_black_kingside: bool, 
    pub can_castle_black_queenside: bool,
}
/* A board is a 8x8 array of squares */
#[derive(Clone, Copy)]
pub struct BoardState {
    pub squares: [[Square; 10]; 10],
    pub active_color: Color, 
    pub castle_rights: CastleRights,
    pub en_passant: Option<Position>,
}

impl BoardState { 
    /* Creates a board state from a FEN string */
    pub fn new(fen: &str) -> Result<BoardState, &str> {
        //Creating an 8x8 array of uninitialized arrays
        let mut squares = [[Square {piece: None, color: (Color::White) }; 10]; 10]; //Setting to white and then updating later
        //Assigning colors, but not charged
        for index in 1..9  {
            for inner_index in 1..9 {
                squares[index][inner_index].color = BoardState::get_color(&index, &inner_index);
            }
        }

        let mut fen = fen.to_string();

        let fen_strings: Vec<&str> = fen.split(' ').collect();
        if fen_strings.len() != 6 {
            return Err("Invalid fen string!");
        }

        let position_str: Vec<&str> = fen_strings[0].split('/').collect();
        let mut col: usize;
        let mut row_string: &str; //String that stores the current row info
        
        for row in 0..8 {
            row_string = position_str[row];
            col = 1;
            for fen_entry in row_string.chars() {
                if fen_entry.is_digit(10) {
                    col += fen_entry.to_digit(10).unwrap() as usize;
                } else {
                    squares[row + 1][col].piece = BoardState::parse_fen_entry(&fen_entry).unwrap();
                    col += 1;
                }
            }
        }

        let active_color = match fen_strings[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("fen string color malformed!"),
        };

        let mut can_castle_white_kingside = false;
        let mut can_castle_white_queenside = false;
        let mut can_castle_black_kingside = false;
        let mut can_castle_black_queenside = false;
        /* This is unforgiveably stupid */
        for fen_entry in fen_strings[2].chars() {
            match fen_entry {
                'K' => can_castle_white_kingside = true,
                'Q' => can_castle_white_queenside = true,
                'k' => can_castle_black_kingside = true,
                'q' => can_castle_black_queenside = true,
                '-' => {},
                _ => panic!("fen string castling info is malformed!"),
            }
        }

        let castle_rights: CastleRights = CastleRights {
            can_castle_white_kingside,
            can_castle_white_queenside,
            can_castle_black_kingside,
            can_castle_black_queenside
        };

        //Variables for enpassant goodness
        let en_passant: Option<Position>;
        let x: usize;
        let mut y: usize;
        if fen_strings[3].len() == 1 && fen_strings[3] == "-" {
            en_passant = None;
        } else if fen_strings[3].len() == 2 { 
            /* Parse enpassant string */
            let en_passant_string: Vec<char> = fen_strings[3].chars().collect();
            match en_passant_string[0] {
                'a' => x = 1,
                'b' => x = 2,
                'c' => x = 3,
                'd' => x = 4,
                'e' => x = 5,
                'f' => x = 6,
                'g' => x = 7,
                'h' => x = 8,
                _ => panic!("fen string enpassant malformed!"),
            };
            y = (9 - en_passant_string[1].to_digit(10).unwrap_or_else(|| panic!("fen string enpassant malformed!"))) as usize;
            /* This is just atrocious Evan, fix this with an unwrap or else*/
            /*
            if !en_passant_string[1].is_digit(10) {
                return Err("fen string enpassant malformed!")
            }
            //Accounting for how we represent the board state in our own coordinates
            y = (9 - en_passant_string[1].to_digit(10).unwrap()) as usize; 
            */
            if y < 1 || y > 8 {
                return Err("fen string enpassant malformed!")
            }
            en_passant = Some(Position{ x , y }.swap()); //Accounting for how we index array
        
        } else {
            return Err("fen string enpassant malformed!")
        }

        Ok(BoardState { squares, active_color, castle_rights, en_passant})
    }


    fn parse_fen_entry(entry: &char) -> Result<Option<Piece>, &str> {
        match entry {
            'r' => Ok(Some(Piece {piece_type: PieceType::Rook, color: Color::Black})),
            'n' => Ok(Some(Piece {piece_type: PieceType::Knight, color: Color::Black})),
            'b' => Ok(Some(Piece {piece_type: PieceType::Bishop, color: Color::Black})),
            'q' => Ok(Some(Piece {piece_type: PieceType::Queen, color: Color::Black})),
            'k' => Ok(Some(Piece {piece_type: PieceType::King, color: Color::Black})),
            'p' => Ok(Some(Piece {piece_type: PieceType::Pawn, color: Color::Black})),
            'R' => Ok(Some(Piece {piece_type: PieceType::Rook, color: Color::White})),
            'N' => Ok(Some(Piece {piece_type: PieceType::Knight, color: Color::White})),
            'B' => Ok(Some(Piece {piece_type: PieceType::Bishop, color: Color::White})),
            'Q' => Ok(Some(Piece {piece_type: PieceType::Queen, color: Color::White})),
            'K' => Ok(Some(Piece {piece_type: PieceType::King, color: Color::White})),
            'P' => Ok(Some(Piece {piece_type: PieceType::Pawn, color: Color::White})),
            _ => Err("Not a valid fen piece!"),
        }
    }
    

    /* Gets the color of a board from its coordinates */
    fn get_color(val1: &usize, val2: &usize) -> Color {
        if val1 < &0 || val1 > &8 || val2 < &0 || val2 > &8 {
            panic!("Not a valid coordinate {} {}", val1, val2);
        }

        let val1_is_odd: bool = val1 % 2 == 0;
        let val2_is_odd: bool = val2 % 2 == 0;

        if (val1_is_odd && val2_is_odd) || (!val1_is_odd && !val2_is_odd) {
            return Color::White;
        } else {
            return Color::Black;
        }
    }

    /* Updates a board state given a move, which was already been prechecked to be valid */
    pub fn make_move(&mut self, current_move: &Move) {
        let move_type: &MoveType = &current_move.move_type;
        self.en_passant = None; //Reseting en_passant square to None after every move, this will be updated later depending on move
        match move_type {
            MoveType::standard(val) => {
                self.squares[val.before.x][val.before.y].piece = None;
                self.squares[val.after.x][val.after.y].piece = Some(val.piece_moved);
            },

            MoveType::castle(val) => {
                let y_positions: Vec<usize>;
                if val.is_kingside  {
                    y_positions = vec![5,8,7,6];
                }
                else {
                    y_positions = vec![5,1,3,4];
                }

                self.squares[8][y_positions[0]].piece = None;
                self.squares[8][y_positions[1]].piece = None;
                self.squares[8][y_positions[2]].piece = Some(Piece {piece_type: PieceType::King, color: self.active_color });
                self.squares[8][y_positions[3]].piece = Some(Piece {piece_type: PieceType::Rook, color: self.active_color });

            },
            MoveType::promotion(val) => {
                self.squares[val.before.x][val.before.y].piece = None;
                self.squares[val.after.x][val.after.y].piece = Some(val.promote_to);
            }
            MoveType::enPassant(val) => {
                self.squares[val.before.x][val.before.y].piece = None;
                self.squares[val.after.x][val.after.y].piece = Some(Piece{piece_type: PieceType::Pawn, color: self.active_color});
                self.squares[val.en_passant.x][val.en_passant.y].piece = None;
            }
        }

        //Changing color
        match self.active_color {
            Color::Black => self.active_color = Color::White,
            Color::White => self.active_color = Color::Black, 
        };
    }
 
    /*
    pub fn undo_move(&mut self, current_move: &Move) {
        let move_type: &MoveType = &current_move.move_type;
        //self.en_passant = None

        match move_type {
            MoveType::standard(val) => {
                //Two cases, if en passant or not 
                //If en passant, put the pawns back into place
                //otherwise, just put things back where they were
                
                /* non en passant case */
            },
            MoveType::castle(val) => {

            },
            MoveType::promotion(val) => {

            }
        }
    }
    */
    
    /* Checks if the king is in check given a certain position 
        Returns True if the king of active color is in check, false otherwise
    */
    pub fn is_in_check(self: BoardState) -> bool {
        /* 
        * Looking for check on the diagonals 
        */
        //Checking pawn moves
        let board = self;
        let mut king_pos_opt: Option<Position> = None;
        let king_pos: Position;
        for i in 1..9 {
            for j in 1..9 {
                if board.squares[i][j].is_occupied() {
                    let p = board.squares[i][j].piece.unwrap();
                    if p.piece_type == PieceType::King && p.color == board.active_color {
                        king_pos_opt= Some(Position{x: i, y: j});
                        break;
                    }
                }
            }
        }

        king_pos = king_pos_opt.expect("Could not find the king!");

        let pawn_square_right: Position;
        let pawn_square_left: Position;
        match board.active_color {
            Color::White => {
                pawn_square_right = Position{x: king_pos.x - 1 , y: king_pos.y - 1};
                pawn_square_left = Position{x: king_pos.x - 1 , y: king_pos.y + 1};
            },
            Color::Black => {
                pawn_square_right = Position{x: king_pos.x + 1 , y: king_pos.y - 1};
                pawn_square_left = Position{x: king_pos.x + 1 , y: king_pos.y + 1};
            },
        }
        
        /* Checking for checks by pawns */
        if pawn_square_right.is_valid_position() {
            if board.squares[pawn_square_right.x][pawn_square_right.y].is_occupied() {
                let p = board.squares[pawn_square_right.x][pawn_square_right.y].piece.unwrap();
                if p.piece_type == PieceType::Pawn && p.color != board.active_color {
                    println!("In check by pawn");
                    return true;
                }
            }
        }

        if pawn_square_left.is_valid_position() {
            if board.squares[pawn_square_left.x][pawn_square_left.y].is_occupied() {
                let p = board.squares[pawn_square_left.x][pawn_square_left.y].piece.unwrap();
                if p.piece_type == PieceType::Pawn && p.color != board.active_color {
                    println!("in check by pawn");
                    return true;
                }
            }
        }

        /* Checking rank checks */
        let mut candidates = board.generate_rook_moves_helper(&king_pos, board.active_color);
        for mv in candidates {
            if board.squares[mv.x][mv.y].is_occupied() {
                let p = board.squares[mv.x][mv.y].piece.unwrap();
                if p.color != board.active_color {
                    match p.piece_type {
                        PieceType::Rook => {
                            println!("In check by rook");
                            return true
                        },
                        PieceType::Queen => {
                            println!("In check by queen");
                            return true
                        },
                        _ => {},
                    }
                }
            }
        }

        /* Checking check by diagonols */
        candidates = board.generate_bishop_moves_helper(&king_pos, board.active_color);
        for mv in candidates {
            if board.squares[mv.x][mv.y].is_occupied() {
                let p = board.squares[mv.x][mv.y].piece.unwrap();
                if p.color != board.active_color {
                    match p.piece_type {
                        PieceType::Bishop => {
                            println!("In check by bishop");
                            return true
                        },
                        PieceType::Queen => {
                            println!("In check by queen");
                            return true
                        },
                        _ => {},
                    }
                }
            }
        }

        /* Looking for checks by knight */
        candidates =  board.generate_knight_moves_helper(&king_pos);
        for mv in candidates {
            if !mv.is_valid_position() {
                continue;
            }
            if board.squares[mv.x][mv.y].is_occupied() {
                let p = board.squares[mv.x][mv.y].piece.unwrap();
                if p.piece_type == PieceType::Knight && p.color != board.active_color {
                    println!("In check by knight");
                    return true;
                }
            }
        }

        /* Looking for checks by king */
        candidates = self.generate_king_moves_helper(&king_pos);

        for mv in candidates {
            if !mv.is_valid_position() {
                continue;
            }
            if board.squares[mv.x][mv.y].is_occupied() {
                let p = board.squares[mv.x][mv.y].piece.unwrap();
                if p.piece_type == PieceType::King && p.color != board.active_color {
                    println!("In check by King");
                    return true;
                }
            }
        }


        return false;
    }

    pub fn gen_all_moves(self) -> Vec<Move> {
        /* Storing the positions of the white and black pieces */
        let mut white_pieces_pos: HashSet<Position> = HashSet::new();
        let mut black_pieces_pos: HashSet<Position> = HashSet::new();
        let mut opt_king_pos: Option<Position> = None;
        
        //TODO Make this a function
        for x in 1..9 {
            for y in 1..9 {
                let curr_piece: Option<Piece> = self.squares[x][y].piece;
                match curr_piece {
                    Some(val) => {
                        match val.color {
                            Color::White => {white_pieces_pos.insert(Position { x, y });},
                            Color::Black => {black_pieces_pos.insert(Position { x, y });},
                        }

                        //Found the king 
                        if val.piece_type == PieceType::King && val.color == self.active_color {
                            opt_king_pos = Some(Position{ x, y });
                        }
                    },
                    None => {},
                }
            }
        }

        let king_pos: Position = opt_king_pos.expect("Could not find the king!");
        
        /* Current set is the one we are on */
        let curr_set: HashSet<Position>;
        match self.active_color {
            Color::White => {curr_set = white_pieces_pos},
            Color::Black => {curr_set = black_pieces_pos},
        }

        let mut move_set: Vec<Move> = Vec::new(); /* change this to a set later */
        
        
        let mut curr_piece: Piece;
        let mut before: Position;
        let mut after: Position; 
        for pos in &curr_set {
            curr_piece = self.squares[pos.x][pos.y].piece.unwrap(); //Guarantted to not be None
            
            match curr_piece.piece_type {
                /*
                * Pawn Moves
                */
                PieceType::Pawn => {
                    
                    let (right, left, oneup, twoup, en_passant_left, en_passant_right) = self.generate_pawn_moves_helper(&pos, &curr_piece.color);
                    let (first_move, is_promotion) = self.generate_pawn_permissions(&pos, &curr_piece.color);
                    
                    if !is_promotion {
                        /* Capturing a piece but not a promotion */

                        /* Generating moves for 3 pawn moves */
                        move_set.extend(self.generate_moves_from_piece(MoveValue::StandardMove, curr_piece.clone(), pos.clone(), vec![oneup], None, None));
                        
            
                        //Moving up two
                        if first_move && !self.squares[twoup.x][twoup.y].is_occupied() {
                            before = pos.clone();
                            after = twoup.clone();
                            move_set.push(standard(before, after, curr_piece.clone(), None));
                        }

                        //Checking enpassant
                        match self.en_passant {
                            Some(val) => {
                                let side_right = Position{x: pos.x, y: pos.y - 1};
                                let side_left = Position{x: pos.x, y: pos.y + 1};
                                if side_right == val {
                                    move_set.push(enPassant(pos.clone(), en_passant_right.clone(), 
                                    val.clone(), Some(Piece {piece_type: PieceType::Pawn, color: self.active_color.opposite()})));
                                } else if side_left == val {
                                    move_set.push(enPassant(pos.clone(), en_passant_left.clone(), 
                                    val.clone(), Some(Piece {piece_type: PieceType::Pawn, color: self.active_color.opposite()})));
                                }
                            },
                            None => {},
                        }
                    } else {
                        //Generating all promotion moves
                        move_set.extend(self.generate_moves_from_piece(MoveValue::PromotionMove, curr_piece.clone(), 
                        pos.clone(), vec![right, left, oneup], None, None));
                    }

                },
                //Make a fn for this
                PieceType::King => {
                    let possible_king_positions: Vec<Position> = self.generate_king_moves_helper(&pos);
                    move_set.extend(self.generate_moves_from_piece(MoveValue::StandardMove, curr_piece.clone(), 
                    pos.clone(), possible_king_positions, None, None));

                },
                PieceType::Knight => {
                    let possible_knight_positions: Vec<Position> = self.generate_knight_moves_helper(pos);
                    move_set.extend(self.generate_moves_from_piece(MoveValue::StandardMove, curr_piece.clone(), 
                    pos.clone(), possible_knight_positions, None, None));
                },

                PieceType::Rook => {
                        //Look vertical and horizontal until you hit a piece
                        //For four loops from 0..8, each stopping one a certain position
                        //Store all possible positions, then add moves
                        //Consider making a "generate rook moves"
                        let possible_rook_positions: Vec<Position> = self.generate_rook_moves_helper(pos, self.active_color);
                        move_set.extend(self.generate_moves_from_piece(MoveValue::StandardMove, curr_piece.clone(), 
                        pos.clone(), possible_rook_positions, None, None));
                }
                PieceType::Bishop => {
                    //Look diagonally, for four loops
                    //"Generate bishop moves"
                        let possible_bishop_positions: Vec<Position> = self.generate_bishop_moves_helper(pos, self.active_color);
                        move_set.extend(self.generate_moves_from_piece(MoveValue::StandardMove, curr_piece.clone(), 
                        pos.clone(), possible_bishop_positions, None, None));
                },
                PieceType::Queen => {
                    //Look diagonally, vertically, and horizontally
                    //copy the loops from above
                    //Vec.push(generatebishop moves, generate rook moves, )
                        let possible_queen_positions: Vec<Position> = self.generate_rook_moves_helper(pos, self.active_color);
                        move_set.extend(self.generate_moves_from_piece(MoveValue::StandardMove, curr_piece.clone(), 
                        pos.clone(), possible_queen_positions, None, None));
                },
                PieceType::None => {},
            }
        }
        
        //Find the king in every call to, fix this with a do, undo move pattern
        let mut legal_moves: Vec<Move> = Vec::new();
        for mv in move_set {
            let mut board_copy: BoardState = self;
            board_copy.make_move(&mv);
            board_copy.active_color = board_copy.active_color.opposite();
            //Why does this work?
            if !board_copy.is_in_check() {
                legal_moves.push(mv);
            }
        }

        if legal_moves.len() == 0 {
            if self.is_in_check(){
                println!("GAME OVER BY CHECKMATE: {} has defeated {}", self.active_color.opposite().color_to_string(), self.active_color.color_to_string());
            } else {
                println!("Game over by Stalemate!");
            }
            exit(1);
        }
        
        legal_moves
    }

    /* Generates moves from a given piece and related info */
    fn generate_moves_from_piece(&self, move_type: MoveValue, curr_piece: Piece, curr_piece_position: Position, possible_positions: Vec<Position>, castle_bool: Option<bool>, en_passant: Option<Position>) -> Vec<Move> {
        let mut generated_moves: Vec<Move> = Vec::new();
        let is_stoppable: bool = 
            (curr_piece.piece_type == PieceType::Rook) || 
            (curr_piece.piece_type == PieceType::Bishop) || 
            (curr_piece.piece_type == PieceType::Queen);
        
            for position in possible_positions {
                if !position.is_valid_position() {
                    continue;
                }

                match move_type {
                    MoveValue::StandardMove => {
                        match self.squares[position.x][position.y].piece {
                            Some(val) => {
                                if val.color == self.active_color.opposite() {
                                    generated_moves.push(standard(curr_piece_position.clone(), position, curr_piece.clone(), Some(val)))
                                }
                                if is_stoppable {
                                    continue; //Break?
                                }
                            },
                            None => {
                                generated_moves.push(standard(curr_piece_position.clone(), position, curr_piece.clone(), None))
                            }
                        }
                    },
                    MoveValue::CastleMove => {
                        generated_moves.push(castle(castle_bool.unwrap()));
                    },
                    MoveValue::PromotionMove => {
                        match self.squares[position.x][position.y].piece {
                            Some(val) => {
                                if val.color == self.active_color.opposite() {
                                    generated_moves.push(promotion(curr_piece_position.clone(), position.clone(), 
                                    Piece{piece_type: PieceType::Queen, color: self.active_color}, Some(val)));
                                }
                            },
                            None => {
                                generated_moves.push(promotion(curr_piece_position.clone(), position.clone(), 
                                Piece{piece_type: PieceType::Queen, color: self.active_color}, None));
                            }
                        }
                    },
                    MoveValue::EnPassantMove => {
                        generated_moves.push(enPassant(curr_piece_position.clone(), position.clone(), en_passant.unwrap(),
                        Some(Piece{piece_type :PieceType::Pawn, color: self.active_color.opposite()})));
                    }
                }
        }

        generated_moves
    }
    /* Generate pawn moves for a given square/color */
    fn generate_pawn_moves_helper(&self, pos: &Position, color: &Color) -> 
    (
        Position,Position,Position, Position,Position,Position,
    ) {
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
                right = Position {x: pos.x - 1, y: pos.y + 1};
                left = Position {x: pos.x - 1, y: pos.y - 1};
                oneup = Position {x: pos.x - 1, y: pos.y};
                twoup =  Position {x: pos.x - 2, y: pos.y};
                en_passant_left = Position {x: pos.x - 1, y: pos.y + 1};
                en_passant_right= Position {x: pos.x - 1, y: pos.y - 1};
                //Figure these out
                //first_move = pos.x == 7;
                //is_promotion = pos.x == 2;
            },
            Color::Black => {
                right = Position {x: pos.x - 1, y: pos.y + 1};
                left = Position {x: pos.x - 1, y: pos.y - 1};
                oneup = Position {x: pos.x + 1, y: pos.y};
                twoup =  Position {x: pos.x + 2, y: pos.y};
                en_passant_left = Position {x: pos.x + 1, y: pos.y + 1};
                en_passant_right= Position {x: pos.x + 1, y: pos.y - 1};
                //Figure these out
                //first_move = pos.x == 2;
                //is_promotion = pos.x - 1 == 7;
            }
        }

        (right, left, oneup, twoup, en_passant_left, en_passant_right)
    }

    /* Generates two bools to record whether a pawn move is the first move by that pawn or if it is a promotion move */
    fn generate_pawn_permissions(&self, pos: &Position, color: &Color) -> (bool, bool) {
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
    /* Generates possible king_moves given a square */
    fn generate_king_moves_helper(&self, pos: &Position) -> Vec<Position> {
        vec![
            Position{x: pos.x - 1, y: pos.y - 1},
            Position{x: pos.x - 1, y: pos.y},
            Position{x: pos.x - 1, y: pos.y + 1},
            Position{x: pos.x, y: pos.y - 1},
            Position{x: pos.x, y: pos.y + 1},
            Position{x: pos.x + 1, y: pos.y - 1},
            Position{x: pos.x + 1, y: pos.y},
            Position{x: pos.x + 1, y: pos.y + 1},
        ].into_iter()
        .filter(|pos| pos.is_valid_position())
        .collect()
    }
    /* Given a position, this function generates a set of positions for that knight */
    fn generate_knight_moves_helper(&self, pos: &Position) -> Vec<Position> {
        let mut possible_knight_positions: Vec<Position> = Vec::new();
        for u in 0..3 {
            for v in 0..3 {
                if u != v && u != 0 && v != 0 {
                    possible_knight_positions.push(Position{x: pos.x + u, y: pos.y + v});
                    if pos.x >= u && pos.y >= v {
                        possible_knight_positions.push(Position{x: pos.x - u, y:  pos.y - v});
                        possible_knight_positions.push(Position{x: pos.x - u, y: pos.y + v});
                        possible_knight_positions.push(Position{x: pos.x + u, y: pos.y - v});
                    } else if pos.x >= u && pos.y < v {
                        possible_knight_positions.push(Position{x: pos.x - u, y: pos.y + v});
                    } else if pos.x < u && pos.y >= v {
                        possible_knight_positions.push(Position{x: pos.x + u,y:  pos.y - v});
                    }
                }
            }
        }

        possible_knight_positions
        .into_iter()
        .filter(|pos| pos.is_valid_position())
        .collect()
    }

    /* Given a position on the board and a color, this function generates a set of squares
    a rook of that color placed on that position could move to */
    fn generate_rook_moves_helper(&self, pos: &Position, color: Color) -> Vec<Position> {
        let mut rook_positions: Vec<Position> = Vec::new();
        let mut curr_pos: Position; 
        /*Looking horizontally */
        for index in 1..8 {
            curr_pos = Position{x: pos.x, y: pos.y + index};
            if !curr_pos.is_valid_position() {
                break;
            }

            if self.squares[curr_pos.x][curr_pos.y].is_occupied() {
                if self.squares[curr_pos.x][curr_pos.y].piece.unwrap().color == color.opposite() {
                    rook_positions.push(curr_pos);
                }
                break;
            } else {
                rook_positions.push(curr_pos);
            }
        }

        for index in 1..8 {
            curr_pos = Position{x: pos.x, y: pos.y - index};
            if !curr_pos.is_valid_position() {
                break;
            }

            if self.squares[curr_pos.x][curr_pos.y].is_occupied() {
                if self.squares[curr_pos.x][curr_pos.y].piece.unwrap().color == color.opposite() {
                    rook_positions.push(curr_pos);
                }
                break;
            } else {
                rook_positions.push(curr_pos);
            }
        }
        /* Looking vertically */
        for index in 1..8 {
            curr_pos = Position{x: pos.x + index, y: pos.y};
            if !curr_pos.is_valid_position() {
                break;
            }

            if self.squares[curr_pos.x][curr_pos.y].is_occupied() {
                if self.squares[curr_pos.x][curr_pos.y].piece.unwrap().color == color.opposite() {
                    rook_positions.push(curr_pos);
                }
                break;
            } else {
                rook_positions.push(curr_pos);
            }
        }
        for index in 1..8 {
            curr_pos = Position{x: pos.x - index, y: pos.y};
            if !curr_pos.is_valid_position() {
                break;
            }

            if self.squares[curr_pos.x][curr_pos.y].is_occupied() {
                if self.squares[curr_pos.x][curr_pos.y].piece.unwrap().color == color.opposite() {
                    rook_positions.push(curr_pos);
                }
                break;
            } else {
                rook_positions.push(curr_pos);
            }
        }
        return rook_positions;
    }

    /* Given a position on the board and a color, this fucntion generates a set of squares
    a bishop of that color placed on that position could move to */
    fn generate_bishop_moves_helper(&self, pos: &Position, color: Color) -> Vec<Position> {
        let mut bishop_positions: Vec<Position> = Vec::new();
        let mut curr_pos: Position; 
        /* each for loop corresponds to a diagonal */
        for index in 1..8 {
            curr_pos = Position{x: pos.x + index, y: pos.y + index};
            if !curr_pos.is_valid_position() {
                break;
            }

            if self.squares[curr_pos.x][curr_pos.y].is_occupied() {
                if self.squares[curr_pos.x][curr_pos.y].piece.unwrap().color == color.opposite() {
                    bishop_positions.push(curr_pos);
                }
                break;
            } else {
                bishop_positions.push(curr_pos);
            }
        }

        for index in 1..8 {
            curr_pos = Position{x: pos.x + index, y: pos.y - index};
            if !curr_pos.is_valid_position() {
                break;
            }

            if self.squares[curr_pos.x][curr_pos.y].is_occupied() {
                if self.squares[curr_pos.x][curr_pos.y].piece.unwrap().color == color.opposite() {
                    bishop_positions.push(curr_pos);
                }
                break;
            } else {
                bishop_positions.push(curr_pos);
            }
        }

        for index in 1..8 {
            curr_pos = Position{x: pos.x - index, y: pos.y + index};
            if !curr_pos.is_valid_position() {
                break;
            }

            if self.squares[curr_pos.x][curr_pos.y].is_occupied() {
                if self.squares[curr_pos.x][curr_pos.y].piece.unwrap().color == color.opposite() {
                    bishop_positions.push(curr_pos);
                }
                break;
            } else {
                bishop_positions.push(curr_pos);
            }
        }

        for index in 1..8 {
            curr_pos = Position{x: pos.x - index, y: pos.y - index};
            if !curr_pos.is_valid_position() {
                break;
            }

            if self.squares[curr_pos.x][curr_pos.y].is_occupied() {
                if self.squares[curr_pos.x][curr_pos.y].piece.unwrap().color == color.opposite() {
                    bishop_positions.push(curr_pos);
                }
                break;
            } else {
                bishop_positions.push(curr_pos);
            }
        }

        return bishop_positions;
    }



    pub fn print_board(&self) {
        for index in 1..9 {
            print!("[{}]", index);
            for inner_index in 1..9 {
                print!("{}", self.squares[index][inner_index].symbol());
            }
            print!("\n");
        }
        println!("   [1][2][3][4][5][6][7][8]");
    }
}

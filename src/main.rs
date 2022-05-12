#[derive(Clone, Copy)]
enum Piece {
    King, 
    Queen, 
    Bishop,
    Knight,
    Rook,
    Pawn,
}
/* //TODO
#[derive(Clone, Copy)]
enum Color {
    White,
    Black,
}
*/

#[derive(Clone, Copy)]
struct Square {
    piece: Option<Piece>,
    //color: Color 
}

impl Square {
    pub fn new() -> Square {
        Square { piece: None }
    }

    fn symbol(&self) -> &str {
        match self.piece {
            Some(Piece::King)   => "[K]",
            Some(Piece::Queen)  => "[Q]",
            Some(Piece::Rook)   => "[R]",
            Some(Piece::Bishop) => "[B]",
            Some(Piece::Knight) => "[N]",
            Some(Piece::Pawn)   => "[p]",
            None                => "[ ]"
        }
    }

}

struct Board {
    squares: [[Square; 8]; 8]
}

impl Board {
    fn new() -> Board {
        let mut squares = [[Square { piece: None }; 8]; 8];

        for index in 0..8 {
            match index {
                //Populating first and back row
                0 | 7 => {
                    for innerIndex in 0..8 {
                        let piece = match innerIndex {
                            0 | 7 => Some(Piece::Rook),
                            1 | 6 => Some(Piece::Knight),
                            2 | 5 => Some(Piece::Bishop),
                            3 => Some(Piece::King),
                            4 => Some(Piece::Queen),
                            _ => {panic!("Not a valid piece type")}
                        };
                        squares[index][innerIndex] = Square{ piece };
                    }
                }, 
                //Matching pawn rows
                1 | 6 => { 
                    for innerIndex in 0..8 {
                        let piece = Some(Piece::Pawn);
                        squares[index][innerIndex] = Square { piece };
                    }
                },
                _ => {}
            }
        }

        Board { squares }
    }

    fn printBoard(&self) {
        println!("   [1][2][3][4][5][6][7][8]");
        for index in 0..8 {
            print!("[{}]", index);
            for innerIndex in 0..8 {
                print!("{}", self.squares[index][innerIndex].symbol());
            }
            print!("\n");
        }
    }
}

fn main() {
    let board = Board::new();
    board.printBoard();
}
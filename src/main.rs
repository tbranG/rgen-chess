mod pieces;

use pieces::ColorCode;
use pieces::ChessPieceCode;

fn init_boards (
    pt_board: &mut [[i8; 8]; 8],
    o_board: &mut [[i8; 8]; 8]
) {
    for i in 0..8 {
        for j in 0..8 {
            o_board[i][j] = match i {
                0 | 1 => ColorCode::White as i8,
                6 | 7 => ColorCode::Black as i8,
                _ => -1
            };

            let tmp_tuple: (i8, i8) = (i as i8 , j as i8);

            pt_board[i][j] = match tmp_tuple {
                (1 | 6, _) => ChessPieceCode::Pawn as i8,
                (0 | 7, 0 | 7) => ChessPieceCode::Rook as i8,
                (0 | 7, 1 | 6) => ChessPieceCode::Knight as i8,
                (0 | 7, 2 | 5) => ChessPieceCode::Bishop as i8,
                (0 | 7, 3) => ChessPieceCode::Queen as i8,
                (0 | 7, 4) => ChessPieceCode::King as i8,
                _ => -1
            }
        }
    }
}

fn main() {
    let mut ocupation_board: [[i8; 8]; 8] = [[0; 8]; 8];
    let mut piece_type_board: [[i8; 8]; 8] = [[0; 8]; 8];

    init_boards(&mut piece_type_board, &mut ocupation_board);

    println!("Ocupation board:");
    for line in ocupation_board {
        println!("{:?}", line);
    }

    println!("Piece type board:");
    for line in piece_type_board {
        println!("{:?}", line);
    }
}


mod pieces;

use pieces::PieceColorCode;
use pieces::ChessPieceCode;
use pieces::get_piece_weight;

/// Set piece_type board and occupation board to their initial state <br>(only called at game init)
fn init_boards(
    pt_board: &mut [[i8; 8]; 8],
    o_board: &mut [[i8; 8]; 8],
    w_board: &mut [[f32; 8]; 8]
) {
    for i in 0..8 {
        for j in 0..8 {
            o_board[i][j] = match i {
                0 | 1 => PieceColorCode::White as i8,
                6 | 7 => PieceColorCode::Black as i8,
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
            };

            w_board[i][j] = match tmp_tuple {
                (1 | 6, _) => get_piece_weight(ChessPieceCode::Pawn),
                (0 | 7, 0 | 7) => get_piece_weight(ChessPieceCode::Rook),
                (0 | 7, 1 | 6) => get_piece_weight(ChessPieceCode::Knight),
                (0 | 7, 2 | 5) => get_piece_weight(ChessPieceCode::Bishop),
                (0 | 7, 3) => get_piece_weight(ChessPieceCode::Queen),
                (0 | 7, 4) => get_piece_weight(ChessPieceCode::King),
                _ => 0f32
            }
        }
    }
}

fn update_boards(
    pt_board: &mut [[i8; 8]; 8],
    o_board: &mut [[i8; 8]; 8],
    w_board: &mut [[f32; 8]; 8],
    piece_old_coords: (i8, i8),
    piece_new_coords: (i8, i8),
    piece_type: ChessPieceCode,
    piece_color: PieceColorCode,
    piece_weight: f32
) {
    let old_x = piece_old_coords.0 as usize;
    let old_y = piece_old_coords.1 as usize;
    let new_x = piece_new_coords.0 as usize;
    let new_y = piece_new_coords.1 as usize;

    if pt_board[new_x][new_y] != -1 || o_board[new_x][new_y] != -1 {
        //TODO: handle piece removal
    }

    pt_board[old_x][old_y] = -1;
    o_board[old_x][old_y] = -1;
    w_board[old_x][old_y] = 0f32;

    pt_board[new_x][new_y] = piece_type as i8;
    o_board[new_x][new_y] = piece_color as i8;
    w_board[new_x][new_y] = piece_weight;
}

fn main() {
    let mut occupation_board: [[i8; 8]; 8] = [[0; 8]; 8];
    let mut piece_type_board: [[i8; 8]; 8] = [[0; 8]; 8];
    let mut piece_weight_board: [[f32; 8]; 8] = [[0f32; 8]; 8];

    init_boards(&mut piece_type_board, &mut occupation_board, &mut piece_weight_board);

    println!("Ocupation board:");
    for line in occupation_board {
        println!("{:?}", line);
    }

    println!("Piece type board:");
    for line in piece_type_board {
        println!("{:?}", line);
    }

    println!("Piece weight board:");
    for line in piece_weight_board {
        println!("{:?}", line);
    }
}

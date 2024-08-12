mod bot;
mod pieces;

use pieces::*;
use std::vec;

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
                (1 | 6, _) => PieceTypeCode::Pawn as i8,
                (0 | 7, 0 | 7) => PieceTypeCode::Rook as i8,
                (0 | 7, 1 | 6) => PieceTypeCode::Knight as i8,
                (0 | 7, 2 | 5) => PieceTypeCode::Bishop as i8,
                (0 | 7, 3) => PieceTypeCode::Queen as i8,
                (0 | 7, 4) => PieceTypeCode::King as i8,
                _ => -1
            };

            w_board[i][j] = match tmp_tuple {
                (1 | 6, _) => get_piece_weight(PieceTypeCode::Pawn),
                (0 | 7, 0 | 7) => get_piece_weight(PieceTypeCode::Rook),
                (0 | 7, 1 | 6) => get_piece_weight(PieceTypeCode::Knight),
                (0 | 7, 2 | 5) => get_piece_weight(PieceTypeCode::Bishop),
                (0 | 7, 3) => get_piece_weight(PieceTypeCode::Queen),
                (0 | 7, 4) => get_piece_weight(PieceTypeCode::King),
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
    piece_type: PieceTypeCode,
    piece_color: PieceColorCode,
    piece_weight: f32,
    wp_taken_list: &mut Vec<PieceRef>,
    bp_taken_list: &mut Vec<PieceRef>
) {
    let old_x = piece_old_coords.0 as usize;
    let old_y = piece_old_coords.1 as usize;
    let new_x = piece_new_coords.0 as usize;
    let new_y = piece_new_coords.1 as usize;

    if pt_board[new_x][new_y] != -1 || o_board[new_x][new_y] != -1 {
        //Get info of the piece that will be removed
        let rpiece_color: PieceColorCode = match o_board[new_x][new_y] {
            1 => PieceColorCode::White,
            2 => PieceColorCode::Black,
            _ => panic!("invalid state, there should be a piece at i={} j={}", new_x, new_y)
        };

        let rpiece_type: PieceTypeCode = match pt_board[new_x][new_y] {
            1 => PieceTypeCode::Pawn,
            2 => PieceTypeCode::Rook,
            3 => PieceTypeCode::Knight,
            4 => PieceTypeCode::Bishop,
            5 => PieceTypeCode::Queen,
            6 => PieceTypeCode::King,
            _ => panic!("invalid state, there should be a piece at i={} j={}", new_x, new_y)
        };

        //TODO: Game over state if piece type is equal 6

        let piece_ref = PieceRef { piece_type: rpiece_type, piece_color: rpiece_color };
        if rpiece_color == PieceColorCode::White {
            wp_taken_list.push(piece_ref);
        }else {
            bp_taken_list.push(piece_ref);
        }
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

    let mut wp_taken_list: Vec<PieceRef> = vec![];
    let mut bp_taken_list: Vec<PieceRef> = vec![];

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

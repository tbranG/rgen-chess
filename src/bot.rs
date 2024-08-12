use crate::pieces::{get_piece_weight, PieceColorCode, PieceCoordinates, PieceTypeCode};

pub fn build_weight_board(pt_board: &mut [[i8; 8]; 8]) -> [[f32; 8]; 8] {
    let mut board: [[f32; 8]; 8] = [[0f32; 8]; 8];

    for i in 0..8 {
        for j in 0..8 {
            let piece_type = match pt_board[i][j] {
                1 => PieceTypeCode::Pawn,
                2 => PieceTypeCode::Rook,
                3 => PieceTypeCode::Knight,
                4 => PieceTypeCode::Bishop,
                5 => PieceTypeCode::Queen,
                6 => PieceTypeCode::King,
                _ => PieceTypeCode::Nil
            };

            board[i][j] = get_piece_weight(piece_type);
        }
    };

    board
}

pub fn is_square_available(occupation_board: &[[i8; 8]; 8], coord: PieceCoordinates) -> bool{
    occupation_board[coord.i][coord.j] == -1
}

pub fn is_piece_mine(occupation_board: &[[i8; 8]; 8], coord: PieceCoordinates) -> bool{
    let val: PieceColorCode = match occupation_board[coord.i][coord.j] {
        1 => PieceColorCode::White,
        2 => PieceColorCode::Black,
        _ => PieceColorCode::Nil
    };

    val == PieceColorCode::Black
}

pub fn is_coordinate_oob(coord: PieceCoordinates) -> bool {
    coord.i < 0 || coord.i > 7 || coord.j < 0 || coord.j > 7
}

//TODO: include piece capturing to available movements
pub fn get_piece_available_movements(piece: PieceTypeCode, piece_coords: PieceCoordinates, occupation_board: &[[i8; 8]; 8]) -> Vec<PieceCoordinates>{
    let mut available_moves: Vec<PieceCoordinates> = vec![];

    match piece {
        PieceTypeCode::Pawn => {
            if is_square_available(occupation_board, PieceTypeCode::new(piece_coords.i - 1, piece_coords.j)) {
                available_moves.push(PieceCoordinates::new(piece_coords.i - 1, piece_coords.j))
            }
        },
        PieceTypeCode::Rook => {
            let mut check_left = true;
            let mut check_right = true;
            let mut check_up = true;
            let mut check_down = true;

            let j_left = piece_coords.j - 1;
            check_left = is_coordinate_oob(PieceCoordinates::new(0, j_left));

            let j_right = piece_coords.j + 1;
            check_right = is_coordinate_oob(PieceCoordinates::new(0, j_right));

            let i_up = piece_coords.i - 1;
            check_up = is_coordinate_oob(PieceCoordinates::new(i_up, 0));

            let i_down = piece_coords.i + 1;
            check_down = is_coordinate_oob(PieceCoordinates::new(i_down, 0));

            //circular search for available squares
            loop {
                if !check_left && !check_right && !check_up && !check_down {
                    break;
                }

                //TODO: finish rook logic
            }
        }
    }

    available_moves
}
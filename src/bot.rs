use crate::pieces::{get_piece_weight, PieceColorCode, PieceCoordinates, PieceTypeCode};

pub fn build_weight_board(pt_board: &[[i8; 8]; 8], w_board: &mut [[f32; 8]; 8]) {
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

            w_board[i][j] = get_piece_weight(piece_type);
        }
    };
}

fn is_square_available(occupation_board: &[[i8; 8]; 8], coord: PieceCoordinates) -> bool {
    if !is_coordinate_oob(PieceCoordinates::new(coord.i, coord.j)) {
        occupation_board[coord.i as usize][coord.j as usize] == -1
    } else {
        false
    }
}

fn is_piece_mine(occupation_board: &[[i8; 8]; 8], coord: PieceCoordinates) -> bool {
    let val: PieceColorCode = match occupation_board[coord.i as usize][coord.j as usize] {
        1 => PieceColorCode::White,
        2 => PieceColorCode::Black,
        _ => PieceColorCode::Nil
    };

    val == PieceColorCode::Black
}

fn is_coordinate_oob(coord: PieceCoordinates) -> bool {
    coord.i < 0 || coord.i > 7 || coord.j < 0 || coord.j > 7
}

fn can_capture(occupation_board: &[[i8; 8]; 8], coord: PieceCoordinates) -> bool {
    !is_coordinate_oob(coord.clone()) &&
    !is_square_available(occupation_board, coord.clone()) &&
    !is_piece_mine(occupation_board, coord.clone())
}

//TODO: include piece capturing to available movements
pub fn get_piece_available_movements(piece: PieceTypeCode, piece_coords: PieceCoordinates, occupation_board: &[[i8; 8]; 8]) -> Vec<PieceCoordinates>{
    let mut available_moves: Vec<PieceCoordinates> = vec![];

    match piece {
        PieceTypeCode::Pawn => {
            if is_square_available(occupation_board, PieceCoordinates::new(piece_coords.i + 1, piece_coords.j)) {
                available_moves.push(PieceCoordinates::new(piece_coords.i + 1, piece_coords.j))
            }

            //checking lower diagonals
            if can_capture(occupation_board, PieceCoordinates::new(piece_coords.i + 1, piece_coords.j - 1))  {
                available_moves.push(PieceCoordinates::new(piece_coords.i + 1, piece_coords.j - 1))
            }
            if can_capture(occupation_board, PieceCoordinates::new(piece_coords.i + 1, piece_coords.j + 1))  {
                available_moves.push(PieceCoordinates::new(piece_coords.i + 1, piece_coords.j + 1))
            }
        },
        PieceTypeCode::Rook => {
            let mut check_left = true;
            let mut check_right = true;
            let mut check_up = true;
            let mut check_down = true;

            let mut j_left = piece_coords.j - 1;
            let mut j_right = piece_coords.j + 1;
            let mut i_up = piece_coords.i - 1;
            let mut i_down = piece_coords.i + 1;

            //circular search for available squares
            loop {
                if !check_left && !check_right && !check_up && !check_down {
                    break;
                }

                if !is_coordinate_oob(PieceCoordinates::new(piece_coords.i, j_left)) && (
                    is_square_available(occupation_board, PieceCoordinates::new(piece_coords.i, j_left)) ||
                    can_capture(occupation_board, PieceCoordinates::new(piece_coords.i, j_left))
                    ) {

                    available_moves.push(PieceCoordinates::new(piece_coords.i, j_left));
                    j_left -= 1;
                }else {
                    check_left = false;
                }

                if !is_coordinate_oob(PieceCoordinates::new(i_up, piece_coords.j)) && (
                    is_square_available(occupation_board, PieceCoordinates::new(i_up, piece_coords.j)) ||
                    can_capture(occupation_board, PieceCoordinates::new(i_up, piece_coords.j))
                    ) {

                    available_moves.push(PieceCoordinates::new(i_up, piece_coords.j));
                    i_up -= 1;
                }else {
                    check_up = false;
                }

                if !is_coordinate_oob(PieceCoordinates::new(piece_coords.i, j_right)) && (
                    is_square_available(occupation_board, PieceCoordinates::new(piece_coords.i, j_right)) ||
                    can_capture(occupation_board, PieceCoordinates::new(piece_coords.i, j_right))
                    ) {

                    available_moves.push(PieceCoordinates::new(piece_coords.i, j_right));
                    j_right += 1;
                }else {
                    check_right = false;
                }

                if !is_coordinate_oob(PieceCoordinates::new(i_down, piece_coords.j)) && (
                    is_square_available(occupation_board, PieceCoordinates::new(i_down, piece_coords.j)) ||
                    can_capture(occupation_board, PieceCoordinates::new(i_down, piece_coords.j))
                    ) {

                    available_moves.push(PieceCoordinates::new(i_down, piece_coords.j));
                    i_down += 1;
                }else {
                    check_down = false;
                }
            }
        },
        PieceTypeCode::Knight => {
            let mut upper_i: i8 = 0;
            let mut lower_i: i8 = 0;
            let mut j: i8 = 0;

            j = piece_coords.j - 2;
            upper_i = piece_coords.i - 1;
            lower_i = piece_coords.i + 1;

            if  is_square_available(occupation_board, PieceCoordinates::new(upper_i, j)) ||
                can_capture(occupation_board, PieceCoordinates::new(upper_i, j)) {
                available_moves.push(PieceCoordinates::new(upper_i, j));
            }
            if  is_square_available(occupation_board, PieceCoordinates::new(lower_i, j)) ||
                can_capture(occupation_board, PieceCoordinates::new(lower_i, j)) {
                available_moves.push(PieceCoordinates::new(lower_i, j));
            }

            j += 1;
            upper_i -= 1;
            lower_i += 1;

            if  is_square_available(occupation_board, PieceCoordinates::new(upper_i, j)) ||
                can_capture(occupation_board, PieceCoordinates::new(upper_i, j)) {
                available_moves.push(PieceCoordinates::new(upper_i, j));
            }
            if  is_square_available(occupation_board, PieceCoordinates::new(lower_i, j)) ||
                can_capture(occupation_board, PieceCoordinates::new(lower_i, j)) {
                available_moves.push(PieceCoordinates::new(lower_i, j));
            }

            j = piece_coords.j + 1;

            if  is_square_available(occupation_board, PieceCoordinates::new(upper_i, j)) ||
                can_capture(occupation_board, PieceCoordinates::new(upper_i, j)) {
                available_moves.push(PieceCoordinates::new(upper_i, j));
            }
            if  is_square_available(occupation_board, PieceCoordinates::new(lower_i, j)) ||
                can_capture(occupation_board, PieceCoordinates::new(lower_i, j)) {
                available_moves.push(PieceCoordinates::new(lower_i, j));
            }

            j += 1;
            upper_i += 1;
            lower_i -= 1;

            if  is_square_available(occupation_board, PieceCoordinates::new(upper_i, j)) ||
                can_capture(occupation_board, PieceCoordinates::new(upper_i, j)) {
                available_moves.push(PieceCoordinates::new(upper_i, j));
            }
            if  is_square_available(occupation_board, PieceCoordinates::new(lower_i, j)) ||
                can_capture(occupation_board, PieceCoordinates::new(lower_i, j)) {
                available_moves.push(PieceCoordinates::new(lower_i, j));
            }
        }
        PieceTypeCode::Bishop => {
            let mut check_fq = true;
            let mut check_sq = true;
            let mut check_tq = true;
            let mut check_fq = true;

            let mut fq_coords: PieceCoordinates = PieceCoordinates::new(piece_coords.i - 1, piece_coords.j + 1);
            let mut sq_coords: PieceCoordinates = PieceCoordinates::new(piece_coords.i - 1, piece_coords.j - 1);
            let mut tq_coords: PieceCoordinates = PieceCoordinates::new(piece_coords.i + 1, piece_coords.j - 1);
            let mut fq_coords: PieceCoordinates = PieceCoordinates::new(piece_coords.i + 1, piece_coords.j + 1);

            loop {
                if !check_fq && !check_sq && !check_tq && !check_fq {
                    break;
                }


            }
        }
        _ => panic!("Tried to calculate a set of movements for a piece of type: Unknown")
    }

    available_moves
}
use regex::Regex;
use crate::pieces::{PieceCoordinates, PieceTypeCode};

fn preprocess_string(input: &str) -> bool {
    let reg = Regex::new(r"\[(\[([0-9],?){8}\],?){8}\]");
    reg.unwrap().is_match(input)
}

pub fn read_board(input: &String, board: &mut [[i8; 8]; 8]) {
    assert!(preprocess_string(input.as_str()));

    let mut i: i8 = 0;
    let mut j: i8 = 0;

    let mut tmp_num: String = String::new();

    for c in input.chars() {
        match c {
            '['  => j = 0,
            ']' => {
                if i >= 8 {
                    break;
                }

                if !tmp_num.is_empty() {
                    board[i as usize][j as usize] = tmp_num.parse().unwrap();
                    tmp_num.clear();
                }

                i += 1;
                j = 0;
            },
            ',' => {
                if !tmp_num.is_empty() {
                    board[i as usize][j as usize] = tmp_num.parse().unwrap();
                    tmp_num.clear();
                }

                j += 1;
            },
            _ => tmp_num.push(c)
        }
    }
}

pub fn translate_move_to_notation(piece_coordinates: PieceCoordinates, piece_type: PieceTypeCode) -> String {
    let row= char::from(((8 - piece_coordinates.i) + 48) as u8);
    let col = match piece_coordinates.j {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => panic!("Unexpected piece coordinate j")
    };

    let ptype = match piece_type {
        PieceTypeCode::Pawn => ' ',
        PieceTypeCode::Knight => 'N',
        PieceTypeCode::Bishop => 'B',
        PieceTypeCode::Queen => 'Q',
        PieceTypeCode::King => 'K',
        _ => panic!("Unexpected piece type code")
    };

    let char_vec = vec![ptype, col, row];
    let movement = char_vec.iter().collect();

    movement
}
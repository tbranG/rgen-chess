mod bot;
mod pieces;
mod parser;

use std::io::BufRead;
use crate::bot::{build_weight_board, get_piece_available_movements};
use crate::parser::read_board;
use crate::pieces::{PieceCoordinates, PieceTypeCode};

//REMEMBER: AI will always play for the black pieces
fn main() {
    let mut occupation_board: [[i8; 8]; 8] = [[0; 8]; 8];
    let mut piece_type_board: [[i8; 8]; 8] = [[0; 8]; 8];

    let mut occupation_board_input: String = String::new();
    let mut piece_type_board_input: String = String::new();

    let stdin = std::io::stdin();

    println!("Insert occupation board: ");
    stdin.lock().read_line(&mut occupation_board_input).unwrap();
    read_board(&occupation_board_input, &mut occupation_board);

    println!("Insert piece type board: ");
    stdin.lock().read_line(&mut piece_type_board_input).unwrap();
    read_board(&piece_type_board_input, &mut piece_type_board);

    let mut piece_weight_board: [[f32; 8]; 8] = [[0f32; 8]; 8];
    build_weight_board(&piece_type_board, &mut piece_weight_board);

    let first_black_pawn = PieceCoordinates::new(1, 0);

    let available_moves = get_piece_available_movements(PieceTypeCode::Pawn, first_black_pawn, &occupation_board);

    print!("Number of moves: {:?}\n", available_moves.len());
    println!("Move:");
    println!("i={:?}", available_moves.get(0).unwrap().i);
    println!("j={:?}", available_moves.get(0).unwrap().j);
}

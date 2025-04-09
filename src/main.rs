mod bot;
mod pieces;
mod parser;

use std::io::BufRead;
use crate::bot::{build_weight_board, calculate_best_movement};
use crate::parser::read_board;

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

    let selected_movement = calculate_best_movement(&occupation_board, &piece_type_board, &piece_weight_board);

    println!("Move:");
    println!("{:?}", selected_movement);
}

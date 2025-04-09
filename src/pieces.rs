#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PieceTypeCode {
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
    Nil
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum PieceColorCode {
    White = 1,
    Black = 2,
    Nil
}

pub struct PieceRef {
    pub piece_type: PieceTypeCode,
    pub piece_color: PieceColorCode
}

#[derive(Clone)]
pub struct PieceCoordinates {
    pub i: i8,
    pub j: i8
}

impl PieceCoordinates {
    pub fn new(i: i8, j: i8) -> Self {
        PieceCoordinates { i, j }
    }
}

pub fn get_piece_weight(piece: PieceTypeCode) -> f32 {
    match piece {
        PieceTypeCode::Pawn => 1.5f32,
        PieceTypeCode::Rook => 4f32,
        PieceTypeCode::Knight => 2f32,
        PieceTypeCode::Bishop => 4f32,
        PieceTypeCode::Queen => 8f32,
        PieceTypeCode::King => 10f32,
        PieceTypeCode::Nil => 0f32
    }
}
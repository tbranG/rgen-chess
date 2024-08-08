pub enum ChessPieceCode {
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6
}

pub enum PieceColorCode {
    White = 1,
    Black = 2
}

pub fn get_piece_weight(piece: ChessPieceCode) -> f32 {
    match piece {
        ChessPieceCode::Pawn => 0.5,
        ChessPieceCode::Rook => 4f32,
        ChessPieceCode::Knight => 1.5,
        ChessPieceCode::Bishop => 4f32,
        ChessPieceCode::Queen => 7f32,
        ChessPieceCode::King => 10f32
    }
}
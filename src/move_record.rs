use crate::{ChessMove, Color, Piece};

pub struct MoveRecord {
    pub chess_move: ChessMove,
    pub captured_piece: Option<Piece>,
    pub side_that_moved: Color,
}

impl MoveRecord {
    pub fn new(
        chess_move: ChessMove,
        captured_piece: Option<Piece>,
        side_that_moved: Color,
    ) -> Self {
        Self {
            chess_move,
            captured_piece,
            side_that_moved,
        }
    }
}

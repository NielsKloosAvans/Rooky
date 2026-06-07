use crate::{ChessMove, Color, Piece};

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[cfg(test)]
mod tests {
    use crate::{PieceKind, Square};

    use super::*;

    #[test]
    fn move_record_stores_chess_move() {
        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove::new(e2, e4);

        let record = MoveRecord::new(chess_move, None, Color::White);

        assert_eq!(record.chess_move, chess_move);
    }

    #[test]
    fn move_record_stores_captured_piece() {
        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove::new(e2, e4);
        let captured_piece = Piece::new(Color::Black, PieceKind::Queen);

        let record = MoveRecord::new(chess_move, Some(captured_piece), Color::White);

        assert_eq!(record.captured_piece, Some(captured_piece));
    }

    #[test]
    fn move_record_stores_side_that_moved() {
        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove::new(e2, e4);

        let record = MoveRecord::new(chess_move, None, Color::White);

        assert_eq!(record.side_that_moved, Color::White);
    }
}

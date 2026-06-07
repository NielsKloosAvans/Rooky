use crate::{ChessMove, Color, Piece, PieceKind, Square};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Board {
    // The board is stored as one flat array instead of 8 arrays of 8.
    // Each slot is either Some(piece) or None.
    squares: [Option<Piece>; 64],
}

impl Board {
    pub fn empty() -> Board {
        Board {
            squares: [None; 64],
        }
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        let index = square.index();
        self.squares[index]
    }

    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        let index = square.index();
        self.squares[index] = Some(piece);
    }

    pub fn remove_piece(&mut self, square: Square) -> Option<Piece> {
        let index = square.index();
        self.squares[index].take()
    }

    pub fn is_empty(&self, square: Square) -> bool {
        self.piece_at(square).is_none()
    }

    pub fn make_move(&mut self, chess_move: ChessMove) -> Option<Piece> {
        if let Some(piece) = self.remove_piece(chess_move.from) {
            let captured_piece = self.piece_at(chess_move.to);
            self.set_piece(chess_move.to, piece);
            captured_piece
        } else {
            None
        }
    }

    pub fn starting_position() -> Board {
        let mut board = Board::empty();

        let back_rank = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        for (file, kind) in back_rank.iter().enumerate() {
            let white_square = Square::new(file as u8, 0).unwrap();
            let white_piece = Piece::new(Color::White, *kind);

            let black_square = Square::new(file as u8, 7).unwrap();
            let black_piece = Piece::new(Color::Black, *kind);

            board.set_piece(white_square, white_piece);
            board.set_piece(black_square, black_piece);
        }

        for file in 0..8 {
            let white_square = Square::new(file, 1).unwrap();
            let white_pawn = Piece::new(Color::White, PieceKind::Pawn);

            let black_square = Square::new(file, 6).unwrap();
            let black_pawn = Piece::new(Color::Black, PieceKind::Pawn);

            board.set_piece(white_square, white_pawn);
            board.set_piece(black_square, black_pawn);
        }

        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board_has_no_piece_on_e4() {
        let board = Board::empty();

        let e4 = Square::new(4, 3).unwrap();

        assert!(board.is_empty(e4));
    }

    #[test]
    fn set_piece_stores_piece_on_e4() {
        let mut board = Board::empty();

        let e4 = Square::new(4, 3).unwrap();

        let white_queen = Piece::new(Color::White, PieceKind::Queen);

        board.set_piece(e4, white_queen);

        assert_eq!(board.piece_at(e4), Some(white_queen));
    }

    #[test]
    fn remove_piece_on_e4() {
        let mut board = Board::empty();

        let e4 = Square::new(4, 3).unwrap();

        let white_queen = Piece::new(Color::White, PieceKind::Queen);

        board.set_piece(e4, white_queen);

        assert_eq!(board.remove_piece(e4), Some(white_queen));
        assert_eq!(board.piece_at(e4), None);
    }

    #[test]
    fn remove_piece_on_empty_square() {
        let mut board = Board::empty();

        let e4 = Square::new(4, 3).unwrap();

        assert_eq!(board.remove_piece(e4), None);
    }

    #[test]
    fn starting_position_has_expected_key_squares() {
        let board = Board::starting_position();

        let expected_positions = [
            (3, 0, Some(Piece::new(Color::White, PieceKind::Queen))),
            (4, 0, Some(Piece::new(Color::White, PieceKind::King))),
            (3, 7, Some(Piece::new(Color::Black, PieceKind::Queen))),
            (4, 7, Some(Piece::new(Color::Black, PieceKind::King))),
            (4, 1, Some(Piece::new(Color::White, PieceKind::Pawn))),
            (7, 1, Some(Piece::new(Color::White, PieceKind::Pawn))),
            (4, 6, Some(Piece::new(Color::Black, PieceKind::Pawn))),
            (7, 6, Some(Piece::new(Color::Black, PieceKind::Pawn))),
            (4, 3, None),
        ];

        for (file, rank, expected_piece) in expected_positions {
            let square = Square::new(file, rank).unwrap();

            assert_eq!(board.piece_at(square), expected_piece);
        }
    }

    #[test]
    fn board_is_not_empty_after_setting_a_piece() {
        let mut board = Board::empty();

        let e4 = Square::new(4, 3).unwrap();
        let white_queen = Piece::new(Color::White, PieceKind::Queen);

        board.set_piece(e4, white_queen);

        assert!(!board.is_empty(e4))
    }

    #[test]
    fn make_move_moves_piece_from_e2_to_e4() {
        let mut board = Board::starting_position();
        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();

        let e2_to_e4 = ChessMove::new(e2, e4);

        board.make_move(e2_to_e4);

        assert!(board.is_empty(e2));
        assert_eq!(
            board.piece_at(e4),
            Some(Piece::new(Color::White, PieceKind::Pawn))
        );
    }

    #[test]
    fn make_move_to_empty_square_returns_none() {
        let mut board = Board::starting_position();
        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();

        let e2_to_e4 = ChessMove::new(e2, e4);

        let captured_piece = board.make_move(e2_to_e4);

        assert_eq!(captured_piece, None);
    }

    #[test]
    fn make_move_to_occupied_square_returns_captured_piece() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let e5 = Square::new(4, 4).unwrap();
        let white_queen = Piece::new(Color::White, PieceKind::Queen);
        let black_pawn = Piece::new(Color::Black, PieceKind::Pawn);

        board.set_piece(e4, white_queen);
        board.set_piece(e5, black_pawn);

        let captured_piece = board.make_move(ChessMove::new(e4, e5));

        assert_eq!(captured_piece, Some(black_pawn));
        assert!(board.is_empty(e4));
        assert_eq!(board.piece_at(e5), Some(white_queen));
    }

    #[test]
    fn make_move_from_empty_square_does_nothing() {
        let mut board = Board::empty();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();

        let e2_to_e4 = ChessMove::new(e2, e4);

        board.make_move(e2_to_e4);

        assert!(board.is_empty(e2));
        assert!(board.is_empty(e4));
    }
}

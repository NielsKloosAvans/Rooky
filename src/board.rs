use crate::{ChessMove, Color, FenError, Piece, PieceKind, Square};

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

    pub fn from_fen_piece_placement(fen: &str) -> Result<Board, FenError> {
        let mut board = Board::empty();

        let ranks: Vec<&str> = fen.split('/').collect();
        if ranks.len() != 8 {
            return Err(FenError::WrongRankCount);
        }

        for (fen_rank, rank_text) in ranks.iter().enumerate() {
            let rank = 7 - fen_rank as u8;
            let mut file = 0;

            for c in rank_text.chars() {
                if c.is_ascii_digit() {
                    let empty_squares = c.to_digit(10).ok_or(FenError::WrongFileCount)? as u8;
                    if empty_squares == 0 {
                        return Err(FenError::WrongFileCount);
                    }

                    file += empty_squares;
                } else {
                    let color = if c.is_ascii_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };
                    let kind = PieceKind::from_char(c).ok_or(FenError::InvalidPiece)?;
                    let square = Square::new(file, rank).ok_or(FenError::WrongFileCount)?;
                    let piece = Piece::new(color, kind);

                    board.set_piece(square, piece);
                    file += 1;
                }
            }

            if file != 8 {
                return Err(FenError::WrongFileCount);
            }
        }

        Ok(board)
    }

    pub fn pawn_moves_from(&self, square: Square) -> Vec<ChessMove> {
        let Some(piece) = self.piece_at(square) else {
            return Vec::new();
        };

        if piece.kind != PieceKind::Pawn {
            return Vec::new();
        }

        let next_rank = match piece.color {
            Color::White => square.rank().checked_add(1),
            Color::Black => square.rank().checked_sub(1),
        };

        let Some(next_rank) = next_rank else {
            return Vec::new();
        };

        let Some(to) = Square::new(square.file(), next_rank) else {
            return Vec::new();
        };

        if self.is_empty(to) {
            vec![ChessMove::new(square, to)]
        } else {
            Vec::new()
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

    #[test]
    fn fen_piece_placement_starting_position_matches_starting_position() {
        let board = Board::from_fen_piece_placement("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        assert_eq!(board, Ok(Board::starting_position()));
    }

    #[test]
    fn fen_piece_placement_empty_board_has_no_piece_on_e4() {
        let board = Board::from_fen_piece_placement("8/8/8/8/8/8/8/8").unwrap();

        let e4 = Square::new(4, 3).unwrap();

        assert!(board.is_empty(e4));
    }

    #[test]
    fn fen_piece_placement_rejects_too_few_ranks() {
        let board = Board::from_fen_piece_placement("rnbqkbnr/pppppppp/8/8/8/8/8");

        assert_eq!(board, Err(FenError::WrongRankCount));
    }

    #[test]
    fn fen_piece_placement_rejects_too_many_ranks() {
        let board = Board::from_fen_piece_placement("rnbqkbnr/pppppppp/8/8/8/8/8/8/8");

        assert_eq!(board, Err(FenError::WrongRankCount));
    }

    #[test]
    fn fen_piece_placement_rejects_rank_with_too_few_files() {
        let board = Board::from_fen_piece_placement("7/8/8/8/8/8/8/8");

        assert_eq!(board, Err(FenError::WrongFileCount));
    }

    #[test]
    fn fen_piece_placement_rejects_rank_with_too_many_files() {
        let board = Board::from_fen_piece_placement("9/8/8/8/8/8/8/8");

        assert_eq!(board, Err(FenError::WrongFileCount));
    }

    #[test]
    fn fen_piece_placement_rejects_unknown_piece_letter() {
        let board = Board::from_fen_piece_placement("x7/8/8/8/8/8/8/8");

        assert_eq!(board, Err(FenError::InvalidPiece));
    }

    #[test]
    fn white_pawn_on_e2_can_move_to_e3() {
        let mut board = Board::empty();
        let e2 = Square::new(4, 1).unwrap();
        let e3 = Square::new(4, 2).unwrap();

        board.set_piece(e2, Piece::new(Color::White, PieceKind::Pawn));

        assert_eq!(board.pawn_moves_from(e2), vec![ChessMove::new(e2, e3)]);
    }

    #[test]
    fn black_pawn_on_e7_can_move_to_e6() {
        let mut board = Board::empty();
        let e7 = Square::new(4, 6).unwrap();
        let e6 = Square::new(4, 5).unwrap();

        board.set_piece(e7, Piece::new(Color::Black, PieceKind::Pawn));

        assert_eq!(board.pawn_moves_from(e7), vec![ChessMove::new(e7, e6)]);
    }

    #[test]
    fn white_pawn_blocked_on_e2_has_no_moves() {
        let mut board = Board::empty();
        let e2 = Square::new(4, 1).unwrap();
        let e3 = Square::new(4, 2).unwrap();

        board.set_piece(e2, Piece::new(Color::White, PieceKind::Pawn));
        board.set_piece(e3, Piece::new(Color::Black, PieceKind::Knight));

        assert!(board.pawn_moves_from(e2).is_empty());
    }

    #[test]
    fn black_pawn_blocked_on_e7_has_no_moves() {
        let mut board = Board::empty();
        let e7 = Square::new(4, 6).unwrap();
        let e6 = Square::new(4, 5).unwrap();

        board.set_piece(e7, Piece::new(Color::Black, PieceKind::Pawn));
        board.set_piece(e6, Piece::new(Color::White, PieceKind::Knight));

        assert!(board.pawn_moves_from(e7).is_empty());
    }

    #[test]
    fn empty_square_has_no_pawn_moves() {
        let board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();

        assert!(board.pawn_moves_from(e4).is_empty());
    }

    #[test]
    fn queen_square_has_no_pawn_moves() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Queen));

        assert!(board.pawn_moves_from(e4).is_empty());
    }
}

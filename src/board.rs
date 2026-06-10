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

        let mut moves = Vec::new();

        if let Some(to) = Square::new(square.file(), next_rank)
            && self.is_empty(to)
        {
            moves.push(ChessMove::new(square, to));

            let starting_rank = match piece.color {
                Color::White => 1,
                Color::Black => 6,
            };

            if square.rank() == starting_rank {
                let double_move_rank = match piece.color {
                    Color::White => square.rank().checked_add(2),
                    Color::Black => square.rank().checked_sub(2),
                };

                if let Some(double_move_rank) = double_move_rank
                    && let Some(double_move_to) = Square::new(square.file(), double_move_rank)
                    && self.is_empty(double_move_to)
                {
                    moves.push(ChessMove::new(square, double_move_to));
                }
            }
        }

        for capture_file in [square.file().checked_sub(1), square.file().checked_add(1)] {
            if let Some(capture_file) = capture_file
                && let Some(capture_to) = Square::new(capture_file, next_rank)
                && let Some(target_piece) = self.piece_at(capture_to)
                && target_piece.color != piece.color
            {
                moves.push(ChessMove::new(square, capture_to));
            }
        }

        moves
    }

    pub fn knight_moves_from(&self, square: Square) -> Vec<ChessMove> {
        let Some(piece) = self.piece_at(square) else {
            return Vec::new();
        };

        if piece.kind != PieceKind::Knight {
            return Vec::new();
        };

        let knight_offsets: [(i8, i8); 8] = [
            (1, 2),
            (2, 1),
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, -1),
            (-2, 1),
            (-1, 2),
        ];

        let mut moves = Vec::new();

        for (file_offset, rank_offset) in knight_offsets {
            let target_file = square.file() as i8 + file_offset;
            let target_rank = square.rank() as i8 + rank_offset;

            if !(0..=7).contains(&target_file) || !(0..=7).contains(&target_rank) {
                continue;
            }

            let target_square = Square::new(target_file as u8, target_rank as u8).unwrap();

            if self.is_empty(target_square) {
                moves.push(ChessMove::new(square, target_square));
            }
            if let Some(target_piece) = self.piece_at(target_square)
                && target_piece.color != piece.color
            {
                moves.push(ChessMove::new(square, target_square));
            }
        }

        moves
    }

    pub fn king_moves_from(&self, square: Square) -> Vec<ChessMove> {
        let Some(piece) = self.piece_at(square) else {
            return Vec::new();
        };

        if piece.kind != PieceKind::King {
            return Vec::new();
        };

        let king_offsets: [(i8, i8); 8] = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];

        let mut moves = Vec::new();

        for (file_offset, rank_offset) in king_offsets {
            let target_file = square.file() as i8 + file_offset;
            let target_rank = square.rank() as i8 + rank_offset;

            if !(0..=7).contains(&target_file) || !(0..=7).contains(&target_rank) {
                continue;
            }

            let target_square = Square::new(target_file as u8, target_rank as u8).unwrap();

            if self.is_empty(target_square) {
                moves.push(ChessMove::new(square, target_square));
            }
            if let Some(target_piece) = self.piece_at(target_square)
                && target_piece.color != piece.color
            {
                moves.push(ChessMove::new(square, target_square));
            }
        }
        moves
    }

    pub fn rook_moves_from(&self, square: Square) -> Vec<ChessMove> {
        let Some(piece) = self.piece_at(square) else {
            return Vec::new();
        };

        if piece.kind != PieceKind::Rook {
            return Vec::new();
        };

        let rook_directions: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut moves = Vec::new();

        for (file_direction, rank_direction) in rook_directions {
            let mut target_file = square.file() as i8 + file_direction;
            let mut target_rank = square.rank() as i8 + rank_direction;

            while (0..=7).contains(&target_file) && (0..=7).contains(&target_rank) {
                let target_square = Square::new(target_file as u8, target_rank as u8).unwrap();

                if self.is_empty(target_square) {
                    moves.push(ChessMove::new(square, target_square));
                }

                if let Some(target_piece) = self.piece_at(target_square) {
                    if target_piece.color != piece.color {
                        moves.push(ChessMove::new(square, target_square));
                    }
                    break;
                }

                target_file += file_direction;
                target_rank += rank_direction;
            }
        }

        moves
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

        assert!(board.pawn_moves_from(e2).contains(&ChessMove::new(e2, e3)));
    }

    #[test]
    fn black_pawn_on_e7_can_move_to_e6() {
        let mut board = Board::empty();
        let e7 = Square::new(4, 6).unwrap();
        let e6 = Square::new(4, 5).unwrap();

        board.set_piece(e7, Piece::new(Color::Black, PieceKind::Pawn));

        assert!(board.pawn_moves_from(e7).contains(&ChessMove::new(e7, e6)));
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

    #[test]
    fn white_pawn_on_e2_can_move_two_squares_to_e4() {
        let mut board = Board::empty();
        let e2 = Square::new(4, 1).unwrap();
        let e3 = Square::new(4, 2).unwrap();
        let e4 = Square::new(4, 3).unwrap();

        board.set_piece(e2, Piece::new(Color::White, PieceKind::Pawn));

        assert_eq!(
            board.pawn_moves_from(e2),
            vec![ChessMove::new(e2, e3), ChessMove::new(e2, e4)]
        );
    }

    #[test]
    fn black_pawn_on_e7_can_move_two_squares_to_e5() {
        let mut board = Board::empty();
        let e7 = Square::new(4, 6).unwrap();
        let e6 = Square::new(4, 5).unwrap();
        let e5 = Square::new(4, 4).unwrap();

        board.set_piece(e7, Piece::new(Color::Black, PieceKind::Pawn));

        assert_eq!(
            board.pawn_moves_from(e7),
            vec![ChessMove::new(e7, e6), ChessMove::new(e7, e5)]
        );
    }

    #[test]
    fn white_pawn_not_on_starting_rank_cannot_move_two_squares() {
        let mut board = Board::empty();
        let e3 = Square::new(4, 2).unwrap();
        let e4 = Square::new(4, 3).unwrap();

        board.set_piece(e3, Piece::new(Color::White, PieceKind::Pawn));

        assert_eq!(board.pawn_moves_from(e3), vec![ChessMove::new(e3, e4)]);
    }

    #[test]
    fn black_pawn_not_on_starting_rank_cannot_move_two_squares() {
        let mut board = Board::empty();
        let e6 = Square::new(4, 5).unwrap();
        let e5 = Square::new(4, 4).unwrap();

        board.set_piece(e6, Piece::new(Color::Black, PieceKind::Pawn));

        assert_eq!(board.pawn_moves_from(e6), vec![ChessMove::new(e6, e5)]);
    }

    #[test]
    fn white_pawn_cannot_double_move_if_e3_is_blocked() {
        let mut board = Board::empty();
        let e2 = Square::new(4, 1).unwrap();
        let e3 = Square::new(4, 2).unwrap();

        board.set_piece(e2, Piece::new(Color::White, PieceKind::Pawn));
        board.set_piece(e3, Piece::new(Color::Black, PieceKind::Knight));

        assert!(board.pawn_moves_from(e2).is_empty());
    }

    #[test]
    fn white_pawn_cannot_double_move_if_e4_is_blocked() {
        let mut board = Board::empty();
        let e2 = Square::new(4, 1).unwrap();
        let e3 = Square::new(4, 2).unwrap();
        let e4 = Square::new(4, 3).unwrap();

        board.set_piece(e2, Piece::new(Color::White, PieceKind::Pawn));
        board.set_piece(e4, Piece::new(Color::Black, PieceKind::Knight));

        assert_eq!(board.pawn_moves_from(e2), vec![ChessMove::new(e2, e3)]);
    }

    #[test]
    fn white_pawn_on_e4_can_capture_black_piece_on_d5() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let d5 = Square::new(3, 4).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Pawn));
        board.set_piece(d5, Piece::new(Color::Black, PieceKind::Knight));

        assert!(board.pawn_moves_from(e4).contains(&ChessMove::new(e4, d5)));
    }

    #[test]
    fn white_pawn_on_e4_can_capture_black_piece_on_f5() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let f5 = Square::new(5, 4).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Pawn));
        board.set_piece(f5, Piece::new(Color::Black, PieceKind::Knight));

        assert!(board.pawn_moves_from(e4).contains(&ChessMove::new(e4, f5)));
    }

    #[test]
    fn black_pawn_on_e5_can_capture_white_piece_on_d4() {
        let mut board = Board::empty();
        let e5 = Square::new(4, 4).unwrap();
        let d4 = Square::new(3, 3).unwrap();

        board.set_piece(e5, Piece::new(Color::Black, PieceKind::Pawn));
        board.set_piece(d4, Piece::new(Color::White, PieceKind::Knight));

        assert!(board.pawn_moves_from(e5).contains(&ChessMove::new(e5, d4)));
    }

    #[test]
    fn black_pawn_on_e5_can_capture_white_piece_on_f4() {
        let mut board = Board::empty();
        let e5 = Square::new(4, 4).unwrap();
        let f4 = Square::new(5, 3).unwrap();

        board.set_piece(e5, Piece::new(Color::Black, PieceKind::Pawn));
        board.set_piece(f4, Piece::new(Color::White, PieceKind::Knight));

        assert!(board.pawn_moves_from(e5).contains(&ChessMove::new(e5, f4)));
    }

    #[test]
    fn white_pawn_cannot_capture_empty_diagonal_square() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let e5 = Square::new(4, 4).unwrap();
        let d5 = Square::new(3, 4).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Pawn));

        let moves = board.pawn_moves_from(e4);

        assert_eq!(moves, vec![ChessMove::new(e4, e5)]);
        assert!(!moves.contains(&ChessMove::new(e4, d5)));
    }

    #[test]
    fn white_pawn_cannot_capture_own_piece() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let e5 = Square::new(4, 4).unwrap();
        let d5 = Square::new(3, 4).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Pawn));
        board.set_piece(d5, Piece::new(Color::White, PieceKind::Knight));

        let moves = board.pawn_moves_from(e4);

        assert_eq!(moves, vec![ChessMove::new(e4, e5)]);
        assert!(!moves.contains(&ChessMove::new(e4, d5)));
    }

    #[test]
    fn pawn_on_a_file_does_not_wrap_left() {
        let mut board = Board::empty();
        let a4 = Square::new(0, 3).unwrap();
        let a5 = Square::new(0, 4).unwrap();
        let h5 = Square::new(7, 4).unwrap();

        board.set_piece(a4, Piece::new(Color::White, PieceKind::Pawn));
        board.set_piece(h5, Piece::new(Color::Black, PieceKind::Knight));

        let moves = board.pawn_moves_from(a4);

        assert_eq!(moves, vec![ChessMove::new(a4, a5)]);
        assert!(!moves.contains(&ChessMove::new(a4, h5)));
    }

    #[test]
    fn pawn_on_h_file_does_not_wrap_right() {
        let mut board = Board::empty();
        let h4 = Square::new(7, 3).unwrap();
        let h5 = Square::new(7, 4).unwrap();
        let a5 = Square::new(0, 4).unwrap();

        board.set_piece(h4, Piece::new(Color::White, PieceKind::Pawn));
        board.set_piece(a5, Piece::new(Color::Black, PieceKind::Knight));

        let moves = board.pawn_moves_from(h4);

        assert_eq!(moves, vec![ChessMove::new(h4, h5)]);
        assert!(!moves.contains(&ChessMove::new(h4, a5)));
    }

    #[test]
    fn white_knight_on_e4_can_move_to_8_squares() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Knight));

        let expected_moves = vec![
            ChessMove::new(e4, Square::new(5, 5).unwrap()),
            ChessMove::new(e4, Square::new(6, 4).unwrap()),
            ChessMove::new(e4, Square::new(6, 2).unwrap()),
            ChessMove::new(e4, Square::new(5, 1).unwrap()),
            ChessMove::new(e4, Square::new(3, 1).unwrap()),
            ChessMove::new(e4, Square::new(2, 2).unwrap()),
            ChessMove::new(e4, Square::new(2, 4).unwrap()),
            ChessMove::new(e4, Square::new(3, 5).unwrap()),
        ];

        assert_eq!(board.knight_moves_from(e4), expected_moves);
    }

    #[test]
    fn knight_on_a1_does_not_wrap_off_board() {
        let mut board = Board::empty();
        let a1 = Square::new(0, 0).unwrap();

        board.set_piece(a1, Piece::new(Color::White, PieceKind::Knight));

        let expected_moves = vec![
            ChessMove::new(a1, Square::new(1, 2).unwrap()),
            ChessMove::new(a1, Square::new(2, 1).unwrap()),
        ];

        assert_eq!(board.knight_moves_from(a1), expected_moves);
    }

    #[test]
    fn knight_can_capture_enemy_piece() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let f6 = Square::new(5, 5).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Knight));
        board.set_piece(f6, Piece::new(Color::Black, PieceKind::Pawn));

        assert!(
            board
                .knight_moves_from(e4)
                .contains(&ChessMove::new(e4, f6))
        );
    }

    #[test]
    fn knight_cannot_capture_own_piece() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let f6 = Square::new(5, 5).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Knight));
        board.set_piece(f6, Piece::new(Color::White, PieceKind::Pawn));

        assert!(
            !board
                .knight_moves_from(e4)
                .contains(&ChessMove::new(e4, f6))
        );
    }

    #[test]
    fn empty_square_has_no_knight_moves() {
        let board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();

        assert!(board.knight_moves_from(e4).is_empty());
    }

    #[test]
    fn pawn_square_has_no_knight_moves() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Pawn));

        assert!(board.knight_moves_from(e4).is_empty());
    }

    #[test]
    fn white_king_on_e4_can_move_to_8_squares() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::King));

        let expected_moves = vec![
            ChessMove::new(e4, Square::new(4, 4).unwrap()),
            ChessMove::new(e4, Square::new(5, 4).unwrap()),
            ChessMove::new(e4, Square::new(5, 3).unwrap()),
            ChessMove::new(e4, Square::new(5, 2).unwrap()),
            ChessMove::new(e4, Square::new(4, 2).unwrap()),
            ChessMove::new(e4, Square::new(3, 2).unwrap()),
            ChessMove::new(e4, Square::new(3, 3).unwrap()),
            ChessMove::new(e4, Square::new(3, 4).unwrap()),
        ];

        assert_eq!(board.king_moves_from(e4), expected_moves);
    }

    #[test]
    fn king_on_a1_does_not_wrap_off_board() {
        let mut board = Board::empty();
        let a1 = Square::new(0, 0).unwrap();

        board.set_piece(a1, Piece::new(Color::White, PieceKind::King));

        let expected_moves = vec![
            ChessMove::new(a1, Square::new(0, 1).unwrap()),
            ChessMove::new(a1, Square::new(1, 1).unwrap()),
            ChessMove::new(a1, Square::new(1, 0).unwrap()),
        ];

        assert_eq!(board.king_moves_from(a1), expected_moves);
    }

    #[test]
    fn king_can_capture_enemy_piece() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let e5 = Square::new(4, 4).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::King));
        board.set_piece(e5, Piece::new(Color::Black, PieceKind::Pawn));

        assert!(board.king_moves_from(e4).contains(&ChessMove::new(e4, e5)));
    }

    #[test]
    fn king_cannot_capture_own_piece() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let e5 = Square::new(4, 4).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::King));
        board.set_piece(e5, Piece::new(Color::White, PieceKind::Pawn));

        assert!(!board.king_moves_from(e4).contains(&ChessMove::new(e4, e5)));
    }

    #[test]
    fn empty_square_has_no_king_moves() {
        let board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();

        assert!(board.king_moves_from(e4).is_empty());
    }

    #[test]
    fn knight_square_has_no_king_moves() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Knight));

        assert!(board.king_moves_from(e4).is_empty());
    }

    #[test]
    fn rook_on_e4_can_move_along_rank_and_file() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Rook));

        let expected_moves = vec![
            ChessMove::new(e4, Square::new(4, 4).unwrap()),
            ChessMove::new(e4, Square::new(4, 5).unwrap()),
            ChessMove::new(e4, Square::new(4, 6).unwrap()),
            ChessMove::new(e4, Square::new(4, 7).unwrap()),
            ChessMove::new(e4, Square::new(5, 3).unwrap()),
            ChessMove::new(e4, Square::new(6, 3).unwrap()),
            ChessMove::new(e4, Square::new(7, 3).unwrap()),
            ChessMove::new(e4, Square::new(4, 2).unwrap()),
            ChessMove::new(e4, Square::new(4, 1).unwrap()),
            ChessMove::new(e4, Square::new(4, 0).unwrap()),
            ChessMove::new(e4, Square::new(3, 3).unwrap()),
            ChessMove::new(e4, Square::new(2, 3).unwrap()),
            ChessMove::new(e4, Square::new(1, 3).unwrap()),
            ChessMove::new(e4, Square::new(0, 3).unwrap()),
        ];

        assert_eq!(board.rook_moves_from(e4), expected_moves);
    }

    #[test]
    fn rook_stops_before_own_piece() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let e5 = Square::new(4, 4).unwrap();
        let e6 = Square::new(4, 5).unwrap();
        let e7 = Square::new(4, 6).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Rook));
        board.set_piece(e6, Piece::new(Color::White, PieceKind::Pawn));

        let moves = board.rook_moves_from(e4);

        assert!(moves.contains(&ChessMove::new(e4, e5)));
        assert!(!moves.contains(&ChessMove::new(e4, e6)));
        assert!(!moves.contains(&ChessMove::new(e4, e7)));
    }

    #[test]
    fn rook_stops_before_own_piece_on_same_rank() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let f4 = Square::new(5, 3).unwrap();
        let g4 = Square::new(6, 3).unwrap();
        let h4 = Square::new(7, 3).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Rook));
        board.set_piece(g4, Piece::new(Color::White, PieceKind::Pawn));

        let moves = board.rook_moves_from(e4);

        assert!(moves.contains(&ChessMove::new(e4, f4)));
        assert!(!moves.contains(&ChessMove::new(e4, g4)));
        assert!(!moves.contains(&ChessMove::new(e4, h4)));
    }

    #[test]
    fn rook_can_capture_enemy_piece_then_stops() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();
        let e5 = Square::new(4, 4).unwrap();
        let e6 = Square::new(4, 5).unwrap();
        let e7 = Square::new(4, 6).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::Rook));
        board.set_piece(e6, Piece::new(Color::Black, PieceKind::Pawn));

        let moves = board.rook_moves_from(e4);

        assert!(moves.contains(&ChessMove::new(e4, e5)));
        assert!(moves.contains(&ChessMove::new(e4, e6)));
        assert!(!moves.contains(&ChessMove::new(e4, e7)));
    }

    #[test]
    fn rook_on_a1_does_not_wrap() {
        let mut board = Board::empty();
        let a1 = Square::new(0, 0).unwrap();

        board.set_piece(a1, Piece::new(Color::White, PieceKind::Rook));

        let expected_moves = vec![
            ChessMove::new(a1, Square::new(0, 1).unwrap()),
            ChessMove::new(a1, Square::new(0, 2).unwrap()),
            ChessMove::new(a1, Square::new(0, 3).unwrap()),
            ChessMove::new(a1, Square::new(0, 4).unwrap()),
            ChessMove::new(a1, Square::new(0, 5).unwrap()),
            ChessMove::new(a1, Square::new(0, 6).unwrap()),
            ChessMove::new(a1, Square::new(0, 7).unwrap()),
            ChessMove::new(a1, Square::new(1, 0).unwrap()),
            ChessMove::new(a1, Square::new(2, 0).unwrap()),
            ChessMove::new(a1, Square::new(3, 0).unwrap()),
            ChessMove::new(a1, Square::new(4, 0).unwrap()),
            ChessMove::new(a1, Square::new(5, 0).unwrap()),
            ChessMove::new(a1, Square::new(6, 0).unwrap()),
            ChessMove::new(a1, Square::new(7, 0).unwrap()),
        ];

        assert_eq!(board.rook_moves_from(a1), expected_moves);
    }

    #[test]
    fn empty_square_has_no_rook_moves() {
        let board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();

        assert!(board.rook_moves_from(e4).is_empty());
    }

    #[test]
    fn king_square_has_no_rook_moves() {
        let mut board = Board::empty();
        let e4 = Square::new(4, 3).unwrap();

        board.set_piece(e4, Piece::new(Color::White, PieceKind::King));

        assert!(board.rook_moves_from(e4).is_empty());
    }
}

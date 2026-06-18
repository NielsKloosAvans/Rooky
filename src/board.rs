use crate::{ChessMove, Color, FenError, Piece, PieceKind, Square};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Board {
    // The board is stored as one flat array instead of 8 arrays of 8.
    // Each slot is either Some(piece) or None.
    squares: [Option<Piece>; 64],
}

#[derive(Debug, Clone, Copy)]
struct Direction {
    file_delta: i8,
    rank_delta: i8,
}

impl Direction {
    fn new(file_delta: i8, rank_delta: i8) -> Direction {
        Direction {
            file_delta,
            rank_delta,
        }
    }
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

    pub fn is_in_check(&self, color: Color) -> bool {
        let Some(king_square) = self.king_square(color) else {
            return false;
        };

        self.is_square_attacked(king_square, color.opposite())
    }

    pub fn is_checkmate(&self, color: Color) -> bool {
        self.is_in_check(color) && self.legal_moves_for(color).is_empty()
    }

    pub fn is_stalemate(&self, color: Color) -> bool {
        !self.is_in_check(color) && self.legal_moves_for(color).is_empty()
    }

    pub fn pseudo_legal_moves_for(&self, color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::new(file, rank).unwrap();

                let Some(piece) = self.piece_at(square) else {
                    continue;
                };

                if piece.color != color {
                    continue;
                }

                moves.extend(self.piece_moves_from(square));
            }
        }

        moves
    }

    pub fn legal_moves_for(&self, color: Color) -> Vec<ChessMove> {
        let mut legal_moves = Vec::new();

        for chess_move in self.pseudo_legal_moves_for(color) {
            let mut board_after_move = *self;

            board_after_move.make_move(chess_move);

            if !board_after_move.is_in_check(color) {
                legal_moves.push(chess_move);
            }
        }
        legal_moves
    }

    pub fn king_square(&self, color: Color) -> Option<Square> {
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::new(file, rank).unwrap();

                let Some(piece) = self.piece_at(square) else {
                    continue;
                };

                if piece.color == color && piece.kind == PieceKind::King {
                    return Some(square);
                }
            }
        }
        None
    }

    pub fn is_square_attacked(&self, target: Square, by_color: Color) -> bool {
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::new(file, rank).unwrap();

                let Some(piece) = self.piece_at(square) else {
                    continue;
                };

                if piece.color != by_color {
                    continue;
                }

                if self.piece_attacks_square(square, piece, target) {
                    return true;
                }
            }
        }

        false
    }

    fn piece_attacks_square(&self, from: Square, piece: Piece, target: Square) -> bool {
        match piece.kind {
            PieceKind::Pawn => Self::pawn_attacks_square(from, piece.color, target),
            PieceKind::Knight => self
                .knight_moves_from(from)
                .into_iter()
                .any(|chess_move| chess_move.to == target),
            PieceKind::Bishop => self
                .bishop_moves_from(from)
                .into_iter()
                .any(|chess_move| chess_move.to == target),
            PieceKind::Rook => self
                .rook_moves_from(from)
                .into_iter()
                .any(|chess_move| chess_move.to == target),
            PieceKind::Queen => self
                .queen_moves_from(from)
                .into_iter()
                .any(|chess_move| chess_move.to == target),
            PieceKind::King => self
                .king_moves_from(from)
                .into_iter()
                .any(|chess_move| chess_move.to == target),
        }
    }

    fn pawn_attacks_square(from: Square, color: Color, target: Square) -> bool {
        let rank_delta = match color {
            Color::White => 1,
            Color::Black => -1,
        };

        let target_rank = from.rank() as i8 + rank_delta;
        let file_delta = target.file() as i8 - from.file() as i8;

        target.rank() as i8 == target_rank && (file_delta == -1 || file_delta == 1)
    }

    pub fn piece_moves_from(&self, square: Square) -> Vec<ChessMove> {
        let Some(piece) = self.piece_at(square) else {
            return Vec::new();
        };

        match piece.kind {
            PieceKind::Pawn => self.pawn_moves_from(square),
            PieceKind::Knight => self.knight_moves_from(square),
            PieceKind::Bishop => self.bishop_moves_from(square),
            PieceKind::Rook => self.rook_moves_from(square),
            PieceKind::Queen => self.queen_moves_from(square),
            PieceKind::King => self.king_moves_from(square),
        }
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

        let promotion_rank = match piece.color {
            Color::White => 7,
            Color::Black => 0,
        };

        if let Some(to) = Square::new(square.file(), next_rank)
            && self.is_empty(to)
        {
            Self::push_pawn_moves(&mut moves, square, to, promotion_rank == next_rank);

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
                Self::push_pawn_moves(&mut moves, square, capture_to, next_rank == promotion_rank);
            }
        }
        moves
    }

    fn push_pawn_moves(moves: &mut Vec<ChessMove>, from: Square, to: Square, is_promotion: bool) {
        if is_promotion {
            for kind in [
                PieceKind::Queen,
                PieceKind::Rook,
                PieceKind::Bishop,
                PieceKind::Knight,
            ] {
                moves.push(ChessMove::new_promotion(from, to, Some(kind)));
            }
        } else {
            moves.push(ChessMove::new(from, to));
        }
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

        let rook_directions = [
            Direction::new(0, 1),
            Direction::new(1, 0),
            Direction::new(0, -1),
            Direction::new(-1, 0),
        ];

        self.sliding_moves_from(square, piece, &rook_directions)
    }

    fn sliding_moves_from(
        &self,
        square: Square,
        piece: Piece,
        directions: &[Direction],
    ) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        for direction in directions {
            let mut target_file = square.file() as i8 + direction.file_delta;
            let mut target_rank = square.rank() as i8 + direction.rank_delta;

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
                target_file += direction.file_delta;
                target_rank += direction.rank_delta;
            }
        }
        moves
    }

    pub fn bishop_moves_from(&self, square: Square) -> Vec<ChessMove> {
        let Some(piece) = self.piece_at(square) else {
            return Vec::new();
        };

        if piece.kind != PieceKind::Bishop {
            return Vec::new();
        };

        let bishop_directions = [
            Direction::new(1, 1),
            Direction::new(-1, 1),
            Direction::new(1, -1),
            Direction::new(-1, -1),
        ];

        self.sliding_moves_from(square, piece, &bishop_directions)
    }

    pub fn queen_moves_from(&self, square: Square) -> Vec<ChessMove> {
        let Some(piece) = self.piece_at(square) else {
            return Vec::new();
        };

        if piece.kind != PieceKind::Queen {
            return Vec::new();
        };

        let queen_directions = [
            Direction::new(1, 1),
            Direction::new(0, 1),
            Direction::new(-1, 1),
            Direction::new(1, -1),
            Direction::new(0, -1),
            Direction::new(1, 0),
            Direction::new(-1, -1),
            Direction::new(-1, 0),
        ];

        self.sliding_moves_from(square, piece, &queen_directions)
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
            let piece_to_place = match chess_move.promotion {
                Some(kind) => Piece::new(piece.color, kind),
                None => piece,
            };
            self.set_piece(chess_move.to, piece_to_place);
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
#[path = "board_tests.rs"]
mod tests;

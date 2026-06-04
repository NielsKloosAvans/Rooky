use crate::PieceKind::Knight;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square {
    // Files and ranks are zero-based: a1 is (0, 0), e4 is (4, 3).
    pub file: u8,
    pub rank: u8,
}

pub struct Board {
    // The board is stored as one flat array instead of 8 arrays of 8.
    // Each slot is either Some(piece) or None.
    pub squares: [Option<Piece>; 64],
}

impl Board {
    pub fn empty() -> Board {
        Board {
            squares: [None; 64],
        }
    }
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        // Convert a 2D square into a 1D array index.
        // Example: e4 is file 4, rank 3, so index is 3 * 8 + 4 = 28.
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

    pub fn starting_position() -> Board {
        let mut board = Board::empty();
        let black_pawn = Piece {
            color: Color::Black,
            kind: PieceKind::Pawn,
        };
        let black_knight = Piece {
            color: Color::Black,
            kind: PieceKind::Knight,
        };
        let black_queen = Piece {
            color: Color::Black,
            kind: PieceKind::Queen,
        };
        let black_king = Piece {
            color: Color::Black,
            kind: PieceKind::King,
        };
        let black_rook = Piece {
            color: Color::Black,
            kind: PieceKind::Rook,
        };
        let white_pawn = Piece {
            color: Color::White,
            kind: PieceKind::Pawn,
        };
        let white_knight = Piece {
            color: Color::White,
            kind: PieceKind::Knight,
        };
        let white_queen = Piece {
            color: Color::White,
            kind: PieceKind::Queen,
        };
        let white_king = Piece {
            color: Color::White,
            kind: PieceKind::King,
        };
        let white_rook = Piece {
            color: Color::White,
            kind: PieceKind::Rook,
        };
        let a1 = Square::new(0, 0).unwrap();
        let a2 = Square::new(0, 1).unwrap();
        let b1 = Square::new(1, 0).unwrap();
        let b2 = Square::new(1, 1).unwrap();
        let c1 = Square::new(2, 0).unwrap();
        let c2 = Square::new(2, 1).unwrap();
        let d1 = Square::new(3, 0).unwrap();
        let d2 = Square::new(3, 1).unwrap();
        let e1 = Square::new(4, 0).unwrap();
        let e2 = Square::new(4, 1).unwrap();
        let f1 = Square::new(5, 0).unwrap();
        let f2 = Square::new(5, 1).unwrap();
        let g1 = Square::new(6, 0).unwrap();
        let g2 = Square::new(6, 1).unwrap();
        let h1 = Square::new(7, 0).unwrap();
        let h2 = Square::new(7, 1).unwrap();
        let a7 = Square::new(0, 6).unwrap();
        let a8 = Square::new(0, 7).unwrap();
        let b7 = Square::new(1, 6).unwrap();
        let b8 = Square::new(1, 7).unwrap();
        let c7 = Square::new(2, 6).unwrap();
        let c8 = Square::new(2, 7).unwrap();
        let d7 = Square::new(3, 6).unwrap();
        let d8 = Square::new(3, 7).unwrap();
        let e7 = Square::new(4, 6).unwrap();
        let e8 = Square::new(4, 7).unwrap();
        let f7 = Square::new(5, 6).unwrap();
        let f8 = Square::new(5, 7).unwrap();
        let g7 = Square::new(6, 6).unwrap();
        let g8 = Square::new(6, 7).unwrap();
        let h7 = Square::new(7, 6).unwrap();
        let h8 = Square::new(7, 7).unwrap();

        board.set_piece(a1, white_rook);
        board.set_piece(a2, white_pawn);
        board.set_piece(b1, white_rook);
        board.set_piece(b2, white_pawn);
        board.set_piece(c1, white_rook);
        board.set_piece(c2, white_pawn);
        board.set_piece(d1, white_rook);
        board.set_piece(d2, white_pawn);
        board.set_piece(e1, white_rook);
        board.set_piece(e2, white_pawn);
        board.set_piece(f1, white_rook);
        board.set_piece(f2, white_pawn);
        board.set_piece(g1, white_rook);
        board.set_piece(g2, white_pawn);
        board.set_piece(h1, white_rook);
        board.set_piece(h2, white_pawn);
        board.set_piece(a7, white_pawn);
        board.set_piece(a8, white_rook);
        board.set_piece(b7, white_pawn);
        board.set_piece(b8, white_rook);
        board.set_piece(c7, white_pawn);
        board.set_piece(c8, white_rook);
        board.set_piece(d7, white_pawn);
        board.set_piece(d8, white_rook);
        board.set_piece(e7, white_pawn);
        board.set_piece(e8, white_rook);
        board.set_piece(f7, white_pawn);
        board.set_piece(f8, white_rook);
        board.set_piece(g7, white_pawn);
        board.set_piece(g8, white_rook);
        board.set_piece(h7, white_pawn);
        board.set_piece(h8, white_rook);

        board
    }
}

impl Square {
    pub fn new(file: u8, rank: u8) -> Option<Square> {
        // Valid board coordinates are 0 through 7.
        if file >= 8 || rank >= 8 {
            return None;
        }
        Some(Square { file, rank })
    }

    pub fn index(&self) -> usize {
        let index = self.rank as usize * 8 + self.file as usize;
        return index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_can_store_a_white_king() {
        let piece = Piece {
            color: Color::White,
            kind: PieceKind::King,
        };

        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.kind, PieceKind::King);
    }

    #[test]
    fn piece_can_store_a_black_knight() {
        let piece = Piece {
            color: Color::Black,
            kind: PieceKind::Knight,
        };

        assert_eq!(piece.color, Color::Black);
        assert_eq!(piece.kind, PieceKind::Knight);
    }

    #[test]
    fn square_stores_zero_based_file_and_rank() {
        let square = Square { file: 4, rank: 3 };

        assert_eq!(square.file, 4);
        assert_eq!(square.rank, 3);
    }

    #[test]
    fn square_new_rejects_file_outside_the_board() {
        assert_eq!(Square::new(8, 0), None);
    }

    #[test]
    fn square_new_accepts_coordinates_inside_the_board() {
        assert_eq!(Square::new(4, 3), Some(Square { file: 4, rank: 3 }));
    }

    #[test]
    fn square_new_rejects_rank_outside_the_board() {
        assert_eq!(Square::new(0, 8), None);
    }

    #[test]
    fn empty_board_has_no_piece_on_e4() {
        let board = Board::empty();

        let e4 = Square::new(4, 3).unwrap();

        assert_eq!(board.piece_at(e4), None);
    }

    #[test]
    fn set_piece_stores_piece_on_e4() {
        let mut board = Board::empty();

        let e4 = Square::new(4, 3).unwrap();

        let white_queen = Piece {
            color: Color::White,
            kind: PieceKind::Queen,
        };

        board.set_piece(e4, white_queen);

        assert_eq!(board.piece_at(e4), Some(white_queen));
    }

    #[test]
    fn square_index_maps_e4_to_28() {
        let e4 = Square::new(4, 3).unwrap();

        assert_eq!(e4.index(), 28);
    }

    #[test]
    fn square_index_maps_a1_to_0() {
        let a1 = Square::new(0, 0).unwrap();

        assert_eq!(a1.index(), 0);
    }

    #[test]
    fn square_index_maps_h8_to_63() {
        let h8 = Square::new(7, 7).unwrap();

        assert_eq!(h8.index(), 63);
    }

    #[test]
    fn remove_piece_on_e4() {
        let mut board = Board::empty();

        let e4 = Square::new(4, 3).unwrap();

        let white_queen = Piece {
            color: Color::White,
            kind: PieceKind::Queen,
        };

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
}

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

        board.remove_piece(e4);

        assert_eq!(board.piece_at(e4), None);
    }
}

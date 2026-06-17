use crate::{PieceKind, Square};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ChessMove {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceKind>,
}

impl ChessMove {
    pub fn new(from: Square, to: Square) -> ChessMove {
        ChessMove {
            from,
            to,
            promotion: None,
        }
    }

    pub fn new_promotion(from: Square, to: Square, promotion: Option<PieceKind>) -> ChessMove {
        ChessMove {
            from,
            to,
            promotion,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_chess_move() {
        let e4 = Square::new(4, 3).unwrap();
        let e5 = Square::new(4, 4).unwrap();
        let e4_to_e5 = ChessMove::new(e4, e5);

        assert_eq!(e4_to_e5.from, Square::new(4, 3).unwrap());
        assert_eq!(e4_to_e5.to, Square::new(4, 4).unwrap());
        assert_eq!(e4_to_e5.promotion, None);
    }

    #[test]
    fn new_promotion_chess_move() {
        let e7 = Square::new(4, 6).unwrap();
        let e8 = Square::new(4, 7).unwrap();
        let e7_to_e8_queen = ChessMove::new_promotion(e7, e8, Some(PieceKind::Queen));

        assert_eq!(e7_to_e8_queen.from, e7);
        assert_eq!(e7_to_e8_queen.to, e8);
        assert_eq!(e7_to_e8_queen.promotion, Some(PieceKind::Queen));
    }
}

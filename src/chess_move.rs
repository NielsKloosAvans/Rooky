use crate::Square;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ChessMove {
    pub from: Square,
    pub to: Square,
}

impl ChessMove {
    pub fn new(from: Square, to: Square) -> ChessMove {
        ChessMove { from, to }
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
    }
}

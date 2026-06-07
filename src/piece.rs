use crate::Color;

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

impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Piece {
        Piece { color, kind }
    }
}

impl PieceKind {
    pub fn from_char(c: char) -> Option<PieceKind> {
        match c.to_ascii_lowercase() {
            'k' => Some(PieceKind::King),
            'q' => Some(PieceKind::Queen),
            'r' => Some(PieceKind::Rook),
            'b' => Some(PieceKind::Bishop),
            'n' => Some(PieceKind::Knight),
            'p' => Some(PieceKind::Pawn),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_can_store_a_white_king() {
        let piece = Piece::new(Color::White, PieceKind::King);

        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.kind, PieceKind::King);
    }

    #[test]
    fn piece_can_store_a_black_knight() {
        let piece = Piece::new(Color::Black, PieceKind::Knight);

        assert_eq!(piece.color, Color::Black);
        assert_eq!(piece.kind, PieceKind::Knight);
    }
}

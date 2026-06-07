#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn white_opposite_is_black() {
        assert_eq!(Color::White.opposite(), Color::Black);
    }

    #[test]
    fn black_opposite_is_white() {
        assert_eq!(Color::Black.opposite(), Color::White);
    }
}

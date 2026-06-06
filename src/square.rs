#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square {
    // Files and ranks are zero-based: a1 is (0, 0), e4 is (4, 3).
    // file: 0..7, where a = 0 and h = 7
    file: u8,
    // rank: 0..7, where rank 1 = 0 and rank 8 = 7
    rank: u8,
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
        self.rank as usize * 8 + self.file as usize
    }

    pub fn file(&self) -> u8 {
        self.file
    }

    pub fn rank(&self) -> u8 {
        self.rank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_stores_zero_based_file_and_rank() {
        let square = Square::new(4, 3).unwrap();

        assert_eq!(square.file(), 4);
        assert_eq!(square.rank(), 3);
    }

    #[test]
    fn square_new_rejects_file_outside_the_board() {
        assert_eq!(Square::new(8, 0), None);
    }

    #[test]
    fn square_new_accepts_coordinates_inside_the_board() {
        let square = Square::new(4, 3).unwrap();

        assert_eq!(square.file(), 4);
        assert_eq!(square.rank(), 3);
    }

    #[test]
    fn square_new_rejects_rank_outside_the_board() {
        assert_eq!(Square::new(0, 8), None);
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
}

use crate::{Board, ChessMove, Color, Piece};

pub struct Game {
    pub board: Board,
    pub side_to_move: Color,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::starting_position(),
            side_to_move: Color::White,
        }
    }
    pub fn make_move(&mut self, chess_move: ChessMove) -> Option<Piece> {
        self.side_to_move = self.side_to_move.opposite();
        self.board.make_move(chess_move)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Square;

    use super::*;

    #[test]
    fn new_game_starts_with_white_to_move() {
        let game = Game::new();

        assert_eq!(game.side_to_move, Color::White);
    }

    #[test]
    fn new_game_starts_with_chess_starting_position() {
        let game = Game::new();

        assert_eq!(game.board, Board::starting_position())
    }

    #[test]
    fn game_makes_move_switches_side_to_move_from_white_to_black() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();

        let chess_move = ChessMove { from: e2, to: e4 };

        game.make_move(chess_move);

        assert_eq!(game.side_to_move, Color::Black);
    }
}

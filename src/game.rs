use crate::{Board, ChessMove, Color, Piece};

pub struct Game {
    pub board: Board,
    pub side_to_move: Color,
    move_history: Vec<ChessMove>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::starting_position(),
            side_to_move: Color::White,
            move_history: Vec::new(),
        }
    }
    pub fn make_move(&mut self, chess_move: ChessMove) -> Option<Piece> {
        let captured_piece = self.board.make_move(chess_move);
        self.move_history.push(chess_move);
        self.side_to_move = self.side_to_move.opposite();
        captured_piece
    }
    pub fn move_history(&self) -> &[ChessMove] {
        &self.move_history
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::PieceKind;
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

    #[test]
    fn new_game_has_empty_move_history() {
        let game = Game::new();

        assert!(game.move_history.is_empty());
    }

    #[test]
    fn game_make_move_adds_move_to_history() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();

        let chess_move = ChessMove { from: e2, to: e4 };

        game.make_move(chess_move);

        assert_eq!(game.move_history.len(), 1);
    }

    #[test]
    fn game_make_move_records_the_move_that_was_played() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();

        let chess_move = ChessMove { from: e2, to: e4 };

        game.make_move(chess_move);

        assert_eq!(game.move_history(), &[chess_move]);
    }

    #[test]
    fn game_make_move_to_empty_square_returns_none() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove { from: e2, to: e4 };

        let result = game.make_move(chess_move);

        assert!(result.is_none());
        assert_eq!(game.move_history(), &[chess_move]);
    }

    #[test]
    fn game_make_move_to_occupied_square_returns_captured_piece() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove { from: e2, to: e4 };
        let piece = Piece {
            color: Color::Black,
            kind: PieceKind::Pawn,
        };
        game.board.set_piece(e4, piece);

        let result = game.make_move(chess_move);

        assert!(result.is_some());
        assert_eq!(game.move_history(), &[chess_move]);
        assert_eq!(result.unwrap().color, Color::Black);
    }
}

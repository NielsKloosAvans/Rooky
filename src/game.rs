use crate::{Board, ChessMove, Color, MoveRecord, Piece};

pub struct Game {
    pub board: Board,
    pub side_to_move: Color,
    move_history: Vec<MoveRecord>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::starting_position(),
            side_to_move: Color::White,
            move_history: Vec::new(),
        }
    }

    pub fn from_fen(fen: &str) -> Option<Game> {
        let mut parts = fen.split_whitespace();
        let board = Board::from_fen_piece_placement(parts.next()?)?;
        let side_to_move = match parts.next()? {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return None,
        };

        if parts.next().is_some() {
            return None;
        }

        Some(Game {
            board,
            side_to_move,
            move_history: Vec::new(),
        })
    }

    pub fn make_move(&mut self, chess_move: ChessMove) -> Option<Piece> {
        let side_that_moved = self.side_to_move;
        let captured_piece = self.board.make_move(chess_move);
        let move_record = MoveRecord::new(chess_move, captured_piece, side_that_moved);

        self.move_history.push(move_record);
        self.side_to_move = self.side_to_move.opposite();
        captured_piece
    }
    pub fn move_history(&self) -> &[MoveRecord] {
        &self.move_history
    }
    pub fn last_move(&self) -> Option<&MoveRecord> {
        self.move_history.last()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ChessMove, Square};

    use super::*;

    const STARTING_FEN_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

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

        assert!(game.move_history().is_empty());
    }

    #[test]
    fn game_make_move_adds_move_to_history() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove { from: e2, to: e4 };

        game.make_move(chess_move);

        assert_eq!(game.move_history().len(), 1);
    }

    #[test]
    fn game_make_move_records_the_move_that_was_played() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove { from: e2, to: e4 };
        let expected_record = MoveRecord::new(chess_move, None, Color::White);

        game.make_move(chess_move);

        assert_eq!(game.move_history(), &[expected_record]);
    }

    #[test]
    fn new_game_has_no_last_move() {
        let game = Game::new();
        assert!(game.last_move().is_none());
    }

    #[test]
    fn game_make_move_updates_last_move() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove { from: e2, to: e4 };
        let expected_record = MoveRecord::new(chess_move, None, Color::White);

        game.make_move(chess_move);

        assert_eq!(game.last_move(), Some(&expected_record));
    }

    #[test]
    fn game_from_fen_sets_board() {
        let game = Game::from_fen(&format!("{STARTING_FEN_BOARD} w")).unwrap();

        assert_eq!(game.board, Board::starting_position());
    }

    #[test]
    fn game_from_fen_sets_white_to_move() {
        let game = Game::from_fen("8/8/8/8/8/8/8/8 w").unwrap();

        assert_eq!(game.side_to_move, Color::White);
    }

    #[test]
    fn game_from_fen_sets_black_to_move() {
        let game = Game::from_fen("8/8/8/8/8/8/8/8 b").unwrap();

        assert_eq!(game.side_to_move, Color::Black);
    }

    #[test]
    fn game_from_fen_starts_with_empty_move_history() {
        let game = Game::from_fen("8/8/8/8/8/8/8/8 w").unwrap();

        assert!(game.move_history().is_empty());
    }

    #[test]
    fn game_from_fen_rejects_invalid_board() {
        let game = Game::from_fen("8/8/8/8/8/8/8 w");

        assert!(game.is_none());
    }

    #[test]
    fn game_from_fen_rejects_invalid_side_to_move() {
        let game = Game::from_fen("8/8/8/8/8/8/8/8 x");

        assert!(game.is_none());
    }
}

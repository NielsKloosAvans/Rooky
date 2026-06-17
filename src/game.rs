use crate::{Board, ChessMove, Color, FenError, MoveError, MoveRecord, Piece};

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

    pub fn from_fen(fen: &str) -> Result<Game, FenError> {
        let mut parts = fen.split_whitespace();
        let board_text = parts.next().ok_or(FenError::MissingBoard)?;
        let board = Board::from_fen_piece_placement(board_text)?;
        let side_to_move_text = parts.next().ok_or(FenError::MissingSideToMove)?;
        let side_to_move = match side_to_move_text {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FenError::InvalidSideToMove),
        };

        if parts.next().is_some() {
            return Err(FenError::TooManyFields);
        }

        Ok(Game {
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

    pub fn try_make_move(&mut self, chess_move: ChessMove) -> Result<Option<Piece>, MoveError> {
        let legal_moves = self.board.legal_moves_for(self.side_to_move);

        if !legal_moves.contains(&chess_move) {
            return Err(MoveError::IllegalMove);
        }

        let captured_piece = self.make_move(chess_move);

        Ok(captured_piece)
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
    use crate::{ChessMove, FenError, MoveError, PieceKind, Square};

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
        let chess_move = ChessMove::new(e2, e4);

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
        let chess_move = ChessMove::new(e2, e4);

        game.make_move(chess_move);

        assert_eq!(game.move_history().len(), 1);
    }

    #[test]
    fn game_make_move_records_the_move_that_was_played() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove::new(e2, e4);
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
        let chess_move = ChessMove::new(e2, e4);
        let expected_record = MoveRecord::new(chess_move, None, Color::White);

        game.make_move(chess_move);

        assert_eq!(game.last_move(), Some(&expected_record));
    }

    #[test]
    fn try_make_move_accepts_legal_move() {
        let mut game = Game::new();
        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove::new(e2, e4);

        let result = game.try_make_move(chess_move);

        assert!(matches!(result, Ok(None)));
        assert_eq!(game.board.piece_at(e2), None);
        assert_eq!(
            game.board.piece_at(e4),
            Some(Piece::new(Color::White, PieceKind::Pawn))
        );
        assert_eq!(game.side_to_move, Color::Black);
        assert_eq!(game.move_history().len(), 1);
    }

    #[test]
    fn try_make_move_rejects_illegal_move() {
        let mut game = Game::new();
        let e3 = Square::new(4, 2).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove::new(e3, e4);

        let result = game.try_make_move(chess_move);

        assert!(matches!(result, Err(MoveError::IllegalMove)));
    }

    #[test]
    fn try_make_move_rejected_move_does_not_change_board() {
        let mut game = Game::new();
        let board_before = game.board;
        let e3 = Square::new(4, 2).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove::new(e3, e4);

        let _ = game.try_make_move(chess_move);

        assert_eq!(game.board, board_before);
    }

    #[test]
    fn try_make_move_rejected_move_does_not_switch_side_to_move() {
        let mut game = Game::new();
        let e3 = Square::new(4, 2).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove::new(e3, e4);

        let _ = game.try_make_move(chess_move);

        assert_eq!(game.side_to_move, Color::White);
    }

    #[test]
    fn try_make_move_rejected_move_does_not_add_to_move_history() {
        let mut game = Game::new();
        let e3 = Square::new(4, 2).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove::new(e3, e4);

        let _ = game.try_make_move(chess_move);

        assert!(game.move_history().is_empty());
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

        assert!(matches!(game, Err(FenError::WrongRankCount)));
    }

    #[test]
    fn game_from_fen_rejects_invalid_side_to_move() {
        let game = Game::from_fen("8/8/8/8/8/8/8/8 x");

        assert!(matches!(game, Err(FenError::InvalidSideToMove)));
    }

    #[test]
    fn game_from_fen_rejects_missing_board() {
        let game = Game::from_fen("");

        assert!(matches!(game, Err(FenError::MissingBoard)));
    }

    #[test]
    fn game_from_fen_rejects_missing_side_to_move() {
        let game = Game::from_fen("8/8/8/8/8/8/8/8");

        assert!(matches!(game, Err(FenError::MissingSideToMove)));
    }

    #[test]
    fn game_from_fen_rejects_too_many_fields() {
        let game = Game::from_fen("8/8/8/8/8/8/8/8 w extra");

        assert!(matches!(game, Err(FenError::TooManyFields)));
    }
}

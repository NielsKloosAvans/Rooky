use crate::{Board, Color, MoveRecord, Piece};

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
    pub fn make_move(&mut self, move_record: MoveRecord) -> Option<Piece> {
        let captured_piece = self.board.make_move(move_record.chess_move);
        let last_move = self.move_history.last_mut();
        if let Some(last_move) = last_move {
            last_move.captured_piece = captured_piece;
        }
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
    use crate::ChessMove;
    use crate::Color;
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
        let move_record = MoveRecord {
            chess_move,
            captured_piece: None,
            side_that_moved: Color::White,
        };
        game.make_move(move_record);

        assert_eq!(game.side_to_move, Color::Black);
    }

    #[test]
    fn new_game_has_empty_move_history() {
        let game = Game::new();

        assert!(game.move_history.is_empty());
    }

    #[test]
    fn move_record_stores_chess_move() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove { from: e2, to: e4 };
        let move_record = MoveRecord {
            chess_move,
            captured_piece: None,
            side_that_moved: Color::White,
        };
        game.make_move(move_record);

        assert_eq!(game.move_history.len(), 1);
        assert_eq!(game.move_history[0].chess_move.from, e2);
        assert_eq!(game.move_history[0].chess_move.to, e4);
        assert_eq!(game.move_history[0].side_that_moved, Color::White);
    }

    #[test]
    fn move_record_stores_captured_piece() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove { from: e2, to: e4 };
        let move_record = MoveRecord {
            chess_move,
            captured_piece: Some(Piece::new(Color::Black, PieceKind::Queen)),
            side_that_moved: Color::White,
        };
        game.make_move(move_record);

        assert_eq!(game.move_history.len(), 1);
        assert_eq!(
            game.move_history[0].captured_piece,
            Some(Piece::new(Color::Black, PieceKind::Queen))
        );
        assert_eq!(
            game.move_history[0].captured_piece.unwrap().color,
            Color::Black
        );
    }

    #[test]
    fn move_record_stores_side_that_moved() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove { from: e2, to: e4 };
        let move_record = MoveRecord {
            chess_move,
            captured_piece: None,
            side_that_moved: Color::White,
        };
        game.make_move(move_record);

        assert_eq!(game.move_history.len(), 1);
        assert_eq!(game.move_history[0].side_that_moved, Color::White);
    }

    #[test]
    fn new_game_has_no_last_move() {
        let game = Game::new();
        assert!(game.move_history.is_empty());
    }

    #[test]
    fn game_make_move_updates_last_move() {
        let mut game = Game::new();

        let e2 = Square::new(4, 1).unwrap();
        let e4 = Square::new(4, 3).unwrap();
        let chess_move = ChessMove { from: e2, to: e4 };
        let move_record = MoveRecord {
            chess_move,
            captured_piece: None,
            side_that_moved: Color::White,
        };
        game.make_move(move_record);

        assert_eq!(game.move_history.len(), 1);
        assert_eq!(game.move_history[0].chess_move.from, e2);
        assert_eq!(game.move_history[0].chess_move.to, e4);
        assert_eq!(game.move_history[0].side_that_moved, Color::White);
    }
}

# Rooky

Rooky is a small chess engine written in Rust as a learning project.

The goal is not to build a strong engine immediately. The goal is to learn Rust
by building the engine step by step: board representation, move generation, FEN
parsing, game state, and eventually search.

## Current Status

Rooky can currently:

- Represent a chess board with 64 indexed squares.
- Store pieces with a color and piece kind.
- Build the normal starting position.
- Parse the board, side-to-move, and castling-rights fields of FEN into a
  `Game`.
- Move pieces on the board and record captured pieces.
- Track side to move and move history.
- Generate pseudo-legal moves for:
  - Pawns
  - Knights
  - Bishops
  - Rooks
  - Queens
  - Kings
- Generate pseudo-legal moves for a whole side.
- Generate legal moves that do not leave the moving side in check.
- Detect attacked squares.
- Find a side's king square.
- Detect whether a side is in check.
- Detect checkmate and stalemate.
- Generate and execute pawn promotions, including capture promotions.
- Store and validate castling rights from FEN.

Castling moves and en passant are not implemented yet. Castling rights are
stored in the game state, but they are not yet updated when a king or rook moves
and are not yet used during move generation. Position evaluation, search, UCI,
and the remaining FEN fields are also future work.

## Board Coordinates

The board uses zero-based coordinates:

- `a1` is `file = 0`, `rank = 0`
- `h1` is `file = 7`, `rank = 0`
- `a8` is `file = 0`, `rank = 7`
- `h8` is `file = 7`, `rank = 7`

Square indexes are stored in one flat array:

```text
index = rank * 8 + file
```

That means:

- `a1` has index `0`
- `e4` has index `28`
- `h8` has index `63`

## Running The Project

Run the test suite:

```sh
cargo test
```

Check formatting:

```sh
cargo fmt --check
```

Run Clippy:

```sh
cargo clippy --all-targets --all-features
```

## Project Structure

```text
src/
  board.rs        Board storage, FEN board parsing, attacks, check, and moves
  board_tests.rs  Board tests kept separate from board implementation
  chess_move.rs   Move type with from/to squares and optional promotion
  color.rs        White/black color type
  fen_error.rs    FEN parsing errors
  game.rs         Game state, FEN parsing, castling rights, and move history
  move_record.rs  Stored move history entries
  piece.rs        Piece and piece kind types
  square.rs       File/rank square coordinates and square indexes
```

## Learning Roadmap

Good next steps:

1. Update castling rights when a king or rook moves or an original rook is
   captured.
2. Generate and execute legal castling moves.
3. Add en passant state, move generation, and execution.
4. Parse the remaining FEN fields: en passant square, halfmove clock, and
   fullmove number.
5. Add a simple position evaluation.
6. Add search with minimax and alpha-beta pruning.
7. Connect the engine to a UCI loop so it can run in chess GUIs.

## Development Style

This project is built with small tests first. Each new chess rule should usually
start with one clear failing test, followed by the smallest implementation that
makes it pass.

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
- Parse the board and side-to-move parts of FEN into a `Game`.
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
- Detect attacked squares.
- Find a side's king square.
- Detect whether a side is in check.

The move generator does not yet filter legal moves. That means checkmate,
stalemate, pins, castling, en passant, promotion, and full legal move generation
are still future work.

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
  chess_move.rs   Move type with from/to squares
  color.rs        White/black color type
  fen_error.rs    FEN parsing errors
  game.rs         Game state, side to move, and move history
  move_record.rs  Stored move history entries
  piece.rs        Piece and piece kind types
  square.rs       File/rank square coordinates and square indexes
```

## Learning Roadmap

Good next steps:

1. Add legal move filtering so moves that leave your own king in check are removed.
2. Add checkmate and stalemate detection.
3. Add special pawn rules: promotion and en passant.
4. Add castling.
5. Parse full FEN fields: castling rights, en passant square, halfmove clock,
   and fullmove number.
6. Add a simple position evaluation.
7. Add search with minimax and alpha-beta pruning.
8. Connect the engine to a UCI loop so it can run in chess GUIs.

## Development Style

This project is built with small tests first. Each new chess rule should usually
start with one clear failing test, followed by the smallest implementation that
makes it pass.

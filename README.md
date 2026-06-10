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
- Parse the board and side-to-move parts of FEN.
- Move pieces on the board and record captured pieces.
- Track side to move and move history.
- Generate basic pseudo-legal moves for:
  - Pawns
  - Knights
  - Kings
  - Rooks

The move generator does not yet handle all chess rules. Check, checkmate,
castling, en passant, promotion, pins, and legal move filtering are still future
work.

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
  board.rs        Board storage, FEN board parsing, and piece move generation
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

1. Add bishop move generation.
2. Add queen move generation by combining rook-like and bishop-like directions.
3. Add legal move filtering so kings cannot move into check.
4. Add check detection.
5. Add special pawn rules: promotion and en passant.
6. Add castling.
7. Parse full FEN fields: castling rights, en passant square, halfmove clock,
   and fullmove number.
8. Add a simple position evaluation.
9. Add search with minimax and alpha-beta pruning.
10. Connect the engine to a UCI loop so it can run in chess GUIs.

## Development Style

This project is built with small tests first. Each new chess rule should usually
start with one clear failing test, followed by the smallest implementation that
makes it pass.

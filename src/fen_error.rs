#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenError {
    WrongRankCount,
    WrongFileCount,
    InvalidPiece,
    InvalidSideToMove,
    TooManyFields,
    MissingBoard,
    MissingSideToMove,
}

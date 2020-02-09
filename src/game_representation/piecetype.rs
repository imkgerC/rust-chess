/// Type of chess piece
///
/// A simple enum containing only un-colored chess piece types. It is represented as a byte
/// so that every variant has a unique number that can be used as an index.
/// * King = 1
/// * Pawn = 2
/// * Knight = 3
/// * Rook = 4
/// * Queen = 5
/// * Bishop = 6
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
    King = 1,
    Pawn = 2,
    Knight = 3,
    Rook = 4,
    Queen = 5,
    Bishop = 6,
}

/// Type of chess piece
/// 
/// A simple enum containing only un-colored chess piece types. It is represented as a byte
/// so that every variant has a unique number that can be used as an index, furthermore every byte-value
/// for the corresponding piece type has only one bit set in the full byte.
#[repr(u8)]
pub enum PieceType {
    King = 1 << 1,
    Pawn = 1 << 2,
    Knight = 1 << 3,
    Rook = 1 << 4,
    Queen = 1 << 5,
    Bishop = 1 << 6
}


use crate::core::bitboard;
use crate::move_generation::core::MoveGenColor;

pub fn single_pawn_pushes<T: MoveGenColor>(pawns: u64, empty: u64) -> u64 {
    if T::is_white() {
        bitboard::bitboard_north(pawns, 1) & empty
    } else {
        bitboard::bitboard_south(pawns, 1) & empty
    }
}

pub fn double_pawn_pushes<T: MoveGenColor>(pushed_pawns: u64, empty: u64) -> u64 {
    if T::is_white() {
        bitboard::bitboard_north(pushed_pawns & bitboard::constants::RANKS[2], 1) & empty
    } else {
        bitboard::bitboard_south(pushed_pawns & bitboard::constants::RANKS[5], 1) & empty
    }
}

use crate::core::bitboard;
use crate::game_representation::{Color, Game, PieceType};
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

pub fn can_be_attacked_from(destination: u64, piece: PieceType, state: &Game) -> u64 {
    let attacked = match piece {
        PieceType::Pawn => {
            (if state.color_to_move == Color::White {
                let rank_shifted = bitboard::bitboard_south(destination, 1);
                bitboard::bitboard_east_one(rank_shifted)
                    | bitboard::bitboard_west_one(rank_shifted)
            } else {
                let rank_shifted = bitboard::bitboard_north(destination, 1);
                bitboard::bitboard_east_one(rank_shifted)
                    | bitboard::bitboard_west_one(rank_shifted)
            }) & state.board.pawns
        }
        PieceType::King => {
            let left_right = destination
                | bitboard::bitboard_west_one(destination)
                | bitboard::bitboard_east_one(destination);
            (left_right
                | bitboard::bitboard_north(left_right, 1)
                | bitboard::bitboard_south(left_right, 1))
                & state.board.kings
        }
        PieceType::Knight => {
            let index = destination.trailing_zeros();
            /*let mut mask = 0;

            let two_east = bitboard::bitboard_east_one(bitboard::bitboard_east_one(destination));
            let two_west = bitboard::bitboard_west_one(bitboard::bitboard_west_one(destination));
            let east_west = two_east | two_west;
            let two_north = bitboard::bitboard_north(destination, 2);
            let two_south = bitboard::bitboard_south(destination, 2);
            let north_south = two_north | two_south;

            mask |= bitboard::bitboard_north(east_west, 1);
            mask |= bitboard::bitboard_south(east_west, 1);
            mask |= bitboard::bitboard_east_one(north_south);
            mask |= bitboard::bitboard_west_one(north_south);

            mask*/
            bitboard::constants::KNIGHT_MASKS[index as usize] & state.board.knights
        }
        PieceType::Rook => {
            let index = destination.trailing_zeros();
            bitboard::constants::ROOK_RAYS[index as usize] & state.board.rooks
        }
        PieceType::Bishop => {
            let index = destination.trailing_zeros();
            bitboard::constants::BISHOP_RAYS[index as usize] & state.board.bishops
        }
        PieceType::Queen => {
            can_be_attacked_from(destination, PieceType::Bishop, state)
                | can_be_attacked_from(destination, PieceType::Queen, state)
        }
    };
    if state.color_to_move == Color::White {
        attacked & state.board.whites
    } else {
        attacked & !state.board.whites
    }
}

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
        PieceType::Rook => rook_rays(destination, state) & !state.board.bishops,
        PieceType::Bishop => bishop_rays(destination, state) & !&state.board.rooks,
        PieceType::Queen => {
            let queens = state.board.bishops & state.board.rooks;
            (bishop_rays(destination, state) | rook_rays(destination, state)) & queens
        }
    };
    if state.color_to_move == Color::White {
        attacked & state.board.whites
    } else {
        attacked & !state.board.whites
    }
}

fn bishop_rays(field: u64, state: &Game) -> u64 {
    let all_pieces = state.board.bishops
        | state.board.rooks
        | state.board.pawns
        | state.board.knights
        | state.board.kings;
    let own_pieces;
    if state.color_to_move == Color::White {
        own_pieces = all_pieces & state.board.whites;
    } else {
        own_pieces = all_pieces & !state.board.whites;
    }
    let empty = !all_pieces;
    let mut mask = 0;
    let mut fill = field;
    while fill != mask {
        mask |= fill;
        let left_right = bitboard::bitboard_east_one(mask) | bitboard::bitboard_west_one(mask);
        fill = (bitboard::bitboard_north(left_right, 1)
            | bitboard::bitboard_south(left_right, 1)
            | mask)
            & (empty | field);
    }
    let left_right = bitboard::bitboard_east_one(mask) | bitboard::bitboard_west_one(mask);
    fill = (bitboard::bitboard_north(left_right, 1) | bitboard::bitboard_south(left_right, 1))
        & own_pieces;
    mask |= fill;
    mask & state.board.bishops
}

fn rook_rays(field: u64, state: &Game) -> u64 {
    let all_pieces = state.board.bishops
        | state.board.rooks
        | state.board.pawns
        | state.board.knights
        | state.board.kings;
    let own_pieces;
    if state.color_to_move == Color::White {
        own_pieces = all_pieces & state.board.whites;
    } else {
        own_pieces = all_pieces & !state.board.whites;
    }
    let empty = !all_pieces;
    let mut mask = 0;
    let mut fill = field;
    while fill != mask {
        mask |= fill;
        fill = (bitboard::bitboard_north(mask, 1) | bitboard::bitboard_south(mask, 1) | mask)
            & (empty | field);
    }
    fill = (bitboard::bitboard_north(mask, 1) | bitboard::bitboard_south(mask, 1)) & own_pieces;
    mask |= fill;

    let mut lr_mask = 0;
    let mut fill = field;
    while fill != lr_mask {
        lr_mask |= fill;
        fill =
            (bitboard::bitboard_east_one(lr_mask) | bitboard::bitboard_west_one(lr_mask) | lr_mask)
                & (empty | field);
    }
    fill =
        (bitboard::bitboard_east_one(lr_mask) | bitboard::bitboard_west_one(lr_mask)) & own_pieces;
    lr_mask |= fill;

    (mask | lr_mask) & state.board.rooks
}

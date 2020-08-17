use crate::core::bitboard;
use crate::game_representation::{Color, Game, PieceType};
use crate::move_generation::core::MoveGenColor;
use crate::move_generation::Action;
use crate::move_generation::core::{FieldIterator, QuietActionIterator, PawnPushIterator};

pub fn all_moves<T: MoveGenColor>(pinned: u64, in_check: bool, state: &Game) -> Vec<Action> {
    // missing: captures, king, en passant, promotion
    if in_check {
        unimplemented!();
    }

    let all_pieces = state.board.bishops
        | state.board.rooks
        | state.board.pawns
        | state.board.knights
        | state.board.kings;
    let own_pieces;
    let other_pieces;
    let last_rank;
    if T::is_white() {
        own_pieces = all_pieces & state.board.whites;
        other_pieces = all_pieces & !state.board.whites;
        last_rank = bitboard::constants::RANKS[7];
    } else {
        own_pieces = all_pieces & !state.board.whites;
        other_pieces = all_pieces & state.board.whites;
        last_rank = bitboard::constants::RANKS[0];
    }
    let empty = !all_pieces;

    let pushed_pawns = single_pawn_pushes::<T>(state.board.pawns & own_pieces & !pinned, empty);
    let double_pawns = double_pawn_pushes::<T>(pushed_pawns, empty);
    let mut iter: Box<dyn Iterator<Item = Action>> = Box::new(PawnPushIterator::new::<T>(pushed_pawns & !last_rank, double_pawns));

    for bishop_index in FieldIterator::new(state.board.bishops & own_pieces & !pinned & !state.board.rooks) {
        let bishop = 1 << bishop_index;
        let rays = bishop_rays(bishop, own_pieces, other_pieces);
        iter = Box::new(iter.chain(QuietActionIterator::new(rays & !other_pieces, PieceType::Bishop, bishop_index)));
    }

    for rook_index in FieldIterator::new(state.board.rooks & own_pieces & !pinned & !state.board.bishops) {
        let rook = 1 << rook_index;
        let rays = rook_rays(rook, own_pieces, other_pieces);
        iter = Box::new(iter.chain(QuietActionIterator::new(rays & !other_pieces, PieceType::Rook, rook_index)));
    }

    for queen_index in FieldIterator::new(state.board.rooks & state.board.bishops & own_pieces & !pinned) {
        let queen = 1 << queen_index;
        let rays = rook_rays(queen, own_pieces, other_pieces) | bishop_rays(queen, own_pieces, other_pieces);
        iter = Box::new(iter.chain(QuietActionIterator::new(rays & !other_pieces, PieceType::Queen, queen_index)));
    }

    for knight_index in FieldIterator::new(state.board.knights & own_pieces & !pinned) {
        let pos = bitboard::constants::KNIGHT_MASKS[knight_index as usize] & !own_pieces;
        iter = Box::new(iter.chain(QuietActionIterator::new(pos & !other_pieces, PieceType::Knight, knight_index)));
    }

    return iter.collect();
}

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
            bitboard::constants::KNIGHT_MASKS[index as usize] & state.board.knights
        }
        PieceType::Rook => rays_to_rooks(destination, state) & !state.board.bishops,
        PieceType::Bishop => rays_to_bishops(destination, state) & !&state.board.rooks,
        PieceType::Queen => {
            let queens = state.board.bishops & state.board.rooks;
            (rays_to_bishops(destination, state) | rays_to_rooks(destination, state)) & queens
        }
    };
    if state.color_to_move == Color::White {
        attacked & state.board.whites
    } else {
        attacked & !state.board.whites
    }
}

fn bishop_rays(bishop: u64, own_pieces: u64, other_pieces: u64) -> u64 {
    let empty = !(own_pieces | other_pieces);
    let mut mask = 0;
    let mut fill = bishop;
    while fill != mask {
        mask |= fill;
        let left_right = bitboard::bitboard_east_one(mask) | bitboard::bitboard_west_one(mask);
        fill = (bitboard::bitboard_north(left_right, 1)
            | bitboard::bitboard_south(left_right, 1)
            | mask)
            & (empty | bishop);
    }
    let left_right = bitboard::bitboard_east_one(mask) | bitboard::bitboard_west_one(mask);
    fill = (bitboard::bitboard_north(left_right, 1) | bitboard::bitboard_south(left_right, 1))
        & other_pieces; // captures
    mask |= fill;
    mask & !bishop
}

fn rook_rays(rook: u64, own_pieces: u64, other_pieces: u64) -> u64 {
    let empty = !(own_pieces | other_pieces);
    let mut mask = 0;
    let mut fill = rook;
    while fill != mask {
        mask |= fill;
        fill = (bitboard::bitboard_north(mask, 1) | bitboard::bitboard_south(mask, 1) | mask)
            & (empty | rook);
    }
    fill = (bitboard::bitboard_north(mask, 1) | bitboard::bitboard_south(mask, 1)) & other_pieces;
    mask |= fill;

    let mut lr_mask = 0;
    let mut fill = rook;
    while fill != lr_mask {
        lr_mask |= fill;
        fill =
            (bitboard::bitboard_east_one(lr_mask) | bitboard::bitboard_west_one(lr_mask) | lr_mask)
                & (empty | rook);
    }
    fill = (bitboard::bitboard_east_one(lr_mask) | bitboard::bitboard_west_one(lr_mask))
        & other_pieces;
    lr_mask |= fill;

    (mask | lr_mask) & !rook
}

fn rays_to_bishops(field: u64, state: &Game) -> u64 {
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

fn rays_to_rooks(field: u64, state: &Game) -> u64 {
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

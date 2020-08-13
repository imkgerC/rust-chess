pub use crate::game_representation::{Color, Game, PieceType};

use crate::core::{bitboard, ParserError};
use crate::move_generation::pseudolegal;

/// A standard chess halfmove action.
///
/// This struct contains a two byte representation of a move in chess. It only contains the moved piece type,
/// castling information, capture information, promotion information, from and to squares.
/// The internal structure can be subject to change and is currently as follows:
/// from_byte:
/// bit 0-2 => from_x
/// bit 3-5 => from_y
/// bit 6-7 => the first two bits of piece type
/// to_byte:
/// bit 0-2 => to_x
/// bit 3-5 => to_y
/// bit 6 => castling -> 1 if castling
/// bit 7 => the last bit of the piece type
/// special_byte:
/// bit 0: is_capture
/// bit 1: is_promotion
/// bit 2-4: capture_type, if capture, else is_kingside_castling in bit 2
/// bit 5-7: promotion_type
pub struct Action {
    from: u8,
    to: u8,
    special: u8,
}

/// A basic enum describing an action with further special information
///
/// Each enum has a different type of parameters:
/// * Quiet: No further data
/// * Capture: The captured piece
/// * Promotion: The type that is promoted to
/// * PromotionCapture: The type that is promoted to and the captured piece
#[derive(Debug, PartialEq)]
pub enum ActionType {
    Quiet,
    Capture(PieceType),
    Promotion(PieceType),
    PromotionCapture(PieceType, PieceType),
    Castling(bool),
}

impl Action {
    /// Returns a new Action struct with the corresponding values
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (0,6),
    ///     (0,7),
    ///     PieceType::Pawn,
    ///     ActionType::Promotion(PieceType::Rook));
    /// assert_eq!(action.get_from(), (0,6));
    /// ```
    pub fn new(from: (u8, u8), to: (u8, u8), piece: PieceType, actiontype: ActionType) -> Action {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;
        assert!(from_x < 8);
        assert!(to_x < 8);
        assert!(from_y < 8);
        assert!(to_y < 8);
        return Action::new_from_index(from_x + 8 * from_y, to_x + 8 * to_y, piece, actiontype);
    }

    /// todo: testing
    pub fn new_from_index(from: u8, to: u8, piece: PieceType, actiontype: ActionType) -> Action {
        let piece = piece as u8;

        let mut special = 0;
        let is_castling;
        match actiontype {
            ActionType::Quiet => {
                is_castling = 0;
            }
            ActionType::Capture(captured) => {
                is_castling = 0;
                special |= 0b1;
                special |= (captured as u8) << 2;
            }
            ActionType::Castling(is_kingside_castling) => {
                is_castling = 1;
                special |= (is_kingside_castling as u8) << 2;
            }
            ActionType::Promotion(promoted) => {
                is_castling = 0;
                special |= 0b10;
                special |= (promoted as u8) << 5;
            }
            ActionType::PromotionCapture(promoted, captured) => {
                is_castling = 0;
                special |= 0b11;
                special |= (captured as u8) << 2;
                special |= (promoted as u8) << 5;
            }
        }

        Action {
            from: from | (piece << 6),
            to: to | ((piece << 5) & 0b1000_0000) | (is_castling << 6),
            special,
        }
    }

    /// todo: testing
    pub fn from_pgn(pgn_string: &str, state: &Game) -> Result<Action, ParserError> {
        if pgn_string == "0-0" || pgn_string == "O-O" {
            // kingside castling
            let color = state.color_to_move as u8;
            return Ok(Action::new_from_index(
                60 - color * 56,
                63 - color * 56,
                PieceType::King,
                ActionType::Castling(true),
            ));
        }
        if pgn_string == "0-0-0" || pgn_string == "O-O-O" {
            // queenside castling
            let color = state.color_to_move as u8;
            return Ok(Action::new_from_index(
                60 - color * 56,
                56 - color * 56,
                PieceType::King,
                ActionType::Castling(true),
            ));
        }
        if pgn_string.len() == 2 {
            // simple pawn push
            let to_index = bitboard::field_repr_to_index(pgn_string)?;
            let color_sign = (-(state.color_to_move as i8)) * 2 + 1;
            let mut index_delta = 8 * color_sign;
            if (1 << (to_index as i8 + index_delta)) & state.board.pawns == 0 {
                index_delta *= 2;
            }
            let from_index = (to_index as i8 + index_delta) as u8;
            return Ok(Action::new_from_index(
                from_index,
                to_index,
                PieceType::Pawn,
                ActionType::Quiet,
            ));
        }
        if pgn_string.len() < 2 {
            return Err(ParserError::InvalidParameter("Wrong length of pgn action"));
        }
        let mut chars = pgn_string.chars().collect::<Vec<_>>();
        let piece;
        if chars[0].is_uppercase() {
            piece = bitboard::char_to_piecetype(chars[0])?;
            chars.remove(0);
        } else {
            piece = PieceType::Pawn;
        }
        if chars.len() < 2 {
            return Err(ParserError::InvalidParameter("Wrong length of pgn action"));
        }

        // promotion
        let promotion_piece;
        if chars[chars.len() - 2] == '=' {
            promotion_piece = Some(bitboard::char_to_piecetype(chars[chars.len() - 1])?);
            chars.remove(chars.len() - 1);
            chars.remove(chars.len() - 1);
        } else {
            promotion_piece = None;
        }

        let to_rank = bitboard::str_to_rank(&chars[chars.len() - 1].to_string())?;
        let to_file = bitboard::str_to_file(chars[chars.len() - 2])?;
        chars.remove(chars.len() - 1);
        chars.remove(chars.len() - 1);

        let capture_index = chars
            .iter()
            .enumerate()
            .map(|(i, c)| if *c == 'x' { Some(i) } else { None })
            .fold(None, |a, b| if a.is_none() { b } else { a });
        let is_capture = capture_index.is_some();
        if is_capture {
            chars.remove(capture_index.expect("Was checked, can't happen"));
        }

        let from_rank;
        let from_file;
        if chars.len() == 2 {
            // fully specified
            from_file = bitboard::str_to_file(chars[0])?;
            from_rank = bitboard::str_to_rank(&chars[1].to_string())?;
        } else if chars.len() == 1 {
            if chars[0].is_numeric() {
                // rank specified
                from_rank = bitboard::str_to_rank(&chars[0].to_string())?;
                let to_index = to_file + to_rank * 8;
                let destination = 1 << (to_index);
                let mask = pseudolegal::can_be_attacked_from(destination, piece, state)
                    | bitboard::constants::RANKS[from_rank as usize];
                if mask.count_ones() != 1 {
                    return Err(ParserError::InvalidParameter(
                        "Multiple options for source square found",
                    ));
                }
                let from_index = mask.trailing_zeros() as u8;
                if from_rank != from_index / 8 {
                    return Err(ParserError::InvalidParameter(
                        "Source square is not on same rank as specified",
                    ));
                }
                from_file = from_index % 8;
            } else {
                // file specified
                from_file = bitboard::str_to_file(chars[0])?;
                let to_index = to_file + to_rank * 8;
                let destination = 1 << (to_index);
                let mask = pseudolegal::can_be_attacked_from(destination, piece, state)
                    | bitboard::constants::FILES[from_file as usize];
                if mask.count_ones() != 1 {
                    return Err(ParserError::InvalidParameter(
                        "Multiple options for source square found",
                    ));
                }
                let from_index = mask.trailing_zeros() as u8;
                if from_file != from_index % 8 {
                    return Err(ParserError::InvalidParameter(
                        "Source square is not on same file as specified",
                    ));
                }
                from_rank = from_index / 8;
            }
        } else {
            // no specification
            let to_index = to_file + to_rank * 8;
            let destination = 1 << (to_index);
            let mask = pseudolegal::can_be_attacked_from(destination, piece, state);
            if mask.count_ones() != 1 {
                return Err(ParserError::InvalidParameter(
                    "Multiple options for source square found",
                ));
            }
            let from_index = mask.trailing_zeros() as u8;
            from_rank = from_index / 8;
            from_file = from_index % 8;
        }

        let action_type;
        if promotion_piece.is_some() && is_capture {
            // promotion capture
            let capture_piece = state.board.get_piecetype_on(to_rank * 8 + to_file);
            if capture_piece.is_none() {
                return Err(ParserError::InvalidParameter(
                    "No piece to capture on destination",
                ));
            }
            action_type = ActionType::PromotionCapture(
                promotion_piece.expect("Cannot happen, checked"),
                capture_piece.expect("Cannot happend, checked"),
            );
        } else if promotion_piece.is_some() {
            // promotion
            action_type = ActionType::Promotion(promotion_piece.expect("Cannot happen, checked"));
        } else if is_capture {
            // capture
            let capture_piece = state.board.get_piecetype_on(to_rank * 8 + to_file);
            if capture_piece.is_none() {
                return Err(ParserError::InvalidParameter(
                    "No piece to capture on destination",
                ));
            }
            action_type = ActionType::Capture(capture_piece.expect("Was checked, can't happen"));
        } else {
            // quiet
            action_type = ActionType::Quiet;
        }
        Ok(Action::new(
            (from_file, from_rank),
            (to_file, to_rank),
            piece,
            action_type,
        ))
    }

    /// Returns the coordinates moved from
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (0,6),
    ///     (0,7),
    ///     PieceType::Pawn,
    ///     ActionType::Promotion(PieceType::Rook));
    /// assert_eq!(action.get_from(), (0,6));
    /// ```
    #[inline(always)]
    pub fn get_from(&self) -> (u8, u8) {
        (self.from & 0b111, (self.from >> 3) & 0b111)
    }

    /// Returns the coordinates moved to
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (0,6),
    ///     (0,7),
    ///     PieceType::Pawn,
    ///     ActionType::Promotion(PieceType::Rook));
    /// assert_eq!(action.get_to(), (0,7));
    /// ```
    #[inline(always)]
    pub fn get_to(&self) -> (u8, u8) {
        (self.to & 0b111, (self.to >> 3) & 0b111)
    }

    /// Returns the index moved from
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (0,6),
    ///     (0,7),
    ///     PieceType::Pawn,
    ///     ActionType::Promotion(PieceType::Rook));
    /// assert_eq!(action.get_from_index(), 48);
    /// ```
    #[inline(always)]
    pub fn get_from_index(&self) -> u8 {
        self.from & 0b11_1111
    }

    /// Returns the index moved to
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (0,6),
    ///     (0,7),
    ///     PieceType::Pawn,
    ///     ActionType::Promotion(PieceType::Rook));
    /// assert_eq!(action.get_to_index(), 56);
    /// ```
    #[inline(always)]
    pub fn get_to_index(&self) -> u8 {
        self.to & 0b11_1111
    }

    /// Returns the moved piece
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (0,6),
    ///     (0,7),
    ///     PieceType::Pawn,
    ///     ActionType::Promotion(PieceType::Rook));
    /// assert_eq!(action.get_piecetype(), PieceType::Pawn);
    /// ```
    #[inline(always)]
    pub fn get_piecetype(&self) -> PieceType {
        let piece = (self.from >> 6) | ((self.to >> 5) & 0b100);
        unsafe { std::mem::transmute(piece) }
    }

    /// Returns a fully filled ActionType enum for the action
    ///
    /// Gives you any information you may need except for the from_square, to_square
    /// and the moved piece. This method is always safe to call and will return valid data.
    /// This means, if the move is of type:
    /// * Quiet: Nothing else
    /// * Capture: The captured piece
    /// * Promotion: The piece that was promoted to
    /// * PromotionCapture: The piece that was promoted to and the captured piece
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (4,7),
    ///     (2,7),
    ///     PieceType::King,
    ///     ActionType::PromotionCapture(PieceType::Knight, PieceType::Queen));
    /// assert_eq!(action.get_action_type(),
    ///     ActionType::PromotionCapture(PieceType::Knight, PieceType::Queen));
    /// ```
    #[inline(always)]
    pub fn get_action_type(&self) -> ActionType {
        if self.is_capture() && self.is_promotion() {
            ActionType::PromotionCapture(
                self.get_promotion_piece()
                    .expect("was checked beforehand, should not happen"),
                self.get_capture_piece()
                    .expect("was checked beforehand, should not happen"),
            )
        } else if self.is_capture() {
            ActionType::Capture(
                self.get_capture_piece()
                    .expect("was checked beforehand, should not happen"),
            )
        } else if self.is_promotion() {
            ActionType::Promotion(
                self.get_promotion_piece()
                    .expect("was checked beforehand, should not happen"),
            )
        } else if self.is_castling() {
            ActionType::Castling(self.is_kingside_castling())
        } else {
            ActionType::Quiet
        }
    }

    /// Checks if the action is a castling move
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (4,7),
    ///     (2,7),
    ///     PieceType::King,
    ///     ActionType::Castling(false));
    /// assert_eq!(action.is_castling(), true);
    #[inline(always)]
    pub fn is_castling(&self) -> bool {
        self.to & 0b100_0000 > 0
    }

    /// Checks if the action is kingside castling
    ///
    /// ATTENTION: If the action this is called on is not actually a castling move, then this will return part of the capture piece information
    /// which may or may not be set. Behaviour in that case is not guaranteed and can be subject to change!
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (4,7),
    ///     (2,7),
    ///     PieceType::King,
    ///     ActionType::Castling(false));
    /// assert_eq!(action.is_kingside_castling(), false);
    /// let action = Action::new(
    ///     (0,6),
    ///     (0,7),
    ///     PieceType::Pawn,
    ///     ActionType::Capture(PieceType::Knight));
    /// // action is not a castling move right now, thus the method call is bad
    /// assert_eq!(action.is_kingside_castling(), true); // DO NOT DO THAT
    #[inline(always)]
    pub fn is_kingside_castling(&self) -> bool {
        self.special & 0b100 > 0
    }

    /// Checks if the action is a capture
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (0,6),
    ///     (0,7),
    ///     PieceType::Rook,
    ///     ActionType::Capture(PieceType::Rook));
    /// assert_eq!(action.is_capture(), true);
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        self.special & 0b1 > 0
    }

    /// Checks if the action is a promotion
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (0,6),
    ///     (0,7),
    ///     PieceType::Pawn,
    ///     ActionType::Promotion(PieceType::Rook));
    /// assert_eq!(action.is_promotion(), true);
    /// ```
    #[inline(always)]
    pub fn is_promotion(&self) -> bool {
        self.special & 0b10 > 0
    }

    /// Returns the promoted piece if it is a promotion, else None
    ///
    /// This method can always be called and does both checking if it is a promotion and retrieving the piecetype information
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (0,6),
    ///     (0,7),
    ///     PieceType::Pawn,
    ///     ActionType::Promotion(PieceType::Rook));
    /// assert_eq!(action.get_promotion_piece(), Some(PieceType::Rook));
    /// ```
    #[inline(always)]
    pub fn get_promotion_piece(&self) -> Option<PieceType> {
        if !self.is_promotion() {
            return None;
        }
        Some(unsafe { std::mem::transmute((self.special >> 5) & 0b111) })
    }

    /// Returns the captured piece if it is a capture, else None
    ///
    /// This method can always be called and does both checking if it is a capture and retrieving the piecetype information
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::PieceType;
    /// # use core::move_generation::{ActionType, Action};
    /// let action = Action::new(
    ///     (0,0),
    ///     (7,7),
    ///     PieceType::Queen,
    ///     ActionType::Capture(PieceType::Rook));
    /// assert_eq!(action.get_capture_piece(), Some(PieceType::Rook));
    /// ```
    #[inline(always)]
    pub fn get_capture_piece(&self) -> Option<PieceType> {
        if !self.is_capture() {
            return None;
        }
        Some(unsafe { std::mem::transmute((self.special >> 2) & 0b111) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_out() {
        let action = Action::new((0, 1), (2, 3), PieceType::Queen, ActionType::Quiet);
        assert_eq!(action.get_from().0, 0);
        assert_eq!(action.get_from().1, 1);
        assert_eq!(action.get_to().0, 2);
        assert_eq!(action.get_to().1, 3);
        assert_eq!(action.get_piecetype(), PieceType::Queen);
        assert_eq!(action.is_capture(), false);
        assert_eq!(action.is_promotion(), false);
        assert_eq!(action.get_capture_piece(), None);
        assert_eq!(action.get_promotion_piece(), None);

        let action = Action::new(
            (0, 6),
            (1, 7),
            PieceType::Pawn,
            ActionType::PromotionCapture(PieceType::Queen, PieceType::Knight),
        );
        assert_eq!(action.get_from().0, 0);
        assert_eq!(action.get_from().1, 6);
        assert_eq!(action.get_to().0, 1);
        assert_eq!(action.get_to().1, 7);
        assert_eq!(action.get_piecetype(), PieceType::Pawn);
        assert_eq!(action.is_promotion(), true);
        assert_eq!(action.is_capture(), true);
        assert_eq!(action.get_capture_piece(), Some(PieceType::Knight));
        assert_eq!(action.get_promotion_piece(), Some(PieceType::Queen));
    }
}

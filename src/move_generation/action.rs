pub use crate::game_representation::{Color, PieceType};

/// A standard chess halfmove action.
///
/// This struct contains a two byte representation of a move in chess. It only contains the moved piece type,
/// the color of the playing player and from and to squares. Currently it does not support pawn promotions or castling.
/// The internal structure will be subject to change and currently is as follows:
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
/// bit 2-4: promotion_type, if not castling, else castling in bit 2
/// bit 5-7: capture_type
pub struct Action {
    from: u8,
    to: u8,
    special: u8,
}

pub enum ActionType {
    Quiet,
    Capture(PieceType),
    Promotion(PieceType),
    PromotionCapture(PieceType, PieceType),
    Castling(bool),
}

impl Action {
    pub fn new(from: (u8, u8), to: (u8, u8), piece: PieceType, actiontype: ActionType) -> Action {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;
        assert!(from_x < 8);
        assert!(to_x < 8);
        assert!(from_y < 8);
        assert!(to_y < 8);
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
            from: from_x | (from_y << 3) | (piece << 6),
            to: (to_x + (to_y << 3)) | ((piece << 5) & 0b1000_0000) | (is_castling << 6),
            special,
        }
    }

    #[inline(always)]
    pub fn get_from(&self) -> (u8, u8) {
        (self.from & 0b111, (self.from >> 3) & 0b111)
    }

    #[inline(always)]
    pub fn get_to(&self) -> (u8, u8) {
        (self.to & 0b111, (self.to >> 3) & 0b111)
    }

    #[inline(always)]
    pub fn get_from_index(&self) -> u8 {
        self.from & 0b11_1111
    }

    #[inline(always)]
    pub fn get_to_index(&self) -> u8 {
        self.to & 0b11_1111
    }

    #[inline(always)]
    pub fn get_piecetype(&self) -> PieceType {
        let piece = (self.from >> 6) | ((self.to >> 5) & 0b100);
        unsafe { std::mem::transmute(piece) }
    }

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

    #[inline(always)]
    pub fn is_castling(&self) -> bool {
        self.to & 0b100_0000 > 0
    }

    #[inline(always)]
    pub fn is_kingside_castling(&self) -> bool {
        self.special & 0b100 > 0
    }

    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        self.special & 0b1 > 0
    }

    #[inline(always)]
    pub fn is_promotion(&self) -> bool {
        self.special & 0b10 > 0
    }

    #[inline(always)]
    pub fn get_promotion_piece(&self) -> Option<PieceType> {
        if !self.is_promotion() {
            return None;
        }
        Some(unsafe { std::mem::transmute((self.special >> 5) & 0b111) })
    }

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

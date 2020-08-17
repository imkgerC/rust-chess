use crate::game_representation::PieceType;
use crate::move_generation::{Action, ActionType};

pub trait MoveGenColor {
    fn is_white() -> bool;
}

pub struct WhiteMoveGenColor;
impl MoveGenColor for WhiteMoveGenColor {
    fn is_white() -> bool {
        return true;
    }
}

pub struct BlackMoveGenColor;
impl MoveGenColor for BlackMoveGenColor {
    fn is_white() -> bool {
        return false;
    }
}

pub struct FieldIterator {
    data: u64,
}

impl FieldIterator {
    pub fn new(data: u64) -> Self {
        FieldIterator { data }
    }
}

impl Iterator for FieldIterator {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.data == 0 {
            return None;
        }
        let index = self.data.trailing_zeros();
        self.data &= !(1 << index);
        return Some(index as u8);
    }
}

pub struct PawnPushIterator {
    single: FieldIterator,
    double: FieldIterator,
    delta: i8,
}

impl PawnPushIterator {
    pub fn new<T: MoveGenColor>(single: u64, double: u64) -> PawnPushIterator {
        if T::is_white() {
            PawnPushIterator {
                single: FieldIterator::new(single),
                double: FieldIterator::new(double),
                delta: 8,
            }
        } else {
            PawnPushIterator {
                single: FieldIterator::new(single),
                double: FieldIterator::new(double),
                delta: -8,
            }
        }
    }
}

impl Iterator for PawnPushIterator {
    type Item = Action;

    fn next(&mut self) -> Option<Action> {
        if let Some(to) = self.single.next() {
            Some(Action::new_from_index(
                (to as i8 + self.delta) as u8,
                to,
                PieceType::Pawn,
                ActionType::Quiet,
            ))
        } else if let Some(to) = self.double.next() {
            Some(Action::new_from_index(
                (to as i8 + 2 * self.delta) as u8,
                to,
                PieceType::Pawn,
                ActionType::Quiet,
            ))
        } else {
            None
        }
    }
}

pub struct QuietActionIterator {
    fields: FieldIterator,
    piece: PieceType,
    from: u8,
}

impl QuietActionIterator {
    pub fn new(data: u64, piece: PieceType, from: u8) -> QuietActionIterator {
        QuietActionIterator {
            fields: FieldIterator::new(data),
            piece,
            from,
        }
    }
}

impl Iterator for QuietActionIterator {
    type Item = Action;

    fn next(&mut self) -> Option<Action> {
        if let Some(to) = self.fields.next() {
            Some(Action::new_from_index(
                self.from,
                to,
                self.piece,
                ActionType::Quiet,
            ))
        } else {
            None
        }
    }
}

pub use super::PieceType;
pub use super::Color;

pub struct Action {
    from: u8,
    to: u8
}

impl Action {
    pub fn new(from_x: u8, from_y: u8, to_x: u8, to_y: u8, piece: PieceType, color: Color) -> Action {
        assert!(from_x < 8);
        assert!(to_x < 8);
        assert!(from_y < 8);
        assert!(to_y < 8);
        let piece = piece as u8;
        let color = color as u8;
        return Action {
            from: from_x | (from_y << 3) | (piece << 6),
            to: to_x + (to_y << 3) | ((piece << 5) & 0b1000_0000)  | (color << 6)
        };
    }

    #[inline(always)]
    pub fn get_from(&self) -> (u8, u8) {
        return (self.from & 0b111, (self.from >> 3) & 0b111);
    }

    #[inline(always)]
    pub fn get_to(&self) -> (u8, u8) {
        return (self.to & 0b111, (self.to >> 3) & 0b111);
    }

    #[inline(always)]
    pub fn get_from_raw(&self) -> u8 {
        return self.from;
    }

    #[inline(always)]
    pub fn get_to_raw(&self) -> u8 {
        return self.to;
    }

    #[inline(always)]
    pub fn get_color(&self) -> Color {
        return unsafe { std::mem::transmute((self.to >> 6) & 1) };
    }

    #[inline(always)]
    pub fn get_piecetype(&self) -> PieceType {
        let piece = (self.from >> 6) | ((self.to >> 5) & 0b100);
        return unsafe { std::mem::transmute(piece) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_out() {
        let action = Action::new(0, 1, 2, 3, PieceType::Queen, Color::White);
        assert_eq!(action.get_from().0, 0);
        assert_eq!(action.get_from().1, 1);
        assert_eq!(action.get_to().0, 2);
        assert_eq!(action.get_to().1, 3);
        assert_eq!(action.get_color(), Color::White);
        assert_eq!(action.get_piecetype(), PieceType::Queen);

        let action = Action::new(7, 1, 5, 1, PieceType::Bishop, Color::Black);
        assert_eq!(action.get_from().0, 7);
        assert_eq!(action.get_from().1, 1);
        assert_eq!(action.get_to().0, 5);
        assert_eq!(action.get_to().1, 1);
        assert_eq!(action.get_color(), Color::Black);
        assert_eq!(action.get_piecetype(), PieceType::Bishop);
    }
}
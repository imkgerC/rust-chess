pub struct Castling {
    data: u8,
}

const WHITE_KINGSIDE: u8 = 1 << 0;
const WHITE_QUEENSIDE: u8 = 1 << 1;
const BLACK_KINGSIDE: u8 = 1 << 2;
const BLACK_QUEENSIDE: u8 = 1 << 3;

impl Castling {
    pub fn new() -> Castling {
        return Castling {
            data: WHITE_KINGSIDE | WHITE_QUEENSIDE | BLACK_KINGSIDE | BLACK_QUEENSIDE,
        };
    }

    pub fn is_available(&self, data: u8) -> bool {
        return (self.data & data) > 0;
    }

    pub fn remove(&mut self, data: u8) {
        self.data = self.data & !data;
    }

    #[inline(always)]
    pub fn get_white_kingside() -> u8 {
        return WHITE_KINGSIDE;
    }

    #[inline(always)]
    pub fn get_white_queenside() -> u8 {
        return WHITE_QUEENSIDE;
    }

    #[inline(always)]
    pub fn get_black_kingside() -> u8 {
        return BLACK_KINGSIDE;
    }

    #[inline(always)]
    pub fn get_black_queenside() -> u8 {
        return BLACK_QUEENSIDE;
    }
}

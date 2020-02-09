/// Basic struct containing castling information for both players in a single byte
///
/// The byte has a single bit flag for every type of castling:
/// * Bit 0 is WHITE_KINGSIDE
/// * Bit 1 is WHITE_QUEENSIDE
/// * Bit 2 is BLACK_KINGSIDE
/// * Bit 3 is BLACK_QUEENSIDE
pub struct Castling {
    data: u8,
}

const WHITE_KINGSIDE: u8 = 1;
const WHITE_QUEENSIDE: u8 = 1 << 1;
const BLACK_KINGSIDE: u8 = 1 << 2;
const BLACK_QUEENSIDE: u8 = 1 << 3;

impl Castling {
    /// Returns a new Castling struct with all castling bits set
    pub fn new() -> Castling {
        Castling {
            data: WHITE_KINGSIDE | WHITE_QUEENSIDE | BLACK_KINGSIDE | BLACK_QUEENSIDE,
        }
    }

    /// Returns a new Castling struct with the data byte set as specified
    #[inline(always)]
    pub fn from_raw(data: u8) -> Castling {
        Castling { data }
    }

    /// Compares with the given data and returns true if this is set
    #[inline(always)]
    pub fn is_available(&self, data: u8) -> bool {
        (self.data & data) > 0
    }

    /// Removes the bits set in the data byte from the Castling struct
    #[inline(always)]
    pub fn remove(&mut self, data: u8) {
        self.data &= !data;
    }

    /// Returns a byte with the WHITE_KINGSIDE bit set
    #[inline(always)]
    pub fn get_white_kingside() -> u8 {
        WHITE_KINGSIDE
    }

    /// Returns a byte with the WHITE_QUEENSIDE bit set
    #[inline(always)]
    pub fn get_white_queenside() -> u8 {
        WHITE_QUEENSIDE
    }

    /// Returns a byte with the BLACK_KINGSIDE bit set
    #[inline(always)]
    pub fn get_black_kingside() -> u8 {
        BLACK_KINGSIDE
    }

    /// Returns a byte with the BLACK_QUEENSIDE bit set
    #[inline(always)]
    pub fn get_black_queenside() -> u8 {
        BLACK_QUEENSIDE
    }
}

impl Default for Castling {
    fn default() -> Self {
        Castling::new()
    }
}

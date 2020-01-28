pub struct Action {
    from: u8,
    to: u8
}

impl Action {
    pub fn new(from_x: u8, to_x: u8, from_y: u8, to_y: u8) -> Action {
        assert!(from_x < 8);
        assert!(to_x < 8);
        assert!(from_y < 8);
        assert!(to_y < 8);
        return Action {
            from: from_x + (from_y << 3),
            to: to_x + (to_y << 3)
        };
    }

    #[inline(always)]
    pub fn new_raw(from: u8, to: u8) -> Action {
        return Action{from, to};
    }

    #[inline(always)]
    pub fn get_from(&self) -> (u8, u8) {
        return (self.from & 0b111, self.from >> 3);
    }

    #[inline(always)]
    pub fn get_to(&self) -> (u8, u8) {
        return (self.to & 0b111, self.to >> 3);
    }

    #[inline(always)]
    pub fn get_from_raw(&self) -> u8 {
        return self.from;
    }

    #[inline(always)]
    pub fn get_to_raw(&self) -> u8 {
        return self.to;
    }
}
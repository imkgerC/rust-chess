#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub fn get_opponent_color(self) -> Color {
        unsafe { std::mem::transmute(1 - (self as u8)) }
    }
}

/// A basic enum for both colors of the chess players
///
/// Has an internal representation as a single byte with `White = 0` and `Black = 1`
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    /// Returns the opposite Color of the set color
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::Color;
    /// assert_eq!(Color::White.get_opponent_color(), Color::Black);
    /// assert_eq!(Color::Black.get_opponent_color(), Color::White);
    /// ```
    pub fn get_opponent_color(self) -> Color {
        unsafe { std::mem::transmute(1 - (self as u8)) }
    }
}

use super::{Board, Color, Castling, bitboard};

pub struct Game {
    // 50 move rule
    half_move_clock: u8,
    full_move_clock: u32,
    color_to_move: Color,
    board: Board,
    // shift index of en_passant square, if available; 255 otherwise
    en_passant: u8,
    castling: Castling,
}

impl Game {
    pub fn startpos() -> Game {
        return Game {
            half_move_clock: 0,
            full_move_clock: 1,
            color_to_move: Color::White,
            board: Board::startpos(),
            en_passant: 255,
            castling: Castling::new(),
        };
    }

    pub fn to_fen(&self) -> String {
        let mut ret = self.board.to_fen();
        ret.push_str(" ");
        match self.color_to_move {
            Color::White => {
                ret.push_str("w ");
            },
            Color::Black => {
                ret.push_str("b ");
            }
        };

        // castling information
        let mut any_castle = false;
        if self.castling.is_available(Castling::get_white_kingside()) {
            any_castle = true;
            ret.push_str("K");
        }
        if self.castling.is_available(Castling::get_white_queenside()) {
            any_castle = true;
            ret.push_str("Q");
        }
        if self.castling.is_available(Castling::get_black_kingside()) {
            any_castle = true;
            ret.push_str("k");
        }
        if self.castling.is_available(Castling::get_black_queenside()) {
            any_castle = true;
            ret.push_str("q");
        }
        if !any_castle {
            ret.push_str("-");
        }
        ret.push_str(" ");

        // en passant information
        if self.en_passant < 255 {
            ret.push_str(&bitboard::index_to_field_repr(self.en_passant));
            ret.push_str(" ");
        } else {
            ret.push_str("- ");
        }

        ret.push_str(&format!("{} ", self.half_move_clock));
        ret.push_str(&format!("{}", self.full_move_clock));

        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fen_startpos_test() {
        let state = Game::startpos();
        assert_eq!(&state.to_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
}
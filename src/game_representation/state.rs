use super::{Board, Castling, Color, PieceType};
use crate::core::{bitboard, ParserError};
use crate::move_generation::{Action, ActionType};

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
        Game {
            half_move_clock: 0,
            full_move_clock: 1,
            color_to_move: Color::White,
            board: Board::startpos(),
            en_passant: 255,
            castling: Castling::new(),
        }
    }

    pub fn to_fen(&self) -> String {
        let mut ret = self.board.to_fen();
        ret.push_str(" ");
        match self.color_to_move {
            Color::White => {
                ret.push_str("w ");
            }
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
            ret.push_str(
                &bitboard::index_to_field_repr(self.en_passant)
                    .expect("Index is wrong and could not be converted"),
            );
            ret.push_str(" ");
        } else {
            ret.push_str("- ");
        }

        ret.push_str(&format!("{} ", self.half_move_clock));
        ret.push_str(&format!("{}", self.full_move_clock));

        ret
    }

    pub fn execute_action(&mut self, action: &Action) {
        self.board.execute_action(action, self.color_to_move);

        match action.get_action_type() {
            ActionType::Castling(_) => match self.color_to_move {
                Color::White => {
                    self.castling
                        .remove(Castling::get_white_kingside() | Castling::get_white_queenside());
                }
                Color::Black => {
                    self.castling
                        .remove(Castling::get_black_kingside() | Castling::get_black_queenside());
                }
            },
            ActionType::Capture(_) => {
                // reset 50 move rule
                self.half_move_clock = 0;
            }
            _ => {}
        };
        self.en_passant = 255;
        match action.get_piecetype() {
            PieceType::King => {
                match self.color_to_move {
                    Color::White => {
                        self.castling.remove(
                            Castling::get_white_kingside() | Castling::get_white_queenside(),
                        );
                    }
                    Color::Black => {
                        self.castling.remove(
                            Castling::get_black_kingside() | Castling::get_black_queenside(),
                        );
                    }
                };
            }
            PieceType::Pawn => {
                // reset 50 move rule
                self.half_move_clock = 0;
                // set en passant if appropriate
                if i8::abs((action.get_to_index() as i8) - (action.get_from_index() as i8)) == 16 {
                    let color_sign = -(self.color_to_move as i8);
                    self.en_passant = (action.get_from_index() as i8 + (color_sign * 8)) as u8;
                }
            }
            _ => {}
        };

        self.full_move_clock += self.color_to_move as u32;
        self.color_to_move = self.color_to_move.get_opponent_color();
        self.half_move_clock += 1;
    }

    pub fn from_fen(fen: &str) -> Result<Game, ParserError> {
        // parts: 0|board 1|color 2|castling 3|en_passant 4|half_move 5|full_move
        let parts: Vec<&str> = fen.split(' ').collect();
        if parts.len() != 6 {
            return Err(ParserError::WrongParameterNumber);
        }
        let board = Board::from_fen(parts[0])?;

        let color_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(ParserError::InvalidParameter("Color information is wrong")),
        };

        let mut castling = 0;
        let chars: Vec<char> = parts[2].chars().collect();
        if chars[0] == '-' {
            castling = 0;
        } else if chars.len() > 4 {
            return Err(ParserError::WrongParameterNumber);
        } else {
            for c in chars {
                match c {
                    'K' => {
                        castling |= Castling::get_white_kingside();
                    }
                    'Q' => {
                        castling |= Castling::get_white_queenside();
                    }
                    'k' => {
                        castling |= Castling::get_black_kingside();
                    }
                    'q' => {
                        castling |= Castling::get_black_queenside();
                    }
                    _ => {
                        return Err(ParserError::InvalidParameter(
                            "Castling information is wrong",
                        ));
                    }
                }
            }
        }
        let castling = Castling::from_raw(castling);

        let en_passant = if parts[3] == "-" {
            255
        } else {
            bitboard::field_repr_to_index(parts[3])?
        };

        let half_move_clock = if let Ok(x) = parts[4].parse() {
            x
        } else {
            return Err(ParserError::InvalidParameter(
                "Full move clock is not a number",
            ));
        };
        let full_move_clock = if let Ok(x) = parts[5].parse() {
            x
        } else {
            return Err(ParserError::InvalidParameter(
                "Full move clock is not a number",
            ));
        };

        Ok(Game {
            board,
            castling,
            en_passant,
            half_move_clock,
            full_move_clock,
            color_to_move,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fen_startpos_test() {
        let state = Game::startpos();
        assert_eq!(
            &state.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn fen_io_test() {
        assert_eq!(
            Game::from_fen("r4rk1/2qn3p/2p1pb2/2Pp1pp1/p1bPn3/P2N1NP1/2Q1PPBP/BR3RK1 w - g6 0 21")
                .unwrap()
                .to_fen(),
            "r4rk1/2qn3p/2p1pb2/2Pp1pp1/p1bPn3/P2N1NP1/2Q1PPBP/BR3RK1 w - g6 0 21"
        );
        assert_eq!(
            Game::from_fen("r5kr/1pp1Qpp1/p1b1p3/R3P2p/3P4/1PN5/4NP1q/4K1R1 w - h6 0 21")
                .unwrap()
                .to_fen(),
            "r5kr/1pp1Qpp1/p1b1p3/R3P2p/3P4/1PN5/4NP1q/4K1R1 w - h6 0 21"
        );
        assert_eq!(
            Game::from_fen("3r1rk1/1p2qp1p/p1pnb1p1/P2pn3/NP1P4/3BPP2/5QPP/1R2R1K1 w - - 0 21")
                .unwrap()
                .to_fen(),
            "3r1rk1/1p2qp1p/p1pnb1p1/P2pn3/NP1P4/3BPP2/5QPP/1R2R1K1 w - - 0 21"
        );
        assert_eq!(
            Game::from_fen("2r2qk1/r3ppbp/1pbp2p1/p1n5/2P1P3/1PN2P2/P1R1QBPP/3R1BK1 w - - 4 21")
                .unwrap()
                .to_fen(),
            "2r2qk1/r3ppbp/1pbp2p1/p1n5/2P1P3/1PN2P2/P1R1QBPP/3R1BK1 w - - 4 21"
        );
        assert_eq!(
            Game::from_fen("n3k2r/3nppb1/q2p2p1/2pP2P1/1p2PP2/1P2BN2/2P1NK2/3Q1R2 w k - 3 21")
                .unwrap()
                .to_fen(),
            "n3k2r/3nppb1/q2p2p1/2pP2P1/1p2PP2/1P2BN2/2P1NK2/3Q1R2 w k - 3 21"
        );
        assert_eq!(
            Game::from_fen("r2q1rk1/6pp/p7/1p2P1n1/P1pp1p2/1PP2P2/2Q3PP/R1B2RK1 w - - 0 21")
                .unwrap()
                .to_fen(),
            "r2q1rk1/6pp/p7/1p2P1n1/P1pp1p2/1PP2P2/2Q3PP/R1B2RK1 w - - 0 21"
        );
        assert_eq!(
            Game::from_fen("r5k1/2Q2pp1/p1R5/3qp1p1/8/P2P2P1/1r2PP1P/5RK1 w - - 0 21")
                .unwrap()
                .to_fen(),
            "r5k1/2Q2pp1/p1R5/3qp1p1/8/P2P2P1/1r2PP1P/5RK1 w - - 0 21"
        );
        assert_eq!(
            Game::from_fen("r2qrbk1/ppp2ppp/2n1p3/3nP2N/3P2Q1/P4P2/1P4PP/R1B1R2K w - - 2 21")
                .unwrap()
                .to_fen(),
            "r2qrbk1/ppp2ppp/2n1p3/3nP2N/3P2Q1/P4P2/1P4PP/R1B1R2K w - - 2 21"
        );
        assert_eq!(
            Game::from_fen("r1k4r/2q3pp/p3pb2/1p1p4/2n1B3/4B3/PPP1QPPP/R4RK1 w - d6 0 21")
                .unwrap()
                .to_fen(),
            "r1k4r/2q3pp/p3pb2/1p1p4/2n1B3/4B3/PPP1QPPP/R4RK1 w - d6 0 21"
        );
        assert_eq!(
            Game::from_fen("3r4/p1k2p2/1pn1b1p1/4p2p/2P5/B2B1P2/PP4PP/2KR4 w - h6 0 21")
                .unwrap()
                .to_fen(),
            "3r4/p1k2p2/1pn1b1p1/4p2p/2P5/B2B1P2/PP4PP/2KR4 w - h6 0 21"
        );
        assert_eq!(
            Game::from_fen("r3r1k1/pp4bp/q1pBb1p1/2P1p3/4B3/2P3P1/P4P1P/1Q1RR1K1 w - - 1 21")
                .unwrap()
                .to_fen(),
            "r3r1k1/pp4bp/q1pBb1p1/2P1p3/4B3/2P3P1/P4P1P/1Q1RR1K1 w - - 1 21"
        );
        assert_eq!(
            Game::from_fen("r3r1k1/1b2bpp1/p2p3p/1p1Pp3/q1P5/8/BPPN1PPP/R2QR1K1 w - - 0 21")
                .unwrap()
                .to_fen(),
            "r3r1k1/1b2bpp1/p2p3p/1p1Pp3/q1P5/8/BPPN1PPP/R2QR1K1 w - - 0 21"
        );
        assert_eq!(
            Game::from_fen("r2r2k1/2qbbppp/pn1pp3/Bp6/3NP1PP/5P2/PPP5/2KRQB1R w - - 1 21")
                .unwrap()
                .to_fen(),
            "r2r2k1/2qbbppp/pn1pp3/Bp6/3NP1PP/5P2/PPP5/2KRQB1R w - - 1 21"
        );
        assert_eq!(
            Game::from_fen("1k1r3r/1bq2ppp/pp3n2/8/P2N1P2/R1Nn4/1PP2QPP/5R1K w - - 0 21")
                .unwrap()
                .to_fen(),
            "1k1r3r/1bq2ppp/pp3n2/8/P2N1P2/R1Nn4/1PP2QPP/5R1K w - - 0 21"
        );
        assert_eq!(
            Game::from_fen("r6r/qQ3pp1/p2bpk1p/8/8/3B2P1/PP3P1P/3R1RK1 w - - 3 21")
                .unwrap()
                .to_fen(),
            "r6r/qQ3pp1/p2bpk1p/8/8/3B2P1/PP3P1P/3R1RK1 w - - 3 21"
        );
        assert_eq!(
            Game::from_fen("8/pp2rpkp/3R2p1/2r1pb2/8/1BP5/PP3PPP/5RK1 w - - 4 21")
                .unwrap()
                .to_fen(),
            "8/pp2rpkp/3R2p1/2r1pb2/8/1BP5/PP3PPP/5RK1 w - - 4 21"
        );
        assert_eq!(
            Game::from_fen("r6k/1ppq2pn/p2p3p/4pr2/6Q1/1PPPB2P/1P3PP1/R4RK1 w - - 1 21")
                .unwrap()
                .to_fen(),
            "r6k/1ppq2pn/p2p3p/4pr2/6Q1/1PPPB2P/1P3PP1/R4RK1 w - - 1 21"
        );
        assert_eq!(
            Game::from_fen("r1q1n1k1/pb2bppp/4p1n1/2N5/8/P3PNB1/2Q1BPPP/2Rr2K1 w - - 0 21")
                .unwrap()
                .to_fen(),
            "r1q1n1k1/pb2bppp/4p1n1/2N5/8/P3PNB1/2Q1BPPP/2Rr2K1 w - - 0 21"
        );
        assert_eq!(
            Game::from_fen("2r2rk1/1pq2p1p/p1n3p1/2PBp3/Q2pP3/8/PP3PPP/2R2RK1 w - - 2 21")
                .unwrap()
                .to_fen(),
            "2r2rk1/1pq2p1p/p1n3p1/2PBp3/Q2pP3/8/PP3PPP/2R2RK1 w - - 2 21"
        );
        assert_eq!(
            Game::from_fen(
                "1k1r1b1r/ppq2ppp/1n2p3/n2pP2N/2pP1B2/PbP2N2/1P2BPPP/1RQ1R1K1 w - - 8 21"
            )
            .unwrap()
            .to_fen(),
            "1k1r1b1r/ppq2ppp/1n2p3/n2pP2N/2pP1B2/PbP2N2/1P2BPPP/1RQ1R1K1 w - - 8 21"
        );
        assert_eq!(
            Game::from_fen("r1b2rk1/4b1pp/pBpppn2/4q3/4P3/1RN5/P1PQB1PP/5RK1 w - - 12 21")
                .unwrap()
                .to_fen(),
            "r1b2rk1/4b1pp/pBpppn2/4q3/4P3/1RN5/P1PQB1PP/5RK1 w - - 12 21"
        );
        assert_eq!(
            Game::from_fen("r4b1r/4nkpp/1P2q3/p2ppp2/2n2P2/4B3/PPN1Q1PP/1K1R1B1R w - - 1 21")
                .unwrap()
                .to_fen(),
            "r4b1r/4nkpp/1P2q3/p2ppp2/2n2P2/4B3/PPN1Q1PP/1K1R1B1R w - - 1 21"
        );
        assert_eq!(
            Game::from_fen("1r1qr1k1/R2b1p1p/3p1pp1/2pP4/8/2PP2PP/3Q1PBK/5R2 w - - 2 21")
                .unwrap()
                .to_fen(),
            "1r1qr1k1/R2b1p1p/3p1pp1/2pP4/8/2PP2PP/3Q1PBK/5R2 w - - 2 21"
        );
        assert_eq!(
            Game::from_fen("r4rk1/pp2pp1p/6pb/3p4/2nP1nq1/1QP2N2/PP3PPB/R3RBK1 w - - 0 21")
                .unwrap()
                .to_fen(),
            "r4rk1/pp2pp1p/6pb/3p4/2nP1nq1/1QP2N2/PP3PPB/R3RBK1 w - - 0 21"
        );
        assert_eq!(
            Game::from_fen("2rr2k1/p5p1/1pn1pb1p/8/8/2P1B1PP/P4PB1/2RR2K1 w - - 0 21")
                .unwrap()
                .to_fen(),
            "2rr2k1/p5p1/1pn1pb1p/8/8/2P1B1PP/P4PB1/2RR2K1 w - - 0 21"
        );
        assert_eq!(
            Game::from_fen("4r1k1/2qnrppp/1p1p4/pPpP3n/4PB2/P1N4P/2Q2PP1/3RR1K1 w - - 3 21")
                .unwrap()
                .to_fen(),
            "4r1k1/2qnrppp/1p1p4/pPpP3n/4PB2/P1N4P/2Q2PP1/3RR1K1 w - - 3 21"
        );
        assert_eq!(
            Game::from_fen("rr4k1/3bppbp/p4np1/1pp5/4P3/1P1BBP2/P2KN1PP/2R4R w - - 0 21")
                .unwrap()
                .to_fen(),
            "rr4k1/3bppbp/p4np1/1pp5/4P3/1P1BBP2/P2KN1PP/2R4R w - - 0 21"
        );
        assert_eq!(
            Game::from_fen("r6r/ppp1k3/1n2b2b/4p2p/4PppP/5PP1/PPPNB2B/2KR1R2 w - - 1 21")
                .unwrap()
                .to_fen(),
            "r6r/ppp1k3/1n2b2b/4p2p/4PppP/5PP1/PPPNB2B/2KR1R2 w - - 1 21"
        );
        assert_eq!(
            Game::from_fen("3r1rk1/ppq2bpp/B1p2p2/3Pp3/4P3/2N1bP2/1P4PP/R3QR1K w - - 0 21")
                .unwrap()
                .to_fen(),
            "3r1rk1/ppq2bpp/B1p2p2/3Pp3/4P3/2N1bP2/1P4PP/R3QR1K w - - 0 21"
        );
        assert_eq!(
            Game::from_fen(
                "r2qrbk1/1b1n1p2/3p1np1/p1pPp2p/1pP1P3/PP2BN1P/2BQ1PP1/R3RNK1 w - - 0 21"
            )
            .unwrap()
            .to_fen(),
            "r2qrbk1/1b1n1p2/3p1np1/p1pPp2p/1pP1P3/PP2BN1P/2BQ1PP1/R3RNK1 w - - 0 21"
        );
    }
}

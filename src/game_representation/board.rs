use super::{bitboard, Action, PieceType};

pub struct Board {
    bishops: u64,
    rooks: u64,
    knights: u64,
    whites: u64,
    pawns: u64,
    kings: u64,
}

impl Board {
    pub fn startpos() -> Board {
        let pawns = bitboard::from_repr("8/00000000/8/8/8/8/00000000/8");
        let kings = bitboard::from_repr("403/8/8/8/8/8/8/403");
        let rooks = bitboard::from_repr("02030/8/8/8/8/8/8/02030");
        let knights = bitboard::from_repr("10401/8/8/8/8/8/8/10401");
        let bishops = bitboard::from_repr("200102/8/8/8/8/8/8/200102");
        let whites = bitboard::from_repr("8/8/8/8/8/8/00000000/00000000");
        return Board {
            pawns,
            rooks,
            knights,
            kings,
            bishops,
            whites,
        };
    }

    /// returns the board-part of a FEN-string
    pub fn to_fen(&self) -> String {
        let mut res_str = String::new();
        for rank in 0..8 {
            let mut files_skipped = 0;
            for file in 0..8 {
                let piece_on = self.get_piecestr_on(file, rank);
                if piece_on == "" {
                    files_skipped += 1;
                    continue;
                }
                if files_skipped > 0 {
                    res_str.push_str(&format!("{}", files_skipped));
                }
                files_skipped = 0;
                res_str.push_str(piece_on);
            }
            if files_skipped > 0 {
                res_str.push_str(&format!("{}", files_skipped));
            }
            if rank != 7 {
                res_str.push_str("/");
            }
        }
        return res_str;
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut pawns = 0;
        let mut whites = 0;
        let mut knights = 0;
        let mut bishops = 0;
        let mut rooks = 0;
        let mut kings = 0;
        for (rank, rank_str) in fen.split('/').enumerate() {
            let mut file = 0;
            for c in rank_str.chars() {
                let shift = file + rank * 8;
                if shift > 63 {
                    panic!(format!(
                        "shift is too high with file {} and rank {} fen {}",
                        file, rank, fen
                    ));
                }
                match c {
                    'p' => {
                        pawns |= 0b1 << shift;
                        file += 1;
                    }
                    'r' => {
                        rooks |= 0b1 << shift;
                        file += 1;
                    }
                    'b' => {
                        bishops |= 0b1 << shift;
                        file += 1;
                    }
                    'n' => {
                        knights |= 0b1 << shift;
                        file += 1;
                    }
                    'k' => {
                        kings |= 0b1 << shift;
                        file += 1;
                    }
                    'q' => {
                        bishops |= 0b1 << shift;
                        rooks |= 0b1 << shift;
                        file += 1;
                    }
                    'P' => {
                        pawns |= 0b1 << shift;
                        whites |= 0b1 << shift;
                        file += 1;
                    }
                    'R' => {
                        rooks |= 0b1 << shift;
                        whites |= 0b1 << shift;
                        file += 1;
                    }
                    'B' => {
                        bishops |= 0b1 << shift;
                        whites |= 0b1 << shift;
                        file += 1;
                    }
                    'N' => {
                        knights |= 0b1 << shift;
                        whites |= 0b1 << shift;
                        file += 1;
                    }
                    'K' => {
                        kings |= 0b1 << shift;
                        whites |= 0b1 << shift;
                        file += 1;
                    }
                    'Q' => {
                        bishops |= 0b1 << shift;
                        rooks |= 0b1 << shift;
                        whites |= 0b1 << shift;
                        file += 1;
                    }
                    '1' => {
                        file += 1;
                    }
                    '2' => {
                        file += 2;
                    }
                    '3' => {
                        file += 3;
                    }
                    '4' => {
                        file += 4;
                    }
                    '5' => {
                        file += 5;
                    }
                    '6' => {
                        file += 6;
                    }
                    '7' => {
                        file += 7;
                    }
                    '8' => {
                        file += 8;
                    }
                    _ => {
                        panic!("Illegal character in board fen");
                    }
                }
            }
        }
        return Board {
            pawns,
            rooks,
            knights,
            kings,
            bishops,
            whites,
        };
    }

    fn get_piecestr_on(&self, file: u8, rank: u8) -> &str {
        return self.get_piecestr_at(rank * 8 + file);
    }

    fn get_piecestr_at(&self, shift: u8) -> &str {
        if self.pawns >> shift & 1 == 1 {
            if self.whites >> shift & 1 == 1 {
                return "P";
            }
            return "p";
        }
        if self.knights >> shift & 1 == 1 {
            if self.whites >> shift & 1 == 1 {
                return "N";
            }
            return "n";
        }
        if self.kings >> shift & 1 == 1 {
            if self.whites >> shift & 1 == 1 {
                return "K";
            }
            return "k";
        }
        if self.bishops >> shift & 1 == 1 {
            if self.rooks >> shift & 1 == 1 {
                if self.whites >> shift & 1 == 1 {
                    return "Q";
                }
                return "q";
            }
            if self.whites >> shift & 1 == 1 {
                return "B";
            }
            return "b";
        }
        if self.rooks >> shift & 1 == 1 {
            if self.whites >> shift & 1 == 1 {
                return "R";
            }
            return "r";
        }
        if self.kings >> shift & 1 == 1 {
            if self.whites >> shift & 1 == 1 {
                return "K";
            }
            return "k";
        }
        return "";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fen_io_test() {
        assert_eq!(
            "r6r/1b2k1bq/8/8/7B/8/8/R3K2R",
            &Board::from_fen("r6r/1b2k1bq/8/8/7B/8/8/R3K2R").to_fen()
        );
        assert_eq!(
            "8/8/8/2k5/2pP4/8/B7/4K3",
            &Board::from_fen("8/8/8/2k5/2pP4/8/B7/4K3").to_fen()
        );
        assert_eq!(
            "r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR",
            &Board::from_fen("r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR").to_fen()
        );
        assert_eq!(
            "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R",
            &Board::from_fen("r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R").to_fen()
        );
        assert_eq!(
            "2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R",
            &Board::from_fen("2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R").to_fen()
        );
        assert_eq!(
            "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R",
            &Board::from_fen("rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R").to_fen()
        );
        assert_eq!(
            "2r5/3pk3/8/2P5/8/2K5/8/8",
            &Board::from_fen("2r5/3pk3/8/2P5/8/2K5/8/8").to_fen()
        );
        assert_eq!(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R",
            &Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R").to_fen()
        );
        assert_eq!(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1",
            &Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1")
                .to_fen()
        );
        assert_eq!(
            "3k4/3p4/8/K1P4r/8/8/8/8",
            &Board::from_fen("3k4/3p4/8/K1P4r/8/8/8/8").to_fen()
        );
        assert_eq!(
            "8/8/4k3/8/2p5/8/B2P2K1/8",
            &Board::from_fen("8/8/4k3/8/2p5/8/B2P2K1/8").to_fen()
        );
        assert_eq!(
            "8/8/1k6/2b5/2pP4/8/5K2/8",
            &Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8").to_fen()
        );
        assert_eq!(
            "5k2/8/8/8/8/8/8/4K2R",
            &Board::from_fen("5k2/8/8/8/8/8/8/4K2R").to_fen()
        );
        assert_eq!(
            "3k4/8/8/8/8/8/8/R3K3",
            &Board::from_fen("3k4/8/8/8/8/8/8/R3K3").to_fen()
        );
        assert_eq!(
            "r3k2r/1b4bq/8/8/8/8/7B/R3K2R",
            &Board::from_fen("r3k2r/1b4bq/8/8/8/8/7B/R3K2R").to_fen()
        );
        assert_eq!(
            "r3k2r/8/3Q4/8/8/5q2/8/R3K2R",
            &Board::from_fen("r3k2r/8/3Q4/8/8/5q2/8/R3K2R").to_fen()
        );
        assert_eq!(
            "2K2r2/4P3/8/8/8/8/8/3k4",
            &Board::from_fen("2K2r2/4P3/8/8/8/8/8/3k4").to_fen()
        );
        assert_eq!(
            "8/8/1P2K3/8/2n5/1q6/8/5k2",
            &Board::from_fen("8/8/1P2K3/8/2n5/1q6/8/5k2").to_fen()
        );
        assert_eq!(
            "4k3/1P6/8/8/8/8/K7/8",
            &Board::from_fen("4k3/1P6/8/8/8/8/K7/8").to_fen()
        );
        assert_eq!(
            "8/P1k5/K7/8/8/8/8/8",
            &Board::from_fen("8/P1k5/K7/8/8/8/8/8").to_fen()
        );
        assert_eq!(
            "K1k5/8/P7/8/8/8/8/8",
            &Board::from_fen("K1k5/8/P7/8/8/8/8/8").to_fen()
        );
        assert_eq!(
            "8/k1P5/8/1K6/8/8/8/8",
            &Board::from_fen("8/k1P5/8/1K6/8/8/8/8").to_fen()
        );
        assert_eq!(
            "8/8/2k5/5q2/5n2/8/5K2/8",
            &Board::from_fen("8/8/2k5/5q2/5n2/8/5K2/8").to_fen()
        );
    }

    #[test]
    fn fen_startpos() {
        assert_eq!(&Board::startpos().to_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }
}

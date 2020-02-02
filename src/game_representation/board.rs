use super::{bitboard, Action, Color, ParserError, PieceType};

/// The board part of a chess game state
///
/// This is a simple minimal [bitboard](https://www.chessprogramming.org/Bitboards) implementation of a chess board.
/// Its' internal structure is defined by only six bitboards, one for every piece type except queens
/// and one for color. Queens are represented as a set bit on the bishop and rook bitboard.
///
/// The mapping of chess fields to shifts in the bitboard and to (x, y) position is shown in the following graphic.
/// ```text
///      a  b  c  d  e  f  g  h        a   b   c   d   e   f   g   h
///   +-------------------------+   +---------------------------------+
/// 8 |  0  1  2  3  4  5  6  7 | 8 | 0,0 1,0 2,0 3,0 4,0 5,0 6,0 7,0 | 8
/// 7 |  8  9 10 11 12 13 14 15 | 7 | 0,1 1,1 2,1 3,1 4,1 5,1 6,1 7,1 | 7
/// 6 | 16 17 18 19 20 21 22 23 | 6 | 0,2 1,2 2,2 3,2 4,2 5,2 6,2 7,2 | 6
/// 5 | 24 25 26 27 28 29 30 31 | 5 | 0,3 1,3 2,3 3,3 4,3 5,3 6,3 7,3 | 5
/// 4 | 32 33 34 35 36 37 38 39 | 4 | 0,4 1,4 2,4 3,4 4,4 5,4 6,4 7,4 | 4
/// 3 | 40 41 42 43 44 45 46 47 | 3 | 0,5 1,5 2,5 3,5 4,5 5,5 6,5 7,5 | 3
/// 2 | 48 49 50 51 52 53 54 55 | 2 | 0,6 1,6 2,6 3,6 4,6 5,6 6,6 7,6 | 2
/// 1 | 56 57 58 59 60 61 62 63 | 1 | 0,7 1,7 2,7 3,7 4,7 5,7 6,7 7,7 | 1
///   +-------------------------+   +---------------------------------+
///      a  b  c  d  e  f  g  h        a   b   c   d   e   f   g   h
/// ```
pub struct Board {
    bishops: u64,
    rooks: u64,
    knights: u64,
    whites: u64,
    pawns: u64,
    kings: u64,
}

impl Board {
    /// Returns a board initialized with the standard chess starting position
    /// # Examples
    /// ```
    /// # use core::game_representation::Board;
    /// assert_eq!(&Board::startpos().to_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    /// ```
    pub fn startpos() -> Board {
        let pawns = bitboard::from_repr("8/00000000/8/8/8/8/00000000/8")
            .expect("Error in parsing pawn position");
        let kings =
            bitboard::from_repr("403/8/8/8/8/8/8/403").expect("Error in parsing king position");
        let rooks =
            bitboard::from_repr("02030/8/8/8/8/8/8/02030").expect("Error in parsing rook position");
        let knights = bitboard::from_repr("10401/8/8/8/8/8/8/10401")
            .expect("Error in parsing knight position");
        let bishops = bitboard::from_repr("200102/8/8/8/8/8/8/200102")
            .expect("Error in parsing bishop position");
        let whites = bitboard::from_repr("8/8/8/8/8/8/00000000/00000000")
            .expect("Error in parsing white position");
        Board {
            pawns,
            rooks,
            knights,
            kings,
            bishops,
            whites,
        }
    }

    /// This method will execute any action on the board, using only the information inside
    /// the action. It will not check, if this move is legal in any way: USE WITH CAUTION.
    /// There are not tests to look if a particular field even has the needed piece, if it does not,
    /// this piece will be created from thin air.
    /// There is no checking if a check occurs through this action or king is captured or a king is even
    /// on the board.
    ///
    /// Currently does not support promotions or castling.
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::{Board, Color, PieceType, Action};
    /// let mut b = Board::startpos();
    /// let a = Action::new(4, 6, 4, 4, PieceType::Pawn, Color::White); // this is e2e4
    /// b.execute_action(&a);
    /// assert_eq!("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR", &b.to_fen());
    /// ```
    pub fn execute_action(&mut self, action: &Action) {
        // assumes action is legal
        let (from_x, from_y) = action.get_from();
        let (to_x, to_y) = action.get_to();
        let shift_from = from_x + from_y * 8;
        let shift_to = to_x + to_y * 8;
        let not_from_bit = !(1 << shift_from);
        let not_to_bit = !(1 << shift_to);
        let color = action.get_color();
        let piecetype = action.get_piecetype();

        let pawn_to_bit = ((piecetype == PieceType::Pawn) as u64) << shift_to;
        let knight_to_bit = ((piecetype == PieceType::Knight) as u64) << shift_to;
        let king_to_bit = ((piecetype == PieceType::King) as u64) << shift_to;
        let white_to_bit = ((color == Color::White) as u64) << shift_to;
        let bishop_to_bit =
            ((piecetype == PieceType::Bishop || piecetype == PieceType::Queen) as u64) << shift_to;
        let rook_to_bit =
            ((piecetype == PieceType::Rook || piecetype == PieceType::Queen) as u64) << shift_to;

        // just unset everywhere, so we don't need complex logic
        self.rooks &= not_from_bit;
        self.pawns &= not_from_bit;
        self.kings &= not_from_bit;
        self.bishops &= not_from_bit;
        self.knights &= not_from_bit;
        self.whites &= not_from_bit;

        // just unset everywhere, so we don't need complex logic
        self.rooks &= not_to_bit;
        self.pawns &= not_to_bit;
        self.kings &= not_to_bit;
        self.bishops &= not_to_bit;
        self.knights &= not_to_bit;
        self.whites &= not_to_bit;

        // set with bit to avoid branching
        self.kings |= king_to_bit;
        self.pawns |= pawn_to_bit;
        self.knights |= knight_to_bit;
        self.whites |= white_to_bit;
        self.rooks |= rook_to_bit;
        self.bishops |= bishop_to_bit;
    }

    /// Returns the board-part of a FEN-string
    ///
    /// For examples see [`execute_action`]
    ///
    /// [`execute_action`]: #method.execute_action
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
        res_str
    }

    /// Constructs a new Board from only the board-part of a FEN
    ///
    /// # Examples
    /// ```
    /// # use core::game_representation::Board;
    /// let b = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
    /// assert_eq!(&b.to_fen(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    /// ```
    pub fn from_fen(fen: &str) -> Result<Board, ParserError> {
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
        Ok(Board {
            pawns,
            rooks,
            knights,
            kings,
            bishops,
            whites,
        })
    }

    fn get_piecestr_on(&self, file: u8, rank: u8) -> &str {
        self.get_piecestr_at(rank * 8 + file)
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
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wikipedia_fen_opening_test() {
        // moves and fens taken from wikipedia [https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation]
        let mut b = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        let a = Action::new(4, 6, 4, 4, PieceType::Pawn, Color::White);
        b.execute_action(&a);
        assert_eq!("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR", &b.to_fen());
        let a = Action::new(2, 1, 2, 3, PieceType::Pawn, Color::Black);
        b.execute_action(&a);
        assert_eq!(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR",
            &b.to_fen()
        );
        let a = Action::new(6, 7, 5, 5, PieceType::Knight, Color::White);
        b.execute_action(&a);
        assert_eq!(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R",
            &b.to_fen()
        );
    }

    #[test]
    fn fen_io_test() {
        assert_eq!(
            "r6r/1b2k1bq/8/8/7B/8/8/R3K2R",
            &Board::from_fen("r6r/1b2k1bq/8/8/7B/8/8/R3K2R")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "8/8/8/2k5/2pP4/8/B7/4K3",
            &Board::from_fen("8/8/8/2k5/2pP4/8/B7/4K3").unwrap().to_fen()
        );
        assert_eq!(
            "r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR",
            &Board::from_fen("r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R",
            &Board::from_fen("r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R",
            &Board::from_fen("2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R",
            &Board::from_fen("rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "2r5/3pk3/8/2P5/8/2K5/8/8",
            &Board::from_fen("2r5/3pk3/8/2P5/8/2K5/8/8")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R",
            &Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1",
            &Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "3k4/3p4/8/K1P4r/8/8/8/8",
            &Board::from_fen("3k4/3p4/8/K1P4r/8/8/8/8").unwrap().to_fen()
        );
        assert_eq!(
            "8/8/4k3/8/2p5/8/B2P2K1/8",
            &Board::from_fen("8/8/4k3/8/2p5/8/B2P2K1/8")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "8/8/1k6/2b5/2pP4/8/5K2/8",
            &Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "5k2/8/8/8/8/8/8/4K2R",
            &Board::from_fen("5k2/8/8/8/8/8/8/4K2R").unwrap().to_fen()
        );
        assert_eq!(
            "3k4/8/8/8/8/8/8/R3K3",
            &Board::from_fen("3k4/8/8/8/8/8/8/R3K3").unwrap().to_fen()
        );
        assert_eq!(
            "r3k2r/1b4bq/8/8/8/8/7B/R3K2R",
            &Board::from_fen("r3k2r/1b4bq/8/8/8/8/7B/R3K2R")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "r3k2r/8/3Q4/8/8/5q2/8/R3K2R",
            &Board::from_fen("r3k2r/8/3Q4/8/8/5q2/8/R3K2R")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "2K2r2/4P3/8/8/8/8/8/3k4",
            &Board::from_fen("2K2r2/4P3/8/8/8/8/8/3k4").unwrap().to_fen()
        );
        assert_eq!(
            "8/8/1P2K3/8/2n5/1q6/8/5k2",
            &Board::from_fen("8/8/1P2K3/8/2n5/1q6/8/5k2")
                .unwrap()
                .to_fen()
        );
        assert_eq!(
            "4k3/1P6/8/8/8/8/K7/8",
            &Board::from_fen("4k3/1P6/8/8/8/8/K7/8").unwrap().to_fen()
        );
        assert_eq!(
            "8/P1k5/K7/8/8/8/8/8",
            &Board::from_fen("8/P1k5/K7/8/8/8/8/8").unwrap().to_fen()
        );
        assert_eq!(
            "K1k5/8/P7/8/8/8/8/8",
            &Board::from_fen("K1k5/8/P7/8/8/8/8/8").unwrap().to_fen()
        );
        assert_eq!(
            "8/k1P5/8/1K6/8/8/8/8",
            &Board::from_fen("8/k1P5/8/1K6/8/8/8/8").unwrap().to_fen()
        );
        assert_eq!(
            "8/8/2k5/5q2/5n2/8/5K2/8",
            &Board::from_fen("8/8/2k5/5q2/5n2/8/5K2/8").unwrap().to_fen()
        );
    }

    #[test]
    fn fen_startpos() {
        assert_eq!(
            &Board::startpos().to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }
}

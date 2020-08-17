//! Working with bitboards more ergonomically
//!
//! This module contains helper functions and constants that are imporatant
//! for working with bitboards without going insane.

use super::ParserError;
use crate::game_representation::PieceType;

pub mod constants {
    //! This module contains all constants for working with bitboards

    /// Bitboard of ones for a given rank, index is rank - 1
    pub const RANKS: [u64; 8] = [
        18374686479671623680,
        71776119061217280,
        280375465082880,
        1095216660480,
        4278190080,
        16711680,
        65280,
        255,
    ];

    /// Bitboard of ones for a given file, index is a = 0, b = 1, ...
    pub const FILES: [u64; 8] = [
        72340172838076673,
        144680345676153346,
        289360691352306692,
        578721382704613384,
        1157442765409226768,
        2314885530818453536,
        4629771061636907072,
        9259542123273814144,
    ];

    pub const KNIGHT_MASKS: [u64; 64] = [
        132096,
        329728,
        659712,
        1319424,
        2638848,
        5277696,
        10489856,
        4202496,
        33816580,
        84410376,
        168886289,
        337772578,
        675545156,
        1351090312,
        2685403152,
        1075839008,
        8657044482,
        21609056261,
        43234889994,
        86469779988,
        172939559976,
        345879119952,
        687463207072,
        275414786112,
        2216203387392,
        5531918402816,
        11068131838464,
        22136263676928,
        44272527353856,
        88545054707712,
        175990581010432,
        70506185244672,
        567348067172352,
        1416171111120896,
        2833441750646784,
        5666883501293568,
        11333767002587136,
        22667534005174272,
        45053588738670592,
        18049583422636032,
        145241105196122112,
        362539804446949376,
        725361088165576704,
        1450722176331153408,
        2901444352662306816,
        5802888705324613632,
        11533718717099671552,
        4620693356194824192,
        288234782788157440,
        576469569871282176,
        1224997833292120064,
        2449995666584240128,
        4899991333168480256,
        9799982666336960512,
        1152939783987658752,
        2305878468463689728,
        1128098930098176,
        2257297371824128,
        4796069720358912,
        9592139440717824,
        19184278881435648,
        38368557762871296,
        4679521487814656,
        9077567998918656,
    ];
}

/// Returns a bitboard from a simple fen-like representation
///
/// The representation needs to contain exactly 8 ranks, each delimited
/// by a single `'/'`. Each rank needs to have no more than 8 fields, it should be exactly 8
/// but will not error if it is less. A `0` depicts a set bit at that field, any other number
/// is interpreted as a series of non-set bits in the bitboard. Any character other than `/` or any digit smaller
/// than or equal to `8` is an invalid character.
///
/// # Errors
/// * A rank is overfull, e.g. `070/8/8/8/8/8/8/8`
/// * There are too many ranks, e.g. `8/8/8/8/8/8/8/8/8`
/// * Not enough ranks, e.g. `8/8/8/8/8/8/8`
/// * Invalid characters in the representation, e.g. `8/8/8/8/8/8/8/7p`
///
/// # Examples
/// ```
/// use core::core::bitboard;
///
/// // creates bitboard with 2 set bits in the second to top rank
/// assert_eq!(bitboard::from_repr("8/0303/8/8/8/8/8/8").unwrap(), 4352);
/// ```
pub fn from_repr(repr: &str) -> Result<u64, &str> {
    let ranks: Vec<&str> = repr.split('/').collect();
    if ranks.len() != 8 {
        return Err("Incorrect number of ranks");
    }
    let mut ret = 0u64;
    for (rank_idx, rank) in ranks.iter().enumerate() {
        let mut file = 0;
        for c in rank.chars() {
            if file > 7 {
                return Err("Rank is overfull in repr");
            }
            match c {
                '0' => {
                    let index = rank_idx * 8 + file;
                    ret |= 0b1 << index;
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
                    return Err("incorrect character in representation string");
                }
            }
        }
    }
    Ok(ret)
}

/// Returns the string representation for the given field index
///
/// The index is the shift by which you need to shift a 1 value to have a bitboard with only that field set.
/// The index for any field can be seen on the following map:
/// ```text
///      a  b  c  d  e  f  g  h   
///   +-------------------------+
/// 8 |  0  1  2  3  4  5  6  7 | 8
/// 7 |  8  9 10 11 12 13 14 15 | 7
/// 6 | 16 17 18 19 20 21 22 23 | 6
/// 5 | 24 25 26 27 28 29 30 31 | 5
/// 4 | 32 33 34 35 36 37 38 39 | 4
/// 3 | 40 41 42 43 44 45 46 47 | 3
/// 2 | 48 49 50 51 52 53 54 55 | 2
/// 1 | 56 57 58 59 60 61 62 63 | 1
///   +-------------------------+
///      a  b  c  d  e  f  g  h   
/// ```
///
/// # Errors
/// * when the index is bigger than 63
pub fn index_to_field_repr(index: u8) -> Result<String, ParserError> {
    let mut ret = String::new();
    let file = index % 8;
    let rank = index / 8;
    ret.push_str(file_to_str(file)?);
    ret.push_str(rank_to_str(rank)?);
    Ok(ret)
}

/// Moves all pieces on the bitboard north by the amount
///
/// Pieces will be happily shifted away if shifted of the board
/// Is useless if the amount is 8 or greater, might lead to undefined behaviour in the future
#[inline(always)]
pub const fn bitboard_north(board: u64, amount: u8) -> u64 {
    board >> (8 * amount)
}

/// Moves all pieces on the bitboard south by the amount
///
/// Pieces will be happily shifted away if shifted of the board
/// Is useless if the amount is 8 or greater, might lead to undefined behaviour in the future
#[inline(always)]
pub const fn bitboard_south(board: u64, amount: u8) -> u64 {
    board << (8 * amount)
}

/// Moves all pieces on the bitboard east by one
///
/// Overflow is cared for, pieces will be shifted away if shifted over the border
#[inline(always)]
pub const fn bitboard_east_one(board: u64) -> u64 {
    (board & !constants::FILES[7]) << 1
}

/// Moves all pieces on the bitboard east by one
///
/// Overflow is cared for, pieces will be shifted away if shifted over the border
#[inline(always)]
pub const fn bitboard_west_one(board: u64) -> u64 {
    (board & !constants::FILES[0]) >> 1
}

/// Returns the field index for the given string representation
///
/// The index is the shift by which you need to shift a 1 value to have a bitboard with only that field set.
/// The index for any field can be seen on the following map:
/// ```text
///      a  b  c  d  e  f  g  h   
///   +-------------------------+
/// 8 |  0  1  2  3  4  5  6  7 | 8
/// 7 |  8  9 10 11 12 13 14 15 | 7
/// 6 | 16 17 18 19 20 21 22 23 | 6
/// 5 | 24 25 26 27 28 29 30 31 | 5
/// 4 | 32 33 34 35 36 37 38 39 | 4
/// 3 | 40 41 42 43 44 45 46 47 | 3
/// 2 | 48 49 50 51 52 53 54 55 | 2
/// 1 | 56 57 58 59 60 61 62 63 | 1
///   +-------------------------+
///      a  b  c  d  e  f  g  h   
/// ```
///
/// # Errors
/// * if the repr string is not length 2
/// * if the first character is not a number from the range 1-8
/// * if the second character is not a letter from the range a-h
pub fn field_repr_to_index(repr: &str) -> Result<u8, ParserError> {
    let chars: Vec<char> = repr.chars().collect();
    if chars.len() != 2 {
        return Err(ParserError::WrongParameterNumber);
    }
    let index = str_to_rank(&chars[1].to_string())? * 8 + str_to_file(chars[0])?;
    Ok(index)
}

/// Parses a san field representation to the corresponding coordinates
pub fn field_repr_to_coords(repr: &str) -> Result<(u8, u8), ParserError> {
    index_to_coords(field_repr_to_index(repr)?)
}

/// Parses a field index and returns the coordinates
pub fn index_to_coords(index: u8) -> Result<(u8, u8), ParserError> {
    if index > 63 {
        return Err(ParserError::InvalidParameter("index too high"));
    }
    Ok((index % 8, index / 8))
}

/// Returns the file number for the given file character
///
/// * 'a' -> 0
/// * 'b' -> 1
/// * 'c' -> 2
/// * ...
///
/// # Errors
/// * if the input character is not in the range 'a'-'h'
pub fn str_to_file(file: char) -> Result<u8, ParserError> {
    match file {
        'a' => Ok(0),
        'b' => Ok(1),
        'c' => Ok(2),
        'd' => Ok(3),
        'e' => Ok(4),
        'f' => Ok(5),
        'g' => Ok(6),
        'h' => Ok(7),
        _ => Err(ParserError::InvalidParameter(
            "File provided is unknown/invalid",
        )),
    }
}

/// Returns the file string for the given file number
///
/// * 0 -> "a"
/// * 1 -> "b"
/// * 2 -> "c"
/// * ...
///
/// # Errors
/// * if the input number is not in the range 0-7
pub fn file_to_str(file: u8) -> Result<&'static str, ParserError> {
    match file {
        0 => Ok("a"),
        1 => Ok("b"),
        2 => Ok("c"),
        3 => Ok("d"),
        4 => Ok("e"),
        5 => Ok("f"),
        6 => Ok("g"),
        7 => Ok("h"),
        _ => Err(ParserError::InvalidParameter("File is too big")),
    }
}

/// Returns the rank number for the given rank string
///
/// * "8" -> 0
/// * "7" -> 1
/// * "6" -> 2
/// * ...
///
/// # Errors
/// * if the input string is not in the range "1"-"8"
pub fn str_to_rank(rank: &str) -> Result<u8, ParserError> {
    let rank: u8 = if let Ok(rank) = rank.parse() {
        rank
    } else {
        return Err(ParserError::InvalidParameter(
            "Rank provided is not a number",
        ));
    };
    if !(rank <= 8 && rank > 0) {
        return Err(ParserError::InvalidParameter("Rank is out of bounds"));
    }
    Ok(8 - rank)
}

/// Returns the rank string for the given rank number
///
/// * 0 -> "8"
/// * 1 -> "7"
/// * 2 -> "6"
/// * ...
///
/// # Errors
/// * if the input number is not in the range 0-7
pub fn rank_to_str(rank: u8) -> Result<&'static str, ParserError> {
    match rank {
        0 => Ok("8"),
        1 => Ok("7"),
        2 => Ok("6"),
        3 => Ok("5"),
        4 => Ok("4"),
        5 => Ok("3"),
        6 => Ok("2"),
        7 => Ok("1"),
        _ => Err(ParserError::InvalidParameter("Rank is out of bounds")),
    }
}

/// Returns the Piecetype for a given uppercase char
/// # Errors
/// * if the input is not one of KNBQR
/// # Examples
/// ```
/// # use core::core::bitboard;
/// # use core::game_representation::PieceType;
/// assert_eq!(bitboard::char_to_piecetype('Q').unwrap(), PieceType::Queen);
/// ```
pub fn char_to_piecetype(c: char) -> Result<PieceType, ParserError> {
    match c {
        'K' => Ok(PieceType::King),
        'N' => Ok(PieceType::Knight),
        'B' => Ok(PieceType::Bishop),
        'Q' => Ok(PieceType::Queen),
        'R' => Ok(PieceType::Rook),
        _ => Err(ParserError::InvalidParameter("Piecetype is invalid")),
    }
}

/// Returns the san char for a given Piecetype
/// # Examples
/// ```
/// # use core::core::bitboard;
/// # use core::game_representation::PieceType;
/// assert_eq!(bitboard::piecetype_to_char(PieceType::Queen), 'Q');
/// ```
pub fn piecetype_to_char(piece: PieceType) -> char {
    match piece {
        PieceType::King => 'K',
        PieceType::Knight => 'N',
        PieceType::Bishop => 'B',
        PieceType::Queen => 'Q',
        PieceType::Rook => 'R',
        PieceType::Pawn => ' ',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitboard_shifts() {
        let initial = 1 << field_repr_to_index("e2").unwrap();
        assert_eq!(
            index_to_field_repr(bitboard_north(initial, 3).trailing_zeros() as u8).unwrap(),
            "e5"
        );
        assert_eq!(bitboard_south(initial, 3), 0);
        assert_eq!(bitboard_north(initial, 7), 0);
        assert_eq!(
            index_to_field_repr(bitboard_south(initial, 1).trailing_zeros() as u8).unwrap(),
            "e1"
        );
        assert_eq!(
            index_to_field_repr(bitboard_east_one(initial).trailing_zeros() as u8).unwrap(),
            "f2"
        );
    }

    #[test]
    fn parsing_repr() {
        assert_eq!(from_repr("8/0303/8/8/8/8/8/8").unwrap(), 4352);
        assert_eq!(from_repr("8/8/8/8/8/8/8/8").unwrap(), 0);
        assert_eq!(
            from_repr("8/00000000/8/8/8/8/00000000/8").unwrap(),
            71776119061282560
        );
        assert_eq!(
            from_repr("403/8/8/8/8/8/8/403").unwrap(),
            1152921504606846992
        );
        assert_eq!(
            from_repr("060/8/8/8/8/8/8/060").unwrap(),
            9295429630892703873
        );
        assert_eq!(
            from_repr("10401/8/8/8/8/8/8/10401").unwrap(),
            4755801206503243842
        );
        assert_eq!(
            from_repr("20202/8/8/8/8/8/8/20202").unwrap(),
            2594073385365405732
        );
        assert_eq!(
            from_repr("8/8/8/8/8/8/00000000/00000000").unwrap(),
            18446462598732840960
        );
    }

    #[test]
    #[should_panic]
    fn repr_rank_too_long() {
        from_repr("81/8/8/8/8/8/8/8").unwrap();
    }

    #[test]
    #[should_panic]
    fn repr_invalid_character() {
        from_repr("9/8/8/8/8/8/8/8").unwrap();
    }

    #[test]
    #[should_panic]
    fn repr_not_enough_ranks() {
        from_repr("8/8/8/8/8/8/8").unwrap();
    }

    #[test]
    #[should_panic]
    fn repr_too_many_ranks() {
        from_repr("8/8/8/8/8/8/8/8/8/8/8").unwrap();
    }

    #[test]
    fn file_or_rank_out_of_bounds() {
        for x in 8..=255 {
            if let Ok(_) = file_to_str(x) {
                panic!("should not work");
            }
            if let Ok(_) = rank_to_str(x) {
                panic!("should not work");
            }
        }
    }

    #[test]
    fn file_or_rank_in_bounds() {
        for x in 0..8 {
            if let Err(_) = file_to_str(x) {
                panic!("should not work");
            }
            if let Err(_) = rank_to_str(x) {
                panic!("should not work");
            }
        }
    }

    #[test]
    fn repr_file_rank_mapping() {
        // rank to str
        assert_eq!(rank_to_str(0).unwrap(), "8");
        assert_eq!(rank_to_str(1).unwrap(), "7");
        assert_eq!(rank_to_str(2).unwrap(), "6");
        assert_eq!(rank_to_str(3).unwrap(), "5");
        assert_eq!(rank_to_str(4).unwrap(), "4");
        assert_eq!(rank_to_str(5).unwrap(), "3");
        assert_eq!(rank_to_str(6).unwrap(), "2");
        assert_eq!(rank_to_str(7).unwrap(), "1");

        // str to rank
        assert_eq!(str_to_rank("8").unwrap(), 0);
        assert_eq!(str_to_rank("7").unwrap(), 1);
        assert_eq!(str_to_rank("6").unwrap(), 2);
        assert_eq!(str_to_rank("5").unwrap(), 3);
        assert_eq!(str_to_rank("4").unwrap(), 4);
        assert_eq!(str_to_rank("3").unwrap(), 5);
        assert_eq!(str_to_rank("2").unwrap(), 6);
        assert_eq!(str_to_rank("1").unwrap(), 7);

        // file to str
        assert_eq!(file_to_str(0).unwrap(), "a");
        assert_eq!(file_to_str(1).unwrap(), "b");
        assert_eq!(file_to_str(2).unwrap(), "c");
        assert_eq!(file_to_str(3).unwrap(), "d");
        assert_eq!(file_to_str(4).unwrap(), "e");
        assert_eq!(file_to_str(5).unwrap(), "f");
        assert_eq!(file_to_str(6).unwrap(), "g");
        assert_eq!(file_to_str(7).unwrap(), "h");

        // str to file
        assert_eq!(str_to_file('a').unwrap(), 0);
        assert_eq!(str_to_file('b').unwrap(), 1);
        assert_eq!(str_to_file('c').unwrap(), 2);
        assert_eq!(str_to_file('d').unwrap(), 3);
        assert_eq!(str_to_file('e').unwrap(), 4);
        assert_eq!(str_to_file('f').unwrap(), 5);
        assert_eq!(str_to_file('g').unwrap(), 6);
        assert_eq!(str_to_file('h').unwrap(), 7);
    }

    #[test]
    fn field_repr_in_bounds() {
        for index in 0..64 {
            if let Err(_) = index_to_field_repr(index) {
                panic!("should not work");
            }
        }
    }

    #[test]
    fn field_repr_out_of_bounds() {
        for index in 64..=255 {
            if let Ok(_) = index_to_field_repr(index) {
                panic!("should not work");
            }
        }
    }

    #[test]
    fn field_repr_mapping() {
        assert_eq!(index_to_field_repr(0).unwrap(), "a8");
        assert_eq!(index_to_field_repr(63).unwrap(), "h1");
        assert_eq!(index_to_field_repr(32).unwrap(), "a4");
        assert_eq!(index_to_field_repr(26).unwrap(), "c5");
        assert_eq!(index_to_field_repr(58).unwrap(), "c1");
        assert_eq!(index_to_field_repr(48).unwrap(), "a2");
        assert_eq!(index_to_field_repr(13).unwrap(), "f7");

        assert_eq!(field_repr_to_index("a8").unwrap(), 0);
        assert_eq!(field_repr_to_index("h1").unwrap(), 63);
        assert_eq!(field_repr_to_index("a4").unwrap(), 32);
        assert_eq!(field_repr_to_index("c5").unwrap(), 26);
        assert_eq!(field_repr_to_index("c1").unwrap(), 58);
        assert_eq!(field_repr_to_index("a2").unwrap(), 48);
        assert_eq!(field_repr_to_index("f7").unwrap(), 13);
    }

    #[test]
    fn field_repr_io_test() {
        for index in 0..64 {
            assert_eq!(
                field_repr_to_index(&index_to_field_repr(index).unwrap()).unwrap(),
                index
            );
        }
    }

    #[test]
    fn char_to_piecetype_test() {
        use super::super::super::game_representation::PieceType;
        assert_eq!(char_to_piecetype('K').unwrap(), PieceType::King);
        assert_eq!(char_to_piecetype('Q').unwrap(), PieceType::Queen);
        assert_eq!(char_to_piecetype('B').unwrap(), PieceType::Bishop);
        assert_eq!(char_to_piecetype('R').unwrap(), PieceType::Rook);
        assert_eq!(char_to_piecetype('N').unwrap(), PieceType::Knight);

        assert!(char_to_piecetype('E').is_err());
        assert!(char_to_piecetype('1').is_err());
        assert!(char_to_piecetype('3').is_err());
        assert!(char_to_piecetype('e').is_err());
        assert!(char_to_piecetype('ö').is_err());
        assert!(char_to_piecetype('w').is_err());
        assert!(char_to_piecetype('5').is_err());
        assert!(char_to_piecetype('d').is_err());
        assert!(char_to_piecetype('1').is_err());
        assert!(char_to_piecetype('g').is_err());
        assert!(char_to_piecetype('z').is_err());
    }
}

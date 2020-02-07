//! Working with bitboards more ergonomically
//!
//! This module contains helper functions and constants that are imporatant
//! for working with bitboards without going insane.

use super::ParserError;

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

pub fn field_repr_to_coords(repr: &str) -> Result<(u8, u8), ParserError> {
    index_to_coords(field_repr_to_index(repr)?)
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
}

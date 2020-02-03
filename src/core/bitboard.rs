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
/// use core::game_representation::bitboard;
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

pub fn index_to_field_repr(index: u8) -> Result<String, ParserError> {
    let mut ret = String::new();
    let file = index % 8;
    let rank = index / 8;
    ret.push_str(file_to_str(file)?);
    ret.push_str(rank_to_str(rank)?);
    Ok(ret)
}

pub fn field_repr_to_index(repr: &str) -> Result<u8, ParserError> {
    let chars: Vec<char> = repr.chars().collect();
    if chars.len() != 2 {
        return Err(ParserError::WrongParameterNumber);
    }
    let index = str_to_rank(&chars[1].to_string())? * 8 + str_to_file(chars[0])?;
    Ok(index)
}

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
}

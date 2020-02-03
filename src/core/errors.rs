/// Common error for any parsing problems
///
/// * WrongParameterNumber if anything has the wrong length
/// * InvalidParameter if a parameter is not in the correct bounds
#[derive(Debug)]
pub enum ParserError {
    WrongParameterNumber,
    InvalidParameter(&'static str),
}

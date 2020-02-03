#[derive(Debug)]
pub enum ParserError {
    WrongParameterNumber,
    InvalidParameter(&'static str),
}

pub type ZumaResult<T> = Result<T, ZumaError>;

#[derive(Debug)]
pub enum ZumaError {
    ParseError(ParseError),
}

#[derive(Debug)]
pub struct ParseError {

}

impl From<lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>, &str>> for ZumaError {
    fn from(lalrpop_err: lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>, &str>) -> Self {
        
        match lalrpop_err {
            lalrpop_util::ParseError::InvalidToken { location } => {}
            lalrpop_util::ParseError::UnrecognizedEOF { location, expected } => {}
            lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {}
            lalrpop_util::ParseError::ExtraToken { token } => {}
            lalrpop_util::ParseError::User { error } => {}
        }
    }
}
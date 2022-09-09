use thiserror::Error;
use std::num::ParseFloatError;

#[derive(Debug, Error)]
pub enum CalcError {
    #[error("Broken AST")]
    BrokenAstErr,
    #[error("Undiffined command: {0}")]
    UDcommandErr(String),
}

#[derive(Debug, Error)]
pub enum ParseNumError {
    #[error("Invalid hex format: {0}")]
    InvalidHexFormat(String),
    #[error("Invalid binary format: {0}")]
    InvalidBinFormat(String),
    #[error("{0}")]
    CantParse(#[from] ParseFloatError)
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("{0}")]
    UnExpectedToken(#[from] TkError),
    #[error("Undefined command: {0}")]
    UDcommandErr(String),
    #[error("{0}")]
    CantParse(#[from] ParseNumError),
    #[error("There is no token to process")]
    NoToken,
}

#[derive(Debug, Error)]
pub enum TkError {
    #[error("expected TkNumber but {0}")]
    NotTkNumber(String),
    #[error("expected TkOperator but {0}")]
    NotTkOperator(String),
    #[error("undefined variable: {0}")]
    UndefinedVar(String),
    #[error("expected {0} but {1}")]
    NotExpected(String, String),
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("invalid input {0}")]
    InvalidInput(String),
    #[error("expected TkOperator but {0}")]
    NotTkOperator(String),
}
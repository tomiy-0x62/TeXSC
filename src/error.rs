use thiserror::Error;
use std::num::ParseFloatError;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("couldn't read config: {0}")]
    ConfigReadErr(String),
    #[error("couldn't write config: {0}")]
    ConfigWriteErr(String),
    #[error("broken AST")]
    BrokenAstErr,
    #[error("undiffined command '{0}'")]
    UDcommandErr(String),
    #[error("Invalid hex format '{0}'")]
    InvalidHexFormat(String),
    #[error("Invalid binary format '{0}'")]
    InvalidBinFormat(String),
    #[error("{0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("There is no token to process")]
    NoToken,
    #[error("invalid input {0}")]
    InvalidInput(String),
    #[error("expected TkNumber but {0}")]
    NotTkNumber(String),
    #[error("expected TkOperator but {0}")]
    NotTkOperator(String),
    #[error("expected TkVariable but {0}")]
    NotTkVariable(String),
    #[error("undefined variable '{0}'")]
    UDvariableErr(String),
    #[error("expected {0} but {1}")]
    UnexpectedToken(String, String),
    #[error("undiffined tsc command {0}")]
    UDtsccommand(String),
    #[error("expected {0} but {1}")]
    UnexpectedInput(String, String),
}
use std::num::ParseFloatError;
use thiserror::Error;

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
    #[error("un processed token '{0}'\n{1}")]
    UnprocessedToekn(String, String),
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
    #[error("expected TkNumber but {0}\n{1}")]
    NotTkNumber(String, String),
    #[error("expected TkBrace '{0}' but {1}\n{2}")]
    NotTkBrace(String, String, String),
    #[error("expected TkVariable but {0}\n{1}")]
    NotTkVariable(String, String),
    #[error("undefined variable '{0}'")]
    UDvariableErr(String),
    #[error("expected {0} but {1}")]
    UnexpectedToken(String, String),
    #[error("undiffined tsc command {0}")]
    UDtsccommand(String),
    #[error("expected {0} but {1}")]
    UnexpectedInput(String, String),
    #[error("unexpected operation to lexer: {0}")]
    UnexpectedOpToLexer(String),
}

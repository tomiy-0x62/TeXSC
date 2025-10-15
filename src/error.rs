use bigdecimal::{BigDecimal, ParseBigDecimalError};
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("couldn't load config: {0}")]
    ConfigLoadErr(String),
    #[error("couldn't find config file: {0}")]
    NoConfigErr(String),
    #[error("couldn't read config: {0}")]
    ConfigReadErr(String),
    #[error("couldn't write config: {0}")]
    ConfigWriteErr(String),
    #[error("couldn't read consts: {0}")]
    ConstsReadErr(String),
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
    #[error("Invalid octal format '{0}'")]
    InvalidOctalFormat(String),
    #[error("couldn't parse Float. {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("couldn't parse BigDecimal. {0}")]
    ParseBigDecimalError(#[from] ParseBigDecimalError),
    #[error("couldn't parse Int. {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("{0}")]
    TomlDeserializeError(#[from] toml::de::Error),
    #[error("There is no token to process")]
    NoToken,
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("expected TkNumber but {0}\n{1}")]
    NotTkNumber(String, String),
    #[error("expected TkBrace '{0}' but {1}\n{2}")]
    NotTkBrace(String, String, String),
    #[error("expected TkVariable but {0}\n{1}")]
    NotTkVariable(String, String),
    #[error("expected TkTscCmd but not")]
    NotTkTscCmd,
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
    #[error("couldn't calculate: {0}")]
    CalcErr(String),
    #[error("couldn't convert BigDecimal to {0}: {1}")]
    ConvertErr(String, BigDecimal),
    #[error("received quit command")]
    Quit,
}

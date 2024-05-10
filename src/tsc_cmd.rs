use std::collections::HashMap;
use text_colorizer::*;

use crate::config::*;
use crate::error::*;
use crate::parser::lexer::{self, Lexer};
use crate::parser::*;

pub fn process_tsccommand(
    lex: &Lexer,
    cmd_idx: usize,
    vars: &mut HashMap<String, f64>,
) -> Result<(), MyError> {
    let t1 = &lex.tokens[cmd_idx];
    let t2 = &lex.tokens[cmd_idx + 1];
    Ok(match &*t1.token {
        ":debug" => match &*t2.token {
            "true" => set_dbconf(true)?,
            "false" => set_dbconf(false)?,
            _ => {
                return Err(MyError::UnexpectedInput(
                    "true/false".to_string(),
                    t2.token.clone(),
                ))
            }
        },
        ":logbase" => match t2.token_kind {
            lexer::TokenKind::TkNum => match Parser::f64_from_str(&t2.token) {
                Ok(num) => set_lbconf(num)?,
                Err(e) => return Err(e),
            },
            lexer::TokenKind::TkVariable => match vars.get(&t2.token) {
                Some(num) => set_lbconf(*num)?,
                None => return Err(MyError::UDvariableErr(t2.token.to_string())),
            },
            _ => {
                return Err(MyError::NotTkNumber(
                    t2.token_kind.to_string(),
                    lex.format_err_loc_idx(cmd_idx + 1),
                ))
            }
        },
        ":rlen" => match t2.token_kind {
            lexer::TokenKind::TkNum => match Parser::f64_from_str(&t2.token) {
                Ok(num) => set_ndconf(num as u32)?,
                Err(e) => return Err(e),
            },
            lexer::TokenKind::TkVariable => match vars.get(&t2.token) {
                Some(num) => set_ndconf(*num as u32)?,
                None => return Err(MyError::UDvariableErr(t2.token.to_string())),
            },
            _ => {
                return Err(MyError::NotTkNumber(
                    t2.token_kind.to_string(),
                    lex.format_err_loc_idx(cmd_idx + 1),
                ))
            }
        },
        ":trarg" => match &*t2.token {
            "rad" => set_tfconf(TrigFuncArg::Radian)?,
            "deg" => set_tfconf(TrigFuncArg::Degree)?,
            _ => {
                return Err(MyError::UnexpectedInput(
                    "rad/deg".to_string(),
                    t2.token.clone(),
                ))
            }
        },
        ":astform" => match &*t2.token {
            "tree" => set_afconf(AstFormat::Tree)?,
            "sexpr" => set_afconf(AstFormat::Sexpr)?,
            "both" => set_afconf(AstFormat::Both)?,
            "none" => set_afconf(AstFormat::None)?,
            _ => {
                return Err(MyError::UnexpectedInput(
                    "rad/deg".to_string(),
                    t2.token.clone(),
                ))
            }
        },
        ":help" => cmd_help(),
        ":show" => match &*t2.token {
            "var" => (),
            "const" => (),
            "config" => (),
            _ => {
                return Err(MyError::UnexpectedInput(
                    "var/const/config".to_string(),
                    t2.token.clone(),
                ))
            }
        },
        _ => return Err(MyError::UDtsccommand(t2.token.clone())),
    })
}

fn cmd_help() {
    println!(
        "{: <14}
  {}
    {: <12}
        show this help
    {: <12}
        set debug flag
    {: <12}
        set \\log base
    {: <12}
        set result format
    {: <12}
        set ast format
    {: <12}
        show variable or config or embedded const number",
        ":TSC_COMMAND {option}".yellow(),
        "description".yellow(),
        ":help".green(),
        ":debug {true|false}".green(),
        ":logbase {num(f64)}".green(),
        ":rlen {num(u32)}".green(),
        ":astform {tree|sexpr|both|none}".green(),
        ":show {var|config|const}".green()
    );
}

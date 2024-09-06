use std::collections::HashMap;
use text_colorizer::*;

use crate::config::*;
use crate::error::*;
use crate::parser::lexer::{self, Lexer};
use crate::parser::*;
use crate::CONSTS;

pub fn process_tsccommand(
    lex: &Lexer,
    cmd_idx: usize,
    vars: &mut HashMap<String, f64>,
) -> Result<usize, MyError> {
    let t1 = &lex.tokens[cmd_idx];
    let t2 = &lex.tokens[cmd_idx + 1];
    let mut conf = config_writer()?;
    let consumed_token;
    match &*t1.token {
        ":q" => {
            return Err(MyError::UnexpectedInput(
                "true/false".to_string(),
                t2.token.clone(),
            ));
        }
        ":debug" => {
            consumed_token = 2;
            match &*t2.token {
                "true" => conf.debug = true,
                "false" => conf.debug = false,
                _ => {
                    return Err(MyError::UnexpectedInput(
                        "true/false".to_string(),
                        t2.token.clone(),
                    ))
                }
            }
        }
        ":logbase" => {
            consumed_token = 2;
            match t2.token_kind {
                lexer::TokenKind::TkNum => match Parser::f64_from_str(&t2.token) {
                    Ok(num) => conf.log_base = num,
                    Err(e) => return Err(e),
                },
                lexer::TokenKind::TkVariable => match vars.get(&t2.token) {
                    Some(num) => conf.log_base = *num,
                    None => return Err(MyError::UDvariableErr(t2.token.to_string())),
                },
                _ => {
                    return Err(MyError::NotTkNumber(
                        t2.token_kind.to_string(),
                        lex.format_err_loc_idx(cmd_idx + 1),
                    ))
                }
            }
        }
        ":rlen" => {
            consumed_token = 2;
            match t2.token_kind {
                lexer::TokenKind::TkNum => match Parser::f64_from_str(&t2.token) {
                    Ok(num) => conf.num_of_digit = num as u32,
                    Err(e) => return Err(e),
                },
                lexer::TokenKind::TkVariable => match vars.get(&t2.token) {
                    Some(num) => conf.num_of_digit = *num as u32,
                    None => return Err(MyError::UDvariableErr(t2.token.to_string())),
                },
                _ => {
                    return Err(MyError::NotTkNumber(
                        t2.token_kind.to_string(),
                        lex.format_err_loc_idx(cmd_idx + 1),
                    ))
                }
            }
        }
        ":trarg" => {
            consumed_token = 2;
            match &*t2.token {
                "rad" => conf.trig_func_arg = TrigFuncArg::Radian,
                "deg" => conf.trig_func_arg = TrigFuncArg::Degree,
                _ => {
                    return Err(MyError::UnexpectedInput(
                        "rad/deg".to_string(),
                        t2.token.clone(),
                    ))
                }
            }
        }
        ":astform" => {
            consumed_token = 2;
            match &*t2.token {
                "tree" => conf.ast_format = AstFormat::Tree,
                "sexpr" => conf.ast_format = AstFormat::Sexpr,
                "both" => conf.ast_format = AstFormat::Both,
                "none" => conf.ast_format = AstFormat::None,
                _ => {
                    return Err(MyError::UnexpectedInput(
                        "tree|sexpr|both|none".to_string(),
                        t2.token.clone(),
                    ))
                }
            }
        }
        ":write" => {
            consumed_token = 2;
            match &*t2.token {
                "conf" => {
                    println!("config write to {:?}", conf.write_to_file()?);
                }
                _ => {
                    return Err(MyError::UnexpectedInput(
                        "conf".to_string(),
                        t2.token.clone(),
                    ))
                }
            }
        }
        ":reload" => {
            consumed_token = 2;
            match &*t2.token {
                "conf" => {
                    println!("config reloaded from {:?}", conf.load_from_file()?);
                }
                _ => {
                    return Err(MyError::UnexpectedInput(
                        "conf".to_string(),
                        t2.token.clone(),
                    ))
                }
            }
        }
        ":hex" | ":dec" | ":bin" => {
            let mut counter = 0;
            let mut nums = Vec::new();
            loop {
                let t = &lex.tokens[cmd_idx + 1 + counter];
                match t.token_kind {
                    lexer::TokenKind::TkNum => match Parser::u64_from_str(&t.token) {
                        Ok(num) => {
                            nums.push(num);
                            counter += 1;
                        }
                        Err(e) => return Err(e),
                    },
                    _ => break,
                }
            }
            consumed_token = counter + 1;
            let mut msg = String::new();
            for num in nums {
                match &*t1.token {
                    ":hex" => msg += &format!("0x{:X} ", num),
                    ":dec" => msg += &format!("{} ", num),
                    ":bin" => msg += &format!("0b{:b} ", num),
                    _ => unreachable!(),
                }
            }
            println!("{msg}");
        }
        ":help" => {
            consumed_token = 1;
            cmd_help()
        }
        ":show" => {
            consumed_token = 2;
            match &*t2.token {
                "var" => show_variables(vars)?,
                "config" => println!("{}", conf),
                "conf" => println!("{}", conf),
                "const" => show_const()?,
                _ => {
                    return Err(MyError::UnexpectedInput(
                        "var/const/config".to_string(),
                        t2.token.clone(),
                    ))
                }
            }
        }
        _ => return Err(MyError::UDtsccommand(t1.token.clone())),
    }
    Ok(consumed_token)
}

fn show_variables(vars: &HashMap<String, f64>) -> Result<(), MyError> {
    let consts = match CONSTS.read() {
        Ok(consts) => consts,
        Err(e) => return Err(MyError::ConstsReadErr(e.to_string())),
    };
    for (name, value) in vars {
        if consts.get(name).is_none() {
            println!("{:<6}: {}", name, value);
        }
    }
    Ok(())
}

fn show_const() -> Result<(), MyError> {
    match CONSTS.read() {
        Ok(consts) => {
            for (name, value) in consts.iter() {
                println!("{:<6}: {}", name, value);
            }
            Ok(())
        }
        Err(e) => Err(MyError::ConstsReadErr(e.to_string())),
    }
}

fn cmd_help() {
    println!(
        "{: <14}
  {}
    {: <12}
        exit tsc
    {: <12}
        show this help
    {: <12}
        set debug flag
    {: <12}
        set \\log base
    {: <12}
        set result format
    {: <12}
        set format of trigonometric function argument
    {: <12}
        set ast format
    {: <12}
        write current config to config.toml
    {: <12}
        reload config
    {: <12}
        show numbers in hexadecimal formats
    {: <12}
        show numbers in decimal formats
    {: <12}
        show numbers in binary formats
    {: <12}
        show variable or config or embedded const number",
        ":TSC_COMMAND {option}".yellow(),
        "description".yellow(),
        ":q".green(),
        ":help".green(),
        ":debug {true|false}".green(),
        ":logbase {num(f64)}".green(),
        ":rlen {num(u32)}".green(),
        ":trarg {rad|deg}".green(),
        ":astform {tree|sexpr|both|none}".green(),
        ":write conf".green(),
        ":reload conf".green(),
        ":hex {num(u64)} ...".green(),
        ":dec {num(u64)} ...".green(),
        ":bin {num(u64)} ...".green(),
        ":show {var|const|config|conf}".green()
    );
}

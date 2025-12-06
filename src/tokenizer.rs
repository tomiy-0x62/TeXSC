use regex::Regex;
use std::fmt;

use crate::debug;
use crate::debugln;
use crate::error::*;

#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
    TkTexCommand,
    TkTscCommand,
    TkOperator,
    TkVariable,
    TkNum,
    TkBrace,
    TkSeparaotr,
    TkEOT,
}

pub enum NumstrOrVar {
    Num(String),
    Var(String),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::TkTexCommand => write!(f, "TkTexCommand"),
            TokenKind::TkTscCommand => write!(f, "TkTscCommand"),
            TokenKind::TkOperator => write!(f, "TkOperator"),
            TokenKind::TkVariable => write!(f, "TkVariable"),
            TokenKind::TkNum => write!(f, "TkNum"),
            TokenKind::TkBrace => write!(f, "TkBrace"),
            TokenKind::TkSeparaotr => write!(f, "TkSeparaotr"),
            TokenKind::TkEOT => write!(f, "TkEOT"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token: String,
    pub token_kind: TokenKind,
}

/*
tokenの種類
- 数値リテラル: 0x54, 0b100011, 534, 052
- TeXコマンド: \log, \sum
- コマンド: sin, cos
- かっこ: (, ), {, }
- 変数: x, y
*/

pub fn tokenize(formulas: &str) -> Result<(Vec<Token>, Vec<usize>), MyError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut token_loc: Vec<usize> = Vec::new();

    let tex_command = Regex::new(r"\\[A-Za-z]*").unwrap();
    let tsc_command = Regex::new(r":[A-Za-z]*").unwrap();
    let operator = Regex::new(r"\+|-|\*|=|/|!|_|\^|\|").unwrap();
    let var = Regex::new(r"[A-Za-z][A-Za-z0-9]*").unwrap();
    let num = Regex::new(r"0x[0-9a-fA-F]+|0[0-7]+|0b[0-1]+|[0-9]+\.?[0-9]*").unwrap();
    let braces = Regex::new(r"\(|\)|\[|\]|\{|\}").unwrap();
    let separator = Regex::new(r";").unwrap();
    let mut processed_form_idx = 0;

    let mut formulas = formulas;

    'search: loop {
        let mut c = match formulas.chars().next() {
            Some(c) => c,
            None => {
                tokens.push(Token {
                    token: "EOT".to_string(),
                    token_kind: TokenKind::TkEOT,
                });
                token_loc.push(processed_form_idx);
                print_token(&tokens);
                break 'search;
            }
        };
        while c == ' ' {
            formulas = &formulas[1..];
            processed_form_idx += 1;
            c = match formulas.chars().next() {
                Some(c) => c,
                None => {
                    tokens.push(Token {
                        token: "EOT".to_string(),
                        token_kind: TokenKind::TkEOT,
                    });
                    token_loc.push(processed_form_idx);
                    print_token(&tokens);
                    break 'search;
                }
            }
        }
        let mut ismatch = false;
        macro_rules! push_token {
            ($token: ident, $token_len: ident,$tk: ident) => {{
                token_loc.push(processed_form_idx);
                processed_form_idx += $token_len;
                tokens.push(Token {
                    $token,
                    token_kind: TokenKind::$tk,
                });
            }};
        }
        if c == '\\' {
            if let Some(caps) = tex_command.captures(formulas) {
                let token = caps.get(0).unwrap().as_str().to_string();
                let token_len = token.len();
                match &*token {
                    "\\times" => push_token!(token, token_len, TkOperator),
                    "\\cdot" => push_token!(token, token_len, TkOperator),
                    "\\div" => push_token!(token, token_len, TkOperator),
                    "\\pi" => push_token!(token, token_len, TkVariable),
                    _ => {
                        if is_valid_texcommand(&token) {
                            push_token!(token, token_len, TkTexCommand);
                        } else {
                            return Err(MyError::UDcommandErr(token));
                        }
                    }
                }
                formulas = &formulas[token_len..];
                ismatch = true;
            }
        } else if c == ':' {
            if let Some(caps) = tsc_command.captures(formulas) {
                let token = caps.get(0).unwrap().as_str().to_string();
                let token_len = token.len();
                push_token!(token, token_len, TkTscCommand);
                formulas = &formulas[token_len..];
                ismatch = true;
            }
        } else if let Some(caps) = operator.captures(&c.to_string()) {
            let token = caps.get(0).unwrap().as_str().to_string();
            let token_len = token.len();
            push_token!(token, token_len, TkOperator);
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if let Some(caps) = braces.captures(&c.to_string()) {
            let token = caps.get(0).unwrap().as_str().to_string();
            let token_len = token.len();
            push_token!(token, token_len, TkBrace);
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if let Some(caps) = separator.captures(&c.to_string()) {
            let token = caps.get(0).unwrap().as_str().to_string();
            let token_len = token.len();
            push_token!(token, token_len, TkSeparaotr);
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if num.captures(&c.to_string()).is_some() {
            if let Some(caps) = num.captures(formulas) {
                let token = caps.get(0).unwrap().as_str().to_string();
                let token_len = token.len();
                push_token!(token, token_len, TkNum);
                formulas = &formulas[token_len..];
                ismatch = true;
            }
        } else if let Some(caps) = var.captures(formulas) {
            if c != caps.get(0).unwrap().as_str().chars().next().unwrap() {
                return Err(MyError::InvalidInput(c.to_string()));
            }
            let token = caps.get(0).unwrap().as_str().to_string();
            let token_len = token.len();
            push_token!(token, token_len, TkVariable);
            formulas = &formulas[token_len..];
            ismatch = true;
        }
        if !ismatch {
            return Err(MyError::InvalidInput(c.to_string()));
        }

        if formulas.is_empty() {
            token_loc.push(processed_form_idx);
            tokens.push(Token {
                token: "EOT".to_string(),
                token_kind: TokenKind::TkEOT,
            });
            print_token(&tokens);
            break;
        }
    }

    assert_eq!(token_loc.len(), tokens.len());
    Ok((tokens, token_loc))
}

fn print_token(tokens: &[Token]) {
    for token in tokens.iter() {
        debug!("{}:'{}', ", token.token_kind, token.token);
    }
    debugln!("");
}

fn is_valid_texcommand(tc: &String) -> bool {
    matches!(
        &**tc,
        "\\times"
            | "\\cdot"
            | "\\div"
            | "\\frac"
            | "\\sqrt"
            | "\\log"
            | "\\ln"
            | "\\abs"
            | "\\exp"
            | "\\sin"
            | "\\cos"
            | "\\tan"
            | "\\csc"
            | "\\cot"
            | "\\arcsin"
            | "\\arccos"
            | "\\arctan"
    )
}

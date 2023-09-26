use regex::Regex;
use std::collections::HashMap;
use std::fmt;

use crate::debug;
use crate::debugln;
use crate::error::*;

#[derive(Clone, Copy)]
pub enum TokenKind {
    TkTexCommand,
    TkTscCommand,
    TkOperator,
    TkVariable,
    TkNum,
    TkBrace,
    TkEOT,
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
            TokenKind::TkEOT => write!(f, "TkEOT"),
        }
    }
}

pub struct Token {
    pub token: String,
    pub token_kind: TokenKind,
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    token_idx: usize,
    ctx_stack: Vec<usize>,
}

/*
tokenの種類
- 数値リテラル: 0x54, 0b100011, 534
- TeXコマンド: \log, \sum
- コマンド: sin, cos
- かっこ: (, ), {, }
- 変数: x, y
*/

impl Lexer {
    pub fn new(form: String) -> Result<Lexer, MyError> {
        let form = form.replace("\n", "").replace("\t", "").replace("\r", "");
        debugln!("form: '{}'", form);
        let mut tokens: Vec<Token> = Vec::new();
        Lexer::analyze(form, &mut tokens)?;
        Ok(Lexer {
            tokens: tokens,
            token_idx: 0,
            ctx_stack: Vec::new(),
        })
    }

    pub fn save_ctx(&mut self) {
        self.ctx_stack.push(self.token_idx);
    }

    pub fn revert_ctx(&mut self) -> Result<(), MyError> {
        match self.ctx_stack.pop() {
            Some(i) => {
                self.token_idx = i;
            }
            None => {
                return Err(MyError::UnexpectedOpToLexer(
                    "context not yet pushed".to_string(),
                ))
            }
        }
        Ok(())
    }

    pub fn discard_ctx(&mut self) -> Result<(), MyError> {
        match self.ctx_stack.pop() {
            Some(_) => (),
            None => {
                return Err(MyError::UnexpectedOpToLexer(
                    "context not yet pushed".to_string(),
                ))
            }
        }
        Ok(())
    }

    pub fn consume(&mut self, op: String) -> bool {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkOperator => (),
            TokenKind::TkBrace => (),
            TokenKind::TkTexCommand => (),
            _ => return false,
        }
        if self.tokens[self.token_idx].token == op {
            self.token_idx += 1;
            true
        } else {
            false
        }
    }

    pub fn expect(&mut self, op: String) -> Result<(), MyError> {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkOperator => {
                if self.tokens[self.token_idx].token == op {
                    self.token_idx += 1;
                    return Ok(());
                } else {
                    Err(MyError::UnexpectedToken(
                        op,
                        self.tokens[self.token_idx].token.to_string(),
                    ))
                }
            }
            TokenKind::TkBrace => {
                if self.tokens[self.token_idx].token == op {
                    self.token_idx += 1;
                    return Ok(());
                } else {
                    Err(MyError::UnexpectedToken(
                        op,
                        self.tokens[self.token_idx].token.to_string(),
                    ))
                }
            }
            tk => Err(MyError::NotTkOperator(tk.to_string())),
        }
    }

    pub fn expect_number(
        &mut self,
        vars: &HashMap<String, f64>,
    ) -> Result<(String, bool), MyError> {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkNum => {
                self.token_idx += 1;
                Ok((self.tokens[self.token_idx - 1].token.clone(), true))
            }
            TokenKind::TkVariable => {
                self.token_idx += 1;
                match vars.get(&self.tokens[self.token_idx - 1].token) {
                    Some(v) => Ok((v.to_string(), false)),
                    None => Err(MyError::UDvariableErr(
                        self.tokens[self.token_idx - 1].token.to_string(),
                    )),
                }
            }
            _ => Err(MyError::NotTkNumber(
                self.tokens[self.token_idx].token_kind.to_string(),
            )),
        }
    }

    pub fn is_eot(&self) -> bool {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkEOT => true,
            _ => false,
        }
    }

    pub fn now_token(&self) -> &str {
        &self.tokens[self.token_idx].token
    }

    fn analyze(mut formulas: String, tokens: &mut Vec<Token>) -> Result<(), MyError> {
        let tex_command = Regex::new(r"\\[A-Za-z]*").unwrap();
        let tsc_command = Regex::new(r":[A-Za-z]*").unwrap();
        let operator = Regex::new(r"\+|-|\*|=|/|!|_|,|\^|\|").unwrap();
        let var = Regex::new(r"[A-Za-z][A-Za-z0-9]*").unwrap();
        let num = Regex::new(r"0x[0-9a-fA-F]+|0b[0-1]+|[0-9]+\.?[0-9]*").unwrap();
        let braces = Regex::new(r"\(|\)|\[|\]|\{|\}").unwrap();

        'search: loop {
            let mut c = match formulas.chars().nth(0) {
                Some(c) => c,
                None => {
                    tokens.push(Token {
                        token: "EOT".to_string(),
                        token_kind: TokenKind::TkEOT,
                    });
                    Lexer::print_token(tokens);
                    break 'search;
                }
            };
            while c == ' ' {
                formulas = formulas.replacen(" ", "", 1);
                c = match formulas.chars().nth(0) {
                    Some(c) => c,
                    None => {
                        tokens.push(Token {
                            token: "EOT".to_string(),
                            token_kind: TokenKind::TkEOT,
                        });
                        Lexer::print_token(tokens);
                        break 'search;
                    }
                }
            }
            let mut ismatch = false;
            if c == '\\' {
                if let Some(caps) = tex_command.captures(&formulas) {
                    let token = caps.get(0).unwrap().as_str().to_string();
                    match &*token {
                        "\\times" => tokens.push(Token {
                            token,
                            token_kind: TokenKind::TkOperator,
                        }),
                        "\\cdot" => tokens.push(Token {
                            token,
                            token_kind: TokenKind::TkOperator,
                        }),
                        "\\div" => tokens.push(Token {
                            token,
                            token_kind: TokenKind::TkOperator,
                        }),
                        "\\pi" => tokens.push(Token {
                            token,
                            token_kind: TokenKind::TkVariable,
                        }),
                        _ => {
                            if Lexer::is_valid_texcommand(&token) {
                                tokens.push(Token {
                                    token,
                                    token_kind: TokenKind::TkTexCommand,
                                });
                            } else {
                                return Err(MyError::UDcommandErr(token));
                            }
                        }
                    }
                    formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    ismatch = true;
                }
            } else if c == ':' {
                if let Some(caps) = tsc_command.captures(&formulas) {
                    let token = caps.get(0).unwrap().as_str().to_string();
                    tokens.push(Token {
                        token,
                        token_kind: TokenKind::TkTscCommand,
                    });
                    formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    ismatch = true;
                }
            } else if let Some(caps) = operator.captures(&c.to_string()) {
                tokens.push(Token {
                    token: caps.get(0).unwrap().as_str().to_string(),
                    token_kind: TokenKind::TkOperator,
                });
                formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                ismatch = true;
            } else if let Some(caps) = braces.captures(&c.to_string()) {
                tokens.push(Token {
                    token: caps.get(0).unwrap().as_str().to_string(),
                    token_kind: TokenKind::TkBrace,
                });
                formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                ismatch = true;
            } else if let Some(_) = num.captures(&c.to_string()) {
                if let Some(caps) = num.captures(&formulas) {
                    tokens.push(Token {
                        token: caps.get(0).unwrap().as_str().to_string(),
                        token_kind: TokenKind::TkNum,
                    });
                    formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    ismatch = true;
                }
            } else if let Some(caps) = var.captures(&formulas) {
                if c != caps.get(0).unwrap().as_str().chars().nth(0).unwrap() {
                    return Err(MyError::InvalidInput(c.to_string()));
                }
                let token = caps.get(0).unwrap().as_str().to_string();
                tokens.push(Token {
                    token,
                    token_kind: TokenKind::TkVariable,
                });
                formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                ismatch = true;
            }
            if !ismatch {
                return Err(MyError::InvalidInput(c.to_string()));
            }

            if formulas.len() == 0 {
                tokens.push(Token {
                    token: "EOT".to_string(),
                    token_kind: TokenKind::TkEOT,
                });
                Lexer::print_token(tokens);
                break;
            }
        }

        return Ok(());
    }

    fn is_valid_texcommand(tc: &String) -> bool {
        match &**tc {
            "\\times" => true,
            "\\cdot" => true,
            "\\div" => true,
            "\\frac" => true,
            "\\sqrt" => true,
            "\\log" => true,
            "\\ln" => true,
            "\\abs" => true,
            "\\exp" => true,
            "\\sin" => true,
            "\\cos" => true,
            "\\tan" => true,
            "\\csc" => true,
            "\\cot" => true,
            "\\arcsin" => true,
            "\\arccos" => true,
            "\\arctan" => true,
            _ => false,
        }
    }

    fn print_token(tokens: &Vec<Token>) {
        for token in tokens.iter() {
            debug!("{}:'{}', ", token.token_kind, token.token);
        }
        debugln!("");
    }
}

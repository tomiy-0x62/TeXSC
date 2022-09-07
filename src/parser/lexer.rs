
use clap::Error;
use regex::Regex;
use std::collections::HashMap;
use thiserror::Error;
use std::fmt;

pub enum TokenKind {
    TkTexCommand,
    TkOperator,
    TkVariable,
    TkNum,
    TkBrace,
    TkEOT,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::TkTexCommand  => write!(f, "kTexCommand"),
            TokenKind::TkOperator    => write!(f, "TkOperator"),
            TokenKind::TkVariable    => write!(f, "TkVariable"),
            TokenKind::TkNum         => write!(f, "TkNum"),
            TokenKind::TkBrace       => write!(f, "TkBrace"),
            TokenKind::TkEOT         => write!(f, "TkEOT"),
        }
    }
}

pub struct Token {
    pub token: String,
    pub token_kind: TokenKind,
}

pub struct Lexer {
    formulas: String,
    pub tokens: Vec<Token>,
    pub token_idx: usize
}

#[derive(Debug, Error)]
pub enum TkError {
    #[error("expected TkNumber but {0}")]
    NotTkNumber(String),
    #[error("expected TkOperator but {0}")]
    NotTkOperator(String),
    #[error("undiffined variable: {0}")]
    UndiffinedVar(String),
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

/*
tokenの種類
- 数値リテラル: 0x54, 0b100011, 534
- TeXコマンド: \log, \sum
- コマンド: sin, cos
- かっこ: (, ), {, }
- 変数: x, y
*/

impl Lexer {
    pub fn new(form: String) -> Lexer {
        let form = form.replace("\n", "").replace("\t", "");
        Lexer { formulas: form, tokens: Vec::new(), token_idx: 0 }
    }

    pub fn print_form(&self) {
        println!("form: {}", self.formulas.replace("\n", " "));
    }

    fn print_token(&self) {
        for token in self.tokens.iter() {
            // {}でした
            print!("'{}', ", token.token);
        }
        println!("");
    }

    pub fn consume(&mut self, op: String) -> bool {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkOperator => {
                if self.tokens[self.token_idx].token == op {
                    self.token_idx += 1;
                    true
                } else {
                    false
                }
            },
            TokenKind::TkBrace => {
                if self.tokens[self.token_idx].token == op {
                    self.token_idx += 1;
                    true
                } else {
                    false
                }
            },
            TokenKind::TkTexCommand => {
                if self.tokens[self.token_idx].token == op {
                    self.token_idx += 1;
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    pub fn expect(&mut self, op: String) -> Result<(), TkError> {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkOperator => {
                if self.tokens[self.token_idx].token == op {
                    self.token_idx += 1;
                    return Ok(());
                } else {
                    Err(TkError::NotExpected(op, self.tokens[self.token_idx].token.to_string()))
                }
            },
            TokenKind::TkBrace => {
                if self.tokens[self.token_idx].token == op {
                    self.token_idx += 1;
                    return Ok(());
                } else {
                    Err(TkError::NotExpected(op, self.tokens[self.token_idx].token.to_string()))
                }
            },
            _ => Err(TkError::NotTkOperator(self.tokens[self.token_idx].token_kind.to_string())),
        }
    }

    pub fn expect_number(&mut self, vars: &HashMap<String, f64>) -> Result<String, TkError> {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkNum => {
                self.token_idx += 1;
                Ok(self.tokens[self.token_idx-1].token.clone())
            },
            TokenKind::TkVariable => {
                self.token_idx += 1;
                match vars.get(&self.tokens[self.token_idx-1].token) {
                    Some(v) => {
                        // println!("var: {} = {}", self.tokens[self.token_idx-1].token, v);
                        Ok(v.to_string())
                    },
                    None => {
                        Err(TkError::UndiffinedVar(self.tokens[self.token_idx-1].token.to_string()))
                    },
                }
            },
            _ => Err(TkError::NotTkNumber(self.tokens[self.token_idx].token_kind.to_string())),
        }
    }

    pub fn is_eot(&self) -> bool {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkEOT => true,
            _ => false
        }
    }

    pub fn now_token(&self) -> &str {
        &self.tokens[self.token_idx].token
    }
    pub fn analyze(&mut self) -> Result<(), LexerError> {
        let tex_command = Regex::new(r"\\[A-Za-z]*").unwrap(); // OK
        let operator = Regex::new(r"\+|-|\*|=|/|!|_|,|\^|\|").unwrap(); // OK
        let var = Regex::new(r"[A-Za-z][A-Za-z0-9]*").unwrap(); // OK
        let num = Regex::new(r"0x[0-9a-fA-F]+|0b[0-1]+|[0-9]+\.?[0-9]*").unwrap(); // OK
        let braces = Regex::new(r"\(|\)|\[|\]|\{|\}").unwrap(); // OK

        'search:
        loop {
            let mut c = self.formulas.chars().nth(0).unwrap();
            while c == ' ' {
                self.formulas = self.formulas.replacen(" ", "", 1);
                c = match self.formulas.chars().nth(0) {
                    Some(c) => c,
                    None => {
                        self.tokens.push(Token {token: "EOT".to_string(), token_kind: TokenKind::TkEOT});
                        // self.print_token();
                        break 'search
                    },
                }
            }
            let mut ismatch = false;
            if c == '\\' {
                if let Some(caps) = tex_command.captures(&self.formulas) {
                    let token = caps.get(0).unwrap().as_str().to_string().replace(" ", "");
                    match &*token {
                        "\\times" => self.tokens.push(Token {token: token, token_kind: TokenKind::TkOperator}),
                        "\\cdot" => self.tokens.push(Token {token: token, token_kind: TokenKind::TkOperator}),
                        "\\div" => self.tokens.push(Token {token: token, token_kind: TokenKind::TkOperator}),
                        "\\pi" => self.tokens.push(Token {token: std::f64::consts::PI.to_string(), token_kind: TokenKind::TkNum}),
                        _ => self.tokens.push(Token {token: token, token_kind: TokenKind::TkTexCommand}),
                    }
                    self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    ismatch = true;
                }
            } else if let Some(caps) = operator.captures(&c.to_string()) {
                self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_kind: TokenKind::TkOperator});
                self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                ismatch = true;
            } else if let Some(caps) = braces.captures(&c.to_string()) {
                self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_kind: TokenKind::TkBrace});
                self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                ismatch = true;
            } else if let Some(_) = num.captures(&c.to_string()) {
                if let Some(caps) = num.captures(&self.formulas) {
                    self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_kind: TokenKind::TkNum});
                    self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    ismatch = true;
                }
            } else if let Some(caps) = var.captures(&self.formulas) {
                if c != caps.get(0).unwrap().as_str().chars().nth(0).unwrap() { return Err(LexerError::InvalidInput(c.to_string())); }
                let token = caps.get(0).unwrap().as_str().to_string().replace(" ", "");
                if token == "e" {
                    self.tokens.push(Token {token: std::f64::consts::E.to_string(), token_kind: TokenKind::TkNum});
                } else {
                    self.tokens.push(Token {token: token, token_kind: TokenKind::TkVariable});
                }
                self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                ismatch = true;
            } 
            if !ismatch {
                return Err(LexerError::InvalidInput(c.to_string()));
            }

            if self.formulas.len() == 0 {
                self.tokens.push(Token {token: "EOT".to_string(), token_kind: TokenKind::TkEOT});
                // self.print_token();
                break;
            }
        }

        
        /*
        for caps in tex_command.captures_iter(&self.formulas) {
            println!("match '{}'", &caps[0]);
        }*/
        
        if let Some(caps) = operator.captures(&self.formulas) {
            println!("<<< match '{}' >>>", caps.get(0).unwrap().as_str());
            // if let Some(hoge) = caps.get(0)
        }
        return Ok(());

    }
}

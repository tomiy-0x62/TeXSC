use regex::Regex;
use std::fmt;
use text_colorizer::*;

use super::TscCmd;
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

pub struct Lexer {
    form: String,
    pub tokens: Vec<Token>,
    token_loc: Vec<usize>,
    token_idx: usize,
    ctx_stack: Vec<usize>,
}

/*
tokenの種類
- 数値リテラル: 0x54, 0b100011, 534, 052
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
        let mut token_loc: Vec<usize> = Vec::new();
        Lexer::analyze(form.clone(), &mut tokens, &mut token_loc)?;
        assert_eq!(token_loc.len(), tokens.len());
        Ok(Lexer {
            form,
            tokens,
            token_loc,
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
            TokenKind::TkSeparaotr => (),
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

    pub fn consume_seq(&mut self) -> bool {
        if let TokenKind::TkSeparaotr = self.tokens[self.token_idx].token_kind {
            self.token_idx += 1;
            true
        } else {
            false
        }
    }

    pub fn consume_tsc_cmd(&mut self) -> Result<TscCmd, MyError> {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkTscCommand => {
                let token = &*self.tokens[self.token_idx].token;
                self.token_idx += 1;
                Ok(match token {
                    ":hex" => TscCmd::Hex,
                    ":dec" => TscCmd::Dec,
                    ":bin" => TscCmd::Bin,
                    ":oct" => TscCmd::Oct,
                    cmd => return Err(MyError::UDcommandErr(cmd.to_string())),
                })
            }
            _ => Err(MyError::NotTkTscCmd),
        }
    }

    pub fn expect_br(&mut self, br: String) -> Result<(), MyError> {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkBrace => {
                if self.tokens[self.token_idx].token == br {
                    self.token_idx += 1;
                    Ok(())
                } else {
                    Err(MyError::UnexpectedToken(
                        br,
                        self.tokens[self.token_idx].token.to_string(),
                    ))
                }
            }
            tk => Err(MyError::NotTkBrace(
                br,
                tk.to_string(),
                self.format_err_loc(),
            )),
        }
    }

    pub fn expect_number(&mut self) -> Result<NumstrOrVar, MyError> {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkNum => {
                self.token_idx += 1;
                Ok(NumstrOrVar::Num(
                    self.tokens[self.token_idx - 1].token.clone(),
                ))
            }
            TokenKind::TkVariable => {
                self.token_idx += 1;
                Ok(NumstrOrVar::Var(
                    self.tokens[self.token_idx - 1].token.to_string(),
                ))
            }
            _ => Err(MyError::NotTkNumber(
                self.tokens[self.token_idx].token_kind.to_string(),
                self.format_err_loc(),
            )),
        }
    }

    pub fn is_eot(&self) -> bool {
        matches!(self.tokens[self.token_idx].token_kind, TokenKind::TkEOT)
    }

    pub fn now_token(&self) -> &str {
        &self.tokens[self.token_idx].token
    }

    /// 保持しているtoken列からidx番目のtokenを削除
    ///
    /// * `idx` - 削除するtokenのindex
    pub fn del_token(&mut self, idx: usize) {
        self.tokens.remove(idx);
        self.token_loc.remove(idx);
    }

    fn analyze(
        mut formulas: String,
        tokens: &mut Vec<Token>,
        token_loc: &mut Vec<usize>,
    ) -> Result<(), MyError> {
        let tex_command = Regex::new(r"\\[A-Za-z]*").unwrap();
        let tsc_command = Regex::new(r":[A-Za-z]*").unwrap();
        let operator = Regex::new(r"\+|-|\*|=|/|!|_|\^|\|").unwrap();
        let var = Regex::new(r"[A-Za-z][A-Za-z0-9]*").unwrap();
        let num = Regex::new(r"0x[0-9a-fA-F]+|0[0-7]+|0b[0-1]+|[0-9]+\.?[0-9]*").unwrap();
        let braces = Regex::new(r"\(|\)|\[|\]|\{|\}").unwrap();
        let separator = Regex::new(r",").unwrap();
        let mut processed_form_idx = 0;

        'search: loop {
            let mut c = match formulas.chars().nth(0) {
                Some(c) => c,
                None => {
                    tokens.push(Token {
                        token: "EOT".to_string(),
                        token_kind: TokenKind::TkEOT,
                    });
                    token_loc.push(processed_form_idx);
                    Lexer::print_token(tokens);
                    break 'search;
                }
            };
            while c == ' ' {
                formulas = formulas.replacen(" ", "", 1);
                processed_form_idx += 1;
                c = match formulas.chars().nth(0) {
                    Some(c) => c,
                    None => {
                        tokens.push(Token {
                            token: "EOT".to_string(),
                            token_kind: TokenKind::TkEOT,
                        });
                        token_loc.push(processed_form_idx);
                        Lexer::print_token(tokens);
                        break 'search;
                    }
                }
            }
            let mut ismatch = false;
            macro_rules! push_token {
                ($token: ident, $tk: ident) => {{
                    token_loc.push(processed_form_idx);
                    processed_form_idx += $token.len();
                    tokens.push(Token {
                        $token,
                        token_kind: TokenKind::$tk,
                    });
                }};
            }
            if c == '\\' {
                if let Some(caps) = tex_command.captures(&formulas) {
                    let token = caps.get(0).unwrap().as_str().to_string();
                    match &*token {
                        "\\times" => push_token!(token, TkOperator),
                        "\\cdot" => push_token!(token, TkOperator),
                        "\\div" => push_token!(token, TkOperator),
                        "\\pi" => push_token!(token, TkVariable),
                        _ => {
                            if Lexer::is_valid_texcommand(&token) {
                                push_token!(token, TkTexCommand);
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
                    push_token!(token, TkTscCommand);
                    formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    ismatch = true;
                }
            } else if let Some(caps) = operator.captures(&c.to_string()) {
                let token = caps.get(0).unwrap().as_str().to_string();
                push_token!(token, TkOperator);
                formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                ismatch = true;
            } else if let Some(caps) = braces.captures(&c.to_string()) {
                let token = caps.get(0).unwrap().as_str().to_string();
                push_token!(token, TkBrace);
                formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                ismatch = true;
            } else if let Some(caps) = separator.captures(&c.to_string()) {
                let token = caps.get(0).unwrap().as_str().to_string();
                push_token!(token, TkSeparaotr);
                formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                ismatch = true;
            } else if num.captures(&c.to_string()).is_some() {
                if let Some(caps) = num.captures(&formulas) {
                    let token = caps.get(0).unwrap().as_str().to_string();
                    push_token!(token, TkNum);
                    formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    ismatch = true;
                }
            } else if let Some(caps) = var.captures(&formulas) {
                if c != caps.get(0).unwrap().as_str().chars().nth(0).unwrap() {
                    return Err(MyError::InvalidInput(c.to_string()));
                }
                let token = caps.get(0).unwrap().as_str().to_string();
                push_token!(token, TkVariable);
                formulas = formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
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
                Lexer::print_token(tokens);
                break;
            }
        }

        Ok(())
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

    /// parser内でエラーが起こっており、lexer.token_idxにエラーの原因となる
    /// tokenが入っているときに、エラーが数式のどの個所で起こったかを示す文字列を返す
    /// ex)
    /// ```
    /// \frac {3} {
    ///            ^~~
    /// ```
    pub fn format_err_loc(&self) -> String {
        let mut pad = String::with_capacity(self.token_loc[self.token_idx]);
        for _i in 0..self.token_loc[self.token_idx] {
            pad += " ";
        }
        let err_indicator = format!("{}{}", pad, "^".red());
        let mut nami = String::with_capacity(self.tokens[self.token_idx].token.len());
        for _i in 0..self.tokens[self.token_idx].token.len() - 1 {
            nami += &format!("{}", "~".red());
        }
        let res = format!("{}\n{}{}", self.form, err_indicator, nami);
        res
    }

    /// 変数やTSC Commandの処理中等のparser外でエラーが起こっており、lexer.token_idxにエラーの
    /// 原因個所のtoken indexが入っていないときにエラーが数式のどの個所で起こったかを示す文字列を返す
    /// ex)
    /// ```
    /// , x = a
    ///       ^
    ///       ```
    /// * `token_idx` - token_idx: エラーが発生したtokenのindex
    pub fn format_err_loc_idx(&self, token_idx: usize) -> String {
        let mut pad = String::with_capacity(self.token_loc[token_idx]);
        for _i in 0..self.token_loc[token_idx] {
            pad += " ";
        }
        let err_indicator = format!("{}{}", pad, "^".red());
        let mut nami = String::with_capacity(self.tokens[token_idx].token.len());
        for _i in 0..self.tokens[token_idx].token.len() - 1 {
            nami += &format!("{}", "~".red());
        }
        let res = format!("{}\n{}{}", self.form, err_indicator, nami);
        res
    }

    fn print_token(tokens: &[Token]) {
        for token in tokens.iter() {
            debug!("{}:'{}', ", token.token_kind, token.token);
        }
        debugln!("");
    }
}

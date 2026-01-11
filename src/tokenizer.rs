use regex::Regex;
use std::fmt;

use crate::debug;
use crate::debugln;
use crate::error::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    TkTexCommand,
    TkTscCommand,
    TkOperator,
    TkVariable,
    TkNum(NumFormat),
    TkBrace,
    TkSeparaotr,
    TkEOT,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumFormat {
    Scientific,
    Hex,
    Oct,
    Bin,
    Dec,
    DecInt,
}

impl fmt::Display for NumFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumFormat::Scientific => write!(f, "Scientific"),
            NumFormat::Hex => write!(f, "Hex"),
            NumFormat::Oct => write!(f, "Oct"),
            NumFormat::Bin => write!(f, "Bin"),
            NumFormat::Dec => write!(f, "Dec"),
            NumFormat::DecInt => write!(f, "DecInt"),
        }
    }
}

pub enum NumstrOrVar {
    Num((NumFormat, String)),
    Var(String),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::TkTexCommand => write!(f, "TkTexCommand"),
            TokenKind::TkTscCommand => write!(f, "TkTscCommand"),
            TokenKind::TkOperator => write!(f, "TkOperator"),
            TokenKind::TkVariable => write!(f, "TkVariable"),
            TokenKind::TkNum(k) => write!(f, "TkNum({})", k),
            TokenKind::TkBrace => write!(f, "TkBrace"),
            TokenKind::TkSeparaotr => write!(f, "TkSeparaotr"),
            TokenKind::TkEOT => write!(f, "TkEOT"),
        }
    }
}

#[derive(Debug, PartialEq)]
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
    // scientific: 1.16E-6
    let scientific_pat = r"[1-9]\.[0-9]+E(\+|-)[1-9]+";
    // hex: 0x1234, 0x12_34
    let hex_pat = r"0x([0-9a-fA-F]+_?)*[0-9a-fA-F]+";
    // oct: 01234, 0_12_34
    let oct_pat = r"0([0-7]+_?)*[0-7]+";
    // bin: 0b1010, 0b10_10
    let bin_pat = r"0b([0-1]+_?)*[0-1]+";
    // dec(!int): '1.234', '1.2_34'
    let dec_pat = r"([0-9]+(_|,)?)*[0-9]+\.([0-9]+(_|,)?)*[0-9]+";
    // dec(int): '1234', '12_34', '1,234
    let decint_pat = r"([0-9]+(_|,)?)*[0-9]+";
    /*
    let num =
        Regex::new(r"[1-9]\.[0-9]+E(+|-)[1-9]+|(0x([0-9a-fA-F]+_?)*[0-9a-fA-F]+)|(0([0-7]+_?)*[0-7]+)|(0b([0-1]+_?)*[0-1]+)|(([0-9]+(_|,)?)*[0-9]+\.([0-9]+(_|,)?)*[0-9]+)|(([0-9]+(_|,)?)*[0-9]+)")
            .unwrap();
    */
    let num_pat = format!(
        r"(?P<scientific>{})|(?P<hex>{})|(?P<oct>{})|(?P<bin>{})|(?P<dec>{})|(?P<decint>{})",
        scientific_pat, hex_pat, oct_pat, bin_pat, dec_pat, decint_pat
    );
    let num = Regex::new(&num_pat).unwrap();
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
            ($token: ident, $token_len: ident,$tk: expr) => {{
                token_loc.push(processed_form_idx);
                processed_form_idx += $token_len;
                tokens.push(Token {
                    $token,
                    token_kind: $tk,
                });
            }};
        }
        if c == '\\' {
            if let Some(caps) = tex_command.captures(formulas) {
                let token = caps.get(0).unwrap().as_str().to_string();
                let token_len = token.len();
                match &*token {
                    "\\times" => push_token!(token, token_len, TokenKind::TkOperator),
                    "\\cdot" => push_token!(token, token_len, TokenKind::TkOperator),
                    "\\div" => push_token!(token, token_len, TokenKind::TkOperator),
                    "\\pi" => push_token!(token, token_len, TokenKind::TkVariable),
                    _ => {
                        if is_valid_texcommand(&token) {
                            push_token!(token, token_len, TokenKind::TkTexCommand);
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
                push_token!(token, token_len, TokenKind::TkTscCommand);
                formulas = &formulas[token_len..];
                ismatch = true;
            }
        } else if let Some(caps) = operator.captures(&c.to_string()) {
            let token = caps.get(0).unwrap().as_str().to_string();
            let token_len = token.len();
            push_token!(token, token_len, TokenKind::TkOperator);
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if let Some(caps) = braces.captures(&c.to_string()) {
            let token = caps.get(0).unwrap().as_str().to_string();
            let token_len = token.len();
            push_token!(token, token_len, TokenKind::TkBrace);
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if let Some(caps) = separator.captures(&c.to_string()) {
            let token = caps.get(0).unwrap().as_str().to_string();
            let token_len = token.len();
            push_token!(token, token_len, TokenKind::TkSeparaotr);
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if let Some(caps) = num.captures(formulas) {
            let token = caps.get(0).unwrap().as_str().to_string();
            let token_len = token.len();
            if caps.name("scientific").is_some() {
                push_token!(token, token_len, TokenKind::TkNum(NumFormat::Scientific));
            } else if caps.name("hex").is_some() {
                push_token!(token, token_len, TokenKind::TkNum(NumFormat::Hex));
            } else if caps.name("oct").is_some() {
                push_token!(token, token_len, TokenKind::TkNum(NumFormat::Oct));
            } else if caps.name("bin").is_some() {
                push_token!(token, token_len, TokenKind::TkNum(NumFormat::Bin));
            } else if caps.name("dec").is_some() {
                push_token!(token, token_len, TokenKind::TkNum(NumFormat::Dec));
            } else if caps.name("decint").is_some() {
                push_token!(token, token_len, TokenKind::TkNum(NumFormat::DecInt));
            }
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if let Some(caps) = var.captures(formulas) {
            if c != caps.get(0).unwrap().as_str().chars().next().unwrap() {
                return Err(MyError::InvalidInput(c.to_string()));
            }
            let token = caps.get(0).unwrap().as_str().to_string();
            let token_len = token.len();
            push_token!(token, token_len, TokenKind::TkVariable);
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

#[cfg(test)]
mod test {
    use super::{NumFormat, Token, TokenKind};
    #[test]
    fn test_tokenize() {
        let formulas = "1.16E-6 * 0x1 - \\frac{\\sin \\pi}{0b10} / 0x12 + 0.2";
        let t = vec![
            new_token("1.16E-6", TokenKind::TkNum(NumFormat::Scientific)),
            new_token("*", TokenKind::TkOperator),
            new_token("0x1", TokenKind::TkNum(NumFormat::Hex)),
            new_token("-", TokenKind::TkOperator),
            new_token("\\frac", TokenKind::TkTexCommand),
            new_token("{", TokenKind::TkBrace),
            new_token("\\sin", TokenKind::TkTexCommand),
            new_token("\\pi", TokenKind::TkVariable),
            new_token("}", TokenKind::TkBrace),
            new_token("{", TokenKind::TkBrace),
            new_token("0b10", TokenKind::TkNum(NumFormat::Bin)),
            new_token("}", TokenKind::TkBrace),
            new_token("/", TokenKind::TkOperator),
            new_token("0x12", TokenKind::TkNum(NumFormat::Hex)),
            new_token("+", TokenKind::TkOperator),
            new_token("0.2", TokenKind::TkNum(NumFormat::Dec)),
            new_token("EOT", TokenKind::TkEOT),
        ];
        let s = vec![
            0, 8, 10, 14, 16, 21, 22, 27, 30, 31, 32, 36, 38, 40, 45, 47, 50,
        ];
        match super::tokenize(formulas) {
            Ok((tokens, sizes)) => {
                assert_eq!(tokens, t);
                assert_eq!(sizes, s);
            }
            Err(e) => panic!("{}", e),
        }
    }

    fn new_token(t: &str, k: TokenKind) -> Token {
        Token {
            token: t.to_string(),
            token_kind: k,
        }
    }
}

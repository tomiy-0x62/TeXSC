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

fn try_hex_from(formula: &str) -> Option<(String, NumFormat)> {
    assert!(formula.starts_with("0x"));
    let mut token = String::new();
    if let Some(c2) = formula[2..].chars().next() {
        if c2.is_ascii_digit() || ('a'..='f').contains(&c2) || ('A'..='F').contains(&c2) {
            token += "0x";
            token.push(c2);
            let mut is_prev_sep = false;
            for cf in formula[3..].chars() {
                if cf.is_ascii_digit() || ('a'..='f').contains(&cf) || ('A'..='F').contains(&cf) {
                    token.push(cf);
                    is_prev_sep = false;
                } else if cf == '_' {
                    if is_prev_sep {
                        break;
                    } else {
                        token.push(cf);
                        is_prev_sep = true;
                    }
                } else {
                    break;
                }
            }
            if token.ends_with('_') {
                token.pop();
            }
            Some((token, NumFormat::Hex))
        } else {
            Some(('0'.to_string(), NumFormat::DecInt))
        }
    } else {
        Some(('0'.to_string(), NumFormat::DecInt))
    }
}

fn try_bin_from(formula: &str) -> Option<(String, NumFormat)> {
    assert!(formula.starts_with("0b"));
    let mut token = String::new();
    if let Some(c2) = formula[2..].chars().next() {
        if c2 == '0' || c2 == '1' {
            token += "0b";
            token.push(c2);
            let mut is_prev_sep = false;
            for cf in formula[3..].chars() {
                if cf == '0' || cf == '1' {
                    token.push(cf);
                    is_prev_sep = false;
                } else if cf == '_' {
                    if is_prev_sep {
                        break;
                    } else {
                        token.push(cf);
                        is_prev_sep = true;
                    }
                } else {
                    break;
                }
            }
            if token.ends_with('_') {
                token.pop();
            }
            Some((token, NumFormat::Bin))
        } else {
            Some(('0'.to_string(), NumFormat::DecInt))
        }
    } else {
        Some(('0'.to_string(), NumFormat::DecInt))
    }
}

fn try_dec0_from(formula: &str) -> Option<(String, NumFormat)> {
    assert!(formula.starts_with("0."));
    let mut token = String::new();
    if let Some(c2) = formula[2..].chars().next() {
        if c2.is_ascii_digit() {
            token += "0.";
            token.push(c2);
            let mut is_prev_sep = false;
            for cf in formula[3..].chars() {
                if cf.is_ascii_digit() {
                    token.push(cf);
                    is_prev_sep = false;
                } else if cf == ',' || cf == '_' {
                    if is_prev_sep {
                        break;
                    } else {
                        token.push(cf);
                        is_prev_sep = true;
                    }
                } else {
                    break;
                }
            }
            if token.ends_with(',') || token.ends_with('_') {
                token.pop();
            }
            Some((token, NumFormat::Dec))
        } else {
            Some(('0'.to_string(), NumFormat::DecInt))
        }
    } else {
        Some(('0'.to_string(), NumFormat::DecInt))
    }
}

fn try_oct_from(formula: &str) -> Option<(String, NumFormat)> {
    assert!(formula.starts_with('0'));
    let mut token = String::from('0');
    if let Some(c1) = formula[1..].chars().next() {
        if ('0'..='7').contains(&c1) {
            token.push(c1);
            let mut is_prev_sep = false;
            for cf in formula[2..].chars() {
                if ('0'..='7').contains(&cf) {
                    token.push(cf);
                    is_prev_sep = false;
                } else if cf == '_' {
                    if is_prev_sep {
                        break;
                    } else {
                        token.push(cf);
                        is_prev_sep = true;
                    }
                } else {
                    break;
                }
            }
            if token.ends_with('_') {
                token.pop();
            }
            Some((token, NumFormat::Oct))
        } else {
            Some(('0'.to_string(), NumFormat::DecInt))
        }
    } else {
        Some(('0'.to_string(), NumFormat::DecInt))
    }
}

fn try_scientific_from(formula: &str) -> Option<(String, NumFormat)> {
    // formula starts_with "[1-9]\."
    let mut token = String::from(&formula[..1]);
    // token: [1-9]
    let mut pushed = 1;
    if let Some(c2) = formula[2..].chars().next() {
        if c2.is_ascii_digit() {
            token.push('.');
            token.push(c2);
            pushed += 2;
            // token: [1-9]\.[0-9]
            for cf in formula[3..].chars() {
                if cf.is_ascii_digit() {
                    token.push(cf);
                    pushed += 1;
                } else {
                    // token: [1-9]\.[0-9]+
                    if cf == 'E' {
                        token.push('E');
                        pushed += 1;
                        if let Some(cne) = formula[pushed..].chars().next() {
                            if cne == '+' || cne == '-' {
                                token.push(cne);
                                pushed += 1;
                                // token: [1-9]\.[0-9]+E(\+|-)
                                if let Some(cns) = formula[pushed..].chars().next() {
                                    if ('1'..='9').contains(&cns) {
                                        token.push(cns);
                                        pushed += 1;
                                        // token: [1-9]\.[0-9]+E(\+|-)[1-9]
                                        for cfi in formula[pushed..].chars() {
                                            if ('1'..='9').contains(&cfi) {
                                                token.push(cfi);
                                                // token: [1-9]\.[0-9]+E(\+|-)[1-9]+
                                            } else {
                                                // token: [1-9]\.[0-9]+E(\+|-)[1-9]+
                                                return Some((token, NumFormat::Scientific));
                                            }
                                        }
                                        // token: [1-9]\.[0-9]+E(\+|-)[1-9]
                                        return Some((token, NumFormat::Scientific));
                                    } else {
                                        // token: [1-9]\.[0-9]+E(\+|-)
                                        token.pop();
                                        token.pop();
                                        // token: [1-9]\.[0-9]+
                                        return Some((token, NumFormat::Dec));
                                    }
                                } else {
                                    // token: [1-9]\.[0-9]+E(\+|-){EOF}
                                    token.pop();
                                    token.pop();
                                    // token: [1-9]\.[0-9]+
                                    return Some((token, NumFormat::Dec));
                                }
                            } else {
                                // formula: [1-9]\.[0-9]+E(!(\+|-))
                                // token: [1-9]\.[0-9]+E
                                token.pop();
                                return Some((token, NumFormat::Dec));
                            }
                        } else {
                            // formula: [1-9]\.[0-9]+E{EOF}
                            // token: [1-9]\.[0-9]+E
                            token.pop();
                            return Some((token, NumFormat::Dec));
                        }
                    // formula: [1-9]\.[0-9]+(!([0-9]|E))
                    // token: [1-9]\.[0-9]+
                    } else if cf == ',' || cf == '_' {
                        // dec(!int): '1.234', '1.2_34'
                        // let dec_pat = r"([0-9]+(_|,)?)*[0-9]+\.([0-9]+(_|,)?)*[0-9]+";
                        token.push(cf);
                        pushed += 1;
                        let mut is_prev_sep = true;
                        for cfi in formula[pushed..].chars() {
                            let is_sep = cfi == '_' || cfi == ',';
                            if cfi.is_ascii_digit() || (!is_prev_sep && is_sep) {
                                token.push(cfi);
                                is_prev_sep = is_sep;
                            } else if is_prev_sep && is_sep {
                                token.pop();
                                return Some((token, NumFormat::Dec));
                            } else {
                                if is_prev_sep {
                                    token.pop();
                                }
                                return Some((token, NumFormat::Dec));
                            }
                        }
                        if is_prev_sep {
                            token.pop();
                        }
                        return Some((token, NumFormat::Dec));
                        // None
                    }
                    // token: [1-9]\.[0-9]+
                    return Some((token, NumFormat::Dec));
                }
            }
            // token: [1-9]\.[0-9]
            Some((token, NumFormat::Dec))
        } else {
            // token: [1-9]
            Some((token, NumFormat::DecInt))
        }
    } else {
        // token: [1-9]
        Some((token, NumFormat::DecInt))
    }
}

fn try_dec_from(formula: &str) -> Option<(String, NumFormat)> {
    let mut token = String::new();
    let mut have_dot = false;
    let mut is_prev_sep = false;
    let mut is_prev_dot = false;
    for c in formula.chars() {
        if c.is_ascii_digit() {
            token.push(c);
            is_prev_sep = false;
            is_prev_dot = false;
        } else if c == '.' {
            if have_dot {
                return Some((token, NumFormat::Dec));
            } else if is_prev_sep {
                token.pop();
                return Some((token, NumFormat::DecInt));
            } else {
                token.push('.');
                have_dot = true;
                is_prev_sep = false;
                is_prev_dot = true;
            }
        } else if c == ',' || c == '_' {
            if is_prev_sep {
                token.pop();
                if have_dot {
                    return Some((token, NumFormat::Dec));
                } else {
                    return Some((token, NumFormat::DecInt));
                }
            } else if is_prev_dot {
                token.pop();
                return Some((token, NumFormat::DecInt));
            } else {
                token.push(c);
                is_prev_sep = true;
            }
        } else {
            if is_prev_sep {
                token.pop();
            }
            if token.ends_with('.') {
                token.pop();
                have_dot = false;
            }
            if have_dot {
                return Some((token, NumFormat::Dec));
            } else {
                return Some((token, NumFormat::DecInt));
            }
        }
    }
    if is_prev_sep {
        token.pop();
    }
    if token.ends_with('.') {
        token.pop();
        have_dot = false;
    }
    if have_dot {
        Some((token, NumFormat::Dec))
    } else {
        Some((token, NumFormat::DecInt))
    }
}

fn try_num_from(formula: &str) -> Option<(String, NumFormat)> {
    // let mut token = String::new();
    if let Some(c) = formula.chars().next() {
        if c.is_ascii_digit() {
            if formula.starts_with("0x") {
                try_hex_from(formula)
            } else if formula.starts_with("0b") {
                try_bin_from(formula)
            } else if formula.starts_with("0.") {
                try_dec0_from(formula)
            } else if c == '0' {
                if let Some(c1) = formula[1..].chars().next() {
                    if ('0'..='7').contains(&c1) {
                        try_oct_from(formula)
                    } else {
                        try_dec_from(formula)
                    }
                } else {
                    Some(("0".to_string(), NumFormat::DecInt))
                }
            } else {
                assert!(c != '0');
                if let Some(c1) = formula[1..].chars().next() {
                    if c1 == '.' {
                        // formula starts_with "[1-9]."
                        try_scientific_from(formula)
                    } else {
                        // formula starts_with "[1-9]"
                        try_dec_from(formula)
                    }
                } else {
                    // formula: [1-9]
                    Some((formula.to_string(), NumFormat::DecInt))
                }
            }
        } else {
            // formula is not starts_with ascii_digit
            None
        }
    } else {
        // formula is empty
        None
    }
}

pub fn tokenize(formulas: &str) -> Result<(Vec<Token>, Vec<usize>), MyError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut token_loc: Vec<usize> = Vec::new();

    // scientific: 1.16E-6
    // let scientific_pat = r"[1-9]\.[0-9]+E(\+|-)[1-9]+";
    // hex: 0x1234, 0x12_34, 0x01
    // let hex_pat = r"0x([0-9a-fA-F]+_?)*[0-9a-fA-F]+";
    // oct: 01234, 0_12_34, 00712
    // let oct_pat = r"0([0-7]+_?)*[0-7]+";
    // bin: 0b1010, 0b10_10, 0b0110
    // let bin_pat = r"0b([0-1]+_?)*[0-1]+";
    // dec(!int): '1.234', '1.2_34'
    // let dec_pat = r"([0-9]+(_|,)?)*[0-9]+\.([0-9]+(_|,)?)*[0-9]+";
    // dec(int): '1234', '12_34', '1,234
    // let decint_pat = r"([1-9]+(_|,)?)*[0-9]+";
    /*
    let num =
        Regex::new(r"[1-9]\.[0-9]+E(+|-)[1-9]+|(0x([0-9a-fA-F]+_?)*[0-9a-fA-F]+)|(0([0-7]+_?)*[0-7]+)|(0b([0-1]+_?)*[0-1]+)|(([0-9]+(_|,)?)*[0-9]+\.([0-9]+(_|,)?)*[0-9]+)|(([0-9]+(_|,)?)*[0-9]+)")
            .unwrap();
    */
    /*
    let num_pat = format!(
        r"^((?P<scientific>{})|(?P<hex>{})|(?P<oct>{})|(?P<bin>{})|(?P<dec>{})|(?P<decint>{}))",
        scientific_pat, hex_pat, oct_pat, bin_pat, dec_pat, decint_pat
    );
    let num = Regex::new(&num_pat).unwrap();
    */
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
            // let tex_command = Regex::new(r"^\\[A-Za-z]*").unwrap();
            let mut token = String::from("\\");
            for cf in formulas[1..].chars() {
                if cf.is_ascii_uppercase() || cf.is_ascii_lowercase() {
                    token.push(cf);
                } else {
                    break;
                }
            }
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
        } else if c == ':' {
            // let tsc_command = Regex::new(r"^:[A-Za-z]*").unwrap();
            let mut token = String::from(":");
            for cf in formulas[1..].chars() {
                if cf.is_ascii_uppercase() || cf.is_ascii_lowercase() {
                    token.push(cf);
                } else {
                    break;
                }
            }
            let token_len = token.len();
            push_token!(token, token_len, TokenKind::TkTscCommand);
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if "+-*=/!_^|".contains(c) {
            // let operator = Regex::new(r"^(\+|-|\*|=|/|!|_|\^|\|)").unwrap();
            let token = c.to_string();
            let token_len = token.len();
            push_token!(token, token_len, TokenKind::TkOperator);
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if "()[]{}".contains(c) {
            // let braces = Regex::new(r"^(\(|\)|\[|\]|\{|\})").unwrap();
            let token = c.to_string();
            let token_len = token.len();
            push_token!(token, token_len, TokenKind::TkBrace);
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if ";".contains(c) {
            // let separator = Regex::new(r"^;").unwrap();
            let token = c.to_string();
            let token_len = token.len();
            push_token!(token, token_len, TokenKind::TkSeparaotr);
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if let Some((token, num_format)) = try_num_from(formulas) {
            let token_len = token.len();
            push_token!(token, token_len, TokenKind::TkNum(num_format));
            formulas = &formulas[token_len..];
            ismatch = true;
        } else if c.is_ascii_uppercase() || c.is_ascii_lowercase() {
            // let var = Regex::new(r"^[A-Za-z][A-Za-z0-9]*").unwrap();
            let mut token = String::from(c);
            for ch in formulas[1..].chars() {
                if ch.is_ascii_uppercase() || ch.is_ascii_lowercase() || ch.is_ascii_digit() {
                    token.push(ch);
                } else {
                    break;
                }
            }
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
        let formulas = "1.16E-6 * 0x1 - \\frac{\\sin \\pi}{0b10} / 0x12 + 0.2; \\log a ;a=3";
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
            new_token(";", TokenKind::TkSeparaotr),
            new_token("\\log", TokenKind::TkTexCommand),
            new_token("a", TokenKind::TkVariable),
            new_token(";", TokenKind::TkSeparaotr),
            new_token("a", TokenKind::TkVariable),
            new_token("=", TokenKind::TkOperator),
            new_token("3", TokenKind::TkNum(NumFormat::DecInt)),
            new_token("EOT", TokenKind::TkEOT),
        ];
        let s = vec![
            0, 8, 10, 14, 16, 21, 22, 27, 30, 31, 32, 36, 38, 40, 45, 47, 50, 52, 57, 59, 60, 61,
            62, 63,
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
    #[test]
    fn test_tokenize_regression_numeric_literals() {
        use NumFormat::*;
        use TokenKind::*;

        let cases: Vec<(&str, Vec<Token>)> = vec![
            // oct / decint around leading 0
            (
                "07",
                vec![new_token("07", TkNum(Oct)), new_token("EOT", TkEOT)],
            ),
            (
                "078",
                vec![
                    new_token("07", TkNum(Oct)),
                    new_token("8", TkNum(DecInt)),
                    new_token("EOT", TkEOT),
                ],
            ),
            (
                "08",
                vec![new_token("08", TkNum(DecInt)), new_token("EOT", TkEOT)],
            ),
            (
                "09",
                vec![new_token("09", TkNum(DecInt)), new_token("EOT", TkEOT)],
            ),
            (
                "0_8",
                vec![new_token("0_8", TkNum(DecInt)), new_token("EOT", TkEOT)],
            ),
            // repeated separators in prefixed literals
            (
                "012__34",
                vec![
                    new_token("012", TkNum(Oct)),
                    new_token("_", TkOperator),
                    new_token("_", TkOperator),
                    new_token("34", TkNum(DecInt)),
                    new_token("EOT", TkEOT),
                ],
            ),
            (
                "0b10__10",
                vec![
                    new_token("0b10", TkNum(Bin)),
                    new_token("_", TkOperator),
                    new_token("_", TkOperator),
                    new_token("10", TkNum(DecInt)),
                    new_token("EOT", TkEOT),
                ],
            ),
            (
                "0x1__2",
                vec![
                    new_token("0x1", TkNum(Hex)),
                    new_token("_", TkOperator),
                    new_token("_", TkOperator),
                    new_token("2", TkNum(DecInt)),
                    new_token("EOT", TkEOT),
                ],
            ),
            // dot should not be consumed unless it forms a valid decimal literal
            (
                "12.",
                vec![
                    new_token("12", TkNum(DecInt)),
                    // "." itself is invalid, so this case is tested separately as Err below.
                    // This entry is not used directly.
                ],
            ),
            // separators before variables / invalid chars should not be consumed
            (
                "12_a",
                vec![
                    new_token("12", TkNum(DecInt)),
                    new_token("_", TkOperator),
                    new_token("a", TkVariable),
                    new_token("EOT", TkEOT),
                ],
            ),
            (
                "1.2_a",
                vec![
                    new_token("1.2", TkNum(Dec)),
                    new_token("_", TkOperator),
                    new_token("a", TkVariable),
                    new_token("EOT", TkEOT),
                ],
            ),
        ];

        for (input, expected) in cases {
            // "12." は Err ケースとして下で検査する
            if input == "12." {
                continue;
            }

            let (tokens, _) = super::tokenize(input)
                .unwrap_or_else(|e| panic!("tokenize({input:?}) failed: {e}"));

            assert_eq!(tokens, expected, "input = {input:?}");
        }
    }
    #[test]
    fn test_tokenize_regression_invalid_numeric_literals() {
        let invalid_inputs = [
            "12.", "12.a", "12.e3", "12,a", "1.2,a", "08.0,.", "1_.2", "1,.2", "1.2,.",
        ];

        for input in invalid_inputs {
            assert!(
                super::tokenize(input).is_err(),
                "input should be rejected: {input:?}"
            );
        }
    }
}

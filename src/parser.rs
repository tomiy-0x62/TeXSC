
use std::collections::HashMap;
use std::str::FromStr;

pub mod lexer;

struct Node<'a> {
    token: lexer::Token,
    right_nord: Option<&'a Node<'a>>,
    left_nord: Option<&'a Node<'a>>,
    val: f64,
    is_calced: bool,
}

pub struct Parser<'a> {
    lex: lexer::Lexer,
    vars: &'a HashMap<String, f64>,
}

// token(String)かNodeの参照のどちらか一方を持つデータ構造を用意、このデータ構造を仮にHogeとする
// Hogeを要素に持つVecを用意
// tokenを先頭から見て子を持つtoken(TexCommand, Operator)であれば子がNULLなNodeを形成して
// Hogeに入に格納し、Vecにいれる。
// tokenが子をもたないなら(Val, Num, Bracket)それをそのまま

enum MathError {
    DivisionByZero,
    InvalidHexFormat,
    InvalidBinFormat,
}

impl Parser<'_> {

    pub fn print_vars(&self) {
        for i in self.vars.into_iter() {
            println!("{:?}", i);
        }
    }

    fn hex2dec(num_str: &str) -> Result<f64, MathError> {
        let mut num: f64 = 0.0;
        let mut figure: f64 = 1.0;
        for i in num_str.chars() {
            println!("f64::from_str({})", &i.to_string());
            // TODO: a~f,A~Fが来たときに対応
            match f64::from_str(&i.to_string()) {
                Ok(n) => {
                    num += n * 16.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                },
                Err(_) => return Err(MathError::DivisionByZero),
            }
            println!("num: {}", num);
        }
        Ok(num)
    }

    fn bin2dec(num_str: &str) -> Result<f64, MathError> {
        let mut num: f64 = 0.0;
        let mut figure: f64 = 1.0;
        for i in num_str.chars() {
            match f64::from_str(&i.to_string()) {
                Ok(n) => {
                    // TODO: 0か1以外の数が来たときにエラーを返す
                    num += n * 2.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                },
                Err(_) => return Err(MathError::DivisionByZero),
            }
        }
        Ok(num)
    }
    
    fn f64_from_str(num_str: &str) -> Result<f64, MathError> {
        match &num_str[0..2] {
            "0x"=> match Parser::hex2dec(&num_str[2..]) {
                Ok(num) => {
                    println!("<<<Hex found>>>");
                    Ok(num)
                },
                Err(_) => {
                    println!("<<<Hex found>>>");
                    Err(MathError::DivisionByZero)
                },
            },
            "0b"=> match Parser::bin2dec(&num_str[2..]) {
                Ok(num) => {
                    println!("<<<Bin found>>>");
                    Ok(num)
                },
                Err(_) => {
                    println!("<<<Bin found>>>");
                    Err(MathError::DivisionByZero)
                },
            },
            _ => match f64::from_str(num_str) {
                Ok(num) => {
                    println!("<<<Dec found>>>");
                    Ok(num)
                },
                Err(_) => {
                    println!("<<<Dec found>>>");
                    Err(MathError::DivisionByZero)
                },
            },
        }
        
    }

    pub fn new(lex: lexer::Lexer, vars: &mut HashMap<String, f64>) -> Parser {
        // lex から varsを構築
        for i in 0..lex.tokens.len() {
            if lex.tokens[i].token == "," {
                match lex.tokens[i+1].token_type {
                    lexer::TokenType::Var => {},
                    _ => panic!(),
                }
                if !(lex.tokens[i+2].token == "=") {
                    panic!();
                } 
                match lex.tokens[i+3].token_type {
                    // 定数の置き換え
                    lexer::TokenType::Num => {
                        match Parser::f64_from_str(&lex.tokens[i+3].token) {
                            Ok(num) => {
                                vars.insert(lex.tokens[i+1].token.clone(), num);
                            },
                            Err(_) => panic!("f64::from_str(\"{}\") failed", &lex.tokens[i+3].token),
                        }
                    },
                    lexer::TokenType::Var => {
                        if lex.tokens[i+3].token == "e" {
                            vars.insert(lex.tokens[i+1].token.clone(), std::f64::consts::E);
                        } else {
                            panic!();
                        }
                    }
                    lexer::TokenType::TexCommand => {
                        if lex.tokens[i+3].token == "\\pi" {
                            vars.insert(lex.tokens[i+1].token.clone(), std::f64::consts::PI);
                        } else {
                            panic!();
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        Parser { lex: lex, vars: vars }
    }
    /* 
    pub fn build_tree(lex: lexer::Lexer) -> Nord {
        // インタラクティブモードと、その他のモードで変数の扱いが違う
        // ファイル、コマンドラインは1行で完結

        let mut _stack: Vec<String> = Vec::new();
        let mut index: usize = 0;
        loop {
            match lex.tokens[index].token_type {
                lexer::TokenType::TexCommand => {},
                lexer::TokenType::Operator => {
                },
                _ => {},
            }
            index += 1;
        }
    
    }*/
}

use std::collections::HashMap;

pub mod lexer;

struct Nord<'a> {
    token: lexer::Token,
    right_nord: Option<&'a Nord<'a>>,
    left_nord: Option<&'a Nord<'a>>,
    val: f64,
    is_calced: bool,
}

pub struct Parser<'a> {
    lex: lexer::Lexer,
    vars: &'a HashMap<String, f64>,
}

impl Parser<'_> {

    pub fn new(lex: lexer::Lexer, vars: &HashMap<String, f64>) -> Parser<'static> {
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
                        match lex.tokens[i+1].token.parse::<f64>() {
                            Ok(num) => {
                                match vars.insert(lex.tokens[i+1].token, num) {
                                    Some(_) => {},
                                    None => panic!(),
                                }
                            },
                            Err(_) => panic!(),
                        }
                    },
                    lexer::TokenType::Var => {
                        if lex.tokens[i+3].token == "e" {
                            vars.insert(lex.tokens[i+1].token, std::f64::consts::E);
                        } else {
                            panic!();
                        }
                    }
                    lexer::TokenType::TexCommand => {
                        if lex.tokens[i+3].token == "\\pi" {
                            vars.insert(lex.tokens[i+1].token, std::f64::consts::PI);
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

    pub fn build_tree(lex: lexer::Lexer) -> Nord<'static> {
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
    
    }
}
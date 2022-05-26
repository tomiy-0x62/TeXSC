
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

impl Parser<'_> {

    pub fn print_vars(&self) {
        for i in self.vars.into_iter() {
            println!("{:?}", i);
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
                        match f64::from_str(&lex.tokens[i+3].token) {
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

use std::collections::HashMap;
use std::str::FromStr;

pub mod lexer;

pub enum NodeKind {
    NdAdd,
    NdSub,
    NdMul,
    NdDiv,
    NdNum,
    NdSin,
    NdCos,
    NdTan,
    NdCsc,
    NdSec,
    NdCot,
    NdSqrt,
    NdFrac,
    NdLog,
}

struct Node<'a> {
    node_kind: NodeKind,
    right_nord: Option<&'a Node<'a>>,
    left_nord: Option<&'a Node<'a>>,
    val: Option<f64>,
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
            match f64::from_str(&i.to_string()) {
                Ok(n) => {
                    num += n * 16.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                },
                Err(_) => {
                    let n: f64 = match &i.to_string()[0..1] {
                        "a" | "A" => 10.0,
                        "b" | "B" => 11.0,
                        "c" | "C" => 12.0,
                        "d" | "D" => 13.0,
                        "e" | "E" => 14.0,
                        "f" | "F" => 15.0,
                        _ => return Err(MathError::InvalidHexFormat),
                    };
                    num += n * 16.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                },
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
                    if n > 1.0_f64 { return Err(MathError::InvalidBinFormat) }
                    num += n * 2.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                },
                Err(_) => return Err(MathError::DivisionByZero),
            }
        }
        Ok(num)
    }
    
    fn f64_from_str(num_str: &str) -> Result<f64, MathError> {
        if num_str.len() < 2 {
            match f64::from_str(num_str) {
                Ok(num) => {
                    println!("<<<Dec found>>>");
                    return Ok(num);
                },
                Err(_) => {
                    println!("<<<Dec found>>>");
                    return Err(MathError::DivisionByZero);
                },
            }
        } else {
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
        
    }

    pub fn new(lex: lexer::Lexer, vars: &mut HashMap<String, f64>) -> Parser {
        // lex から varsを構築
        for i in 0..lex.tokens.len() {
            if lex.tokens[i].token == "," {
                match lex.tokens[i+1].token_kind {
                    lexer::TokenKind::TkVariable => {},
                    _ => panic!(),
                }
                if !(lex.tokens[i+2].token == "=") {
                    panic!();
                } 
                match lex.tokens[i+3].token_kind {
                    // 定数の置き換え
                    lexer::TokenKind::TkNum => {
                        match Parser::f64_from_str(&lex.tokens[i+3].token) {
                            Ok(num) => {
                                vars.insert(lex.tokens[i+1].token.clone(), num);
                            },
                            Err(_) => panic!("f64::from_str(\"{}\") failed", &lex.tokens[i+3].token),
                        }
                    },
                    lexer::TokenKind::TkVariable => {
                        if lex.tokens[i+3].token == "e" {
                            vars.insert(lex.tokens[i+1].token.clone(), std::f64::consts::E);
                        } else {
                            panic!();
                        }
                    }
                    lexer::TokenKind::TkTexCommand => {
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
    // TODO: consider life time
    fn new_node(&self, kind: NodeKind, left: Node, right: Node) -> Node {
        Node { node_kind: kind, right_nord: Some(&right), left_nord: Some(&left), val: None }
    }

    fn new_node_num(&mut self, val: f64) -> Node {
        Node { node_kind: NodeKind::NdNum, right_nord: None, left_nord: None, val: Some(val) }
    }

    fn expr(&self) -> Node {
        let mut node: Node = self.mul();
        loop {
            if self.lex.consume("+".to_string()) {
                node = self.new_node(NodeKind::NdAdd, node, self.mul());
            } else if self.lex.consume("-".to_string()) {
                node = self.new_node(NodeKind::NdSub, node, self.mul());
            } else {
                return node;
            }
        }
    }

    fn mul(&self) -> Node {
        let mut node: Node = self.primary();
        loop {
            if self.lex.consume("*".to_string()) {
                node = self.new_node(NodeKind::NdMul, node, self.primary());
            } else if self.lex.consume("/".to_string()) {
                node = self.new_node(NodeKind::NdDiv, node, self.primary());
            } else {
                return node;
            }
        }
    }

    fn primary(&self) -> Node {
        if self.lex.consume("(".to_string()) {
            let node: Node = self.expr();
            self.lex.expect(")".to_string());
            return node;
        }
        let val:f64 = match self.lex.expect_number() {
            Ok(v) => {
                match Parser::f64_from_str(&v) {
                    Ok(v) => v,
                    Err(e) => panic!(e),
                }
            },
            Err(e) => panic!(e),
        };
        return self.new_node_num(val);

    }
}
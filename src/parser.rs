
use std::collections::HashMap;
use std::str::FromStr;

pub mod lexer;

pub enum NodeKind {
    // 前置, 1引数
    NdSin,
    NdCos,
    NdTan,
    NdCsc,
    NdSec,
    NdCot,
    NdSqrt,
    NdLog,
    NdAbs,
    // 前置, 2引数
    NdFrac,
    // 中置
    NdAdd,
    NdSub,
    NdMul,
    NdDiv,
    // 数字
    NdNum,
}

pub struct Node {
    pub node_kind: NodeKind,
    pub right_node: Option<Box<Node>>,
    pub left_node: Option<Box<Node>>,
    pub val: Option<f64>,
}

pub struct Parser<'a> {
    lex: lexer::Lexer,
    vars: &'a HashMap<String, f64>,
}

enum ParseNumLiteralError {
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

    fn hex2dec(num_str: &str) -> Result<f64, ParseNumLiteralError> {
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
                        _ => return Err(ParseNumLiteralError::InvalidHexFormat),
                    };
                    num += n * 16.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                },
            }
            println!("num: {}", num);
        }
        Ok(num)
    }

    fn bin2dec(num_str: &str) -> Result<f64, ParseNumLiteralError> {
        let mut num: f64 = 0.0;
        let mut figure: f64 = 1.0;
        for i in num_str.chars() {
            match f64::from_str(&i.to_string()) {
                Ok(n) => {
                    if n > 1.0_f64 { return Err(ParseNumLiteralError::InvalidBinFormat) }
                    num += n * 2.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                },
                Err(_) => return Err(ParseNumLiteralError::DivisionByZero),
            }
        }
        Ok(num)
    }
    
    fn f64_from_str(num_str: &str) -> Result<f64, ParseNumLiteralError> {
        if num_str.len() < 2 {
            match f64::from_str(num_str) {
                Ok(num) => {
                    return Ok(num);
                },
                Err(_) => {
                    return Err(ParseNumLiteralError::DivisionByZero);
                },
            }
        } else {
            match &num_str[0..2] {
                "0x" => match Parser::hex2dec(&num_str[2..]) {
                    Ok(num) => {
                        Ok(num)
                    },
                    Err(_) => {
                        Err(ParseNumLiteralError::DivisionByZero)
                    },
                },
                "0b" => match Parser::bin2dec(&num_str[2..]) {
                    Ok(num) => {
                        Ok(num)
                    },
                    Err(_) => {
                        Err(ParseNumLiteralError::DivisionByZero)
                    },
                },
                _ => match f64::from_str(num_str) {
                    Ok(num) => {
                        Ok(num)
                    },
                    Err(_) => {
                        Err(ParseNumLiteralError::DivisionByZero)
                    },
                },
            }
    }
        
    }

    pub fn new(mut lex: lexer::Lexer, vars: &mut HashMap<String, f64>) -> Parser {
        // lex から varsを構築
        // TODO: lex.tokensから変数部分を削除
        let mut to_delete_el = Vec::<usize>::new();
        for i in 0..lex.tokens.len() {
            if lex.tokens[i].token == "," {
                to_delete_el.push(i);
                match lex.tokens[i+1].token_kind {
                    lexer::TokenKind::TkVariable => to_delete_el.push(i+1),
                    _ => panic!(),
                }
                if !(lex.tokens[i+2].token == "=") {
                    panic!();
                } 
                to_delete_el.push(i+2);
                match lex.tokens[i+3].token_kind {
                    lexer::TokenKind::TkNum => {
                        match Parser::f64_from_str(&lex.tokens[i+3].token) {
                            Ok(num) => {
                                vars.insert(lex.tokens[i+1].token.clone(), num);
                            },
                            Err(_) => panic!("f64::from_str(\"{}\") failed", &lex.tokens[i+3].token),
                        }
                    },
                    _ => panic!(),
                }
                to_delete_el.push(i+3);
            }
        }
        to_delete_el.sort_by(|a, b| b.cmp(a));
        for i in to_delete_el.into_iter() {
            lex.tokens.remove(i);
        }
        Parser { lex: lex, vars: vars }
    }

    pub fn build_ast(&mut self) -> Box<Node> {
        self.expr()
    }

    fn new_node(kind: NodeKind, left: Box<Node>, right: Box<Node>) -> Box<Node> {
        Box::new(Node { node_kind: kind, right_node: Some(right), left_node: Some(left), val: None })
    }

    fn new_node_num(val: f64) -> Box<Node> {
        Box::new(Node { node_kind: NodeKind::NdNum, right_node: None, left_node: None, val: Some(val) })
    }

    fn show_node(place: String, node: &Node) {
        println!("{}: create {{ Kind: {:?}, Val: {:?} }}", place, match node.node_kind {
            NodeKind::NdSin => "NdSin",
            NodeKind::NdCos => "NdCos",
            NodeKind::NdTan => "NdTan",
            NodeKind::NdCsc => "NdCsc",
            NodeKind::NdSec => "NdSec",
            NodeKind::NdCot => "NdCot",
            NodeKind::NdSqrt => "NdSqr",
            NodeKind::NdLog => "NdLog",
            NodeKind::NdAbs => "NdAbs",
            NodeKind::NdFrac => "NdFra",
            NodeKind::NdAdd => "NdAdd",
            NodeKind::NdSub => "NdSub",
            NodeKind::NdMul => "NdMul",
            NodeKind::NdDiv => "NdDiv",
            NodeKind::NdNum => "NdNum",
        }, node.val)
    }

    fn expr(&mut self) -> Box<Node> {
        let mut node: Box<Node> = self.mul();
        loop {
            if self.lex.consume("+".to_string()) {
                node = Parser::new_node(NodeKind::NdAdd, node, self.mul());
            } else if self.lex.consume("-".to_string()) {
                node = Parser::new_node(NodeKind::NdSub, node, self.mul());
            } else {
                Parser::show_node("expr".to_string(), &node);
                return node;
            }
        }
    }

    fn mul(&mut self) -> Box<Node> {
        let mut node: Box<Node> = self.primary();
        Parser::show_node("primary".to_string(), &node);
        loop {
            if self.lex.consume("*".to_string()) {
                node = Parser::new_node(NodeKind::NdMul, node, self.primary());
            } else if self.lex.consume("\\times".to_string()) {
                node = Parser::new_node(NodeKind::NdMul, node, self.primary());
            } else if self.lex.consume("\\cdot".to_string()) {
                node = Parser::new_node(NodeKind::NdMul, node, self.primary());
            }else if self.lex.consume("\\div".to_string()) {
                node = Parser::new_node(NodeKind::NdDiv, node, self.primary());
            } else if self.lex.consume("/".to_string()) {
                node = Parser::new_node(NodeKind::NdDiv, node, self.primary());
            } else {
                Parser::show_node("mul".to_string(), &node);
                return node;
            }
        }
    }

    fn primary(&mut self) -> Box<Node> {
        if self.lex.consume("(".to_string()) {
            let node: Box<Node> = self.expr();
            self.lex.expect(")".to_string());
            return node;
        }
        let val:f64 = match self.lex.expect_number(self.vars) {
            Ok(v) => {
                match Parser::f64_from_str(&v) {
                    Ok(v) => v,
                    Err(e) => panic!("failed to parse '{}' to f64", v),
                }
            },
            Err(e) => panic!(e),
        };
        return Parser::new_node_num(val);

    }
}
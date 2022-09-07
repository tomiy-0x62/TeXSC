
use std::collections::HashMap;
use std::num::ParseFloatError;
use std::str::FromStr;
use thiserror::Error;
use std::fmt;

pub mod lexer;

pub enum NodeKind {
    // 1引数
    NdSin,
    NdCos,
    NdTan,
    NdCsc,
    NdSec,
    NdCot,
    NdAcSin,
    NdAcCos,
    NdAcTan,
    NdSqrt,
    NdLog,
    NdLn,
    NdAbs,
    NdExp,
    // 2引数
    NdAdd,
    NdSub,
    NdMul,
    NdDiv,
    // 数字
    NdNum,
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::NdSin =>  write!(f, "NdSin"),
            NodeKind::NdCos =>  write!(f, "NdCos"),
            NodeKind::NdTan =>  write!(f, "NdTan"),
            NodeKind::NdCsc =>  write!(f, "NdCsc"),
            NodeKind::NdSec =>  write!(f, "NdSec"),
            NodeKind::NdCot =>  write!(f, "NdCot"),
            NodeKind::NdAcSin =>  write!(f, "NdAcSin"),
            NodeKind::NdAcCos =>  write!(f, "NdAcCos"),
            NodeKind::NdAcTan =>  write!(f, "NdAcTan"),
            NodeKind::NdSqrt => write!(f, "NdSqrt"),
            NodeKind::NdLog =>  write!(f, "NdLog"),
            NodeKind::NdLn =>  write!(f, "NdLn"),
            NodeKind::NdAbs =>  write!(f, "NdAbs"),
            NodeKind::NdExp =>  write!(f, "NdExp"),
            NodeKind::NdAdd =>  write!(f, "NdAdd"),
            NodeKind::NdSub =>  write!(f, "NdSub"),
            NodeKind::NdMul =>  write!(f, "NdMul"),
            NodeKind::NdDiv =>  write!(f, "NdDiv"),
            NodeKind::NdNum =>  write!(f, "NdNum"),
        }
    }
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

#[derive(Debug, Error)]
enum ParseNumError {
    #[error("Invalid hex format: {0}")]
    InvalidHexFormat(String),
    #[error("Invalid binary format: {0}")]
    InvalidBinFormat(String),
    #[error("{0}")]
    CantParse(#[from] ParseFloatError)
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("{0}")]
    UnExpectedToken(#[from] lexer::TkError),
    #[error("Undiffined command: {0}")]
    UDcommandErr(String),
    #[error("{0}")]
    CantParse(#[from] ParseNumError),
    #[error("There is no token to process")]
    NoToken,
}

impl Parser<'_> {

    pub fn print_vars(&self) {
        for i in self.vars.into_iter() {
            println!("{:?}", i);
        }
    }

    fn hex2dec(num_str: &str) -> Result<f64, ParseNumError> {
        let mut num: f64 = 0.0;
        let mut figure: f64 = 1.0;
        for i in num_str.chars() {
            // println!("f64::from_str({})", &i.to_string());
            match f64::from_str(&i.to_string()) {
                Ok(n) => {
                    num += n * 16.0_f64.powf(num_str.len() as f64 - figure);
                    figure += 1.0;
                },
                Err(_) => {
                    let n: f64 = match &i.to_string()[0..1] {
                        "a" | "A" => 10.0,
                        "b" | "B" => 11.0,
                        "c" | "C" => 12.0,
                        "d" | "D" => 13.0,
                        "e" | "E" => 14.0,
                        "f" | "F" => 15.0,
                        _ => return Err(ParseNumError::InvalidHexFormat(num_str.to_string())),
                    };
                    num += n * 16.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                },
            }
            // println!("num: {}", num);
        }
        Ok(num)
    }

    fn bin2dec(num_str: &str) -> Result<f64, ParseNumError> {
        let mut num: f64 = 0.0;
        let mut figure: f64 = 1.0;
        for i in num_str.chars() {
            match f64::from_str(&i.to_string()) {
                Ok(n) => {
                    if n > 1.0_f64 { return Err(ParseNumError::InvalidBinFormat(num_str.to_string())) }
                    num += n * 2.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                },
                Err(e) => return Err(ParseNumError::CantParse(e)),
            }
        }
        Ok(num)
    }
    
    fn f64_from_str(num_str: &str) -> Result<f64, ParseNumError> {
        if num_str.len() < 2 {
            match f64::from_str(num_str) {
                Ok(num) => {
                    return Ok(num);
                },
                Err(e) => return Err(ParseNumError::CantParse(e)),
            }
        } else {
            match &num_str[0..2] {
                "0x" => Parser::hex2dec(&num_str[2..]),
                "0b" => Parser::bin2dec(&num_str[2..]),
                _ => match f64::from_str(num_str) {
                    Ok(num) => {
                        Ok(num)
                    },
                    Err(e) => return Err(ParseNumError::CantParse(e)),
                },
            }
        }
    }

    pub fn new(mut lex: lexer::Lexer, vars: &mut HashMap<String, f64>) -> Parser {
        // lex から varsを構築
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

    pub fn build_ast(&mut self) -> Result<Box<Node>, ParserError> {
        if self.lex.is_eot() {
            return Err(ParserError::NoToken);
        }
        let ast = self.expr();
        if !self.lex.is_eot() {
            return Err(ParserError::UDcommandErr(self.lex.now_token().to_string()));
        }
        ast
    }

    fn new_node(kind: NodeKind, left: Box<Node>, right: Box<Node>) -> Box<Node> {
        Box::new(Node { node_kind: kind, right_node: Some(right), left_node: Some(left), val: None })
    }

    fn new_unary_node(kind: NodeKind, left: Box<Node>) -> Box<Node> {
        Box::new(Node { node_kind: kind, right_node: None, left_node: Some(left), val: None })
    }

    fn new_node_num(val: f64) -> Box<Node> {
        Box::new(Node { node_kind: NodeKind::NdNum, right_node: None, left_node: None, val: Some(val) })
    }

    fn show_node(place: String, node: &Node) {
        println!("{}: create {{ Kind: {}, Val: {:?} }}", place, node.node_kind.to_string(), node.val)
    }

    /*
    expr    = mul ("+" mul | "-" mul)*
    mul     = primary ("*" primary | "/" primary | "\cdto" primary | "\times" primary | "\div" primary)*
    primary = num | "(" expr ")" | "\frac" "{" expr "}" "{" expr "}" | "\sqrt" "{" expr "} | "\log" "{" expr "} | "\ln" "{" expr "} | "\sin" "{" expr "} | "\cos" "{" expr "} | "\tan" "{" expr "}
                | "\exp" "{" expr "}
    */

    fn expr(&mut self) -> Result<Box<Node>, ParserError> {
        let mut node: Box<Node> = self.mul()?;
        loop {
            if self.lex.consume("+".to_string()) {
                node = Parser::new_node(NodeKind::NdAdd, node, self.mul()?);
            } else if self.lex.consume("-".to_string()) {
                node = Parser::new_node(NodeKind::NdSub, node, self.mul()?);
            } else {
                // Parser::show_node("expr".to_string(), &node);
                return Ok(node);
            }
        }
    }

    fn mul(&mut self) -> Result<Box<Node>, ParserError> {
        let mut node: Box<Node> = self.primary()?;
        // Parser::show_node("primary".to_string(), &node);
        loop { // why loop?
            if self.lex.consume("*".to_string()) {
                node = Parser::new_node(NodeKind::NdMul, node, self.primary()?);
            } else if self.lex.consume("\\times".to_string()) {
                node = Parser::new_node(NodeKind::NdMul, node, self.primary()?);
            } else if self.lex.consume("\\cdot".to_string()) {
                node = Parser::new_node(NodeKind::NdMul, node, self.primary()?);
            }else if self.lex.consume("\\div".to_string()) {
                node = Parser::new_node(NodeKind::NdDiv, node, self.primary()?);
            } else if self.lex.consume("/".to_string()) {
                node = Parser::new_node(NodeKind::NdDiv, node, self.primary()?);
            } else {
                // Parser::show_node("mul".to_string(), &node);
                return Ok(node);
            }
        }
    }

    fn primary(&mut self) -> Result<Box<Node>, ParserError> {
        if self.lex.consume("(".to_string()) {
            let node: Box<Node> = self.expr()?;
            match self.lex.expect(")".to_string()) {
                Ok(_) => (),
                Err(e) => return Err(ParserError::UnExpectedToken(e)),
            };
            return Ok(node);
        }
        if self.lex.consume("\\frac".to_string()) {
            match self.lex.expect("{".to_string()) {
                Ok(_) => (),
                Err(e) => return Err(ParserError::UnExpectedToken(e)),
            };
            let lnode: Box<Node> = self.expr()?;
            match self.lex.expect("}".to_string()) {
                Ok(_) => (),
                Err(e) => return Err(ParserError::UnExpectedToken(e)),
            };
            match self.lex.expect("{".to_string()) {
                Ok(_) => (),
                Err(e) => return Err(ParserError::UnExpectedToken(e)),
            };
            let rnode: Box<Node> = self.expr()?;
            match self.lex.expect("}".to_string()) {
                Ok(_) => (),
                Err(e) => return Err(ParserError::UnExpectedToken(e)),
            };
            let node = Parser::new_node(NodeKind::NdDiv, lnode, rnode);
            return Ok(node);
        }

        if self.lex.consume("\\sqrt".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdSqrt, self.parg_node()?));
        }
        if self.lex.consume("\\log".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdLog, self.expr()?));
        }
        if self.lex.consume("\\ln".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdLn, self.expr()?));
        }
        if self.lex.consume("\\exp".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdExp, self.carg_node()?));
        }
        if self.lex.consume("\\sin".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdSin, self.expr()?));
        }
        if self.lex.consume("\\cos".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdCos, self.expr()?));
        }
        if self.lex.consume("\\tan".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdTan, self.expr()?));
        }
        if self.lex.consume("\\arcsin".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdAcSin, self.expr()?));
        }
        if self.lex.consume("\\arccos".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdAcCos, self.expr()?));
        }
        if self.lex.consume("\\arctan".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdAcTan, self.expr()?));
        }
        let val:f64 = match self.lex.expect_number(self.vars) {
            Ok(v) => {
                match Parser::f64_from_str(&v) {
                    Ok(v) => v,
                    Err(e) => return Err(ParserError::CantParse(e)),
                }
            },
            Err(e) => return Err(ParserError::UnExpectedToken(e)),
        };
        return Ok(Parser::new_node_num(val));

    }

    fn parg_node(&mut self) -> Result<Box<Node>, ParserError> {
        match self.lex.expect("{".to_string()) {
            Ok(_) => (),
            Err(e) => return Err(ParserError::UnExpectedToken(e)),
        };
        let node: Box<Node> = self.expr()?;
        match self.lex.expect("}".to_string()) {
            Ok(_) => (),
            Err(e) => return Err(ParserError::UnExpectedToken(e)),
        };
        Ok(node)
    }

    fn carg_node(&mut self) -> Result<Box<Node>, ParserError> {
        match self.lex.expect("(".to_string()) {
            Ok(_) => (),
            Err(e) => return Err(ParserError::UnExpectedToken(e)),
        };
        let node: Box<Node> = self.expr()?;
        match self.lex.expect(")".to_string()) {
            Ok(_) => (),
            Err(e) => return Err(ParserError::UnExpectedToken(e)),
        };
        Ok(node)
    }
}
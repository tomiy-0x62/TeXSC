
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;
use std::fmt;
use self::lexer::Token;

use super::config::*;
use super::error::*;

pub mod lexer;

use super::debugln;

#[derive(Clone, Copy)]
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

struct NodeInfo {
    pub node_kind: NodeKind,
    pub rnode_num: Option<usize>,
    pub lnode_num: Option<usize>,
    pub val: Option<f64>,
    pub node_num: u64,
}

pub struct Parser<'a> {
    lex: lexer::Lexer,
    vars: &'a HashMap<String, f64>,
}

impl Parser<'_> {

    pub fn print_vars(&self) {
        for i in self.vars.into_iter() {
            debugln!("{:?}", i);
        }
    }

    pub fn new(mut lex: lexer::Lexer, vars: &mut HashMap<String, f64>) -> Result<Parser, MyError> {
        // lex から varsを構築
        let mut to_delete_el = Vec::<usize>::new();
        for i in 0..lex.tokens.len() {
            if lex.tokens[i].token == "," {
                to_delete_el.push(i);
                match lex.tokens[i+1].token_kind {
                    lexer::TokenKind::TkVariable => to_delete_el.push(i+1),
                    _ => return Err(MyError::NotTkVariable(lex.tokens[i+1].token_kind.to_string())),
                }
                if !(lex.tokens[i+2].token == "=") {
                    return Err(MyError::UnexpectedToken("=".to_string(), lex.tokens[i+2].token.clone()));
                } 
                to_delete_el.push(i+2);
                match lex.tokens[i+3].token_kind {
                    lexer::TokenKind::TkNum => {
                        match Parser::f64_from_str(&lex.tokens[i+3].token) {
                            Ok(num) => {
                                vars.insert(lex.tokens[i+1].token.clone(), num);
                            },
                            Err(e) => return Err(e),
                        }
                    },
                    _ => return Err(MyError::NotTkNumber(lex.tokens[i+3].token_kind.to_string())),
                }
                to_delete_el.push(i+3);
            } else {
                match lex.tokens[i].token_kind {
                    lexer::TokenKind::TkTscCommand => {
                        to_delete_el.push(i);
                        Parser::process_tsccommand(&lex.tokens[i], &lex.tokens[i+1], vars)?;
                        to_delete_el.push(i+1);
                    },
                    _ => (),
                }
            }
        }
        to_delete_el.sort_by(|a, b| b.cmp(a));
        for i in to_delete_el.into_iter() {
            lex.tokens.remove(i);
        }
        Ok(Parser { lex: lex, vars: vars })
    }

    pub fn build_ast(&mut self) -> Result<Box<Node>, MyError> {
        if self.lex.is_eot() {
            return Err(MyError::NoToken);
        }
        let ast = self.expr()?;
        if !self.lex.is_eot() {
            return Err(MyError::UnprocessedToekn(self.lex.now_token().to_string()));
        }
        Parser::show_ast(&ast);
        Ok(ast)
    }

    fn show_ast(ast: &Box<Node>) {
        let conf = read_config().unwrap();
        if cfg!(debug_assertions) || conf.debug {
            let mut node_que = VecDeque::new();
            let mut lyer_que = VecDeque::new();
            let mut node_num = 0;
            let mut node: &Box<Node> = ast  ;
            let mut qln: usize = 1;
            let mut lyer = 0;
            let mut lyered_nodes: Vec<Vec<NodeInfo>> = Vec::new();
            node_que.push_back(node);
            lyer_que.push_back(lyer);
            loop {
                node = match node_que.pop_front() {
                    Some(n) => n,
                    None => break,
                };
                lyer = lyer_que.pop_front().unwrap();
                if lyered_nodes.len() <= lyer {
                    lyered_nodes.push(Vec::new())
                }
                let ln: Option<usize>;
                match &node.left_node {
                    Some(n) => {
                        node_que.push_back(&n);
                        lyer_que.push_back(lyer+1);
                        ln = Some(qln);
                        qln += 1;
                    },
                    None => ln = None,
                }
                let rn: Option<usize>;
                match &node.right_node {
                    Some(n) => {
                        node_que.push_back(&n);
                        lyer_que.push_back(lyer+1);
                        rn = Some(qln);
                        qln += 1;
                    },
                    None => rn = None,
                }
                Parser::print_node(node, lyer as u64, node_num, ln, rn);
                lyered_nodes[lyer].push(NodeInfo {node_kind: node.node_kind, rnode_num: rn, lnode_num: ln, val: node.val, node_num: node_num});

                node_num += 1;
            };

            let mut msg: String = "".to_string();
            for lyer in lyered_nodes {
                for i in 0..lyer.len() {
                    msg += &format!("|----N{:<3}----|    ", lyer[i].node_num);
                }
                msg += "\n";
                for i in 0..lyer.len() {
                    match lyer[i].node_kind {
                        NodeKind::NdNum => msg += &format!("| {:^10} |    ", match lyer[i].val { Some(v) => v.to_string(), None => "None".to_string()}),
                        _ => msg += &format!("|   {:^10}    |    ", lyer[i].node_kind),
                    }
                }
                msg += "\n";
                for i in 0..lyer.len() {
                    msg += &format!("|-N{:<3}--N{:<3}-|    ",
                                    match lyer[i].lnode_num {
                                        Some(v) => v.to_string(),
                                        None => "X".to_string()
                                    },
                                    match lyer[i].rnode_num {
                                        Some(v) => v.to_string(),
                                        None => "X".to_string()
                                    });
                }
                msg += "\n\n";
            }
            eprintln!("{}", msg);
        }
    }

    fn print_node(node: &Box<Node>, lyer: u64, node_num: u64, ln: Option<usize>, rn: Option<usize>) {
        debugln!("{}: lyer: {}, kind: {}, val: {}, left: {}, right: {}",
                node_num,
                lyer,
                node.node_kind.to_string(),
                match node.val {
                    Some(v) => format!("{:.*}", 4, v),
                    None => "None".to_string(),
                },
                match ln {
                    Some(v) => v.to_string(),
                    None => "None".to_string(),
                },
                match rn {
                    Some(v) => v.to_string(),
                    None => "None".to_string(),
                });
    }

    fn process_tsccommand(t1: &Token, t2: &Token, vars: &mut HashMap<String, f64>) -> Result<(), MyError> {
        Ok(match &*t1.token {
            ":debug" => {
                match &*t2.token {
                    "true" => set_dbconfig(true)?,
                    "false" => set_dbconfig(false)?,
                    _ => return Err(MyError::UnexpectedInput("true/false".to_string(), t2.token.clone())),
                }
            },
            ":logbase" => {
                match t2.token_kind {
                    lexer::TokenKind::TkNum => {
                        match Parser::f64_from_str(&t2.token) {
                            Ok(num) => set_lbconf(num)?,
                            Err(e) => return Err(e),
                        }
                    },
                    lexer::TokenKind::TkVariable => {
                        match vars.get(&t2.token) {
                            Some(num) => set_lbconf(*num)?,
                            None => return Err(MyError::UDvariableErr(t2.token.to_string())),
                        }
                    }
                    _ => return Err(MyError::NotTkNumber(t2.token_kind.to_string())),
                }
            },
            ":rfotmat" => {
                match &*t2.token {
                    "bin" => set_rfconf(ResultFormat::Binary)?,
                    "dec" => set_rfconf(ResultFormat::Decimal)?,
                    "hex" => set_rfconf(ResultFormat::Hexadecimal)?,
                    _ => return Err(MyError::UnexpectedInput("bin/dec/hex".to_string(), t2.token.clone())),
                }
            },
            ":rlen" => {
                match t2.token_kind {
                    lexer::TokenKind::TkNum => {
                        match Parser::f64_from_str(&t2.token) {
                            Ok(num) => set_ndconf(num as u32)?,
                            Err(e) => return Err(e),
                        }
                    },
                    lexer::TokenKind::TkVariable => {
                        match vars.get(&t2.token) {
                            Some(num) => set_ndconf(*num as u32)?,
                            None => return Err(MyError::UDvariableErr(t2.token.to_string())),
                        }
                    }
                    _ => return Err(MyError::NotTkNumber(t2.token_kind.to_string())),
                }
            },
            ":trarg" => {
                match &*t2.token {
                    "rad" => set_tfconf(TrigFuncArg::Radian)?,
                    "deg" => set_tfconf(TrigFuncArg::Degree)?,
                    _ => return Err(MyError::UnexpectedInput("rad/deg".to_string(), t2.token.clone())),
                }
            },
            ":help" => (),
            ":show" => {
                match &*t2.token {
                    "var" => (),
                    "const" => (),
                    "config" => (),
                    _ => return Err(MyError::UnexpectedInput("var/const/config".to_string(), t2.token.clone())),
                }
            },
            _ => return Err(MyError::UDtsccommand(t2.token.clone())),
        })
    }

    fn hex2dec(num_str: &str) -> Result<f64, MyError> {
        let mut num: f64 = 0.0;
        let mut figure: f64 = 1.0;
        for i in num_str.chars() {
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
                        _ => return Err(MyError::InvalidHexFormat(num_str.to_string())),
                    };
                    num += n * 16.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                },
            }
        }
        Ok(num)
    }

    fn bin2dec(num_str: &str) -> Result<f64, MyError> {
        let mut num: f64 = 0.0;
        let mut figure: f64 = 1.0;
        for i in num_str.chars() {
            match f64::from_str(&i.to_string()) {
                Ok(n) => {
                    if n > 1.0_f64 { return Err(MyError::InvalidBinFormat(num_str.to_string())) }
                    num += n * 2.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                },
                Err(e) => return Err(MyError::ParseFloatError(e)),
            }
        }
        Ok(num)
    }
    
    fn f64_from_str(num_str: &str) -> Result<f64, MyError> {
        if num_str.len() < 2 {
            match f64::from_str(num_str) {
                Ok(num) => {
                    return Ok(num);
                },
                Err(e) => return Err(MyError::ParseFloatError(e)),
            }
        } else {
            match &num_str[0..2] {
                "0x" => Parser::hex2dec(&num_str[2..]),
                "0b" => Parser::bin2dec(&num_str[2..]),
                _ => match f64::from_str(num_str) {
                    Ok(num) => {
                        Ok(num)
                    },
                    Err(e) => return Err(MyError::ParseFloatError(e)),
                },
            }
        }
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
        debugln!("{}: create {{ Kind: {}, Val: {:?} }}", place, node.node_kind.to_string(), node.val);
    }

    /*
    expr    = mul ("+" mul | "-" mul)*
    mul     = primary ("*" primary | "/" primary | "\cdto" primary | "\times" primary | "\div" primary)*
    primary = num | "(" expr ")" | "\frac" "{" expr "}" "{" expr "}" | "\sqrt" "{" expr "} | "\log"  expr | "\ln" expr | "\sin" expr | "\cos" expr | "\tan" expr
                | "\exp" "(" expr ")" | "\csc" expr | "\sec" expr | "\cot" expr | "\abs" "(" expr ")"
    */


    fn expr(&mut self) -> Result<Box<Node>, MyError> {
        let mut node: Box<Node> = self.mul()?;
        loop {
            if self.lex.consume("+".to_string()) {
                node = Parser::new_node(NodeKind::NdAdd, node, self.mul()?);
            } else if self.lex.consume("-".to_string()) {
                node = Parser::new_node(NodeKind::NdSub, node, self.mul()?);
            } else {
                Parser::show_node("expr".to_string(), &node);
                return Ok(node);
            }
        }
    }

    fn mul(&mut self) -> Result<Box<Node>, MyError> {
        let mut node: Box<Node> = self.primary()?;
        Parser::show_node("primary".to_string(), &node);
        loop {
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
                Parser::show_node("mul".to_string(), &node);
                return Ok(node);
            }
        }
    }

    fn primary(&mut self) -> Result<Box<Node>, MyError> {
        if self.lex.consume("(".to_string()) {
            let node: Box<Node> = self.expr()?;
            self.lex.expect(")".to_string())?;
            return Ok(node);
        }
        if self.lex.consume("\\frac".to_string()) {
            self.lex.expect("{".to_string())?;
            let lnode: Box<Node> = self.expr()?;
            self.lex.expect("}".to_string())?;
            self.lex.expect("{".to_string())?;
            let rnode: Box<Node> = self.expr()?;
            self.lex.expect("}".to_string())?;
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
        if self.lex.consume("\\abs".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdAbs, self.carg_node()?));
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
        if self.lex.consume("\\csc".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdCsc, self.expr()?));
        }
        if self.lex.consume("\\sec".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdSec, self.expr()?));
        }
        if self.lex.consume("\\cot".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdCot, self.expr()?));
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
                Parser::f64_from_str(&v)?
            },
            Err(e) => return Err(e),
        };
        return Ok(Parser::new_node_num(val));

    }

    fn parg_node(&mut self) -> Result<Box<Node>, MyError> {
        self.lex.expect("{".to_string())?;
        let node: Box<Node> = self.expr()?;
        self.lex.expect("}".to_string())?;
        Ok(node)
    }

    fn carg_node(&mut self) -> Result<Box<Node>, MyError> {
        self.lex.expect("(".to_string())?;
        let node: Box<Node> = self.expr()?;
        self.lex.expect(")".to_string())?;
        Ok(node)
    }
}
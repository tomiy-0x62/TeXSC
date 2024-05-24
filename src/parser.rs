use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use self::lexer::NumstrOrVar;
use crate::config::*;
use crate::error::*;
use crate::tsc_cmd;
use crate::CONSTS;

pub mod lexer;

use crate::debugln;

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
    // 前置1引数
    NdNeg,
    // 後置1引数
    NdPow,
    // 数字
    NdNum,
    // 変数
    NdVar,
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::NdSin => write!(f, "NdSin"),
            NodeKind::NdCos => write!(f, "NdCos"),
            NodeKind::NdTan => write!(f, "NdTan"),
            NodeKind::NdCsc => write!(f, "NdCsc"),
            NodeKind::NdSec => write!(f, "NdSec"),
            NodeKind::NdCot => write!(f, "NdCot"),
            NodeKind::NdAcSin => write!(f, "NdAcSin"),
            NodeKind::NdAcCos => write!(f, "NdAcCos"),
            NodeKind::NdAcTan => write!(f, "NdAcTan"),
            NodeKind::NdSqrt => write!(f, "NdSqrt"),
            NodeKind::NdLog => write!(f, "NdLog"),
            NodeKind::NdLn => write!(f, "NdLn"),
            NodeKind::NdAbs => write!(f, "NdAbs"),
            NodeKind::NdExp => write!(f, "NdExp"),
            NodeKind::NdAdd => write!(f, "NdAdd"),
            NodeKind::NdSub => write!(f, "NdSub"),
            NodeKind::NdMul => write!(f, "NdMul"),
            NodeKind::NdDiv => write!(f, "NdDiv"),
            NodeKind::NdNeg => write!(f, "NdNeg"),
            NodeKind::NdPow => write!(f, "NdPow"),
            NodeKind::NdNum => write!(f, "NdNum"),
            NodeKind::NdVar => write!(f, "NdVar"),
        }
    }
}

impl NodeKind {
    fn to_op_str(&self) -> &str {
        match self {
            NodeKind::NdSin => "Sin",
            NodeKind::NdCos => "Cos",
            NodeKind::NdTan => "Tan",
            NodeKind::NdCsc => "Csc",
            NodeKind::NdSec => "Sec",
            NodeKind::NdCot => "Cot",
            NodeKind::NdAcSin => "AcSin",
            NodeKind::NdAcCos => "AcCos",
            NodeKind::NdAcTan => "AcTan",
            NodeKind::NdSqrt => "Sqrt",
            NodeKind::NdLog => "Log",
            NodeKind::NdLn => "Ln",
            NodeKind::NdAbs => "Abs",
            NodeKind::NdExp => "exp",
            NodeKind::NdAdd => "+",
            NodeKind::NdSub => "-",
            NodeKind::NdMul => "*",
            NodeKind::NdDiv => "/",
            NodeKind::NdNeg => "-",
            NodeKind::NdPow => "Pow",
            NodeKind::NdNum => "Num",
            NodeKind::NdVar => "Var",
        }
    }

    fn to_lisp_op_str(&self) -> &str {
        match self {
            NodeKind::NdSin => "sin",
            NodeKind::NdCos => "cos",
            NodeKind::NdTan => "tan",
            NodeKind::NdCsc => "Csc",
            NodeKind::NdSec => "Sec",
            NodeKind::NdCot => "Cot",
            NodeKind::NdAcSin => "asin",
            NodeKind::NdAcCos => "acos",
            NodeKind::NdAcTan => "atan",
            NodeKind::NdSqrt => "sqrt",
            NodeKind::NdLog => "log",
            NodeKind::NdLn => "Ln",
            NodeKind::NdAbs => "abs",
            NodeKind::NdExp => "exp",
            NodeKind::NdAdd => "+",
            NodeKind::NdSub => "-",
            NodeKind::NdMul => "*",
            NodeKind::NdDiv => "/",
            NodeKind::NdNeg => "-",
            NodeKind::NdPow => "expt",
            NodeKind::NdNum => "Num",
            NodeKind::NdVar => "Var",
        }
    }
}

#[derive(Clone, Debug)]
pub enum NumOrVar {
    Num(f64),
    Var(String),
}

pub struct Node {
    pub node_kind: NodeKind,
    pub right_node: Option<Box<Node>>,
    pub left_node: Option<Box<Node>>,
    pub val: Option<NumOrVar>,
}

/*
struct NodeInfo {
    pub node_kind: Option<NodeKind>,
    pub rnode_num: usize,
    pub lnode_num: usize,
    pub val: Option<f64>,
    pub node_num: u64,
}
*/

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
                match lex.tokens[i + 1].token_kind {
                    lexer::TokenKind::TkVariable => to_delete_el.push(i + 1),
                    tk => {
                        return Err(MyError::NotTkVariable(
                            tk.to_string(),
                            lex.format_err_loc_idx(i + 1),
                        ))
                    }
                }
                if !(lex.tokens[i + 2].token == "=") {
                    return Err(MyError::UnexpectedToken(
                        "=".to_string(),
                        lex.tokens[i + 2].token.clone(),
                    ));
                }
                to_delete_el.push(i + 2);
                match lex.tokens[i + 3].token_kind {
                    lexer::TokenKind::TkNum => {
                        match Parser::f64_from_str(&lex.tokens[i + 3].token) {
                            Ok(num) => {
                                vars.insert(lex.tokens[i + 1].token.clone(), num);
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    lexer::TokenKind::TkOperator => {
                        if lex.tokens[i + 3].token == "-" {
                            match lex.tokens[i + 4].token_kind {
                                lexer::TokenKind::TkNum => {
                                    match Parser::f64_from_str(&lex.tokens[i + 4].token) {
                                        Ok(num) => {
                                            vars.insert(lex.tokens[i + 1].token.clone(), -num);
                                            to_delete_el.push(i + 4);
                                        }
                                        Err(e) => return Err(e),
                                    }
                                }
                                tk => {
                                    return Err(MyError::NotTkNumber(
                                        tk.to_string(),
                                        lex.format_err_loc_idx(i + 4),
                                    ))
                                }
                            }
                        } else {
                            return Err(MyError::NotTkNumber(
                                lexer::TokenKind::TkOperator.to_string(),
                                lex.format_err_loc_idx(i + 3),
                            ));
                        }
                    }
                    tk => {
                        return Err(MyError::NotTkNumber(
                            tk.to_string(),
                            lex.format_err_loc_idx(i + 3),
                        ))
                    }
                }
                to_delete_el.push(i + 3);
            } else {
                match lex.tokens[i].token_kind {
                    lexer::TokenKind::TkTscCommand => {
                        to_delete_el.push(i);
                        tsc_cmd::process_tsccommand(&lex, i, vars)?;
                        match &*lex.tokens[i].token {
                            ":help" => (),
                            _ => to_delete_el.push(i + 1),
                        }
                    }
                    _ => (),
                }
            }
        }
        to_delete_el.sort_by(|a, b| b.cmp(a));
        for i in to_delete_el.into_iter() {
            lex.del_token(i);
        }
        // varsに定数をプッシュする
        match CONSTS.read() {
            Ok(consts) => {
                for (name, value) in consts.iter() {
                    vars.insert(name.to_string(), *value);
                }
            }
            Err(e) => return Err(MyError::ConstsReadErr(e.to_string())),
        }
        Ok(Parser { lex, vars })
    }

    pub fn build_ast(&mut self) -> Result<Box<Node>, MyError> {
        if self.lex.is_eot() {
            return Err(MyError::NoToken);
        }
        let ast = self.expr()?;
        if !self.lex.is_eot() {
            return Err(MyError::UnprocessedToekn(
                self.lex.now_token().to_string(),
                self.lex.format_err_loc(),
            ));
        }
        self.show_ast(&ast)?;
        self.show_ast_in_s_expr_rec(&ast)?;
        Ok(ast)
    }

    fn show_ast(&self, ast: &Box<Node>) -> Result<(), MyError> {
        let conf = config_reader()?;
        let is_show_ast = cfg!(debug_assertions)
            || conf.debug
            || conf.ast_format == AstFormat::Tree
            || conf.ast_format == AstFormat::Both;
        if is_show_ast {
            let mut msg = String::new();
            let mut node = ast;
            let mut level = 0;
            let mut tr_node_stack = Vec::new();
            let mut tr_level_stack = Vec::new();
            let mut is_next_have_chiled = true;
            let mut no_show_bar = Vec::new();
            const TREE_WIDTH: i32 = 3;
            loop {
                // edgeを表すやつを追加
                for i in 0..(level * TREE_WIDTH) {
                    if i % TREE_WIDTH == 0 {
                        if i / TREE_WIDTH == level - 1 {
                            if is_next_have_chiled {
                                msg += "├";
                                no_show_bar.retain(|&e| e != level - 1);
                            } else {
                                msg += "└";
                                no_show_bar.push(level - 1);
                            }
                        } else {
                            if !no_show_bar.iter().any(|e| e == &(i / TREE_WIDTH)) {
                                msg += "│";
                            } else {
                                msg += " ";
                            }
                        }
                    } else {
                        if i > TREE_WIDTH * (level - 1) {
                            msg += "─";
                        } else {
                            msg += " ";
                        }
                    }
                }
                // nodeを追加
                match node.node_kind {
                    NodeKind::NdNum | NodeKind::NdVar => match node.val.clone().unwrap() {
                        NumOrVar::Num(n) => msg += &(n.to_string()),
                        NumOrVar::Var(v) => match self.vars.get(&v) {
                            Some(n) => msg += &format!("{} = {}", v, n),
                            None => msg += &v,
                        },
                    },
                    _ => msg += node.node_kind.to_op_str(),
                }
                msg += "\n";
                // treeのトラバース、levelの変更
                if node.left_node.is_some() && node.right_node.is_some() {
                    is_next_have_chiled = true;
                } else {
                    is_next_have_chiled = false;
                }
                if node.right_node.is_some() {
                    tr_node_stack.push(node.right_node.as_ref().unwrap());
                    tr_level_stack.push(level + 1);
                }
                if node.left_node.is_some() {
                    node = node.left_node.as_ref().unwrap();
                    level += 1;
                } else {
                    match tr_node_stack.pop() {
                        Some(n) => {
                            node = n;
                            level = tr_level_stack.pop().unwrap();
                        }
                        None => break,
                    }
                }
            }
            eprintln!("{}", msg);
        }
        Ok(())
    }

    fn show_ast_in_s_expr_rec(&self, node: &Box<Node>) -> Result<(), MyError> {
        let conf = config_reader()?;
        let is_show_ast = cfg!(debug_assertions)
            || conf.debug
            || conf.ast_format == AstFormat::Tree
            || conf.ast_format == AstFormat::Both;
        if is_show_ast {
            let mut s_expr = String::new();
            s_expr = self.show_ast_in_s_expr_rec_inner(node, s_expr, &mut HashSet::new(), false);
            eprintln!("{}\n", s_expr);
        }
        Ok(())
    }

    fn show_ast_in_s_expr_rec_inner(
        &self,
        node: &Box<Node>,
        mut s_expr: String,
        is_var_printed: &mut HashSet<String>,
        is_2arg_left: bool,
    ) -> String {
        match node.node_kind {
            NodeKind::NdNum | NodeKind::NdVar => {
                match node.val.clone().unwrap() {
                    NumOrVar::Num(n) => s_expr += &n.to_string(),
                    NumOrVar::Var(v) => {
                        if v == "\\pi" {
                            s_expr += "pi"
                        } else {
                            if let Some(val) = self.vars.get(&v) {
                                if is_var_printed.get(&v).is_none() {
                                    s_expr = format!("(defvar {} {})\n{}", v, val, s_expr);
                                    is_var_printed.insert(v.clone());
                                }
                            }
                            s_expr += &v
                        }
                    }
                }
                if is_2arg_left {
                    s_expr + " "
                } else {
                    s_expr
                }
            }
            _ => {
                s_expr += &format!("({} ", node.node_kind.to_lisp_op_str());
                if node.left_node.is_some() {
                    s_expr = self.show_ast_in_s_expr_rec_inner(
                        node.left_node.as_ref().unwrap(),
                        s_expr,
                        is_var_printed,
                        true & node.right_node.is_some(),
                    );
                }
                if node.right_node.is_some() {
                    s_expr = self.show_ast_in_s_expr_rec_inner(
                        node.right_node.as_ref().unwrap(),
                        s_expr,
                        is_var_printed,
                        false,
                    );
                }
                s_expr + ") "
            }
        }
    }

    /*
    fn _show_ast(ast: &Box<Node>) {
        let conf = read_config().unwrap();
        if cfg!(debug_assertions) || conf.debug {
            let mut node_que = VecDeque::new();
            let mut lyer_que = VecDeque::new();
            let mut node_num = 0;
            let mut node: Option<&Box<Node>> = Some(ast);
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
                if let Some(unode) = node {
                    let ln: usize;
                    match &unode.left_node {
                        Some(n) => {
                            node_que.push_back(Some(n));
                            lyer_que.push_back(lyer + 1);
                            ln = qln;
                            qln += 1;
                        }
                        None => {
                            node_que.push_back(None);
                            lyer_que.push_back(lyer + 1);
                            ln = qln;
                            qln += 1;
                        }
                    }
                    let rn: usize;
                    match &unode.right_node {
                        Some(n) => {
                            node_que.push_back(Some(n));
                            lyer_que.push_back(lyer + 1);
                            rn = qln;
                            qln += 1;
                        }
                        None => {
                            node_que.push_back(None);
                            lyer_que.push_back(lyer + 1);
                            rn = qln;
                            qln += 1;
                        }
                    }
                    Parser::print_node(unode, lyer as u64, node_num, ln, rn);
                    lyered_nodes[lyer].push(NodeInfo {
                        node_kind: Some(unode.node_kind),
                        rnode_num: rn,
                        lnode_num: ln,
                        val: unode.val,
                        node_num,
                    });

                    node_num += 1;
                } else {
                    node_que.push_back(None);
                    node_que.push_back(None);
                    lyer_que.push_back(lyer + 1);
                    lyer_que.push_back(lyer + 1);
                    qln += 1;
                    qln += 1;
                    debugln!("None");
                    // lyered_nodes[lyer].push(None);
                    lyered_nodes[lyer].push(NodeInfo {
                        node_kind: None,
                        rnode_num: qln - 2,
                        lnode_num: qln - 1,
                        val: None,
                        node_num,
                    });
                    node_num += 1;
                }
                if lyered_nodes[lyer].len() == 2_usize.pow(lyer as u32) {
                    let mut is_all_null = true;
                    for node in &lyered_nodes[lyer] {
                        if let Some(_) = node.node_kind {
                            is_all_null = false;
                        }
                    }
                    if is_all_null {
                        lyered_nodes.pop();
                        break;
                    }
                }
            }

            let ast_height = lyered_nodes.len();
            let mut ml_vec = vec![0; ast_height]; // ml: most left, treeの左側
            let mut bs_vec = vec![0; ast_height]; // bs: box space, box間のスペース
            let mss = 2; // mss: most smallest space, treeの一番下のbox間のスペース
            let box_w = 14;
            for i in 0..lyered_nodes.len() {
                if i == 0 {
                    bs_vec[ast_height - 1] = mss;
                    ml_vec[ast_height - 1] = 0;
                } else {
                    bs_vec[ast_height - 1 - i] = 2 * bs_vec[ast_height - i] + box_w;
                    ml_vec[ast_height - 1 - i] =
                        ml_vec[ast_height - i] + (bs_vec[ast_height - i] + box_w) / 2;
                }
            }
            let mut w_spases = "".to_string();
            let bgt_bs = *bs_vec.iter().max().unwrap(); // bgt_bs: biggest bs
            let bgt_ml = *bs_vec.iter().max().unwrap(); // bgt_ml: biggest ml
            if bgt_bs > bgt_ml {
                for _ in 0..bgt_bs {
                    w_spases += " ";
                }
            } else {
                for _ in 0..bgt_ml {
                    w_spases += " ";
                }
            }
            let mut lyer_idx = 0;
            let mut msg: String = "".to_string();
            for lyer in lyered_nodes {
                let ml_space = &w_spases[..ml_vec[lyer_idx]];
                let box_space = &w_spases[..bs_vec[lyer_idx]];
                msg += ml_space;
                /*for i in 0..lyer.len() {
                    msg += &format!("|----N{:<3}----|{}", lyer[i].node_num, box_space);
                }*/
                msg += "\n";
                msg += ml_space;
                for i in 0..lyer.len() {
                    match lyer[i].node_kind {
                        Some(NodeKind::NdNum) => {
                            msg += &format!(
                                "  {:^10}  {}",
                                match lyer[i].val {
                                    Some(v) => v.to_string(),
                                    None => "None".to_string(),
                                },
                                box_space
                            )
                        }
                        Some(nk) => msg += &format!("  {:^10}  {}", nk.to_string(), box_space),
                        None => msg += &format!("  {:^10}  {}", "None", box_space),
                    }
                }
                msg += "\n";
                msg += ml_space;
                /*for i in 0..lyer.len() {
                    msg += &format!(
                        "|-N{:<3}--N{:<3}-|{}",
                        lyer[i].lnode_num, lyer[i].rnode_num, box_space
                    );
                }*/
                msg += "\n\n";
                lyer_idx += 1;
            }
            eprintln!("{}", msg);
        }
    }
    */

    /*
    fn print_node(node: &Box<Node>, lyer: u64, node_num: u64, ln: usize, rn: usize) {
        debugln!(
            "{}: lyer: {}, kind: {}, val: {}, left: {}, right: {}",
            node_num,
            lyer,
            node.node_kind.to_string(),
            match node.val {
                Some(v) => format!("{:.*}", 4, v),
                None => "None".to_string(),
            },
            ln.to_string(),
            rn.to_string(),
        );
    }
    */

    fn hex2dec(num_str: &str) -> Result<f64, MyError> {
        let mut num: f64 = 0.0;
        let mut figure: f64 = 1.0;
        for i in num_str.chars() {
            match f64::from_str(&i.to_string()) {
                Ok(n) => {
                    num += n * 16.0_f64.powf(num_str.len() as f64 - figure);
                    figure += 1.0;
                }
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
                }
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
                    if n > 1.0_f64 {
                        return Err(MyError::InvalidBinFormat(num_str.to_string()));
                    }
                    num += n * 2.0_f64.powf(num_str.len() as f64 - figure);
                    figure = figure + 1.0;
                }
                Err(e) => return Err(MyError::ParseFloatError(e)),
            }
        }
        Ok(num)
    }

    pub fn f64_from_str(num_str: &str) -> Result<f64, MyError> {
        if num_str.len() < 2 {
            match f64::from_str(num_str) {
                Ok(num) => {
                    return Ok(num);
                }
                Err(e) => return Err(MyError::ParseFloatError(e)),
            }
        } else {
            match &num_str[0..2] {
                "0x" => Parser::hex2dec(&num_str[2..]),
                "0b" => Parser::bin2dec(&num_str[2..]),
                _ => match f64::from_str(num_str) {
                    Ok(num) => Ok(num),
                    Err(e) => return Err(MyError::ParseFloatError(e)),
                },
            }
        }
    }

    fn new_node(kind: NodeKind, left: Box<Node>, right: Box<Node>) -> Box<Node> {
        Box::new(Node {
            node_kind: kind,
            right_node: Some(right),
            left_node: Some(left),
            val: None,
        })
    }

    fn new_unary_node(kind: NodeKind, left: Box<Node>) -> Box<Node> {
        Box::new(Node {
            node_kind: kind,
            right_node: None,
            left_node: Some(left),
            val: None,
        })
    }

    fn new_node_num(val: f64) -> Box<Node> {
        Box::new(Node {
            node_kind: NodeKind::NdNum,
            right_node: None,
            left_node: None,
            val: Some(NumOrVar::Num(val)),
        })
    }

    fn new_node_var(var: String) -> Box<Node> {
        Box::new(Node {
            node_kind: NodeKind::NdVar,
            right_node: None,
            left_node: None,
            val: Some(NumOrVar::Var(var)),
        })
    }

    fn show_node(place: String, node: &Node) {
        debugln!(
            "{}: create {{ Kind: {}, Val: {:?} }}",
            place,
            node.node_kind.to_string(),
            node.val
        );
    }

    /*
    expr      = mul ("+" mul | "-" mul)*
    mul       = noobmul  ("*" noobmul | "/" noobmul | "\cdto" noobmul | "\times" noobmul | "\div" noobmul)*
    noobmul    = sigend (expo)*
    signed    = "-"? expo
    expo      = primary ("^" "{" expr "}")*
    primary   = num | "(" expr ")" | "\frac" "{" expr "}" "{" expr "}" | "\sqrt" "{" expr "} | "\exp" "(" expr ")" | "\abs" "(" expr ")"
                | "\log"  signed | "\ln" signed | "\sin" signed | "\cos" signed | "\tan" signed | "\csc" signed | "\sec" signed | "\cot" signed
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
        let mut node: Box<Node> = self.noobmul()?;
        Parser::show_node("noobmul".to_string(), &node);
        loop {
            if self.lex.consume("*".to_string()) {
                node = Parser::new_node(NodeKind::NdMul, node, self.noobmul()?);
            } else if self.lex.consume("\\times".to_string()) {
                node = Parser::new_node(NodeKind::NdMul, node, self.noobmul()?);
            } else if self.lex.consume("\\cdot".to_string()) {
                node = Parser::new_node(NodeKind::NdMul, node, self.noobmul()?);
            } else if self.lex.consume("\\div".to_string()) {
                node = Parser::new_node(NodeKind::NdDiv, node, self.noobmul()?);
            } else if self.lex.consume("/".to_string()) {
                node = Parser::new_node(NodeKind::NdDiv, node, self.noobmul()?);
            } else {
                Parser::show_node("mul".to_string(), &node);
                return Ok(node);
            }
        }
    }

    fn noobmul(&mut self) -> Result<Box<Node>, MyError> {
        let mut node: Box<Node> = self.signed()?;
        Parser::show_node("signed".to_string(), &node);
        loop {
            self.lex.save_ctx();
            match self.expo() {
                Ok(n) => {
                    self.lex.discard_ctx()?;
                    match n.node_kind {
                        NodeKind::NdNum => {
                            return Err(MyError::InvalidInput(
                                "don't allowed nulmber literal on right operand of noobvious mul"
                                    .to_string(),
                            ));
                        }
                        _ => {
                            node = Parser::new_node(NodeKind::NdMul, node, n);
                        }
                    }
                }
                Err(e) => {
                    self.lex.revert_ctx()?;
                    match e {
                        MyError::NotTkNumber(_, _) => {
                            Parser::show_node("noobmul".to_string(), &node);
                            return Ok(node);
                        }
                        _ => return Err(e),
                    }
                }
            }
        }
    }

    fn signed(&mut self) -> Result<Box<Node>, MyError> {
        if self.lex.consume("-".to_string()) {
            let mut node = self.expo()?;
            node = Parser::new_unary_node(NodeKind::NdNeg, node);
            Ok(node)
        } else {
            Ok(self.expo()?)
        }
    }

    fn expo(&mut self) -> Result<Box<Node>, MyError> {
        let mut node: Box<Node> = self.primary()?;
        Parser::show_node("primary".to_string(), &node);
        loop {
            if self.lex.consume("^".to_string()) {
                self.lex.expect_br("{".to_string())?;
                let cnode: Box<Node> = self.expr()?;
                self.lex.expect_br("}".to_string())?;
                node = Parser::new_node(NodeKind::NdPow, node, cnode);
            } else {
                Parser::show_node("mul".to_string(), &node);
                return Ok(node);
            }
        }
    }

    fn primary(&mut self) -> Result<Box<Node>, MyError> {
        if self.lex.consume("(".to_string()) {
            let node: Box<Node> = self.expr()?;
            self.lex.expect_br(")".to_string())?;
            return Ok(node);
        }
        if self.lex.consume("\\frac".to_string()) {
            self.lex.expect_br("{".to_string())?;
            let lnode: Box<Node> = self.expr()?;
            self.lex.expect_br("}".to_string())?;
            self.lex.expect_br("{".to_string())?;
            let rnode: Box<Node> = self.expr()?;
            self.lex.expect_br("}".to_string())?;
            let node = Parser::new_node(NodeKind::NdDiv, lnode, rnode);
            return Ok(node);
        }

        if self.lex.consume("\\sqrt".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdSqrt, self.carg_node()?));
        }
        if self.lex.consume("\\abs".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdAbs, self.parg_node()?));
        }
        if self.lex.consume("\\exp".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdExp, self.parg_node()?));
        }
        if self.lex.consume("\\log".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdLog, self.signed()?));
        }
        if self.lex.consume("\\ln".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdLn, self.signed()?));
        }
        if self.lex.consume("\\sin".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdSin, self.signed()?));
        }
        if self.lex.consume("\\cos".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdCos, self.signed()?));
        }
        if self.lex.consume("\\tan".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdTan, self.signed()?));
        }
        if self.lex.consume("\\csc".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdCsc, self.signed()?));
        }
        if self.lex.consume("\\sec".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdSec, self.signed()?));
        }
        if self.lex.consume("\\cot".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdCot, self.signed()?));
        }
        if self.lex.consume("\\arcsin".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdAcSin, self.signed()?));
        }
        if self.lex.consume("\\arccos".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdAcCos, self.signed()?));
        }
        if self.lex.consume("\\arctan".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::NdAcTan, self.signed()?));
        }

        let num_node = self.num()?;
        Ok(num_node)
    }

    fn num(&mut self) -> Result<Box<Node>, MyError> {
        match self.lex.expect_number() {
            Ok(v) => match v {
                NumstrOrVar::Num(num) => Ok(Parser::new_node_num(Parser::f64_from_str(&num)?)),
                NumstrOrVar::Var(var) => Ok(Parser::new_node_var(var)),
            },
            Err(e) => return Err(e),
        }
    }

    // parentheses "()" arg node
    fn parg_node(&mut self) -> Result<Box<Node>, MyError> {
        self.lex.expect_br("(".to_string())?;
        let node: Box<Node> = self.expr()?;
        self.lex.expect_br(")".to_string())?;
        Ok(node)
    }

    // curly brackets "{}" arg node
    fn carg_node(&mut self) -> Result<Box<Node>, MyError> {
        self.lex.expect_br("{".to_string())?;
        let node: Box<Node> = self.expr()?;
        self.lex.expect_br("}".to_string())?;
        Ok(node)
    }
}

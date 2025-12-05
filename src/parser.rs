use bigdecimal::BigDecimal;
use std::collections::HashMap;
use std::fmt;

use crate::ast_printer::{show_ast, show_ast_in_s_expr_rec};
use crate::error::*;
use crate::str2num::*;
use crate::tokenizer::tokenize;
use crate::tokenizer::{NumstrOrVar, Token, TokenKind};
use crate::tsc_cmd;
use crate::CONSTS;

use text_colorizer::*;

use crate::debugln;

#[derive(Clone, Copy)]
pub enum NodeKind {
    // 1引数
    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,
    AcSin,
    AcCos,
    AcTan,
    Sqrt,
    Log,
    Ln,
    Abs,
    Exp,
    // 2引数
    Add,
    Sub,
    Mul,
    Div,
    // 前置1引数
    Neg,
    // 後置1引数
    Pow,
    // 数字
    Num,
    // 変数
    Var,
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::Sin => write!(f, "Sin"),
            NodeKind::Cos => write!(f, "Cos"),
            NodeKind::Tan => write!(f, "Tan"),
            NodeKind::Csc => write!(f, "Csc"),
            NodeKind::Sec => write!(f, "Sec"),
            NodeKind::Cot => write!(f, "Cot"),
            NodeKind::AcSin => write!(f, "AcSin"),
            NodeKind::AcCos => write!(f, "AcCos"),
            NodeKind::AcTan => write!(f, "AcTan"),
            NodeKind::Sqrt => write!(f, "Sqrt"),
            NodeKind::Log => write!(f, "Log"),
            NodeKind::Ln => write!(f, "Ln"),
            NodeKind::Abs => write!(f, "Abs"),
            NodeKind::Exp => write!(f, "Exp"),
            NodeKind::Add => write!(f, "Add"),
            NodeKind::Sub => write!(f, "Sub"),
            NodeKind::Mul => write!(f, "Mul"),
            NodeKind::Div => write!(f, "Div"),
            NodeKind::Neg => write!(f, "Neg"),
            NodeKind::Pow => write!(f, "Pow"),
            NodeKind::Num => write!(f, "Num"),
            NodeKind::Var => write!(f, "Var"),
        }
    }
}

impl NodeKind {
    pub fn to_op_str(self) -> String {
        match self {
            NodeKind::Sin => "Sin".to_string(),
            NodeKind::Cos => "Cos".to_string(),
            NodeKind::Tan => "Tan".to_string(),
            NodeKind::Csc => "Csc".to_string(),
            NodeKind::Sec => "Sec".to_string(),
            NodeKind::Cot => "Cot".to_string(),
            NodeKind::AcSin => "AcSin".to_string(),
            NodeKind::AcCos => "AcCos".to_string(),
            NodeKind::AcTan => "AcTan".to_string(),
            NodeKind::Sqrt => "Sqrt".to_string(),
            NodeKind::Log => "Log".to_string(),
            NodeKind::Ln => "Ln".to_string(),
            NodeKind::Abs => "Abs".to_string(),
            NodeKind::Exp => "exp".to_string(),
            NodeKind::Add => "+".to_string(),
            NodeKind::Sub => "-".to_string(),
            NodeKind::Mul => "*".to_string(),
            NodeKind::Div => "/".to_string(),
            NodeKind::Neg => "-".to_string(),
            NodeKind::Pow => "Pow".to_string(),
            NodeKind::Num => "Num".to_string(),
            NodeKind::Var => "Var".to_string(),
        }
    }

    pub fn to_lisp_op_str(self) -> String {
        match self {
            NodeKind::Sin => "sin".to_string(),
            NodeKind::Cos => "cos".to_string(),
            NodeKind::Tan => "tan".to_string(),
            NodeKind::Csc => "csc".to_string(),
            NodeKind::Sec => "sec".to_string(),
            NodeKind::Cot => "cot".to_string(),
            NodeKind::AcSin => "asin".to_string(),
            NodeKind::AcCos => "acos".to_string(),
            NodeKind::AcTan => "atan".to_string(),
            NodeKind::Sqrt => "sqrt".to_string(),
            NodeKind::Log => "log".to_string(),
            NodeKind::Ln => "log".to_string(),
            NodeKind::Abs => "abs".to_string(),
            NodeKind::Exp => "exp".to_string(),
            NodeKind::Add => "+".to_string(),
            NodeKind::Sub => "-".to_string(),
            NodeKind::Mul => "*".to_string(),
            NodeKind::Div => "/".to_string(),
            NodeKind::Neg => "-".to_string(),
            NodeKind::Pow => "expt".to_string(),
            NodeKind::Num => "Num".to_string(),
            NodeKind::Var => "Var".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum NumOrVar {
    Num(BigDecimal),
    Var(String),
}

pub enum TscCmd {
    Hex,
    Dec,
    Bin,
    Oct,
}
impl fmt::Display for TscCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TscCmd::Hex => write!(f, ":hex"),
            TscCmd::Dec => write!(f, ":dec"),
            TscCmd::Bin => write!(f, ":bin"),
            TscCmd::Oct => write!(f, ":oct"),
        }
    }
}

pub enum NodeOrCmd {
    Node(Box<Node>),
    TscCmd(TscCmd),
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

// pub struct Parser<'a> {
pub struct Parser {
    form: String,
    tokens: Vec<Token>,
    token_loc: Vec<usize>,
    token_idx: usize,
    ctx_stack: Vec<usize>,
}

impl Parser {
    pub fn new(form: String) -> Result<Parser, MyError> {
        let (tokens, token_loc) = tokenize(&form)?;
        Ok(Parser {
            form,
            tokens,
            token_loc,
            token_idx: 0,
            ctx_stack: Vec::new(),
        })
    }

    pub fn get_token(&self, idx: usize) -> &Token {
        &self.tokens[idx]
    }

    pub fn build_ast(
        &mut self,
        vars: &mut HashMap<String, BigDecimal>,
    ) -> Result<Vec<NodeOrCmd>, MyError> {
        // から varsを構築
        let mut to_delete_el = Vec::<usize>::new();
        for i in 0..self.tokens.len() {
            if self.tokens[i].token == "," {
                match self.tokens[i + 1].token_kind {
                    TokenKind::TkVariable => {
                        if self.tokens[i + 2].token != "=" {
                            // "," is separator
                            continue;
                        } else {
                            // 変数定義: ", {var} = {value}"
                            to_delete_el.push(i);
                            to_delete_el.push(i + 1);
                            to_delete_el.push(i + 2);
                        }
                    }
                    _tk => {
                        // "," is separator
                        continue;
                    }
                }
                match self.tokens[i + 3].token_kind {
                    TokenKind::TkNum => match bigdecimal_from_str(&self.tokens[i + 3].token) {
                        Ok(num) => {
                            vars.insert(self.tokens[i + 1].token.clone(), num);
                        }
                        Err(e) => return Err(e),
                    },
                    TokenKind::TkOperator => {
                        if self.tokens[i + 3].token == "-" {
                            match self.tokens[i + 4].token_kind {
                                TokenKind::TkNum => {
                                    match bigdecimal_from_str(&self.tokens[i + 4].token) {
                                        Ok(num) => {
                                            vars.insert(self.tokens[i + 1].token.clone(), -num);
                                            to_delete_el.push(i + 4);
                                        }
                                        Err(e) => return Err(e),
                                    }
                                }
                                tk => {
                                    return Err(MyError::NotTkNumber(
                                        tk.to_string(),
                                        self.format_err_loc_idx(i + 4),
                                    ))
                                }
                            }
                        } else {
                            return Err(MyError::NotTkNumber(
                                TokenKind::TkOperator.to_string(),
                                self.format_err_loc_idx(i + 3),
                            ));
                        }
                    }
                    tk => {
                        return Err(MyError::NotTkNumber(
                            tk.to_string(),
                            self.format_err_loc_idx(i + 3),
                        ))
                    }
                }
                to_delete_el.push(i + 3);
            } else if let TokenKind::TkTscCommand = self.tokens[i].token_kind {
                let consumed = tsc_cmd::process_tsccommand(self, i, vars)?;
                for n in 0..consumed {
                    to_delete_el.push(i + n)
                }
            }
        }
        to_delete_el.sort_by(|a, b| b.cmp(a));
        for i in to_delete_el.into_iter() {
            self.del_token(i);
        }
        // varsに定数をプッシュする
        match CONSTS.read() {
            Ok(consts) => {
                for (name, value) in consts.iter() {
                    vars.insert(name.to_string(), value.clone());
                }
            }
            Err(e) => return Err(MyError::ConstsReadErr(e.to_string())),
        }

        if self.is_eot() {
            return Err(MyError::NoToken);
        }
        let ast_or_cmd_vec = self.expr_vec()?;
        if !self.is_eot() {
            return Err(MyError::UnprocessedToekn(
                self.now_token().to_string(),
                self.format_err_loc(),
            ));
        }
        for ast_or_cmd in &ast_or_cmd_vec {
            match ast_or_cmd {
                NodeOrCmd::Node(ast) => {
                    show_ast(ast, vars)?;
                    show_ast_in_s_expr_rec(ast, vars)?;
                }
                NodeOrCmd::TscCmd(_) => {}
            }
        }
        Ok(ast_or_cmd_vec)
    }

    fn is_eot(&self) -> bool {
        matches!(self.tokens[self.token_idx].token_kind, TokenKind::TkEOT)
    }

    fn now_token(&self) -> &str {
        &self.tokens[self.token_idx].token
    }

    /// 保持しているtoken列からidx番目のtokenを削除
    ///
    /// * `idx` - 削除するtokenのindex
    pub fn del_token(&mut self, idx: usize) {
        self.tokens.remove(idx);
        self.token_loc.remove(idx);
    }

    /// parser内でエラーが起こっており、r.token_idxにエラーの原因となる
    /// tokenが入っているときに、エラーが数式のどの個所で起こったかを示す文字列を返す
    /// ex)
    /// ```
    /// \frac {3} {
    ///            ^~~
    /// ```
    pub fn format_err_loc(&self) -> String {
        let mut pad: String = String::with_capacity(self.token_loc[self.token_idx]);
        for _i in 0..self.token_loc[self.token_idx] {
            pad += " ";
        }
        let err_indicator: String = format!("{}{}", pad, "^".red());
        let mut nami: String = String::with_capacity(self.tokens[self.token_idx].token.len());
        for _i in 0..self.tokens[self.token_idx].token.len() - 1 {
            nami += &format!("{}", "~".red());
        }
        let res: String = format!("{}\n{}{}", self.form, err_indicator, nami);
        res
    }

    /// 変数やTSC Commandの処理中等のparser外でエラーが起こっており、r.token_idxにエラーの
    /// 原因個所のtoken indexが入っていないときにエラーが数式のどの個所で起こったかを示す文字列を返す
    /// ex)
    /// ```
    /// , x = a
    ///       ^
    ///       ```
    /// * `token_idx` - token_idx: エラーが発生したtokenのindex
    pub fn format_err_loc_idx(&self, token_idx: usize) -> String {
        let mut pad: String = String::with_capacity(self.token_loc[token_idx]);
        for _i in 0..self.token_loc[token_idx] {
            pad += " ";
        }
        let err_indicator: String = format!("{}{}", pad, "^".red());
        let mut nami: String = String::with_capacity(self.tokens[token_idx].token.len());
        for _i in 0..self.tokens[token_idx].token.len() - 1 {
            nami += &format!("{}", "~".red());
        }
        let res: String = format!("{}\n{}{}", self.form, err_indicator, nami);
        res
    }

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

    pub fn save_ctx(&mut self) {
        self.ctx_stack.push(self.token_idx);
    }

    pub fn revert_ctx(&mut self) -> Result<(), MyError> {
        match self.ctx_stack.pop() {
            Some(i) => {
                self.token_idx = i;
            }
            None => {
                return Err(MyError::UnexpectedOpToLexer(
                    "context not yet pushed".to_string(),
                ))
            }
        }
        Ok(())
    }

    pub fn discard_ctx(&mut self) -> Result<(), MyError> {
        match self.ctx_stack.pop() {
            Some(_) => (),
            None => {
                return Err(MyError::UnexpectedOpToLexer(
                    "context not yet pushed".to_string(),
                ))
            }
        }
        Ok(())
    }

    pub fn consume(&mut self, op: String) -> bool {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkOperator => (),
            TokenKind::TkBrace => (),
            TokenKind::TkSeparaotr => (),
            TokenKind::TkTexCommand => (),
            _ => return false,
        }
        if self.tokens[self.token_idx].token == op {
            self.token_idx += 1;
            true
        } else {
            false
        }
    }

    pub fn consume_seq(&mut self) -> bool {
        if let TokenKind::TkSeparaotr = self.tokens[self.token_idx].token_kind {
            self.token_idx += 1;
            true
        } else {
            false
        }
    }

    pub fn consume_tsc_cmd(&mut self) -> Result<TscCmd, MyError> {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkTscCommand => {
                let token = &*self.tokens[self.token_idx].token;
                self.token_idx += 1;
                Ok(match token {
                    ":hex" => TscCmd::Hex,
                    ":dec" => TscCmd::Dec,
                    ":bin" => TscCmd::Bin,
                    ":oct" => TscCmd::Oct,
                    cmd => return Err(MyError::UDcommandErr(cmd.to_string())),
                })
            }
            _ => Err(MyError::NotTkTscCmd),
        }
    }

    pub fn expect_br(&mut self, br: String) -> Result<(), MyError> {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkBrace => {
                if self.tokens[self.token_idx].token == br {
                    self.token_idx += 1;
                    Ok(())
                } else {
                    Err(MyError::UnexpectedToken(
                        br,
                        self.tokens[self.token_idx].token.to_string(),
                    ))
                }
            }
            tk => Err(MyError::NotTkBrace(
                br,
                tk.to_string(),
                self.format_err_loc(),
            )),
        }
    }

    pub fn expect_number(&mut self) -> Result<NumstrOrVar, MyError> {
        match self.tokens[self.token_idx].token_kind {
            TokenKind::TkNum => {
                self.token_idx += 1;
                Ok(NumstrOrVar::Num(
                    self.tokens[self.token_idx - 1].token.clone(),
                ))
            }
            TokenKind::TkVariable => {
                self.token_idx += 1;
                Ok(NumstrOrVar::Var(
                    self.tokens[self.token_idx - 1].token.to_string(),
                ))
            }
            _ => Err(MyError::NotTkNumber(
                self.tokens[self.token_idx].token_kind.to_string(),
                self.format_err_loc(),
            )),
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

    fn new_node_num(val: BigDecimal) -> Box<Node> {
        Box::new(Node {
            node_kind: NodeKind::Num,
            right_node: None,
            left_node: None,
            val: Some(NumOrVar::Num(val)),
        })
    }

    fn new_node_var(var: String) -> Box<Node> {
        Box::new(Node {
            node_kind: NodeKind::Var,
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
    expr_vec  = ","? (expr | tsc_cmd) (","? expr | ","? tsc_cmd)*
    tsc_cmd   = ":hex" | ":dec" | ":bin"
    expr      = mul ("+" mul | "-" mul)*
    mul       = noobmul  ("*" noobmul | "/" noobmul | "\cdto" noobmul | "\times" noobmul | "\div" noobmul)*
    noobmul   = sigend (expo)*
    signed    = "-"? expo
    expo      = primary ("^" "{" expr "}")*
    primary   = num | "(" expr ")" | "\frac" "{" expr "}" "{" expr "}" | "\sqrt" "{" expr "} | "\exp" "(" expr ")" | "\abs" "(" expr ")"
                | "\log"  signed | "\ln" signed | "\sin" signed | "\cos" signed | "\tan" signed | "\csc" signed | "\sec" signed | "\cot" signed
    */
    /*
    fn signed(&mut self) -> Result<Box<Node>, MyError> {
        if self.consume("-".to_string()) {
            let mut node = self.expo()?;
            node = Parser::new_unary_node(NodeKind::Neg, node);
            Ok(node)
        } else {
            Ok(self.expo()?)
        }
    }
    */

    fn expr_vec(&mut self) -> Result<Vec<NodeOrCmd>, MyError> {
        let mut res = Vec::new();
        loop {
            let b = self.consume_seq();
            if b {
                debugln!("consume seq");
            }
            match self.consume_tsc_cmd() {
                Ok(tsc_cmd) => {
                    debugln!("expr_vec: create TscCmd {tsc_cmd}");
                    res.push(NodeOrCmd::TscCmd(tsc_cmd));
                }
                Err(e) => match e {
                    MyError::NotTkTscCmd => {
                        let node = self.expr()?;
                        res.push(NodeOrCmd::Node(node));
                    }
                    MyError::UDcommandErr(e) => {
                        return Err(MyError::UDcommandErr(e));
                    }
                    _ => unreachable!(),
                },
            }
            if self.is_eot() {
                break;
            }
        }
        Ok(res)
    }

    fn expr(&mut self) -> Result<Box<Node>, MyError> {
        let mut node: Box<Node> = self.mul()?;
        loop {
            if self.consume("+".to_string()) {
                node = Parser::new_node(NodeKind::Add, node, self.mul()?);
            } else if self.consume("-".to_string()) {
                node = Parser::new_node(NodeKind::Sub, node, self.mul()?);
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
            if self.consume("*".to_string())
                || self.consume("\\times".to_string())
                || self.consume("\\cdot".to_string())
            {
                node = Parser::new_node(NodeKind::Mul, node, self.noobmul()?);
            } else if self.consume("\\div".to_string()) || self.consume("/".to_string()) {
                node = Parser::new_node(NodeKind::Div, node, self.noobmul()?);
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
            self.save_ctx();
            match self.expo() {
                Ok(n) => {
                    match n.node_kind {
                        NodeKind::Num => {
                            /*
                            return Err(MyError::InvalidInput(
                                "don't allowed nulmber literal on right operand of noobvious mul"
                                    .to_string(),
                            ));
                            */
                            self.revert_ctx()?;
                            return Ok(node);
                        }
                        _ => {
                            self.discard_ctx()?;
                            node = Parser::new_node(NodeKind::Mul, node, n);
                        }
                    }
                }
                Err(e) => {
                    self.revert_ctx()?;
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
        if self.consume("-".to_string()) {
            let mut node = self.expo()?;
            node = Parser::new_unary_node(NodeKind::Neg, node);
            Ok(node)
        } else {
            Ok(self.expo()?)
        }
    }

    fn expo(&mut self) -> Result<Box<Node>, MyError> {
        let mut node: Box<Node> = self.primary()?;
        Parser::show_node("primary".to_string(), &node);
        loop {
            if self.consume("^".to_string()) {
                self.expect_br("{".to_string())?;
                let cnode: Box<Node> = self.expr()?;
                self.expect_br("}".to_string())?;
                node = Parser::new_node(NodeKind::Pow, node, cnode);
            } else {
                Parser::show_node("mul".to_string(), &node);
                return Ok(node);
            }
        }
    }

    fn primary(&mut self) -> Result<Box<Node>, MyError> {
        if self.consume("(".to_string()) {
            let node: Box<Node> = self.expr()?;
            self.expect_br(")".to_string())?;
            return Ok(node);
        }
        if self.consume("\\frac".to_string()) {
            self.expect_br("{".to_string())?;
            let lnode: Box<Node> = self.expr()?;
            self.expect_br("}".to_string())?;
            self.expect_br("{".to_string())?;
            let rnode: Box<Node> = self.expr()?;
            self.expect_br("}".to_string())?;
            let node = Parser::new_node(NodeKind::Div, lnode, rnode);
            return Ok(node);
        }

        if self.consume("\\sqrt".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::Sqrt, self.carg_node()?));
        }
        if self.consume("\\abs".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::Abs, self.parg_node()?));
        }
        if self.consume("\\exp".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::Exp, self.parg_node()?));
        }
        if self.consume("\\log".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::Log, self.signed()?));
        }
        if self.consume("\\ln".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::Ln, self.signed()?));
        }
        if self.consume("\\sin".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::Sin, self.signed()?));
        }
        if self.consume("\\cos".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::Cos, self.signed()?));
        }
        if self.consume("\\tan".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::Tan, self.signed()?));
        }
        if self.consume("\\csc".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::Csc, self.signed()?));
        }
        if self.consume("\\sec".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::Sec, self.signed()?));
        }
        if self.consume("\\cot".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::Cot, self.signed()?));
        }
        if self.consume("\\arcsin".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::AcSin, self.signed()?));
        }
        if self.consume("\\arccos".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::AcCos, self.signed()?));
        }
        if self.consume("\\arctan".to_string()) {
            return Ok(Parser::new_unary_node(NodeKind::AcTan, self.signed()?));
        }

        let num_node = self.num()?;
        Ok(num_node)
    }

    fn num(&mut self) -> Result<Box<Node>, MyError> {
        match self.expect_number() {
            Ok(v) => match v {
                NumstrOrVar::Num(num) => Ok(Parser::new_node_num(bigdecimal_from_str(&num)?)),
                NumstrOrVar::Var(var) => Ok(Parser::new_node_var(var)),
            },
            Err(e) => Err(e),
        }
    }

    // parentheses "()" arg node
    fn parg_node(&mut self) -> Result<Box<Node>, MyError> {
        self.expect_br("(".to_string())?;
        let node: Box<Node> = self.expr()?;
        self.expect_br(")".to_string())?;
        Ok(node)
    }

    // curly brackets "{}" arg node
    fn carg_node(&mut self) -> Result<Box<Node>, MyError> {
        self.expect_br("{".to_string())?;
        let node: Box<Node> = self.expr()?;
        self.expect_br("}".to_string())?;
        Ok(node)
    }
}

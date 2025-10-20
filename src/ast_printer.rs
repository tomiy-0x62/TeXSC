use bigdecimal::{BigDecimal, FromPrimitive};
use std::collections::{HashMap, HashSet};
use crate::config::*;
use crate::error::*;
use crate::parser::{Node, NodeKind, NumOrVar};


    pub fn show_ast(ast: &Node, vars: &HashMap<String, BigDecimal>) -> Result<(), MyError> {
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
                        } else if !no_show_bar.iter().any(|e| e == &(i / TREE_WIDTH)) {
                            msg += "";
                        } else {
                            msg += " ";
                        }
                    } else if i > TREE_WIDTH * (level - 1) {
                        msg += "─";
                    } else {
                        msg += " ";
                    }
                }
                // nodeを追加
                match node.node_kind {
                    NodeKind::Num | NodeKind::Var => match node.val.clone().unwrap() {
                        NumOrVar::Num(n) => msg += &(n.to_string()),
                        NumOrVar::Var(v) => match vars.get(&v) {
                            Some(n) => msg += &format!("{v} = {n}"),
                            None => msg += &v,
                        },
                    },
                    _ => msg += &node.node_kind.to_op_str(),
                }
                msg += "\n";
                // treeのトラバース、levelの変更
                is_next_have_chiled = node.left_node.is_some() && node.right_node.is_some();
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
            eprintln!("{msg}");
        }
        Ok(())
    }

    pub fn show_ast_in_s_expr_rec(node: &Node, vars: &HashMap<String, BigDecimal>) -> Result<(), MyError> {
        let conf = config_reader()?;
        let is_show_ast = cfg!(debug_assertions)
            || conf.debug
            || conf.ast_format == AstFormat::Sexpr
            || conf.ast_format == AstFormat::Both;
        if is_show_ast {
            let mut s_expr = String::new();
            s_expr = show_ast_in_s_expr_rec_inner(node, vars, s_expr, &mut HashSet::new(), false);
            eprintln!("{s_expr}\n");
        }
        Ok(())
    }

    fn show_ast_in_s_expr_rec_inner(
        node: &Node,
        vars: &HashMap<String, BigDecimal>,
        mut s_expr: String,
        is_var_fn_printed: &mut HashSet<String>,
        is_2arg_left: bool,
    ) -> String {
        match node.node_kind {
            NodeKind::Num | NodeKind::Var => {
                match &node.val {
                    Some(NumOrVar::Num(n)) => s_expr += &n.to_string(),
                    Some(NumOrVar::Var(v)) => {
                        if v == "\\pi" {
                            s_expr += "pi"
                        } else {
                            if let Some(val) = vars.get(v)
                                && is_var_fn_printed.get(v).is_none() {
                                    s_expr = format!("(defvar {v} {val})\n{s_expr}");
                                    is_var_fn_printed.insert(v.clone());
                            }
                            s_expr += v
                        }
                    }
                    None => unreachable!(),
                }
                if is_2arg_left {
                    s_expr + " "
                } else {
                    s_expr
                }
            }
            _ => {
                let mut is_deg2rad = false;
                let mut is_rad2deg = false;
                if config_reader().expect("couldn't read config").trig_func_arg
                    == TrigFuncArg::Degree
                {
                    match node.node_kind {
                        NodeKind::Sin
                        | NodeKind::Cos
                        | NodeKind::Tan
                        | NodeKind::Csc
                        | NodeKind::Sec
                        | NodeKind::Cot => {
                            match node.node_kind {
                                NodeKind::Csc => {
                                    if !is_var_fn_printed.contains("csc") {
                                        s_expr = format!("(defun csc (x) (/ 1 (sin x)))\n{s_expr}");
                                        is_var_fn_printed.insert("csc".to_string());
                                    }
                                }
                                NodeKind::Sec => {
                                    if !is_var_fn_printed.contains("sec") {
                                        s_expr = format!("(defun sec (x) (/ 1 (cos x)))\n{s_expr}");
                                        is_var_fn_printed.insert("sec".to_string());
                                    }
                                }
                                NodeKind::Cot => {
                                    if !is_var_fn_printed.contains("cot") {
                                        s_expr = format!("(defun cot (x) (/ 1 (tan x)))\n{s_expr}");
                                        is_var_fn_printed.insert("cot".to_string());
                                    }
                                }
                                _ => {}
                            }
                            if !is_var_fn_printed.contains("degree2radian") {
                                s_expr = format!(
                                    "(defun degree2radian (deg) (/ (* deg pi) 180))\n{s_expr}"
                                );
                                is_var_fn_printed.insert("degree2radian".to_string());
                            }
                            s_expr +=
                                &format!("({} (degree2radian ", node.node_kind.to_lisp_op_str());
                            is_deg2rad = true;
                        }
                        NodeKind::AcSin | NodeKind::AcCos | NodeKind::AcTan => {
                            if !is_var_fn_printed.contains("radian2degree") {
                                s_expr = format!(
                                    "(defun radian2degree (rad) (/ (* rad 180) pi))\n{s_expr}"
                                );
                                is_var_fn_printed.insert("radian2degree".to_string());
                            }
                            s_expr +=
                                &format!("(radian2degree ({} ", node.node_kind.to_lisp_op_str());
                            is_rad2deg = true;
                        }
                        _ => {
                            s_expr += &format!("({} ", node.node_kind.to_lisp_op_str());
                        }
                    }
                } else {
                    match node.node_kind {
                        NodeKind::Csc => {
                            if !is_var_fn_printed.contains("csc") {
                                s_expr = format!("(defun csc (x) (/ 1 (sin x)))\n{s_expr}");
                                is_var_fn_printed.insert("csc".to_string());
                            }
                            s_expr += &format!("({} ", node.node_kind.to_lisp_op_str());
                        }
                        NodeKind::Sec => {
                            if !is_var_fn_printed.contains("sec") {
                                s_expr = format!("(defun sec (x) (/ 1 (cos x)))\n{s_expr}");
                                is_var_fn_printed.insert("sec".to_string());
                            }
                            s_expr += &format!("({} ", node.node_kind.to_lisp_op_str());
                        }
                        NodeKind::Cot => {
                            if !is_var_fn_printed.contains("cot") {
                                s_expr = format!("(defun cot (x) (/ 1 (tan x)))\n{s_expr}");
                                is_var_fn_printed.insert("cot".to_string());
                            }
                            s_expr += &format!("({} ", node.node_kind.to_lisp_op_str());
                        }
                        _ => {
                            s_expr += &format!("({} ", node.node_kind.to_lisp_op_str());
                        }
                    }
                }
                if node.left_node.is_some() {
                    s_expr = show_ast_in_s_expr_rec_inner(
                        node.left_node.as_ref().unwrap(),
                        vars,
                        s_expr,
                        is_var_fn_printed,
                        true & node.right_node.is_some(),
                    );
                }
                if node.right_node.is_some() {
                    s_expr = show_ast_in_s_expr_rec_inner(
                        node.right_node.as_ref().unwrap(),
                        vars,
                        s_expr,
                        is_var_fn_printed,
                        false,
                    );
                }
                if is_deg2rad || is_rad2deg {
                    s_expr += ")";
                }
                if let NodeKind::Log = node.node_kind {
                    let log_base = &config_reader().expect("couldn't read config").log_base;
                    if *log_base != BigDecimal::from_f64(std::f64::consts::E).unwrap() {
                        if s_expr.ends_with(" ") {
                            s_expr += &format!("{log_base}");
                        } else {
                            s_expr += &format!(" {log_base}");
                        }
                    }
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
                        Some(NodeKind::Num) => {
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

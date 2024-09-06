// TeX Scientific Calculator

use self::parser::NumOrVar;
use clap::{Arg, Command};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{LazyLock, RwLock};

use parser::NodeKind;
use text_colorizer::*;

mod config;
mod error;
mod num_formatter;
mod parser;
mod tsc_cmd;
#[macro_use]
mod macros;
#[cfg(test)]
mod test;

use config::*;
use error::*;
use num_formatter::num_formatter;

pub static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| {
    RwLock::new(config::Config {
        debug: false,
        ast_format: AstFormat::Both,
        trig_func_arg: TrigFuncArg::Radian,
        log_base: std::f64::consts::E,
        num_of_digit: 12,
    })
});

pub static CONSTS: LazyLock<RwLock<HashMap<String, f64>>> = LazyLock::new(|| {
    RwLock::new({
        let mut consts = HashMap::new();
        consts.insert("e".to_string(), std::f64::consts::E);
        consts.insert("\\pi".to_string(), std::f64::consts::PI);
        consts
    })
});

fn main() {
    let app = Command::new("tsc")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("file")
                .help("load formulas from file")
                .short('f')
                .takes_value(true),
        )
        .arg(
            Arg::new("tex formulas")
                .help("tex formulas")
                .required(false),
        );

    match CONFIG.write().expect("").load_from_file() {
        Ok(conf_file) => eprintln!("config loaded from {:?}", conf_file),
        Err(e) => eprintln!("config load failed: {}", e),
    }

    let matches = app.get_matches();

    // formulas from command line arg
    if let Some(form) = matches.value_of("tex formulas") {
        let mut conf = config_writer().expect("couldn't change ast_format config");
        conf.ast_format = AstFormat::None;
        let mut vars: HashMap<String, f64> = HashMap::new();
        for line in form.split('\n') {
            process_form(line.replace("\r", ""), &mut vars);
        }
        return;
    }

    // formulas from file
    if let Some(file_name) = matches.value_of("file") {
        let mut conf = config_writer().expect("couldn't change ast_format config");
        conf.ast_format = AstFormat::None;
        let f: File = File::open(file_name).expect(file_name);
        let reader: BufReader<File> = BufReader::new(f);
        let mut vars: HashMap<String, f64> = HashMap::new();
        for line in reader.lines() {
            process_form(line.unwrap(), &mut vars);
        }
        return;
    }

    // REPL
    let mut vars: HashMap<String, f64> = HashMap::new();
    let mut rl = match Editor::<()>::new() {
        Ok(r) => r,
        Err(_) => panic!("Can't readline!"),
    };
    loop {
        let readline = rl.readline("tsc> ");
        let form = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                line
            }
            Err(ReadlineError::Interrupted) => return,
            Err(ReadlineError::Eof) => return,
            Err(err) => panic!("{}", err),
        };
        if form.trim() == "exit" {
            return;
        }
        process_form(form.to_string(), &mut vars);
    }
}

fn process_form(form: String, vars: &mut HashMap<String, f64>) -> Option<f64> {
    let lex = match parser::lexer::Lexer::new(form) {
        Ok(l) => l,
        Err(e) => {
            eprintlnc!(e);
            return None;
        }
    };
    let mut _pars = match parser::Parser::new(lex, vars) {
        Ok(p) => p,
        Err(e) => {
            eprintlnc!(e);
            return None;
        }
    };
    _pars.print_vars();
    let ast_root = match _pars.build_ast() {
        Ok(ast) => ast,
        Err(e) => match e {
            MyError::NoToken => return None,
            _ => {
                eprintlnc!(e);
                return None;
            }
        },
    };
    let num_of_digit = match config_reader() {
        Ok(c) => c.num_of_digit,
        Err(e) => {
            eprintlnc!(e);
            return None;
        }
    };
    match calc(*ast_root, vars) {
        Ok(result) => {
            debugln!("resutl: {}", result);
            println!("{}", num_formatter(result, num_of_digit));
            Some(result)
        }
        Err(e) => {
            eprintlnc!(e);
            None
        }
    }
}

fn calc(node: parser::Node, vars: &HashMap<String, f64>) -> Result<f64, MyError> {
    match node.node_kind {
        NodeKind::Num | NodeKind::Var => {
            return Ok(match node.val.unwrap() {
                NumOrVar::Num(n) => n,
                NumOrVar::Var(v) => match vars.get(&v) {
                    Some(n) => *n,
                    None => return Err(MyError::UDvariableErr(v)),
                },
            })
        }
        _ => (),
    }

    let loperand: f64;
    let mut roperand: f64 = 1.0;

    if let Some(left) = node.left_node {
        loperand = getoperand(*left, vars)?;
    } else {
        // Num, Var以外でleftがNoneはエラー
        // ここに到達した => 不正なAST
        return Err(MyError::BrokenAstErr);
    }

    if let Some(right) = node.right_node {
        roperand = getoperand(*right, vars)?;
    } else {
        // Num, Var以外でrightがNoneはありえる
        // 前置, 1引数のノードの場合 => 正常
        // それ以外 => 不正なAST
        match node.node_kind {
            NodeKind::Add => return Err(MyError::BrokenAstErr),
            NodeKind::Sub => return Err(MyError::BrokenAstErr),
            NodeKind::Div => return Err(MyError::BrokenAstErr),
            NodeKind::Mul => return Err(MyError::BrokenAstErr),
            _ => (),
        }
    }

    let conf = config_reader()?;

    fn radian2degree(rad: f64) -> f64 {
        rad * 180.0 / std::f64::consts::PI
    }

    match node.node_kind {
        NodeKind::Add => Ok(loperand + roperand),
        NodeKind::Sub => Ok(loperand - roperand),
        NodeKind::Mul => Ok(loperand * roperand),
        NodeKind::Div => Ok(loperand / roperand),
        NodeKind::Sqrt => Ok(loperand.sqrt()),
        NodeKind::Log => Ok(loperand.log(conf.log_base)),
        NodeKind::Ln => Ok(loperand.log(std::f64::consts::E)),
        NodeKind::Abs => Ok(loperand.abs()),
        NodeKind::Exp => Ok(std::f64::consts::E.powf(loperand)),
        NodeKind::Sin => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(loperand.sin()),
            TrigFuncArg::Degree => Ok(loperand.to_radians().sin()),
        },
        NodeKind::Cos => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(loperand.cos()),
            TrigFuncArg::Degree => Ok(loperand.to_radians().cos()),
        },
        NodeKind::Tan => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(loperand.tan()),
            TrigFuncArg::Degree => Ok(loperand.to_radians().tan()),
        },
        NodeKind::Csc => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(1.0 / loperand.sin()),
            TrigFuncArg::Degree => Ok(1.0 / loperand.to_radians().sin()),
        },
        NodeKind::Sec => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(1.0 / loperand.cos()),
            TrigFuncArg::Degree => Ok(1.0 / loperand.to_radians().cos()),
        },
        NodeKind::Cot => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(1.0 / loperand.tan()),
            TrigFuncArg::Degree => Ok(1.0 / loperand.to_radians().tan()),
        },
        NodeKind::AcSin => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(loperand.asin()),
            TrigFuncArg::Degree => Ok(radian2degree(loperand.asin())),
        },
        NodeKind::AcCos => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(loperand.acos()),
            TrigFuncArg::Degree => Ok(radian2degree(loperand.acos())),
        },
        NodeKind::AcTan => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(loperand.atan()),
            TrigFuncArg::Degree => Ok(radian2degree(loperand.atan())),
        },
        NodeKind::Pow => Ok(loperand.powf(roperand)),
        NodeKind::Neg => Ok(-loperand),
        _ => Err(MyError::UDcommandErr(node.node_kind.to_string())),
    }
}

fn getoperand(node: parser::Node, vars: &HashMap<String, f64>) -> Result<f64, MyError> {
    match &node.node_kind {
        NodeKind::Num | NodeKind::Var => {
            return Ok(match node.val.unwrap() {
                NumOrVar::Num(n) => n,
                NumOrVar::Var(v) => match vars.get(&v) {
                    Some(n) => *n,
                    None => return Err(MyError::UDvariableErr(v)),
                },
            })
        }
        _ => (),
    }
    calc(node, vars)
}

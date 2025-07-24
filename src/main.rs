// TeX Scientific Calculator

use self::parser::NumOrVar;
use bigdecimal::{BigDecimal, FromPrimitive};
use clap::{value_parser, Arg, Command};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{LazyLock, RwLock};

use parser::NodeKind;
use text_colorizer::*;

mod config;
mod error;
mod math_functions;
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
        log_base: BigDecimal::from_f64(std::f64::consts::E).unwrap(),
        num_of_digit: 12,
    })
});

pub static CONSTS: LazyLock<RwLock<HashMap<String, BigDecimal>>> = LazyLock::new(|| {
    RwLock::new({
        let mut consts = HashMap::new();
        consts.insert(
            "e".to_string(),
            BigDecimal::from_f64(std::f64::consts::E).unwrap(),
        );
        consts.insert(
            "\\pi".to_string(),
            BigDecimal::from_f64(std::f64::consts::PI).unwrap(),
        );
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
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("tex formulas")
                .help("tex formulas")
                .required(false)
                .value_parser(value_parser!(String)),
        );

    let matches = app.get_matches();

    let is_repl = !matches.args_present();

    match CONFIG
        .write()
        .expect("couldn't write CONFIG")
        .load_from_file()
    {
        Ok(conf_file) => {
            if is_repl {
                eprintln!("config loaded from {conf_file:?}")
            }
        }
        Err(e) => {
            if is_repl {
                eprintln!("config load failed: {e}")
            }
        }
    }

    // formulas from command line arg
    if let Some(form) = matches.get_one::<String>("tex formulas") {
        {
            let mut conf = config_writer().expect("couldn't change ast_format config");
            conf.ast_format = AstFormat::None;
        }
        let mut vars: HashMap<String, BigDecimal> = HashMap::new();
        for line in form.split('\n') {
            if let Err(e) = process_form(line.replace("\r", ""), &mut vars) {
                eprintlnc!(e);
            }
        }
        return;
    }

    // formulas from file
    if let Some(file_name) = matches.get_one::<String>("file") {
        {
            let mut conf = config_writer().expect("couldn't change ast_format config");
            conf.ast_format = AstFormat::None;
        }
        let f: File = File::open(file_name).expect(file_name);
        let reader: BufReader<File> = BufReader::new(f);
        let mut vars: HashMap<String, BigDecimal> = HashMap::new();
        for line in reader.lines() {
            if let Err(e) = process_form(line.unwrap(), &mut vars) {
                eprintlnc!(e);
            }
        }
        return;
    }

    // REPL
    let mut vars: HashMap<String, BigDecimal> = HashMap::new();
    let mut rl = match DefaultEditor::new() {
        Ok(r) => r,
        Err(_) => panic!("Can't readline!"),
    };
    loop {
        let readline = rl.readline("tsc> ");
        let form = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .expect("failed add history");
                line
            }
            Err(ReadlineError::Interrupted) => return,
            Err(ReadlineError::Eof) => return,
            Err(err) => panic!("{}", err),
        };
        if form.trim() == ":q" {
            return;
        }
        match process_form(form.to_string(), &mut vars) {
            Ok(_) => (),
            Err(MyError::Quit) => return,
            Err(MyError::NoToken) => (),
            Err(e) => eprintlnc!(e),
        }
    }
}

fn process_form(
    form: String,
    vars: &mut HashMap<String, BigDecimal>,
) -> Result<BigDecimal, MyError> {
    let lex = parser::lexer::Lexer::new(form)?;
    let mut _pars = parser::Parser::new(lex, vars)?;
    _pars.print_vars();
    let ast_root = _pars.build_ast()?;
    let num_of_digit = match config_reader() {
        Ok(c) => c.num_of_digit,
        Err(e) => {
            return Err(e);
        }
    };
    match calc(*ast_root, vars) {
        Ok(result) => {
            debugln!("resutl: {}", result);
            println!("{}", num_formatter(result.clone(), num_of_digit));
            Ok(result)
        }
        Err(e) => Err(e),
    }
}

fn calc(node: parser::Node, vars: &HashMap<String, BigDecimal>) -> Result<BigDecimal, MyError> {
    match node.node_kind {
        NodeKind::Num | NodeKind::Var => {
            return Ok(match node.val.unwrap() {
                NumOrVar::Num(n) => n,
                NumOrVar::Var(v) => match vars.get(&v) {
                    Some(n) => n.clone(),
                    None => return Err(MyError::UDvariableErr(v)),
                },
            })
        }
        _ => (),
    }

    let loperand: BigDecimal;
    let mut roperand: BigDecimal = BigDecimal::from(1);

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

    fn radian2degree(rad: BigDecimal) -> BigDecimal {
        rad * BigDecimal::from(180) / BigDecimal::from_f64(std::f64::consts::PI).unwrap()
    }

    fn degree2radian(deg: BigDecimal) -> BigDecimal {
        deg * BigDecimal::from_f64(std::f64::consts::PI).unwrap() / BigDecimal::from(180)
    }

    match node.node_kind {
        NodeKind::Add => Ok(loperand + roperand),
        NodeKind::Sub => Ok(loperand - roperand),
        NodeKind::Mul => Ok(loperand * roperand),
        NodeKind::Div => Ok(loperand / roperand),
        NodeKind::Sqrt => loperand
            .sqrt()
            .ok_or(MyError::CalcErr("failed calc sqrt".to_string())),
        NodeKind::Log => Ok(math_functions::log(conf.log_base.clone(), loperand)?),
        NodeKind::Ln => Ok(math_functions::log(
            BigDecimal::from_f64(std::f64::consts::E).unwrap(),
            loperand,
        )?),
        NodeKind::Abs => Ok(loperand.abs()),
        NodeKind::Exp => Ok(loperand.exp()),
        NodeKind::Sin => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(math_functions::sin(loperand)?),
            TrigFuncArg::Degree => Ok(math_functions::sin(degree2radian(loperand))?),
        },
        NodeKind::Cos => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(math_functions::cos(loperand)?),
            TrigFuncArg::Degree => Ok(math_functions::cos(degree2radian(loperand))?),
        },
        NodeKind::Tan => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(math_functions::tan(loperand)?),
            TrigFuncArg::Degree => Ok(math_functions::tan(degree2radian(loperand))?),
        },
        NodeKind::Csc => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(1.0 / math_functions::sin(loperand)?),
            TrigFuncArg::Degree => Ok(1.0 / math_functions::sin(degree2radian(loperand))?),
        },
        NodeKind::Sec => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(1.0 / math_functions::cos(loperand)?),
            TrigFuncArg::Degree => Ok(1.0 / math_functions::cos(degree2radian(loperand))?),
        },
        NodeKind::Cot => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(1.0 / math_functions::tan(loperand)?),
            TrigFuncArg::Degree => Ok(1.0 / math_functions::tan(degree2radian(loperand))?),
        },
        NodeKind::AcSin => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(math_functions::asin(loperand)?),
            TrigFuncArg::Degree => Ok(radian2degree(math_functions::asin(loperand)?)),
        },
        NodeKind::AcCos => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(math_functions::acos(loperand)?),
            TrigFuncArg::Degree => Ok(radian2degree(math_functions::acos(loperand)?)),
        },
        NodeKind::AcTan => match conf.trig_func_arg {
            TrigFuncArg::Radian => Ok(math_functions::atan(loperand)?),
            TrigFuncArg::Degree => Ok(radian2degree(math_functions::atan(loperand)?)),
        },
        NodeKind::Pow => Ok(math_functions::pow(loperand, roperand)?),
        NodeKind::Neg => Ok(-loperand),
        _ => Err(MyError::UDcommandErr(node.node_kind.to_string())),
    }
}

fn getoperand(
    node: parser::Node,
    vars: &HashMap<String, BigDecimal>,
) -> Result<BigDecimal, MyError> {
    match &node.node_kind {
        NodeKind::Num | NodeKind::Var => {
            return Ok(match node.val.unwrap() {
                NumOrVar::Num(n) => n,
                NumOrVar::Var(v) => match vars.get(&v) {
                    Some(n) => n.clone(),
                    None => return Err(MyError::UDvariableErr(v)),
                },
            })
        }
        _ => (),
    }
    calc(node, vars)
}

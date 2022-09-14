// TeX Scientific Calculator

use clap::{Command, Arg};
use std::fs::File;
use std::io::{BufReader, BufRead, stdout, Write};
use std::io;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::RwLock;

use text_colorizer::*;
use parser::NodeKind;

mod parser;
mod config;
mod error;
#[macro_use] mod macros;

use config::*;
use error::*;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = {
        RwLock::new(config::Config { result_format: ResultFormat::Decimal, debug: false, trig_func_arg: TrigFuncArg::Radian, log_base: std::f64::consts::E, num_of_digit: 12 })
    };
}

fn main() {
    let app = Command::new("tsc")
        .version("0.1.0")
        .author("tomiy <tomiy@tomiylab.com>")
        .about("TeXSC: TeX Scientific Calculator") 
        .arg(Arg::new("file")
        .help("load formulas from file")
        .short('f')
        .takes_value(true)
        )
        .arg(Arg::new("tex formulas")
        .help("tex formulas")
        .required(false)
        );

    let matches = app.get_matches();
    
    // formulas from command line arg
    if let Some(form) = matches.value_of("tex formulas") {
        let mut vars: HashMap<String, f64> = HashMap::new();
        for line in form.split('\n') {
            process_form(line.replace("\r", ""), &mut vars);
        }
        return;
    }

    // formulas from file
    if let Some(file_name) = matches.value_of("file") {
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
    loop {
        print!("tsc> ");
        stdout().flush().unwrap();
        let mut form: String = String::new();
        io::stdin().read_line(&mut form)
        .expect("stdin");
        if form.trim() == "exit" {
            return;
        }
        process_form(form.to_string(), &mut vars);
    }
    
}

fn process_form(form: String, vars: &mut HashMap<String, f64>) {
    let lex = match parser::lexer::Lexer::new(form) {
        Ok(l) => l,
        Err(e) => {
            eprintlnc!(e);
            return;
        }
    };
    let mut _pars = match parser::Parser::new(lex, vars) {
        Ok(p) => p,
        Err(e) => {
            eprintlnc!(e);
            return;
        },
    };
    _pars.print_vars();
    let ast_root = match _pars.build_ast() {
        Ok(ast) => ast,
        Err(e) => match e {
            MyError::NoToken => return,
            _ => {
                eprintlnc!(e);
                return;
            },
        },
    };
    match calc(ast_root) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintlnc!(e),
    }
    return;
}

fn calc(node: Box<parser::Node>) -> Result<f64, MyError> {

    match (*node).node_kind {
        NodeKind::NdNum => return Ok((*node).val.unwrap()),
        _ => (),
    }

    let mut loperand: f64 = 1.0;
    let mut roperand: f64 = 1.0;

    if let Some(left) = (*node).left_node {
        loperand = getoperand(left)?;
    } else {
        // NdNum以外でleftがNoneはエラー
        // ここに到達した => 不正なAST
        return Err(MyError::BrokenAstErr);
    }

    if let Some(right) = (*node).right_node {
        roperand = getoperand(right)?;
    } else {
        // NdNum以外でrightがNoneはありえる
        // 前置, 1引数のノードの場合 => 正常
        // それ以外 => 不正なAST
        match (*node).node_kind {
            NodeKind::NdAdd => return Err(MyError::BrokenAstErr),
            NodeKind::NdSub => return Err(MyError::BrokenAstErr),
            NodeKind::NdDiv => return Err(MyError::BrokenAstErr),
            NodeKind::NdMul => return Err(MyError::BrokenAstErr),
            _ => (),
        }
    }

    let conf = match read_config() {
        Ok(c) => c,
        Err(e) => return Err(MyError::ConfigReadErr(e.to_string())),
    };

    match (*node).node_kind {
        NodeKind::NdAdd => Ok(loperand + roperand),
        NodeKind::NdSub => Ok(loperand - roperand),
        NodeKind::NdMul => Ok(loperand * roperand),
        NodeKind::NdDiv => Ok(loperand / roperand),
        NodeKind::NdSqrt => Ok(loperand.sqrt()),
        NodeKind::NdLog => Ok(loperand.log(conf.log_base)),
        NodeKind::NdLn => Ok(loperand.log(std::f64::consts::E)),
        NodeKind::NdAbs => Ok(loperand.abs()),
        NodeKind::NdExp => Ok(std::f64::consts::E.powf(loperand)),
        NodeKind::NdSin => {
            match conf.trig_func_arg {
                TrigFuncArg::Radian => Ok(loperand.sin()),
                TrigFuncArg::Degree => Ok(loperand.to_radians().sin()),
            }
        },
        NodeKind::NdCos =>  {
            match conf.trig_func_arg {
                TrigFuncArg::Radian => Ok(loperand.cos()),
                TrigFuncArg::Degree => Ok(loperand.to_radians().cos()),
            }
        },
        NodeKind::NdTan =>  {
            match conf.trig_func_arg {
                TrigFuncArg::Radian => Ok(loperand.tan()),
                TrigFuncArg::Degree => Ok(loperand.to_radians().tan()),
            }
        },
        NodeKind::NdCsc => {
            match conf.trig_func_arg {
                TrigFuncArg::Radian => Ok(1.0/loperand.sin()),
                TrigFuncArg::Degree => Ok(1.0/loperand.to_radians().sin()),
            }
        },
        NodeKind::NdSec =>  {
            match conf.trig_func_arg {
                TrigFuncArg::Radian => Ok(1.0/loperand.cos()),
                TrigFuncArg::Degree => Ok(1.0/loperand.to_radians().cos()),
            }
        },
        NodeKind::NdCot =>  {
            match conf.trig_func_arg {
                TrigFuncArg::Radian => Ok(1.0/loperand.tan()),
                TrigFuncArg::Degree => Ok(1.0/loperand.to_radians().tan()),
            }
        },
        NodeKind::NdAcSin => Ok(loperand.asin()),
        NodeKind::NdAcCos => Ok(loperand.acos()),
        NodeKind::NdAcTan => Ok(loperand.atan()),
        _  => Err(MyError::UDcommandErr((*node).node_kind.to_string())),
    }

}

fn getoperand(node: Box<parser::Node>) -> Result<f64, MyError> {
    match &(*node).node_kind {
        NodeKind::NdNum => return Ok((*node).val.unwrap()),
        _ => (),
    }
    calc(node)
}

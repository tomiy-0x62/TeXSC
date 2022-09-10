// TeX Scientific Calculator

use clap::{Command, Arg};
use std::fs::File;
use std::io::{BufReader, BufRead, stdout, Write};
use std::io;
use std::collections::HashMap;
use thiserror::Error;
use lazy_static::lazy_static;
use std::sync::RwLock;

use text_colorizer::*;

// mod lexer;
mod parser;
mod config;
mod error;
#[macro_use] mod macros;

use config::*;
use error::*;

macro_rules! eprintlnc {
    ($e:expr) => {
        eprintln!("{}: {}", "Error".red(), $e)
    };
}

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = {
        RwLock::new(config::Config { result_format: ResultFormat::decimal, debug: false, trig_func_arg: TrigFuncArg::radian, log_base: std::f64::consts::E, num_of_digit: 12 })
    };
}


fn main_loop() {
    let mut vars: HashMap<String, f64> = HashMap::new();
    loop {
        print!("tsc> ");
        stdout().flush().unwrap();
        let mut form: String = String::new();
        io::stdin().read_line(&mut form)
        .expect("stdin");
        if form.replace("\n", "").as_str() == "exit" {
            return;
        }
        let mut lex = parser::lexer::Lexer::new(form.to_string());
        lex.print_form();
        match lex.analyze() {
            Ok(_) => (),
            Err(e) => {
                eprintlnc!(e);
                continue;
            }
        };
        let mut _pars = match parser::Parser::new(lex, &mut vars) {
            Ok(p) => p,
            Err(e) => {
                eprintlnc!(e);
                continue;
            },
        };
        _pars.print_vars();
        let ast_root = match _pars.build_ast() {
            Ok(ast) => ast,
            Err(e) => match e { 
                MyError::NoToken => continue,
                _ => {
                    eprintlnc!(e);
                    continue;
                },
            },
        };
        match calc(ast_root) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintlnc!(e),
        };
    }
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
        let mut lex = parser::lexer::Lexer::new(form.to_string());
        lex.print_form();
        match lex.analyze() {
            Ok(_) => (),
            Err(e) => {
                eprintlnc!(e);
                return;
            }
        };
        let mut vars: HashMap<String, f64> = HashMap::new();
        let mut _pars = match parser::Parser::new(lex, &mut vars) {
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

    // formulas from file
    if let Some(file_name) = matches.value_of("file") {
        let f: File = File::open(file_name).expect(file_name);
        let reader: BufReader<File> = BufReader::new(f);
        for result in reader.lines() {
            let mut lex = parser::lexer::Lexer::new(result.unwrap());
            lex.print_form();
            match lex.analyze() {
                Ok(_) => (),
                Err(e) => {
                    eprintlnc!(e);
                    continue;
                }
            };
            let mut vars: HashMap<String, f64> = HashMap::new();
            let mut _pars = match parser::Parser::new(lex, &mut vars) {
                Ok(p) => p,
                Err(e) => {
                    eprintlnc!(e);
                    continue;
                },
            };
            _pars.print_vars();
            let ast_root = match _pars.build_ast() {
                Ok(ast) => ast,
                Err(e) => match e {
                    MyError::NoToken => continue,
                    _ => {
                        eprintlnc!(e);
                        continue;
                    },
                },
            };
            match calc(ast_root) {
                Ok(result) => println!("{}", result),
                Err(e) => eprintlnc!(e),
            }
        }
        return;
    }
    
    // REPL
    main_loop();
    
}

fn calc(node: Box<parser::Node>) -> Result<f64, MyError> {

    match (*node).node_kind {
        parser::NodeKind::NdNum => return Ok((*node).val.unwrap()),
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
            parser::NodeKind::NdSin => (),
            parser::NodeKind::NdCos => (),
            parser::NodeKind::NdTan => (),
            parser::NodeKind::NdCsc => (),
            parser::NodeKind::NdSec => (),
            parser::NodeKind::NdCot => (),
            parser::NodeKind::NdAcSin => (),
            parser::NodeKind::NdAcCos => (),
            parser::NodeKind::NdAcTan => (),
            parser::NodeKind::NdSqrt => (),
            parser::NodeKind::NdLog => (),
            parser::NodeKind::NdLn => (),
            parser::NodeKind::NdExp => (),
            _ => return Err(MyError::BrokenAstErr),
        }
    }

    let conf = match read_config() {
        Ok(c) => c,
        Err(e) => return Err(MyError::ConfigReadErr(e.to_string())),
    };

    match (*node).node_kind {
        parser::NodeKind::NdAdd => Ok(loperand + roperand),
        parser::NodeKind::NdSub => Ok(loperand - roperand),
        parser::NodeKind::NdMul => Ok(loperand * roperand),
        parser::NodeKind::NdDiv => Ok(loperand / roperand),
        parser::NodeKind::NdSqrt => Ok(loperand.sqrt()), // TODO: sqrtの中が負のときの処理を実装
        parser::NodeKind::NdLog => {
            Ok(loperand.log(conf.log_base))
        },
        parser::NodeKind::NdLn => Ok(loperand.log(std::f64::consts::E)),
        parser::NodeKind::NdExp => Ok(std::f64::consts::E.powf(loperand)),
        parser::NodeKind::NdSin => {
            match conf.trig_func_arg {
                TrigFuncArg::radian => Ok(loperand.sin()),
                TrigFuncArg::degree => Ok(loperand.to_radians().sin()),
            }
        },
        parser::NodeKind::NdCos =>  {
            match conf.trig_func_arg {
                TrigFuncArg::radian => Ok(loperand.cos()),
                TrigFuncArg::degree => Ok(loperand.to_radians().cos()),
            }
        },
        parser::NodeKind::NdTan =>  {
            match conf.trig_func_arg {
                TrigFuncArg::radian => Ok(loperand.tan()),
                TrigFuncArg::degree => Ok(loperand.to_radians().tan()),
            }
        },
        parser::NodeKind::NdAcSin => Ok(loperand.asin()),
        parser::NodeKind::NdAcCos => Ok(loperand.acos()),
        parser::NodeKind::NdAcTan => Ok(loperand.atan()),
        _  => Err(MyError::UDcommandErr((*node).node_kind.to_string())),
    }

}

fn getoperand(node: Box<parser::Node>) -> Result<f64, MyError> {
    match &(*node).node_kind {
        parser::NodeKind::NdAdd => calc(node),
        parser::NodeKind::NdSub => calc(node),
        parser::NodeKind::NdMul => calc(node),
        parser::NodeKind::NdDiv => calc(node),
        parser::NodeKind::NdSqrt => calc(node),
        parser::NodeKind::NdLog => calc(node),
        parser::NodeKind::NdLn => calc(node),
        parser::NodeKind::NdExp => calc(node),
        parser::NodeKind::NdSin => calc(node),
        parser::NodeKind::NdCos => calc(node),
        parser::NodeKind::NdTan => calc(node),
        parser::NodeKind::NdAcSin => calc(node),
        parser::NodeKind::NdAcCos => calc(node),
        parser::NodeKind::NdAcTan => calc(node),
        parser::NodeKind::NdNum => Ok((*node).val.unwrap()),
        _  => return Err(MyError::UDcommandErr((*node).node_kind.to_string())),
    }
}

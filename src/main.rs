// TeX Scientific Calculator

use clap::{Command, Arg};
use std::fs::File;
use std::io::{BufReader, BufRead, stdout, Write};
use std::io;
use std::collections::HashMap;
use thiserror::Error;
use parser::ParserError;

// mod lexer;
mod parser;

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
        // lex.print_form();
        match lex.analyze() {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let mut _pars = parser::Parser::new(lex, &mut vars);
        // _pars.print_vars();
        let ast_root = match _pars.build_ast() {
            Ok(ast) => ast,
            Err(e) => match e {
                ParserError::NoToken => continue,
                _ => {
                    println!("{}", e);
                    continue;
                },
            },
        };
        match calc(ast_root) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{}", e),
        }
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
        // lex.print_form();
        match lex.analyze() {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        let mut vars: HashMap<String, f64> = HashMap::new();
        let mut _pars = parser::Parser::new(lex, &mut vars);
        let ast_root = match _pars.build_ast() {
            Ok(ast) => ast,
            Err(e) => match e {
                ParserError::NoToken => return,
                _ => {
                    println!("{}", e);
                    return;
                },
            },
        };
        match calc(ast_root) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{}", e),
        }
        return;
    }

    // formulas from file
    if let Some(file_name) = matches.value_of("file") {
        let f: File = File::open(file_name).expect(file_name);
        let reader: BufReader<File> = BufReader::new(f);
        for result in reader.lines() {
            let mut lex = parser::lexer::Lexer::new(result.unwrap());
            // lex.print_form();
            match lex.analyze() {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };
            let mut vars: HashMap<String, f64> = HashMap::new();
            let mut _pars = parser::Parser::new(lex, &mut vars);
            let ast_root = match _pars.build_ast() {
                Ok(ast) => ast,
                Err(e) => match e {
                    ParserError::NoToken => continue,
                    _ => {
                        println!("{}", e);
                        continue;
                    },
                },
            };
            match calc(ast_root) {
                Ok(result) => println!("{}", result),
                Err(e) => println!("{}", e),
            }
        }
        return;
    }
    
    // REPL
    main_loop();
    
}

#[derive(Debug, Error)]
enum CalcError {
    #[error("Broken AST")]
    BrokenAstErr,
    #[error("Undiffined command: {0}")]
    UDcommandErr(String),
}

fn calc(node: Box<parser::Node>) -> Result<f64, CalcError> {

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
        return Err(CalcError::BrokenAstErr);
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
            _ => return Err(CalcError::BrokenAstErr),
        }
    }

    match (*node).node_kind {
        parser::NodeKind::NdAdd => Ok(loperand + roperand),
        parser::NodeKind::NdSub => Ok(loperand - roperand),
        parser::NodeKind::NdMul => Ok(loperand * roperand),
        parser::NodeKind::NdDiv => Ok(loperand / roperand),
        parser::NodeKind::NdSqrt => Ok(loperand.sqrt()), // TODO: sqrtの中が負のときの処理を実装
        parser::NodeKind::NdLog => Ok(loperand.log(std::f64::consts::E)),
        parser::NodeKind::NdLn => Ok(loperand.log(std::f64::consts::E)),
        parser::NodeKind::NdExp => Ok(std::f64::consts::E.powf(loperand)),
        parser::NodeKind::NdSin => Ok(loperand.sin()),
        parser::NodeKind::NdCos => Ok(loperand.cos()),
        parser::NodeKind::NdTan => Ok(loperand.tan()),
        parser::NodeKind::NdAcSin => Ok(loperand.asin()),
        parser::NodeKind::NdAcCos => Ok(loperand.acos()),
        parser::NodeKind::NdAcTan => Ok(loperand.atan()),
        _  => Err(CalcError::UDcommandErr((*node).node_kind.to_string())),
    }

}

fn getoperand(node: Box<parser::Node>) -> Result<f64, CalcError> {
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
        _  => return Err(CalcError::UDcommandErr((*node).node_kind.to_string())),
    }
}

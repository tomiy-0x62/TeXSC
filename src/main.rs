// TeX Scientific Calculator

use clap::{Command, Arg};
use parser::Node;
use std::fs::File;
use std::io::{BufReader, BufRead, stdout, Write};
use std::io;
use std::collections::HashMap;

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
        lex.print_form();
        lex.analyze();
        let mut _pars = parser::Parser::new(lex, &mut vars);
        _pars.print_vars();
        let mut ast_root = _pars.build_ast();
        if let Ok(result) = calc(ast_root) {
            println!("resutl: {}", result);
        } else {
            println!("failed");
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
        lex.print_form();
        lex.analyze();
        let mut vars: HashMap<String, f64> = HashMap::new();
        let mut _pars = parser::Parser::new(lex, &mut vars);
        let mut ast_root = _pars.build_ast();
        if let Ok(result) = calc(ast_root) {
            println!("resutl: {}", result);
        } else {
            println!("failed");
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
            lex.analyze();
            let mut vars: HashMap<String, f64> = HashMap::new();
            let mut _pars = parser::Parser::new(lex, &mut vars);
            let mut ast_root = _pars.build_ast();
            if let Ok(result) = calc(ast_root) {
                println!("resutl: {}", result);
            } else {
                println!("failed");
            }
        }
        return;
    }
    
    // REPL
    main_loop();
    
}

enum CalcError {
    BrokenAstErr,
    UDcommandErr,
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
        // NdNum以外でleftがNoneはありえる
        // 前置, 1引数のノードの場合 => 正常
        // それ以外 => 不正なAST
        match (*node).node_kind {
            parser::NodeKind::NdSin => (),
            parser::NodeKind::NdCos => (),
            parser::NodeKind::NdTan => (),
            parser::NodeKind::NdCsc => (),
            parser::NodeKind::NdSec => (),
            parser::NodeKind::NdCot => (),
            parser::NodeKind::NdSqrt => (),
            parser::NodeKind::NdLog => (),
            _ => return Err(CalcError::BrokenAstErr),
        }
    }

    match (*node).node_kind {
        parser::NodeKind::NdAdd => Ok(loperand + roperand),
        parser::NodeKind::NdSub => Ok(loperand - roperand),
        parser::NodeKind::NdMul => Ok(loperand * roperand),
        parser::NodeKind::NdDiv => Ok(loperand / roperand),
        _  => Err(CalcError::UDcommandErr),
    }

}

fn getoperand(node: Box<parser::Node>) -> Result<f64, CalcError> {
    match &(*node).node_kind {
        parser::NodeKind::NdAdd => calc(node),
        parser::NodeKind::NdSub => calc(node),
        parser::NodeKind::NdMul => calc(node),
        parser::NodeKind::NdDiv => calc(node),
        parser::NodeKind::NdNum => Ok((*node).val.unwrap()),
        _  => return Err(CalcError::UDcommandErr),
    }
}

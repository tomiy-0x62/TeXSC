// TeX Scientific Calculator

use clap::{Command, Arg};
use std::fs::File;
use std::io::{BufReader, BufRead, stdout, Write};
use std::io;
use std::collections::HashMap;

// mod lexer;
mod parser;

// TODO: .a=\piを入力すると不審な挙動を示すから修正

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
        }
        return;
    }
    
    // REPL
    main_loop();
    
}

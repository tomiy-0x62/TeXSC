// TeX Scientific Calculator

use clap::{Command, Arg};
use std::fs::File;
use std::io::{BufReader, BufRead, stdout, Write};
use std::io;
use regex::Regex;

struct Lexer {
    formulas: String,
    tokens: Vec<String>
}

/*
tokenの種類
- 数値リテラル: 0x54, 0b100011, 534
- TeXコマンド: \log, \sum
- コマンド: sin, cos
- かっこ: (, ), {, }
- 変数: x, y
*/

impl Lexer {
    fn new(form: String) -> Lexer {
        Lexer { formulas: form, tokens: Vec::new() }
    }

    fn print_form(&self) {
        println!("form: {}", self.formulas.replace("\n", " "));
    }

    fn analyze(&mut self) {
        let tex_command = Regex::new(r"\\[A-Za-z]*").unwrap(); // OK
        let operator = Regex::new(r"\+|-|/|\|").unwrap(); // OK
        let command = Regex::new(r"sin|cos|tan|arcsin|arccons|arctan").unwrap(); // OK
        let var = Regex::new(r"[A-Za-z][A-Za-z0-9]*").unwrap(); // OK
        let num = Regex::new(r"0x[0-9]+|0b[0-1]+|[0-9]+\.?[0-9]*").unwrap(); //
        let braces = Regex::new(r"\(|\)|\[|\]|\{|\}").unwrap(); // OK
        let token_types: Vec<Regex> = [tex_command, operator, command, var, num, braces].to_vec();

        
        /*
        for caps in tex_command.captures_iter(&self.formulas) {
            println!("match '{}'", &caps[0]);
        }*/
        
        /* if let Some(caps) = braces.captures(&self.formulas) {
            println!("match '{}'", caps.get(0).unwrap().as_str());
            // if let Some(hoge) = caps.get(0)
        }*/

    }
}

fn main_loop() {
    loop {
        print!("tsc> ");
        stdout().flush().unwrap();
        let mut form = String::new();
        io::stdin().read_line(&mut form)
        .expect("stdin");
        if form.replace("\n", "").as_str() == "exit" {
            return;
        }
        let mut lex = Lexer::new(form.to_string());
        lex.print_form();
        lex.analyze();
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
        let mut lex = Lexer::new(form.to_string());
        lex.print_form();
        lex.analyze();
        return;
    }

    // formulas from file
    if let Some(file_name) = matches.value_of("file") {
        let f = File::open(file_name).expect(file_name);
        let reader = BufReader::new(f);
        for result in reader.lines() {
            let mut lex = Lexer::new(result.unwrap());
            lex.print_form();
            lex.analyze();
        }
        return;
    }
    
    // REPL
    main_loop();
    
}

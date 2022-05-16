// TeX Scientific Calculator

use clap::{Command, Arg};
use std::fs::File;
use std::io::{BufReader, BufRead, stdout, Write};
use std::io;
use regex::Regex;

enum TokenType {
    TexCommand,
    Operator,
    Var,
    Num,
    Brace,
}

struct Token {
    token: String,
    token_type: TokenType,
}

struct BNord<'a> {
    token: Token,
    right_nord: &'a BNord<'a>,
    left_nord: &'a BNord<'a>,
    Val: f64,
    is_calced: bool,
}

struct SNord<'a> {
    token: Token,
    child_nord: &'a SNord<'a>,
    Val: f64,
    is_calced: bool,
}

struct Lexer {
    formulas: String,
    tokens: Vec<Token>
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
        let form = form.replace("\n", "").replace("\t", "");
        Lexer { formulas: form, tokens: Vec::new() }
    }

    fn print_form(&self) {
        println!("form: {}", self.formulas.replace("\n", " "));
    }

    fn print_token(&self) {
        for token in self.tokens.iter() {
            // {}でした
            print!("'{}', ", token.token);
        }
        println!("");
    }

    fn analyze(&mut self) {
        let tex_command: Regex = Regex::new(r"\\[A-Za-z]*").unwrap(); // OK
        let operator: Regex = Regex::new(r"\+|-|/|!|_|,|\^|\|").unwrap(); // OK
        let var: Regex = Regex::new(r"[A-Za-z][A-Za-z0-9]*").unwrap(); // OK
        let num: Regex = Regex::new(r"0x[0-9a-fA-F]+|0b[0-1]+|[0-9]+\.?[0-9]*").unwrap(); // OK
        let braces: Regex = Regex::new(r"\(|\)|\[|\]|\{|\}").unwrap(); // OK
        // let token_types: Vec<Regex> = [tex_command, operator, command, var, num, braces].to_vec();

        loop {
            // TODO: 0b423 -> num:"0", var"b423"と分割失敗してるのを修正
            // 0b423みたいなのがきたらエラーにしたい
            // TODO: a\sindsをどう扱うか決める -> 'a', '\sin', 'ds' or '\sinds'(構文解析のときにpanic)
            let mut c: char = self.formulas.chars().nth(0).unwrap();
            if c == ' ' {
                self.formulas = self.formulas.replacen(" ", "", 1);
                c = self.formulas.chars().nth(0).unwrap();
            }
            let mut ismatch: bool = false;
            if c == '\\' {
                if let Some(caps) = tex_command.captures(&self.formulas) {
                    // println!("<<< match '{}' as tex_command >>>", caps.get(0).unwrap().as_str());
                    self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_type: TokenType::TexCommand});
                    self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    // println!("formulas: '{}'", self.formulas.replace(" ", ""));
                    ismatch = true;
                }
            } else if let Some(caps) = operator.captures(&c.to_string()) {
                // println!("<<< match '{}' as operator >>>", caps.get(0).unwrap().as_str());
                self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_type: TokenType::Operator});
                self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                // println!("formulas: '{}'", self.formulas.replace(" ", ""));
                ismatch = true;
            } else if let Some(caps) = braces.captures(&c.to_string()) {
                // println!("<<< match '{}' as braces >>>", caps.get(0).unwrap().as_str());
                self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_type: TokenType::Brace});
                self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                // println!("formulas: '{}'", self.formulas.replace(" ", ""));
                ismatch = true;
            } else if let Some(_) = num.captures(&c.to_string()) {
                if let Some(caps) = num.captures(&self.formulas) {
                    // println!("<<< match '{}' as num >>>", caps.get(0).unwrap().as_str());
                    self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_type: TokenType::Num});
                    self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    // println!("formulas: '{}'", self.formulas.replace(" ", ""));
                    ismatch = true;
                }
            } else if let Some(caps) = var.captures(&self.formulas) {
                if c != caps.get(0).unwrap().as_str().chars().nth(0).unwrap() { continue; }
                    // println!("<<< match '{}' as var >>>", caps.get(0).unwrap().as_str());
                    self.tokens.push(Token {token: caps.get(0).unwrap().as_str().to_string().replace(" ", ""), token_type: TokenType::Var});
                    self.formulas = self.formulas.replacen(caps.get(0).unwrap().as_str(), "", 1);
                    // println!("formulas: '{}'", self.formulas.replace(" ", ""));
                    ismatch = true;
            } 
            if !ismatch {
                panic!("hoge")
            }

            // println!("formulas: '{}'", self.formulas);

            if self.formulas.len() == 0 {
                self.print_token();
                break;
            }
        }

        
        /*
        for caps in tex_command.captures_iter(&self.formulas) {
            println!("match '{}'", &caps[0]);
        }*/
        
        if let Some(caps) = operator.captures(&self.formulas) {
            println!("<<< match '{}' >>>", caps.get(0).unwrap().as_str());
            // if let Some(hoge) = caps.get(0)
        }

    }
}

fn main_loop() {
    loop {
        print!("tsc> ");
        stdout().flush().unwrap();
        let mut form: String = String::new();
        io::stdin().read_line(&mut form)
        .expect("stdin");
        if form.replace("\n", "").as_str() == "exit" {
            return;
        }
        let mut lex: Lexer = Lexer::new(form.to_string());
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
        let mut lex: Lexer = Lexer::new(form.to_string());
        lex.print_form();
        lex.analyze();
        return;
    }

    // formulas from file
    if let Some(file_name) = matches.value_of("file") {
        let f: File = File::open(file_name).expect(file_name);
        let reader: BufReader<File> = BufReader::new(f);
        for result in reader.lines() {
            let mut lex:Lexer = Lexer::new(result.unwrap());
            lex.print_form();
            lex.analyze();
        }
        return;
    }
    
    // REPL
    main_loop();
    
}

// TeX Scientific Calculator

use bigdecimal::BigDecimal;
use clap::{value_parser, Arg, Command};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use tsc::config::{config_writer, AstFormat};
use tsc::error::MyError;
use tsc::load_config_from_file;
use tsc::process_form;

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

    load_config_from_file();

    // formulas from command line arg
    if let Some(form) = matches.get_one::<String>("tex formulas") {
        {
            let mut conf_w = config_writer().expect("couldn't change ast_format config");
            conf_w.ast_format = AstFormat::None;
        }
        let mut vars: HashMap<String, BigDecimal> = HashMap::new();
        for line in form.split('\n') {
            match process_form(line.replace("\r", ""), &mut vars) {
                Ok(res) => {
                    for (_v, p) in res {
                        print!("{} ", p);
                    }
                    println!();
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        return;
    }

    // formulas from file
    if let Some(file_name) = matches.get_one::<String>("file") {
        {
            let mut conf_w = config_writer().expect("couldn't change ast_format config");
            conf_w.ast_format = AstFormat::None;
        }
        let f: File = File::open(file_name).expect(file_name);
        let reader: BufReader<File> = BufReader::new(f);
        let mut vars: HashMap<String, BigDecimal> = HashMap::new();
        for line in reader.lines() {
            match process_form(
                line.expect("failed split input into lines")
                    .replace("\r", ""),
                &mut vars,
            ) {
                Ok(res) => {
                    for (_v, p) in res {
                        print!("{} ", p);
                    }
                    println!();
                }
                Err(e) => eprintln!("{}", e),
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
        match process_form(form, &mut vars) {
            Ok(res) => {
                for (_v, p) in res {
                    print!("{} ", p);
                }
                println!();
            }
            Err(MyError::Quit) => return,
            Err(MyError::NoToken) => (),
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }
}

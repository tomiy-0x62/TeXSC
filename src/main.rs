// TeX Scientific Calculator

use clap::{Command, Arg};
use std::fs::File;
use std::io::{BufReader, BufRead, stdout, Write};
use std::io;

fn tex_parser(form: String) {
    println!("form: {}", form.replace("\n", " "));
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
        tex_parser(form);
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
        tex_parser(form.to_string());
        return;
    }

    // formulas from file
    if let Some(file_name) = matches.value_of("file") {
        let f = File::open(file_name).expect(file_name);
        let reader = BufReader::new(f);
        for result in reader.lines() {
            tex_parser(result.unwrap());
        }
        return;
    }
    
    // REPL
    main_loop();
    
}

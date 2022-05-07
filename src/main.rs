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
        .arg(Arg::new("formulas")
        .help("Szpecify the formula to calculate")
        .short('c')
        .takes_value(true)
        )
        .arg(Arg::new("file")
        .help("load formulas from file")
        .required(false)
        );

    let matches = app.get_matches();
    
    // formulas from -c option
    if let Some(form) = matches.value_of("formulas") {
        tex_parser(form.to_string());
        return;
    }

    // formulas from file
    if let Some(file_name) = matches.value_of("file") {
        let f = File::open(file_name).expect(file_name);
        let reader = BufReader::new(f);
        let mut buf = String::new();
        for result in reader.lines() {
            buf.push_str(&result.unwrap());
        }
        tex_parser(buf);
        return;
    }
    
    // REPL
    main_loop();
    
}

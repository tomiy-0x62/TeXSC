// TeX Scientific Calculator

use clap::{Command, Arg};

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

    if let Some(form) = matches.value_of("formulas") {
        println!("formulas: {}", form);
    }

    if let Some(file) = matches.value_of("file") {
        println!("file: {}", file);
    }

    println!("Hello, world!");
    
}

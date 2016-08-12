#![allow(dead_code)]

extern crate clap;
use clap::{App, Arg};

mod parsing;


fn main() {
    let matches = App::new("rjs")
                    .arg(Arg::with_name("input")
                            .short("i")
                            .value_name("INPUT")
                            .help("Source to parse")
                            .takes_value(true))
                    .get_matches();
    let input = matches.value_of("input").unwrap();
    println!("input: {}", input);
    
    // let program = parsing::parser::parse_tokens(&tokens);
    // println!("program: {:?}", program);

    let state = parsing::parser::parse_St(input);
    println!("state: {:#?}", state);
}

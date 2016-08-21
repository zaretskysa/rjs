#![allow(dead_code)]

extern crate clap;

use clap::{App, Arg};

mod parsing;
mod evaluating;

use evaluating::evaluator::*;

fn main() {
    let matches = App::new("rjs")
        .arg(Arg::with_name("input")
            .short("i")
            .value_name("INPUT")
            .help("Source to parse")
            .takes_value(true)
            .use_delimiter(false))
        .get_matches();
    let input = matches.value_of("input").unwrap();
    println!("input: {}", input);

    let prog = parsing::parser::parse_Prog(input).unwrap();
    println!("state: {:#?}", prog);

    let mut evaluator = Evaluator::new();
    let value = evaluator.eval(&prog);
    println!("result: {:?}", value);
}

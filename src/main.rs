#![allow(dead_code)]

#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate clap;

use clap::{App, Arg};

mod parsing;
mod evaluating;

// use evaluating::evaluator::*;
// use parsing::parser::*;



peg_file! grammar("grammar.rustpeg");

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

    // let prog = grammar::PrimaryExpression(input).unwrap();
    let prog = grammar::ObjectLiteral(input).unwrap();
    println!("state: {:#?}", prog);

    // let mut evaluator = Evaluator::new();
    // let value = evaluator.eval(&prog);
    // println!("result: {:?}", value);
}

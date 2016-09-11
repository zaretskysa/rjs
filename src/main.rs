#![allow(dead_code)]

#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#![feature(box_syntax)]


extern crate clap;

use clap::{App, Arg};

mod parsing;
mod evaluating;

// use evaluating::evaluator::*;
use parsing::parser;

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

    let prog = parser::parse(input);
    println!("result:\n{:#?}", prog);

    // let mut evaluator = Evaluator::new();
    // let value = evaluator.eval(&prog);
    // println!("result: {:?}", value);
}

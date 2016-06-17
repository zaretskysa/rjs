extern crate combine;

extern crate clap;
use clap::{App, Arg};

mod lexing;


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
    // let result = parser(lexing::lexer::tokens).parse(input);
    let result = lexing::lexer::tokenize(input);
    println!("result: {:?}", result);
}

use parsing::ast::*;

pub fn parse(input: &str) -> Program {
    return grammar::Program(input).unwrap();
}

peg_file! grammar("grammar.rustpeg");

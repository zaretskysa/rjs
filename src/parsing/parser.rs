use parsing::ast::*;

pub fn expression(input: &str) -> SuperExpression {
    return grammar::Expression(input).unwrap();
}

peg_file! grammar("grammar.rustpeg");

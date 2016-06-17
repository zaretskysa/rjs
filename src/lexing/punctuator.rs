use combine::{choice, value, string, try, Parser, ParserExt};
use combine::primitives::{State, Stream, ParseResult};

use lexing::token::*;

pub fn punctuator<I>(input: State<I>) -> ParseResult<Punctuator, I>
where I: Stream<Item=char> {
    let equals = try(string("==")).with(value(Punctuator::Equals));
    let assign = string("=").with(value(Punctuator::Assign));
    let plus = string("+").with(value(Punctuator::Plus));
    let minus = string("-").with(value(Punctuator::Minus));
    let mult = string("*").with(value(Punctuator::Mult));
    let div = string("/").with(value(Punctuator::Div));
    let colon = string(":").with(value(Punctuator::Colon));
    let semicolon = string(";").with(value(Punctuator::Semicolon));
    let comma = string(";").with(value(Punctuator::Comma));
    let open_paren = string("(").with(value(Punctuator::OpenParen));
    let close_paren = string(")").with(value(Punctuator::CloseParen));
    let open_brace = string("{").with(value(Punctuator::OpenBrace));
    let close_brace = string("}").with(value(Punctuator::CloseBrace));
    let logical_and = string("&&").with(value(Punctuator::LogicalAnd));
    let logical_or = string("||").with(value(Punctuator::LogicalOr));
    let not_equals = string("!=").with(value(Punctuator::NotEquals));

    let simple = [
        assign, plus, minus, mult, div, colon, semicolon, comma, open_paren, 
        close_paren, open_brace, close_brace, logical_and, logical_or, not_equals];

    // we are trying to build smaller parser here to avoid 
    // extremly long compilation time (issue https://github.com/Marwes/combine/issues/21)
    equals.or(choice(simple)).parse_state(input)
}

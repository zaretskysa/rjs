use combine::{try, choice, value, string, Parser, ParserExt};
use combine::primitives::{State, Stream, ParseResult};

use lexing::token::*;


pub fn keyword<I>(input: State<I>) -> ParseResult<Keyword, I>
where I: Stream<Item=char> {
    let var = string("var").with(value(Keyword::Var));
    let function = string("function").with(value(Keyword::Function));
    let if_ = string("if").with(value(Keyword::If));
    let else_ = string("else").with(value(Keyword::Else));
    let return_ = string("return").with(value(Keyword::Return));
    let try_ = try(string("try")).with(value(Keyword::Try));
    let throw = try(string("throw")).with(value(Keyword::Throw));
    let catch = try(string("catch")).with(value(Keyword::Catch));
    let continue_ = try(string("continue")).with(value(Keyword::Continue));
    let break_ = string("break").with(value(Keyword::Break));
    let while_ = string("while").with(value(Keyword::While));

    let keywords_with_try = [try_, throw, catch, continue_];
    let keywords = [var, function, if_, else_, return_, break_, while_];
    
    // we are trying to build smaller parser here to avoid 
    // extremly long compilation time (issue https://github.com/Marwes/combine/issues/21)
    choice(keywords_with_try).or(choice(keywords)).parse_state(input)
}

pub fn is_keyword(s: &str) -> bool {
    let keywords: Vec<&'static str> = vec!(
        "var", "function", "if", "else", "return", "try", 
        "throw", "catch", "continue", "break", "while");
    keywords.contains(&s)
}
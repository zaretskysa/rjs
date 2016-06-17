use combine::{not_followed_by, any, char, sep_by, spaces, try, parser, Parser, ParserExt};
use combine::primitives::{State, Stream, ParseResult, ParseError};

use lexing::token::*;
use lexing::boolean::boolean;
use lexing::identifier::identifier;
use lexing::keyword::keyword;
use lexing::number::number;
use lexing::punctuator::punctuator;
use lexing::string::string;


pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError<&str>>
{
   let res = parser(tokens).parse(input);
   match res {
       Ok((toks, _rest)) => Ok(toks),
       Err(e) => Err(e),
   }
}

fn token<I>(input: State<I>) -> ParseResult<Token, I>
where I: Stream<Item=char> {
    let number_tok = parser(number).map(|x| Token::Number(x));
    let string_tok = parser(string).map(|x| Token::String(x));
    let punctuator_tok = parser(punctuator).map(|x| Token::Punctuator(x));
    let identifier_tok = try(parser(identifier)).map(|x| Token::Identifier(x));
    let boolean_tok = try(parser(boolean)).map(|x| Token::Boolean(x));
    let keyword_tok = try(parser(keyword)).map(|x| Token::Keyword(x));

    number_tok
        .or(string_tok)
        .or(punctuator_tok)
        .or(identifier_tok)
        .or(boolean_tok)
        .or(keyword_tok)
        .parse_state(input)
}


pub fn tokens<I>(input: State<I>) -> ParseResult<Vec<Token>, I>
where I: Stream<Item=char> {
    let eof = not_followed_by(any());
    sep_by(parser(token), spaces()).skip(eof).parse_state(input)
}

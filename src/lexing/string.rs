use combine::{satisfy, char, between, many, Parser};
use combine::primitives::{State, Stream, ParseResult};


pub fn string<I>(input: State<I>) -> ParseResult<String, I>
where I: Stream<Item=char> {
    let internal_chars = many::<String, _>(satisfy(|c| c != '"'));
    between(char('"'), char('"'), internal_chars).parse_state(input)
}

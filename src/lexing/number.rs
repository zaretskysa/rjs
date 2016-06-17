use combine::{digit, many1, Parser, ParserExt};
use combine::primitives::{State, Stream, ParseResult};


pub fn number<I>(input: State<I>) -> ParseResult<i32, I>
where I: Stream<Item=char> {
    many1::<String, _>(digit()).map(|x| x.parse().unwrap()).parse_state(input)
}

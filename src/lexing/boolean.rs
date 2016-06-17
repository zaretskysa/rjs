use combine::{value, string, Parser, ParserExt};
use combine::primitives::{State, Stream, ParseResult};


pub fn boolean<I>(input: State<I>) -> ParseResult<bool, I>
where I: Stream<Item=char> {
    let t = string("true").with(value(true));
    let f = string("false").with(value(false));
    t.or(f).parse_state(input)
}

pub fn is_boolean(s: &str) -> bool {
    s == "true" || s == "false"
}
use combine::{parser, letter, many, alpha_num, Parser, ParserExt};
use combine::primitives::{ParseError, Consumed, Error, State, Stream, ParseResult};

use lexing::boolean::{is_boolean};
use lexing::keyword::{is_keyword};


pub fn identifier<I>(input: State<I>) -> ParseResult<String, I>
where I: Stream<Item=char> {
    let build_str = |t| {
        let (first, mut rest): (char, String) = t;
        rest.insert(0, first);
        rest
    };

    let ident = letter().and(many::<String, _>(alpha_num()));

    // holy shit! what does this mean??
    ident.map(build_str).then(|s| parser(move |input| {
        if !is_boolean(&s) && !is_keyword(&s) {
            Ok((s.clone(), Consumed::Empty(input)))
        }
        else {
            let err = ParseError::new(input.position, Error::Message("Not an identifier".into()));
            Err((Consumed::Empty(err)))
        }
    })).parse_state(input)
}


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),
    Boolean(bool),
    String(String),
    Identifier(String),
    Punctuator(Punctuator),
    Keyword(Keyword),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Punctuator {
    Plus,
    Minus,
    Mult,
    Div,
    Assign,
    Colon,
    Semicolon,
    Comma,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    LogicalAnd,
    LogicalOr,
    Equals,
    NotEquals,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Var,
    Function,
    If,
    Else,
    Return,
    Try,
    Catch,
    Throw,
    Continue,
    Break,
    While,
}

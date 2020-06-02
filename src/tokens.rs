#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(String),
    Id(String),
    Class(String),
    Tag(String),
    Property(String),
    Value(String),
    Colon,
    Comma,
    SemiColon,
    Space,
    Newline,
    Tab,
    Return,
    OpenCurlyBrace,
    ClosingCurlyBrace,
    Eof,
}

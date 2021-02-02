use crate::tokens::Token;

pub fn get_delimiter_token<'a>(c: char) -> Option<Token<'a>> {
  match c {
    '*' => Some(Token::Star),
    '/' => Some(Token::ForwardSlash),
    '\\' => Some(Token::BackwardSlash),
    '\'' => Some(Token::SingleQuote),
    '\"' => Some(Token::DoubleQuote),
    '\n' => Some(Token::Newline),
    '\r' => Some(Token::Return),
    '\t' => Some(Token::Tab),
    ' ' => Some(Token::Space),
    '{' => Some(Token::OpenCurlyBrace),
    '}' => Some(Token::ClosingCurlyBrace),
    '[' => Some(Token::OpenBrace),
    ']' => Some(Token::ClosingBrace),
    ':' => Some(Token::Colon),
    ';' => Some(Token::SemiColon),
    ',' => Some(Token::Comma),
    _ => None,
  }
}

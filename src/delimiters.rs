use crate::tokens::Token;
use std::collections::HashMap;

pub fn get_delimiters<'a>() -> HashMap<char, Token<'a>> {
  let mut map = HashMap::new();
  map.insert('\n', Token::Newline);
  map.insert('\r', Token::Return);
  map.insert('\t', Token::Tab);
  map.insert(' ', Token::Space);
  map.insert('{', Token::OpenCurlyBrace);
  map.insert('}', Token::ClosingCurlyBrace);
  map.insert('[', Token::OpenBrace);
  map.insert(']', Token::ClosingBrace);
  map.insert(':', Token::Colon);
  map.insert(';', Token::SemiColon);
  map.insert(',', Token::Comma);

  map
}

use crate::delimiters::get_delimiter_token;
use crate::tokens::Token;

pub fn is_comment<'a>(index: usize, src: &'a str) -> bool {
  let mut iter = src.chars().skip(index);
  iter.next() == Some('/') && iter.next() == Some('*')
}

pub fn add_comment_token<'a>(index: usize, src: &'a str) -> (usize, Token<'a>) {
  let mut iter = src.char_indices().skip(index);

  while let Some((_, c)) = iter.next() {
    if c == '*' {
      if let Some((offset, c2)) = iter.next() {
        if c2 == '/' {
          return (offset + 1, Token::Comment(&src[index..offset + 1]));
        }
      }
    }
  }

  (src.len(), Token::Comment(&src[index..]))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_comment_test() {
    let css = "/* foobar */";
    assert_eq!(is_comment(0, &css), true)
  }

  #[test]
  fn is_comment_test_fail() {
    let css = "/foobar/";
    assert_eq!(is_comment(0, &css), false)
  }

  #[test]
  fn add_comment_token_test() {
    let css = "/* foo */";
    assert_eq!(add_comment_token(0, &css), (9, Token::Comment(&css)))
  }
}

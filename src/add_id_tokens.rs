use crate::selectors::is_selector_delimiter;
use crate::tokens::Token;

pub fn add_id_token<'a>(index: usize, src: &'a str) -> (usize, Token<'a>) {
  let first_letter = index + 1;
  let mut iter2 = src.char_indices().skip(first_letter);

  while let Some((offset, c)) = iter2.next() {
    if !is_selector_delimiter(&c) {
      continue;
    }

    return (offset, Token::Id(&src[index..offset]));
  }

  let offset = src.len();
  return (offset, Token::Id(&src[index..]));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_id_token_test() {
    let css = "#foo";
    let expected = (4, Token::Id("#foo"));
    assert_eq!(add_id_token(0, &css), expected)
  }

  #[test]
  fn add_id_token_test_with_delimiter() {
    let css = "#foo ";
    let expected = (4, Token::Id("#foo"));
    assert_eq!(add_id_token(0, &css), expected)
  }
}

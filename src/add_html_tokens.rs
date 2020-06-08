use crate::html_tags::HTML_TAGS;
use crate::selectors::is_selector_delimiter;
use crate::tokens::Token;

pub fn add_html_token<'a>(index: usize, src: &'a str) -> (usize, Token<'a>) {
  let first_letter = index + 1;
  let mut iter2 = src.char_indices().skip(first_letter);

  while let Some((offset, c)) = iter2.next() {
    if !is_selector_delimiter(&c) {
      continue;
    }

    if HTML_TAGS.contains(&&src[index..offset]) {
      return (offset, Token::Tag(&src[index..offset]));
    } else {
      return (offset, Token::Unknown(&src[index..offset]));
    }
  }

  if HTML_TAGS.contains(&&src[index..]) {
    let offset = src.len();
    return (offset, Token::Tag(&src[index..]));
  } else {
    let offset = src.len();
    (offset, Token::Unknown(&src[index..]))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn add_html_token_test() {
    let css = "html";
    let expected = (4, Token::Tag("html"));
    assert_eq!(add_html_token(0, &css), expected)
  }

  #[test]
  fn add_html_token_test_with_delimiter() {
    let css = "html ";
    let expected = (4, Token::Tag("html"));
    assert_eq!(add_html_token(0, &css), expected)
  }

  #[test]
  fn add_html_token_test_with_unknown_token() {
    let css = "nothtml";
    let expected = (7, Token::Unknown("nothtml"));
    assert_eq!(add_html_token(0, &css), expected)
  }
}

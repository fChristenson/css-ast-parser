use crate::selectors::is_selector_delimiter;
use crate::tokens::Token;

pub fn add_html_token<'a>(index: usize, src: &'a str) -> (usize, Token<'a>) {
  println!("HTML FOUND");
  let first_letter = index + 1;
  let mut iter2 = src.char_indices().skip(first_letter);
  let mut delimiter_found = false;

  while let Some((offset, c)) = iter2.next() {
    if !is_selector_delimiter(&c) {
      continue;
    }

    delimiter_found = true;
    println!("ADDING HTML {} {}", index, offset);
    return (offset, Token::Tag(&src[index..offset]));
  }

  if !delimiter_found {
    let offset = src.len();
    println!("NO DELIMITER ADDING HTML {} {}", index, offset);
    return (offset, Token::Tag(&src[index..]));
  }

  panic!("Error parsing HTML")
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
}

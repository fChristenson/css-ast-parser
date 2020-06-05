use crate::selectors::is_selector_delimiter;
use crate::tokens::Token;

pub fn add_class_token<'a>(index: usize, src: &'a str) -> (usize, Token<'a>) {
  println!("CLASS FOUND");
  let first_letter = index + 1;
  let mut iter2 = src.char_indices().skip(first_letter);
  let mut delimiter_found = false;

  while let Some((offset, c)) = iter2.next() {
    if !is_selector_delimiter(&c) {
      continue;
    }

    delimiter_found = true;
    println!("ADDING CLASS {} {}", index, offset);
    return (offset, Token::Class(&src[index..offset]));
  }

  if !delimiter_found {
    let offset = src.len();
    println!("NO DELIMITER ADDING CLASS {} {}", index, offset);
    return (offset, Token::Class(&src[index..]));
  }

  panic!("Error parsing class")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_class_token_test() {
    let css = ".foo";
    let expected = (4, Token::Class(".foo"));
    assert_eq!(add_class_token(0, &css), expected)
  }

  #[test]
  fn add_class_token_test_with_delimiter() {
    let css = ".foo ";
    let expected = (4, Token::Class(".foo"));
    assert_eq!(add_class_token(0, &css), expected)
  }
}

use crate::tokens::Token;

pub fn add_rule_tokens<'a>(_index: usize, _src: &'a str, _tokens: &mut Vec<Token<'a>>) -> usize {
  println!("RULE FOUND");
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_rule_tokens_test() {
    let css = "color: red;";
    let mut tokens = vec![];
    add_rule_tokens(0, &css, &mut tokens);
    let expected = vec![
      Token::Rule("color"),
      Token::Colon,
      Token::Value("red"),
      Token::SemiColon,
    ];
    assert_eq!(tokens, expected)
  }
}

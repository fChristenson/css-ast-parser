use crate::delimiters::get_delimiter_token;
use crate::tokens::Token;

pub fn add_rule_tokens<'a>(index: usize, src: &'a str, tokens: &mut Vec<Token<'a>>) -> usize {
  let mut iter = src.char_indices().skip(index);
  let mut token_start = index;
  let mut token_end = index;

  while let Some((offset, c)) = iter.next() {
    token_end = offset;

    if c == ':' {
      let val = src[token_start..offset].trim();
      tokens.push(Token::Rule(val));
      tokens.push(Token::Colon);
      token_start = offset + 1;
    } else if c == ';' {
      let val = src[token_start..offset].trim();
      tokens.push(Token::Value(val));
      tokens.push(Token::SemiColon);
      token_start = offset + 1;
    } else if c == '}' {
      return offset;
    } else if let Some(token) = get_delimiter_token(c) {
      token_start = offset + 1;
      tokens.push(token);
    }
  }

  token_end
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_rule_tokens_test() {
    let css = "color: red;";
    let mut tokens = vec![];
    let token_end = add_rule_tokens(0, &css, &mut tokens);
    let expected = vec![
      Token::Rule("color"),
      Token::Colon,
      Token::Space,
      Token::Value("red"),
      Token::SemiColon,
    ];
    assert_eq!(tokens, expected);
    assert_eq!(token_end, 10)
  }

  #[test]
  fn add_rule_tokens_test_multiple_rules() {
    let css = "color: red;color: blue;";
    let mut tokens = vec![];
    let token_end = add_rule_tokens(0, &css, &mut tokens);
    let expected = vec![
      Token::Rule("color"),
      Token::Colon,
      Token::Space,
      Token::Value("red"),
      Token::SemiColon,
      Token::Rule("color"),
      Token::Colon,
      Token::Space,
      Token::Value("blue"),
      Token::SemiColon,
    ];
    assert_eq!(tokens, expected);
    assert_eq!(token_end, 22)
  }

  #[test]
  fn add_rule_tokens_test_multiple_rules_with_space() {
    let css = "color: red; color: blue;";
    let mut tokens = vec![];
    let token_end = add_rule_tokens(0, &css, &mut tokens);
    let expected = vec![
      Token::Rule("color"),
      Token::Colon,
      Token::Space,
      Token::Value("red"),
      Token::SemiColon,
      Token::Space,
      Token::Rule("color"),
      Token::Colon,
      Token::Space,
      Token::Value("blue"),
      Token::SemiColon,
    ];
    assert_eq!(tokens, expected);
    assert_eq!(token_end, 23)
  }

  #[test]
  fn add_rule_tokens_test_multiple_rules_with_newline() {
    let css = "color: red;\ncolor: blue;";
    let mut tokens = vec![];
    let token_end = add_rule_tokens(0, &css, &mut tokens);
    let expected = vec![
      Token::Rule("color"),
      Token::Colon,
      Token::Space,
      Token::Value("red"),
      Token::SemiColon,
      Token::Newline,
      Token::Rule("color"),
      Token::Colon,
      Token::Space,
      Token::Value("blue"),
      Token::SemiColon,
    ];
    assert_eq!(tokens, expected);
    assert_eq!(token_end, 23)
  }
}

mod parse_char;
mod parse_class;
mod parse_html_tag;
mod parse_id;
mod selectors;
mod tokens;

use parse_char::char_to_token;
use parse_class::parse_class_token;
use parse_html_tag::parse_html_tag;
use parse_id::parse_id_token;
use tokens::Token;

pub fn tokenize(src: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let iter = src.chars().enumerate();

    for (index, character) in iter {
        if let Some(token) = char_to_token(&character) {
            tokens.push(token);
        } else if character == '.' {
            let chars = src.chars().skip(index).collect();
            if let Some(token) = parse_class_token(&chars) {
                tokens.push(token);
            }
        } else if character == '#' {
            let chars = src.chars().skip(index).collect();
            if let Some(token) = parse_id_token(&chars) {
                tokens.push(token);
            }
        } else {
            let chars: String = src.chars().skip(index).collect();
            if let Some(token) = parse_html_tag(&chars) {
                tokens.push(token);
            }
        }
    }

    tokens.push(Token::Eof);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_test_class() {
        let css = ".foo".to_string();
        let expected = vec![Token::Class(".foo".to_string()), Token::Eof];
        assert_eq!(tokenize(&css), expected);
    }

    #[test]
    fn scan_test_id() {
        let css = "#foo".to_string();
        let expected = vec![Token::Id("#foo".to_string()), Token::Eof];
        assert_eq!(tokenize(&css), expected);
    }

    #[test]
    fn scan_test_tag() {
        let css = "a".to_string();
        let expected = vec![Token::Tag("a".to_string()), Token::Eof];
        assert_eq!(tokenize(&css), expected);
    }
}

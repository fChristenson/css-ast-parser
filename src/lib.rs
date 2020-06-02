mod parse_char;
mod parse_class;
mod parse_id;
mod selectors;
mod tokens;

use parse_char::char_to_token;
use parse_class::parse_class_token;
use parse_id::parse_id_token;
use tokens::Token;

pub fn tokenize(src: &[char]) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    for (index, character) in src.iter().enumerate() {
        if let Some(token) = char_to_token(&character) {
            tokens.push(token);
        } else if let Some(token) = parse_class_token(&src[index..]) {
            tokens.push(token);
        } else if let Some(token) = parse_id_token(&src[index..]) {
            tokens.push(token);
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
        let css = &['.', 'f', 'o', 'o'];
        let expected = vec![Token::Class(".foo".to_string()), Token::Eof];
        assert_eq!(tokenize(css), expected);
    }

    #[test]
    fn scan_test_id() {
        let css = &['#', 'f', 'o', 'o'];
        let expected = vec![Token::Id("#foo".to_string()), Token::Eof];
        assert_eq!(tokenize(css), expected);
    }

    #[test]
    fn scan_test_tag() {
        let css = &['a'];
        let expected = vec![Token::Tag("a".to_string()), Token::Eof];
        assert_eq!(tokenize(css), expected);
    }
}

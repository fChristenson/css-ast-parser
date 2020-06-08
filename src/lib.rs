mod add_class_tokens;
mod add_comment_tokens;
mod add_html_tokens;
mod add_id_tokens;
mod add_rule_tokens;
mod delimiters;
mod html_tags;
mod selectors;
mod tokens;

use add_class_tokens::add_class_token;
use add_comment_tokens::{add_comment_token, is_comment};
use add_html_tokens::add_html_token;
use add_id_tokens::add_id_token;
use add_rule_tokens::add_rule_tokens;
use delimiters::get_delimiter_token;
use tokens::Token;

pub fn scan<'a>(src: &'a str) -> Vec<Token> {
    let mut iter = src.char_indices();
    let mut tokens: Vec<Token> = vec![];
    let mut token_end = 0;

    while let Some((index, c)) = iter.next() {
        if index != token_end {
            continue;
        }

        if c == '/' && is_comment(index, &src) {
            let (offset, token) = add_comment_token(index, &src);
            token_end = offset;
            tokens.push(token);
        } else if c == '{' {
            tokens.push(Token::OpenCurlyBrace);
            token_end = add_rule_tokens(index + 1, &src, &mut tokens);
        } else if let Some(token) = get_delimiter_token(c) {
            token_end += 1;
            tokens.push(token);
        } else if c == '.' {
            let (offset, token) = add_class_token(index, &src);
            token_end = offset;
            tokens.push(token);
        } else if c == '#' {
            let (offset, token) = add_id_token(index, &src);
            token_end = offset;
            tokens.push(token);
        } else {
            let (offset, token) = add_html_token(index, &src);
            token_end = offset;
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
    fn scan_class_test() {
        let css = ".foo";
        let expected = vec![Token::Class(".foo"), Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_class_test_with_delimiter() {
        let css = ".foo{";
        let expected = vec![Token::Class(".foo"), Token::OpenCurlyBrace, Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_id_test() {
        let css = "#foo";
        let expected = vec![Token::Id("#foo"), Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_id_test_with_delimiter() {
        let css = "#foo{";
        let expected = vec![Token::Id("#foo"), Token::OpenCurlyBrace, Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_html_test_anchor() {
        let css = "a";
        let expected = vec![Token::Tag("a"), Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_html_test_h1() {
        let css = "h1";
        let expected = vec![Token::Tag("h1"), Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_html_test_video_with_delimiter() {
        let css = "video{";
        let expected = vec![Token::Tag("video"), Token::OpenCurlyBrace, Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_selectors() {
        let css = ".foo#bar ul {}";
        let expected = vec![
            Token::Class(".foo"),
            Token::Id("#bar"),
            Token::Space,
            Token::Tag("ul"),
            Token::Space,
            Token::OpenCurlyBrace,
            Token::ClosingCurlyBrace,
            Token::Eof,
        ];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_rules() {
        let css = ".foo {color: red;}";
        let expected = vec![
            Token::Class(".foo"),
            Token::Space,
            Token::OpenCurlyBrace,
            Token::Rule("color"),
            Token::Colon,
            Token::Space,
            Token::Value("red"),
            Token::SemiColon,
            Token::ClosingCurlyBrace,
            Token::Eof,
        ];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_comment() {
        let css = "/* comment */ .foo {color: red;}";
        let expected = vec![
            Token::Comment("/* comment */"),
            Token::Space,
            Token::Class(".foo"),
            Token::Space,
            Token::OpenCurlyBrace,
            Token::Rule("color"),
            Token::Colon,
            Token::Space,
            Token::Value("red"),
            Token::SemiColon,
            Token::ClosingCurlyBrace,
            Token::Eof,
        ];
        assert_eq!(scan(&css), expected)
    }
}

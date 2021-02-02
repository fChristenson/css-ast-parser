mod delimiters;
mod regex;
mod tokens;

use delimiters::get_delimiter_token;
use tokens::Token;

pub fn scan<'a>(src: &'a str) -> Vec<Token> {
    let mut iter = src.char_indices();
    let mut tokens: Vec<Token> = vec![];
    let mut last_token_end = 0;

    while let Some((index, c)) = iter.next() {
        if c == '#' || c == '.' {
            maybe_add_token(src, last_token_end, index, &mut tokens);
            last_token_end = index;
        } else if let Some(token) = get_delimiter_token(c) {
            maybe_add_token(src, last_token_end, index, &mut tokens);
            tokens.push(token);
            last_token_end = index + 1;
        }
    }

    if last_token_end != src.len() {
        tokens.push(Token::Ident(&src[last_token_end..]));
    }

    tokens.push(Token::Eof);
    tokens
}

fn maybe_add_token<'a>(
    src: &'a str,
    last_token_end: usize,
    index: usize,
    tokens: &mut Vec<Token<'a>>,
) {
    if src[last_token_end..index].len() > 1 {
        tokens.push(Token::Ident(&src[last_token_end..index]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_class_test() {
        let css = ".foo";
        let expected = vec![Token::Ident(".foo"), Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_class_test_with_delimiter() {
        let css = ".foo{";
        let expected = vec![Token::Ident(".foo"), Token::OpenCurlyBrace, Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_id_test() {
        let css = "#foo";
        let expected = vec![Token::Ident("#foo"), Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_id_test_with_delimiter() {
        let css = "#foo{";
        let expected = vec![Token::Ident("#foo"), Token::OpenCurlyBrace, Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_html_test_anchor() {
        let css = "a";
        let expected = vec![Token::Ident("a"), Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_html_test_h1() {
        let css = "h1";
        let expected = vec![Token::Ident("h1"), Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_html_test_video_with_delimiter() {
        let css = "video{";
        let expected = vec![Token::Ident("video"), Token::OpenCurlyBrace, Token::Eof];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_selectors() {
        let css = ".foo#bar ul {}";
        let expected = vec![
            Token::Ident(".foo"),
            Token::Ident("#bar"),
            Token::Space,
            Token::Ident("ul"),
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
            Token::Ident(".foo"),
            Token::Space,
            Token::OpenCurlyBrace,
            Token::Ident("color"),
            Token::Colon,
            Token::Space,
            Token::Ident("red"),
            Token::SemiColon,
            Token::ClosingCurlyBrace,
            Token::Eof,
        ];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_multiple_rules() {
        let css = ".foo {color: red; margin: 8px auto;}";
        let expected = vec![
            Token::Ident(".foo"),
            Token::Space,
            Token::OpenCurlyBrace,
            Token::Ident("color"),
            Token::Colon,
            Token::Space,
            Token::Ident("red"),
            Token::SemiColon,
            Token::Space,
            Token::Ident("margin"),
            Token::Colon,
            Token::Space,
            Token::Ident("8px"),
            Token::Space,
            Token::Ident("auto"),
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
            Token::ForwardSlash,
            Token::Star,
            Token::Space,
            Token::Ident("comment"),
            Token::Space,
            Token::Star,
            Token::ForwardSlash,
            Token::Space,
            Token::Ident(".foo"),
            Token::Space,
            Token::OpenCurlyBrace,
            Token::Ident("color"),
            Token::Colon,
            Token::Space,
            Token::Ident("red"),
            Token::SemiColon,
            Token::ClosingCurlyBrace,
            Token::Eof,
        ];
        assert_eq!(scan(&css), expected)
    }

    #[test]
    fn scan_import() {
        let css = "@import 'foo.css';";
        let expected = vec![
            Token::Ident("@import"),
            Token::Space,
            Token::SingleQuote,
            Token::Ident("foo.css"),
            Token::SingleQuote,
            Token::SemiColon,
            Token::Eof,
        ];
        assert_eq!(scan(&css), expected)
    }
}

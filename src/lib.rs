mod selectors;
mod tokens;

use selectors::is_selector_delimiter;
use std::collections::*;
use std::iter::{Skip, SkipWhile};
use std::str::CharIndices;
use std::str::Chars;
use tokens::Token;

pub fn scan<'a>(src: &'a str) -> Vec<Token> {
    let delimiters = get_delimiters();
    let mut iter = src.char_indices();
    let mut tokens: Vec<Token> = vec![];
    let mut token_end = 0;

    while let Some((index, c)) = iter.next() {
        println!("ITER {} {} {}", index, token_end, c);
        if index != token_end {
            println!("CONTINUE {} {} {}", index, token_end, c);
            continue;
        }

        if let Some(token) = delimiters.get(&c) {
            token_end += 1;
            println!("ADDING CHAR {:?}", token);
            tokens.push(*token);
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

fn add_html_token<'a>(index: usize, src: &'a str) -> (usize, Token<'a>) {
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

fn add_class_token<'a>(index: usize, src: &'a str) -> (usize, Token<'a>) {
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

fn add_id_token<'a>(index: usize, src: &'a str) -> (usize, Token<'a>) {
    println!("ID FOUND");
    let first_letter = index + 1;
    let mut iter2 = src.char_indices().skip(first_letter);
    let mut delimiter_found = false;

    while let Some((offset, c)) = iter2.next() {
        if !is_selector_delimiter(&c) {
            continue;
        }

        delimiter_found = true;
        println!("ADDING ID {} {}", index, offset);
        return (offset, Token::Id(&src[index..offset]));
    }

    if !delimiter_found {
        let offset = src.len();
        println!("NO DELIMITER ADDING ID {} {}", index, offset);
        return (offset, Token::Id(&src[index..]));
    }

    panic!("Error parsing id")
}

fn get_delimiters<'a>() -> HashMap<char, Token<'a>> {
    let mut map = HashMap::new();
    map.insert('\n', Token::Newline);
    map.insert('\r', Token::Return);
    map.insert('\t', Token::Tab);
    map.insert(' ', Token::Space);
    map.insert('{', Token::OpenCurlyBrace);
    map.insert('}', Token::ClosingCurlyBrace);
    map.insert('[', Token::OpenBrace);
    map.insert(']', Token::ClosingBrace);
    map.insert(':', Token::Colon);
    map.insert(';', Token::SemiColon);
    map.insert(',', Token::Comma);

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_test_all_delimiters() {
        let mut css = String::new();
        let iter = get_delimiters();
        let mut keys = iter.keys();

        while let Some(c) = keys.next() {
            css.push(*c);
        }

        let expected = vec![
            Token::OpenBrace,
            Token::ClosingBrace,
            Token::Colon,
            Token::Tab,
            Token::Comma,
            Token::OpenCurlyBrace,
            Token::SemiColon,
            Token::Return,
            Token::Newline,
            Token::ClosingCurlyBrace,
            Token::Space,
            Token::Eof,
        ];
        let results = scan(&css);
        for token in expected {
            assert_eq!(results.contains(&token), true);
        }
    }

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
}

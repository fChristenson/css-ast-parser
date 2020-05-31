#[derive(Debug, PartialEq)]
pub enum Token {
    Id(String),
    Class(String),
    Tag(String),
    Property(String),
    Value(String),
    Colon,
    SemiColon,
    OpenCurlyBrace,
    ClosingCurlyBrace,
    Unknown(String),
}

pub fn scan(src: &String) -> Result<Vec<Token>, Error> {
    let mut result = vec![];
    let mut buffer = String::new();

    for character in src.chars() {
        if is_whitespace(character) {
            continue;
        }

        if character == '{' {
            let characters = buffer.clone();
            result.push(parse_selector_token(&characters)?);
            result.push(Token::OpenCurlyBrace);
            buffer.clear();
        } else if character == ':' {
            let characters = buffer.clone();
            result.push(Token::Property(characters));
            result.push(Token::Colon);
            buffer.clear();
        } else if character == ';' {
            let characters = buffer.clone();
            result.push(Token::Value(characters));
            result.push(Token::SemiColon);
            buffer.clear();
        } else if character == '}' {
            result.push(Token::ClosingCurlyBrace);
            buffer.clear();
        } else {
            buffer.push(character);
        }
    }

    Ok(result)
}

#[derive(Debug, PartialEq)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

fn is_whitespace(character: char) -> bool {
    character == '\n' || character == ' ' || character == '\t' || character == '\r'
}

fn parse_selector_token(characters: &str) -> Result<Token, Error> {
    let first_char = characters.chars().take(1).next();

    match first_char {
        Some('#') => {
            let sub_string = &characters[1..characters.len()];
            Ok(Token::Id(String::from(sub_string)))
        }
        Some('.') => {
            let sub_string = &characters[1..characters.len()];
            Ok(Token::Class(String::from(sub_string)))
        }
        _ => Err(Error::new(&format!("Could not parse str {}", characters))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path};

    #[test]
    fn is_whitespace_test() {
        assert_eq!(is_whitespace(' '), true);
        assert_eq!(is_whitespace('\t'), true);
        assert_eq!(is_whitespace('\r'), true);
        assert_eq!(is_whitespace('\t'), true)
    }
    #[test]
    fn class_token_test() {
        let result = parse_selector_token(&".foobar").unwrap();
        assert_eq!(result, Token::Class(String::from("foobar")))
    }

    #[test]
    fn unknown_token_test() {
        let unknown = parse_selector_token(&"fail").unwrap_err();
        assert_eq!(unknown, Error::new(&format!("Could not parse str fail")))
    }
    #[test]
    fn scan_test() {
        let expected_tokens = vec![
            Token::Class(String::from("foo")),
            Token::OpenCurlyBrace,
            Token::Property(String::from("color")),
            Token::Colon,
            Token::Value(String::from("red")),
            Token::SemiColon,
            Token::ClosingCurlyBrace,
        ];
        let path: path::PathBuf = [env!("CARGO_MANIFEST_DIR"), "test_data/test.css"]
            .iter()
            .collect();
        let file = fs::read_to_string(path).unwrap();
        let result = scan(&file).unwrap();
        assert_eq!(expected_tokens, result)
    }

    #[test]
    fn scan_test2() {
        let expected_tokens = vec![
            // .foo
            Token::Class(String::from("foo")),
            Token::OpenCurlyBrace,
            Token::Property(String::from("color")),
            Token::Colon,
            Token::Value(String::from("red")),
            Token::SemiColon,
            Token::ClosingCurlyBrace,
            // #bar
            Token::Id(String::from("bar")),
            Token::OpenCurlyBrace,
            Token::Property(String::from("color")),
            Token::Colon,
            Token::Value(String::from("green")),
            Token::SemiColon,
            Token::Property(String::from("white-space")),
            Token::Colon,
            Token::Value(String::from("nowrap")),
            Token::SemiColon,
            Token::ClosingCurlyBrace,
            // h1
            Token::Tag(String::from("h1")),
            Token::OpenCurlyBrace,
            Token::Property(String::from("padding")),
            Token::Colon,
            Token::Value(String::from("8px 8px")),
            Token::SemiColon,
            Token::ClosingCurlyBrace,
        ];
        let path: path::PathBuf = [env!("CARGO_MANIFEST_DIR"), "test_data/test2.css"]
            .iter()
            .collect();
        let file = fs::read_to_string(path).unwrap();
        let result = scan(&file).unwrap();
        assert_eq!(expected_tokens, result)
    }

    #[test]
    fn scan_test3() {
        let expected_tokens = vec![
            Token::Class(String::from("foo")),
            Token::OpenCurlyBrace,
            Token::Property(String::from("color")),
            Token::Colon,
            Token::Value(String::from("red")),
            Token::SemiColon,
            Token::ClosingCurlyBrace,
            Token::Id(String::from("bar")),
            Token::OpenCurlyBrace,
            Token::Property(String::from("color")),
            Token::Colon,
            Token::Value(String::from("green")),
            Token::SemiColon,
            Token::Property(String::from("white-space")),
            Token::Colon,
            Token::Value(String::from("nowrap")),
            Token::SemiColon,
            Token::ClosingCurlyBrace,
        ];
        let path: path::PathBuf = [env!("CARGO_MANIFEST_DIR"), "test_data/test2.css"]
            .iter()
            .collect();
        let file = fs::read_to_string(path).unwrap();
        let no_whitespace: String = file
            .chars()
            .filter(|c| !is_whitespace(c.to_owned()))
            .collect();
        let result = scan(&no_whitespace).unwrap();
        assert_eq!(expected_tokens, result)
    }
}

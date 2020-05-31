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
        if character == '{' {
            result.push(parse_selector_token(buffer.trim()));
            result.push(Token::OpenCurlyBrace);
            buffer.clear();
        } else if character == ':' {
            result.push(Token::Property(buffer.trim().to_string()));
            result.push(Token::Colon);
            buffer.clear();
        } else if character == ';' {
            result.push(Token::Value(buffer.trim().to_string()));
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

fn parse_selector_token(characters: &str) -> Token {
    let first_char = characters.chars().take(1).next();

    match first_char {
        Some('#') => {
            let sub_string = &characters[1..characters.len()];
            Token::Id(String::from(sub_string))
        }
        Some('.') => {
            let sub_string = &characters[1..characters.len()];
            Token::Class(String::from(sub_string))
        }
        _ => Token::Tag(String::from(characters)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path};

    #[test]
    fn class_token_test() {
        let result = parse_selector_token(&".foobar");
        assert_eq!(result, Token::Class(String::from("foobar")))
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
}

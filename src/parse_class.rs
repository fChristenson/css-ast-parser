use crate::selectors::is_selector_delimiter;
use crate::tokens::Token;

pub fn parse_class_token(characters: &[char]) -> Option<Token> {
    let first_char = characters.first().unwrap_or(&'0');

    if first_char != &'.' || characters.len() < 2 {
        return None;
    }

    let mut iter = characters.iter();
    let mut class_string = String::new();

    if let Some(character) = iter.next() {
        class_string.push(*character);
        while let Some(character) = iter.next() {
            if is_selector_delimiter(&character) {
                return Some(Token::Class(class_string));
            } else {
                class_string.push(*character);
            }
        }
    }

    Some(Token::Class(class_string))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_class_token_with_space_delimiter() {
        let characters = &['.', 'f', 'o', 'o', ' '];
        let token = parse_class_token(characters).unwrap();
        assert_eq!(token, Token::Class(".foo".to_string()))
    }

    #[test]
    fn parse_class_token_with_class_delimiter() {
        let characters = &['.', 'f', 'o', 'o', '.', 'b', 'a', 'r'];
        let token = parse_class_token(characters).unwrap();
        assert_eq!(token, Token::Class(".foo".to_string()))
    }

    #[test]
    fn parse_class_token_test_with_id_delimiter() {
        let characters = &['.', 'f', 'o', 'o', '#', 'b', 'a', 'r'];
        let token = parse_class_token(characters).unwrap();
        assert_eq!(token, Token::Class(".foo".to_string()))
    }

    #[test]
    fn parse_class_token_test_with_opening_brace_delimiter() {
        let characters = &['.', 'f', 'o', 'o', '{'];
        let token = parse_class_token(characters).unwrap();
        assert_eq!(token, Token::Class(".foo".to_string()))
    }

    #[test]
    fn parse_class_token_test_with_no_delimiter() {
        let characters = &['.', 'f', 'o', 'o'];
        let token = parse_class_token(characters).unwrap();
        assert_eq!(token, Token::Class(".foo".to_string()))
    }
}

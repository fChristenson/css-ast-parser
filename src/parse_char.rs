use crate::tokens::Token;

pub fn char_to_token(character: &char) -> Option<Token> {
    match character {
        &'\n' => Some(Token::Newline),
        &'\r' => Some(Token::Return),
        &'\t' => Some(Token::Tab),
        &' ' => Some(Token::Space),
        &'{' => Some(Token::OpenCurlyBrace),
        &'}' => Some(Token::ClosingCurlyBrace),
        &':' => Some(Token::Colon),
        &';' => Some(Token::SemiColon),
        &',' => Some(Token::Comma),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_char_to_token_test() {
        assert_eq!(char_to_token(&'{').unwrap(), Token::OpenCurlyBrace);
        assert_eq!(char_to_token(&'}').unwrap(), Token::ClosingCurlyBrace);
        assert_eq!(char_to_token(&':').unwrap(), Token::Colon);
        assert_eq!(char_to_token(&';').unwrap(), Token::SemiColon);
        assert_eq!(char_to_token(&',').unwrap(), Token::Comma);
        assert_eq!(char_to_token(&'\n').unwrap(), Token::Newline);
        assert_eq!(char_to_token(&' ').unwrap(), Token::Space);
        assert_eq!(char_to_token(&'\t').unwrap(), Token::Tab);
        assert_eq!(char_to_token(&'\r').unwrap(), Token::Return)
    }
}

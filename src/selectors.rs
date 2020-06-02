const SELECTOR_DELIMITERS: [char; 7] = ['.', '#', '>', '+', '~', ' ', '{'];

pub fn is_selector_delimiter(character: &char) -> bool {
    SELECTOR_DELIMITERS.contains(character) || character.is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_selector_delimiter_test() {
        for character in SELECTOR_DELIMITERS.iter() {
            assert_eq!(is_selector_delimiter(character), true)
        }
    }
}

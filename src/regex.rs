pub static class_regex: &str = r"(\.[a-zA-Z]+)";

mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn class_regex_test() {
        let regex = Regex::new(class_regex).unwrap();
        let css = " .foo#bar ul";
        let mut iter = regex.captures_iter(css);

        assert_eq!(&iter.next().unwrap()[1], ".foo");
    }

    #[test]
    fn multi_class_regex_test() {
        let regex = Regex::new(class_regex).unwrap();
        let css = " .foo.bar .baz";
        let mut iter = regex.captures_iter(css);

        assert_eq!(&iter.next().unwrap()[1], ".foo");
        assert_eq!(&iter.next().unwrap()[1], ".bar");
        assert_eq!(&iter.next().unwrap()[1], ".baz");
    }
}

use crate::tokens::Token;

pub fn parse_html_tag(html_tag: &String) -> Option<Token> {
    let mut iter = HTML_TAGS.iter().skip_while(|tag| *tag != &html_tag);

    while let Some(tag) = iter.next() {
        return Some(Token::Tag(tag.to_string()));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_html_tag_test_anchor() {
        let tag = "a".to_string();
        assert_eq!(parse_html_tag(&tag).unwrap(), Token::Tag("a".to_string()))
    }

    #[test]
    fn parse_html_tag_test_h1() {
        let tag = "h1".to_string();
        assert_eq!(parse_html_tag(&tag).unwrap(), Token::Tag("h1".to_string()))
    }

    #[test]
    fn parse_html_tag_test_video() {
        let tag = "video".to_string();
        assert_eq!(
            parse_html_tag(&tag).unwrap(),
            Token::Tag("video".to_string())
        )
    }
}

const HTML_TAGS: [&str; 128] = [
    "a",
    "abbr",
    "acronym",
    "abbr",
    "address",
    "applet",
    "embed",
    "object",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "basefont",
    "bdi",
    "bdo",
    "big",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "center",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "dir",
    "ul",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "font",
    "footer",
    "form",
    "frame",
    "frameset",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hr",
    "html",
    "i",
    "iframe",
    "img",
    "input",
    "ins",
    "kbd",
    "label",
    "legend",
    "li",
    "link",
    "main",
    "map",
    "mark",
    "meta",
    "meter",
    "nav",
    "noframes",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "pre",
    "progress",
    "q",
    "rp",
    "rt",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "small",
    "source",
    "span",
    "strike",
    "del",
    "s",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "svg",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "tt",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
];


use regex::{Regex, RegexBuilder, escape};


const TOKENS: &[&str] = &[
    "++",
    "--",
    "&&",
    "||",
    "==",
    "!=",
    "<=",
    ">=",
    "+=",
    "-=",
    "*=",
    "/=",
    "%=",
    "##",
    "+",
    "-",
    "*",
    "/",
    "%",
    "=",
    "<",
    ">",
    "!",
    "&",
    "|",
    "^",
    "~",
    ",",
    ".",
    ";",
    "(",
    ")",
    "{",
    "}",
    "[",
    "]",
    "#",
    "?",
    ":",
];

lazy_static::lazy_static! {
    static ref RE: Regex = {
        let identifiers = String::from(r"[0-9a-zA-Z_]+");
        let strings = String::from(r#""[^"]*""#);
        let iter = TOKENS.iter().map(|&s| escape(s)).chain(
            vec![identifiers, strings],
        );
        let big_string = iter.collect::<Vec<String>>().join("|");
        let re = RegexBuilder::new(&format!(r"^[\s\n]*({})", big_string))
            .multi_line(true)
            .build()
            .unwrap();
        re
    };
}


/// An iterator over the tokens in a `&str`
pub struct TokenIterator<'a> {
    remaining_text: &'a str,
    re: Regex,
}


impl<'a> Iterator for TokenIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if let Some(cap) = self.re.captures(self.remaining_text) {
            let mat = cap.get(1).unwrap();
            self.remaining_text = &self.remaining_text[mat.end()..];
            return Some(mat.as_str());
        }
        None
    }
}

/// Returns a `TokenIterator` over the tokens in the passed in string
pub fn iter_tokens(text: &str) -> TokenIterator {
    let identifiers = String::from(r"[0-9a-zA-Z_]+");
    let strings = String::from(r#""[^"]*""#);
    let iter = TOKENS.iter().map(|&s| escape(s)).chain(
        vec![identifiers, strings],
    );
    let big_string = iter.collect::<Vec<String>>().join("|");
    let re = RegexBuilder::new(&format!(r"^[\s\n]*({})", big_string))
        .multi_line(true)
        .build()
        .unwrap();

    TokenIterator {
        remaining_text: text,
        re: re,
    }
}


/// An iterator over the indices of tokens.
pub struct TokenIndexIterator<'text> {
    text: &'text str,
    cursor: usize,
}

impl<'text> TokenIndexIterator<'text> {
    pub fn set_cursor(&mut self, cursor: usize) -> bool {
        if self.text.is_char_boundary(cursor) {
            self.cursor = cursor;
            true
        }
        else {
            false
        }
    }
}

impl<'text> Iterator for TokenIndexIterator<'text> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if let Some(cap) = RE.captures(&self.text[self.cursor..]) {
            let mat = cap.get(1).unwrap();
            let ret = Some((self.cursor + mat.start(), self.cursor + mat.end()));
            self.cursor += mat.end();
            ret
        }
        else {
            None
        }
    }
}

pub fn iter_token_indices(text: &str) -> TokenIndexIterator {
    TokenIndexIterator {
        text,
        cursor: 0,
    }
}

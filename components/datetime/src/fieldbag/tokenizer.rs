// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) struct Uts35DateTimePatternTokenizer<'a>(pub &'a str);

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum Token<'a> {
    Symbol(char, usize),
    // TODO: Remove the following annotation when this tokenizer is used for patterns
    #[cfg_attr(not(test), allow(unused))]
    Literal(&'a str),
}

impl<'a> Uts35DateTimePatternTokenizer<'a> {
    pub fn step(&mut self) -> Option<Token<'a>> {
        let starting_str = self.0;
        let mut it = self.0.chars();
        let ch = it.next()?;
        let mut byte_len = ch.len_utf8();
        loop {
            self.0 = it.as_str();
            let Some(next) = it.next() else { break };
            if ch.is_ascii_alphabetic() {
                if next != ch {
                    break;
                }
            } else {
                // FIXME: Handle quoted literals
                if next.is_ascii_alphabetic() {
                    break;
                }
            }
            byte_len += next.len_utf8();
        }
        if ch.is_ascii_alphabetic() {
            Some(Token::Symbol(ch, byte_len))
        } else {
            // FIXME: Avoid indexing
            Some(Token::Literal(&starting_str[0..byte_len]))
        }
    }
}

#[test]
fn test_basic() {
    let mut tokenizer: Uts35DateTimePatternTokenizer<'_> =
        Uts35DateTimePatternTokenizer("abb.😀.ccc");
    assert_eq!(tokenizer.step(), Some(Token::Symbol('a', 1)));
    assert_eq!(tokenizer.step(), Some(Token::Symbol('b', 2)));
    assert_eq!(tokenizer.step(), Some(Token::Literal(".😀.")));
    assert_eq!(tokenizer.step(), Some(Token::Symbol('c', 3)));
    assert_eq!(tokenizer.step(), None);
}

#[test]
fn test_quotes() {
    let mut tokenizer: Uts35DateTimePatternTokenizer<'_> =
        Uts35DateTimePatternTokenizer("..a''bb'c'.d..");
    assert_eq!(tokenizer.step(), Some(Token::Literal("..")));
    assert_eq!(tokenizer.step(), Some(Token::Symbol('a', 1)));
    assert_eq!(tokenizer.step(), Some(Token::Literal("'")));
    assert_eq!(tokenizer.step(), Some(Token::Symbol('b', 2)));
    assert_eq!(tokenizer.step(), Some(Token::Literal("c")));
    assert_eq!(tokenizer.step(), Some(Token::Literal(".")));
    assert_eq!(tokenizer.step(), Some(Token::Symbol('d', 1)));
    assert_eq!(tokenizer.step(), Some(Token::Literal("..")));
    assert_eq!(tokenizer.step(), None);
}

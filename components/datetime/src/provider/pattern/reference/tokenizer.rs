// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[derive(Clone)]
pub(crate) struct Uts35DateTimePatternTokenizer<'a>(pub &'a str);

#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum Token<'a> {
    Symbol(char, usize),
    Literal(&'a str),
    #[cfg_attr(not(test), allow(unused))]
    UnclosedLiteral(&'a str),
}

impl<'a> Uts35DateTimePatternTokenizer<'a> {
    pub fn step(&mut self) -> Option<Token<'a>> {
        let starting_str = self.0;
        let mut it = self.0.chars();
        let ch = it.next()?;
        if ch == '\'' {
            let after_first_quote = it.as_str();
            let Some(second_ch) = it.next() else {
                self.0 = "";
                return Some(Token::UnclosedLiteral(after_first_quote));
            };
            if second_ch == '\'' {
                self.0 = it.as_str();
                return Some(Token::Literal("'"));
            }
            let mut byte_len = second_ch.len_utf8();
            loop {
                let Some(next) = it.next() else {
                    self.0 = "";
                    return Some(Token::UnclosedLiteral(after_first_quote));
                };
                if next == '\'' {
                    let after_first_closing = it.as_str();
                    let mut quote_count = 1usize;
                    while it.as_str().starts_with('\'') {
                        it.next();
                        quote_count += 1;
                    }
                    if quote_count == 1 {
                        self.0 = after_first_closing;
                        return Some(Token::Literal(after_first_quote.get(..byte_len)?));
                    }
                    byte_len += '\''.len_utf8();
                    if quote_count.is_multiple_of(2) {
                        self.0 = after_first_closing;
                    } else {
                        let mut rem = after_first_closing.chars();
                        rem.next();
                        rem.next();
                        self.0 = rem.as_str();
                    }
                    return Some(Token::Literal(after_first_quote.get(..byte_len)?));
                }
                byte_len += next.len_utf8();
            }
        }
        let mut byte_len = ch.len_utf8();
        loop {
            self.0 = it.as_str();
            let Some(next) = it.next() else { break };
            if ch.is_ascii_alphabetic() {
                if next != ch {
                    break;
                }
            } else if next.is_ascii_alphabetic() || next == '\'' {
                break;
            }
            byte_len += next.len_utf8();
        }
        if ch.is_ascii_alphabetic() {
            Some(Token::Symbol(ch, byte_len))
        } else {
            Some(Token::Literal(starting_str.get(..byte_len)?))
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

#[test]
fn test_escaped_and_unclosed_quotes() {
    let mut tokenizer = Uts35DateTimePatternTokenizer("hh 'o''clock' 'a''' 'foo ");
    assert_eq!(tokenizer.step(), Some(Token::Symbol('h', 2)));
    assert_eq!(tokenizer.step(), Some(Token::Literal(" ")));
    assert_eq!(tokenizer.step(), Some(Token::Literal("o'")));
    assert_eq!(tokenizer.step(), Some(Token::Literal("clock")));
    assert_eq!(tokenizer.step(), Some(Token::Literal(" ")));
    assert_eq!(tokenizer.step(), Some(Token::Literal("a'")));
    assert_eq!(tokenizer.step(), Some(Token::Literal(" ")));
    assert_eq!(tokenizer.step(), Some(Token::UnclosedLiteral("foo ")));
    assert_eq!(tokenizer.step(), None);
}

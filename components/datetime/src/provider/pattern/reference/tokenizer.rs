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
        // Case 1: Quoted literal (`'...'`) or escaped quote (`''`)
        if let Some(after_open) = self.0.strip_prefix('\'') {
            if let Some(after_escaped) = after_open.strip_prefix('\'') {
                self.0 = after_escaped;
                return Some(Token::Literal("'"));
            }
            let Some((mut literal, mut remainder)) = after_open.split_once('\'') else {
                self.0 = "";
                return Some(Token::UnclosedLiteral(after_open));
            };
            // Search for nested quotes, like 'o''clock', 'a''', or 'b''''c'.
            let mut nested_quote_count = 0;
            while let Some(s) = remainder.strip_prefix("''") {
                nested_quote_count += 1;
                remainder = s;
            }
            if remainder.starts_with('\'') {
                nested_quote_count += 1;
            }
            // If we found nested quotes, return them as literals including the necessary number
            // of quotes. For example, return 'o''clock' as two token: o' and clock.
            #[allow(clippy::indexing_slicing)] // there are enough quotes in the string
            if nested_quote_count > 0 {
                literal = &after_open[..literal.len() + nested_quote_count];
            }
            self.0 = remainder;
            return Some(Token::Literal(literal));
        }

        // Case 2: Field symbol (run of identical ASCII letters, e.g. `yyyy`)
        let ch = self.0.chars().next()?;
        if ch.is_ascii_alphabetic() {
            let rest = self.0.trim_start_matches(ch);
            let byte_len = self.0.len() - rest.len();
            self.0 = rest;
            return Some(Token::Symbol(ch, byte_len));
        }

        // Case 3: Unquoted literal (run of non-ASCII-alphabetic, non-quote characters)
        let rest = self
            .0
            .trim_start_matches(|c: char| !c.is_ascii_alphabetic() && c != '\'');
        let literal = self.0.get(..self.0.len() - rest.len())?;
        self.0 = rest;
        Some(Token::Literal(literal))
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
    let mut tokenizer = Uts35DateTimePatternTokenizer("hh .'o''clock' 'a'''. 'foo ");
    assert_eq!(tokenizer.step(), Some(Token::Symbol('h', 2)));
    assert_eq!(tokenizer.step(), Some(Token::Literal(" .")));
    assert_eq!(tokenizer.step(), Some(Token::Literal("o'")));
    assert_eq!(tokenizer.step(), Some(Token::Literal("clock")));
    assert_eq!(tokenizer.step(), Some(Token::Literal(" ")));
    assert_eq!(tokenizer.step(), Some(Token::Literal("a'")));
    assert_eq!(tokenizer.step(), Some(Token::Literal(". ")));
    assert_eq!(tokenizer.step(), Some(Token::UnclosedLiteral("foo ")));
    assert_eq!(tokenizer.step(), None);
}

#[test]
fn test_quote_runs() {
    // Note: the expected literals concatenate to "'' 'x a''b"
    let mut tokenizer = Uts35DateTimePatternTokenizer("'''' '''x' 'a''''b'");
    assert_eq!(tokenizer.step(), Some(Token::Literal("'")));
    assert_eq!(tokenizer.step(), Some(Token::Literal("'")));
    assert_eq!(tokenizer.step(), Some(Token::Literal(" ")));
    assert_eq!(tokenizer.step(), Some(Token::Literal("'")));
    assert_eq!(tokenizer.step(), Some(Token::Literal("x")));
    assert_eq!(tokenizer.step(), Some(Token::Literal(" ")));
    assert_eq!(tokenizer.step(), Some(Token::Literal("a''")));
    assert_eq!(tokenizer.step(), Some(Token::Literal("b")));
    assert_eq!(tokenizer.step(), None);
}

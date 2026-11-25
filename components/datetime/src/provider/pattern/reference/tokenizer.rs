// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// A zero-allocation tokenizer for [UTS #35 Date Format Patterns] and skeletons.
///
/// Splits a pattern byte slice into runs of identical ASCII alphabetic field symbols
/// ([`Token::Symbol`]), literal characters ([`Token::Literal`]), and unclosed
/// quoted literals ([`Token::UnclosedLiteral`]).
///
/// [UTS #35 Date Format Patterns]: https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns
#[derive(Clone)]
pub(crate) struct Uts35DateTimePatternTokenizer<'a> {
    bytes: &'a [u8],
    in_quotes: bool,
}

/// A token produced by [`Uts35DateTimePatternTokenizer`].
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum Token<'a> {
    /// A contiguous run of identical ASCII alphabetic field symbol characters:
    /// `(symbol_char, byte_len)`.
    ///
    /// # Examples
    /// - `b"yyyy"` produces `Token::Symbol('y', 4)`.
    /// - `b"G"` produces `Token::Symbol('G', 1)`.
    Symbol(char, usize),
    /// A single literal character, either from unquoted non-alphabetic bytes,
    /// inside a single-quoted literal (`'...'`), or an escaped single quote (`''`).
    /// Ill-formed UTF-8 byte sequences yield [`char::REPLACEMENT_CHARACTER`].
    ///
    /// # Examples
    /// - `": "` in `b"HH: mm"` produces `Token::Literal(':')` then `Token::Literal(' ')`.
    /// - `b"'at'"` produces `Token::Literal('a')` then `Token::Literal('t')`.
    /// - `b"''"` produces `Token::Literal('\'')`.
    Literal(char),
    /// The trailing slice of a single-quoted literal that reached the end of the input
    /// without a matching closing quote (`'`).
    ///
    /// # Examples
    /// - `b"'foo "` produces `Token::UnclosedLiteral(b"foo ")`.
    /// - `b"'"` at the end of input produces `Token::UnclosedLiteral(b"")`.
    #[cfg_attr(not(test), allow(unused))]
    UnclosedLiteral(&'a [u8]),
}

fn next_utf8_char(bytes: &mut &[u8]) -> Option<char> {
    let prefix = bytes.get(..bytes.len().min(4))?;
    let chunk = prefix.utf8_chunks().next()?;
    if let Some(ch) = chunk.valid().chars().next() {
        *bytes = bytes.get(ch.len_utf8()..)?;
        Some(ch)
    } else {
        let invalid_len = chunk.invalid().len();
        debug_assert!(invalid_len > 0);
        *bytes = bytes.get(invalid_len..)?;
        Some(char::REPLACEMENT_CHARACTER)
    }
}

impl<'a> Uts35DateTimePatternTokenizer<'a> {
    /// Creates a new tokenizer over `bytes`.
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            in_quotes: false,
        }
    }

    /// Advances the tokenizer and returns the next [`Token`], or `None` at the end of the input.
    pub fn step(&mut self) -> Option<Token<'a>> {
        if self.in_quotes {
            if let Some(rest) = self.bytes.strip_prefix(b"''") {
                self.bytes = rest;
                return Some(Token::Literal('\''));
            }
            if let Some(rest) = self.bytes.strip_prefix(b"'") {
                self.bytes = rest;
                self.in_quotes = false;
            } else if self.bytes.contains(&b'\'') {
                return next_utf8_char(&mut self.bytes).map(Token::Literal);
            } else {
                let unclosed = core::mem::take(&mut self.bytes);
                self.in_quotes = false;
                return Some(Token::UnclosedLiteral(unclosed));
            }
        }

        // Case 1: Quoted literal (`'...'`) or escaped quote (`''`)
        if let Some(after_open) = self.bytes.strip_prefix(b"'") {
            // Two consecutive single quotes (`''`) outside a quoted literal represent a single `'`.
            if let Some(after_escaped) = after_open.strip_prefix(b"'") {
                self.bytes = after_escaped;
                return Some(Token::Literal('\''));
            }
            self.bytes = after_open;
            self.in_quotes = true;
            return self.step();
        }

        // Case 2: Field symbol (run of identical ASCII letters, e.g. `yyyy`)
        let &b = self.bytes.first()?;
        if b.is_ascii_alphabetic() {
            let byte_len = self.bytes.iter().take_while(|&&c| c == b).count();
            self.bytes = self.bytes.get(byte_len..)?;
            return Some(Token::Symbol(b as char, byte_len));
        }

        // Case 3: Unquoted literal (non-ASCII-alphabetic, non-quote character)
        next_utf8_char(&mut self.bytes).map(Token::Literal)
    }
}

#[test]
fn test_basic() {
    let mut tokenizer: Uts35DateTimePatternTokenizer<'_> =
        Uts35DateTimePatternTokenizer::new("abb.😀.ccc".as_bytes());
    assert_eq!(tokenizer.step(), Some(Token::Symbol('a', 1)));
    assert_eq!(tokenizer.step(), Some(Token::Symbol('b', 2)));
    assert_eq!(tokenizer.step(), Some(Token::Literal('.')));
    assert_eq!(tokenizer.step(), Some(Token::Literal('😀')));
    assert_eq!(tokenizer.step(), Some(Token::Literal('.')));
    assert_eq!(tokenizer.step(), Some(Token::Symbol('c', 3)));
    assert_eq!(tokenizer.step(), None);
}

#[test]
fn test_quotes() {
    let mut tokenizer: Uts35DateTimePatternTokenizer<'_> =
        Uts35DateTimePatternTokenizer::new(b"..a''bb'c'.d..");
    assert_eq!(tokenizer.step(), Some(Token::Literal('.')));
    assert_eq!(tokenizer.step(), Some(Token::Literal('.')));
    assert_eq!(tokenizer.step(), Some(Token::Symbol('a', 1)));
    assert_eq!(tokenizer.step(), Some(Token::Literal('\'')));
    assert_eq!(tokenizer.step(), Some(Token::Symbol('b', 2)));
    assert_eq!(tokenizer.step(), Some(Token::Literal('c')));
    assert_eq!(tokenizer.step(), Some(Token::Literal('.')));
    assert_eq!(tokenizer.step(), Some(Token::Symbol('d', 1)));
    assert_eq!(tokenizer.step(), Some(Token::Literal('.')));
    assert_eq!(tokenizer.step(), Some(Token::Literal('.')));
    assert_eq!(tokenizer.step(), None);
}

#[test]
fn test_escaped_and_unclosed_quotes() {
    let mut tokenizer = Uts35DateTimePatternTokenizer::new(b"hh .'o''clock' 'a'''. 'foo ");
    assert_eq!(tokenizer.step(), Some(Token::Symbol('h', 2)));
    assert_eq!(tokenizer.step(), Some(Token::Literal(' ')));
    assert_eq!(tokenizer.step(), Some(Token::Literal('.')));
    for ch in "o'clock".chars() {
        assert_eq!(tokenizer.step(), Some(Token::Literal(ch)));
    }
    assert_eq!(tokenizer.step(), Some(Token::Literal(' ')));
    assert_eq!(tokenizer.step(), Some(Token::Literal('a')));
    assert_eq!(tokenizer.step(), Some(Token::Literal('\'')));
    assert_eq!(tokenizer.step(), Some(Token::Literal('.')));
    assert_eq!(tokenizer.step(), Some(Token::Literal(' ')));
    assert_eq!(tokenizer.step(), Some(Token::UnclosedLiteral(b"foo ")));
    assert_eq!(tokenizer.step(), None);
}

#[test]
fn test_quote_runs() {
    // Note: the expected literals concatenate to "'' 'x a''b"
    let mut tokenizer = Uts35DateTimePatternTokenizer::new(b"'''' '''x' 'a''''b'");
    for ch in "'' 'x a''b".chars() {
        assert_eq!(tokenizer.step(), Some(Token::Literal(ch)));
    }
    assert_eq!(tokenizer.step(), None);
}

#[test]
fn test_invalid_utf8_bytes() {
    let mut tokenizer = Uts35DateTimePatternTokenizer::new(b"yy\xff'o''\xfe'M'a''unclosed\xff");
    assert_eq!(tokenizer.step(), Some(Token::Symbol('y', 2)));
    assert_eq!(
        tokenizer.step(),
        Some(Token::Literal(char::REPLACEMENT_CHARACTER))
    );
    assert_eq!(tokenizer.step(), Some(Token::Literal('o')));
    assert_eq!(tokenizer.step(), Some(Token::Literal('\'')));
    assert_eq!(
        tokenizer.step(),
        Some(Token::Literal(char::REPLACEMENT_CHARACTER))
    );
    assert_eq!(tokenizer.step(), Some(Token::Symbol('M', 1)));
    assert_eq!(tokenizer.step(), Some(Token::Literal('a')));
    assert_eq!(tokenizer.step(), Some(Token::Literal('\'')));
    assert_eq!(
        tokenizer.step(),
        Some(Token::UnclosedLiteral(b"unclosed\xff"))
    );
    assert_eq!(tokenizer.step(), None);
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// A zero-allocation tokenizer for [UTS #35 Date Format Patterns] and skeletons.
///
/// Splits a pattern string into runs of identical ASCII alphabetic field symbols
/// ([`Token::Symbol`]), literal text segments ([`Token::Literal`]), and unclosed
/// quoted literals ([`Token::UnclosedLiteral`]).
///
/// [UTS #35 Date Format Patterns]: https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns
#[derive(Clone)]
pub(crate) struct Uts35DateTimePatternTokenizer<'a>(pub &'a str);

/// A token produced by [`Uts35DateTimePatternTokenizer`].
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum Token<'a> {
    /// A contiguous run of identical ASCII alphabetic field symbol characters:
    /// `(symbol_char, byte_len)`.
    ///
    /// # Examples
    /// - `"yyyy"` produces `Token::Symbol('y', 4)`.
    /// - `"G"` produces `Token::Symbol('G', 1)`.
    Symbol(char, usize),
    /// A literal string slice, either from unquoted non-alphabetic characters,
    /// a single-quoted literal (`'...'`), or an escaped single quote (`''`).
    ///
    /// Note that a quoted literal containing escaped quotes (`''`) is emitted
    /// across multiple [`Token::Literal`] tokens so each token remains a zero-copy
    /// slice of the input string.
    ///
    /// # Examples
    /// - `": "` in `"HH: mm"` produces `Token::Literal(": ")`.
    /// - `"'at'"` produces `Token::Literal("at")`.
    /// - `"''"` produces `Token::Literal("'")`.
    /// - `"'o''clock'"` produces `Token::Literal("o'")` followed by `Token::Literal("clock")`.
    Literal(&'a str),
    /// The trailing slice of a single-quoted literal that reached the end of the input
    /// without a matching closing quote (`'`).
    ///
    /// # Examples
    /// - `"'foo "` produces `Token::UnclosedLiteral("foo ")`.
    /// - `"'"` at the end of input produces `Token::UnclosedLiteral("")`.
    #[cfg_attr(not(test), allow(unused))]
    UnclosedLiteral(&'a str),
}

impl<'a> Uts35DateTimePatternTokenizer<'a> {
    /// Advances the tokenizer and returns the next [`Token`], or `None` at the end of the input.
    pub fn step(&mut self) -> Option<Token<'a>> {
        // Case 1: Quoted literal (`'...'`) or escaped quote (`''`)
        if let Some(after_open) = self.0.strip_prefix('\'') {
            // Two consecutive single quotes (`''`) outside a quoted literal represent a single `'`.
            if let Some(after_escaped) = after_open.strip_prefix('\'') {
                self.0 = after_escaped;
                return Some(Token::Literal("'"));
            }
            let Some((mut literal, mut remainder)) = after_open.split_once('\'') else {
                self.0 = "";
                return Some(Token::UnclosedLiteral(after_open));
            };
            // Search for nested quotes, like 'o''clock', 'a''', or 'b''''c'.
            // Because `split_once('\'')` already consumed the first `'`, stripping pairs of `"''"`
            // from `remainder` naturally consumes all remaining quotes when the total quote run is
            // odd (`1 + 2k`, closing the literal), and leaves a single leading `'` in `remainder`
            // when the total quote run is even (`1 + 2(k - 1) + 1`, keeping the next `step()` call
            // in quoted literal mode).
            let mut nested_quote_count = 0;
            while let Some(s) = remainder.strip_prefix("''") {
                nested_quote_count += 1;
                remainder = s;
            }
            if remainder.starts_with('\'') {
                nested_quote_count += 1;
            }
            // If we found nested quotes, return them as literals including the necessary number
            // of quotes. For example, return 'o''clock' as two tokens: o' and clock.
            if nested_quote_count > 0 {
                let Some(literal_with_apostrophes) =
                    after_open.get(..literal.len() + nested_quote_count)
                else {
                    debug_assert!(
                        false,
                        "after_open ({after_open:?}) must have at least {} bytes",
                        literal.len() + nested_quote_count
                    );
                    return None;
                };
                literal = literal_with_apostrophes;
            }
            self.0 = remainder;
            return Some(Token::Literal(literal));
        }

        // Case 2: Field symbol (run of identical ASCII letters, e.g. `yyyy`)
        let ch = self.0.chars().next()?;
        if ch.is_ascii_alphabetic() {
            let remainder = self.0.trim_start_matches(ch);
            let byte_len = self.0.len() - remainder.len();
            self.0 = remainder;
            return Some(Token::Symbol(ch, byte_len));
        }

        // Case 3: Unquoted literal (run of non-ASCII-alphabetic, non-quote characters)
        let remainder = self
            .0
            .trim_start_matches(|c: char| !c.is_ascii_alphabetic() && c != '\'');
        let Some(literal) = self.0.get(..self.0.len() - remainder.len()) else {
            debug_assert!(
                false,
                "remainder ({remainder:?}) must be a suffix of self.0 ({:?})",
                self.0
            );
            return None;
        };
        self.0 = remainder;
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

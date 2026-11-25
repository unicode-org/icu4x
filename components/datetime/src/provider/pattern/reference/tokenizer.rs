// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ops::Range;

/// A zero-allocation tokenizer for [UTS #35 Date Format Patterns] and skeletons.
///
/// Splits a pattern byte slice into runs of identical ASCII alphabetic field symbols
/// ([`Token::Symbol`]), literal byte ranges ([`Token::Literal`]), and unclosed
/// quoted literal byte ranges ([`Token::UnclosedLiteral`]).
///
/// Because UTS #35 field symbols and single quotes (`'`) are pure ASCII (`< 0x80`)
/// and UTF-8 is self-synchronizing, every emitted [`Range<usize>`] is guaranteed to
/// fall on valid UTF-8 codepoint boundaries whenever the input is valid UTF-8.
///
/// [UTS #35 Date Format Patterns]: https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns
#[derive(Clone)]
pub(crate) struct Uts35DateTimePatternTokenizer<'a> {
    remaining: &'a [u8],
    pos: usize,
}

/// A token produced by [`Uts35DateTimePatternTokenizer`].
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum Token {
    /// A contiguous run of identical ASCII alphabetic field symbol characters:
    /// `(symbol_char, byte_len)`.
    ///
    /// # Examples
    /// - `b"yyyy"` produces `Token::Symbol('y', 4)`.
    /// - `b"G"` produces `Token::Symbol('G', 1)`.
    Symbol(char, usize),
    /// A literal byte range into the original input, either from unquoted
    /// non-ASCII-alphabetic bytes, a single-quoted literal (`'...'`), or an
    /// escaped single quote (`''`).
    ///
    /// Note that a quoted literal containing escaped quotes (`''`) is emitted
    /// across multiple [`Token::Literal`] tokens so each token remains a single
    /// contiguous subslice of the input.
    ///
    /// # Examples
    /// - `": "` in `b"HH: mm"` produces `Token::Literal(2..4)` (`": "`).
    /// - `"'at'"` produces `Token::Literal(1..3)` (`"at"`).
    /// - `"''"` produces `Token::Literal(0..1)` (`"'"`).
    /// - `"'o''clock'"` produces `Token::Literal(1..3)` (`"o'"`) followed by
    ///   `Token::Literal(4..9)` (`"clock"`).
    Literal(Range<usize>),
    /// The trailing byte range of a single-quoted literal that reached the end
    /// of the input without a matching closing quote (`'`).
    ///
    /// # Examples
    /// - `b"'foo "` produces `Token::UnclosedLiteral(1..5)` (`"foo "`).
    /// - `b"'"` at the end of input produces `Token::UnclosedLiteral(1..1)` (`""`).
    #[cfg_attr(not(test), allow(unused))]
    UnclosedLiteral(Range<usize>),
}

impl<'a> Uts35DateTimePatternTokenizer<'a> {
    /// Creates a new tokenizer over `bytes`.
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            remaining: bytes,
            pos: 0,
        }
    }

    /// Advances the tokenizer and returns the next [`Token`], or `None` at the end of the input.
    pub fn step(&mut self) -> Option<Token> {
        // Case 1: Quoted literal (`'...'`) or escaped quote (`''`)
        if let Some(after_open) = self.remaining.strip_prefix(b"'") {
            // Two consecutive single quotes (`''`) outside a quoted literal represent a single `'`.
            if let Some(after_escaped) = after_open.strip_prefix(b"'") {
                let range = self.pos..self.pos + 1;
                self.remaining = after_escaped;
                self.pos += 2;
                return Some(Token::Literal(range));
            }
            let Some(quote_pos) = after_open.iter().position(|&b| b == b'\'') else {
                let start = self.pos + 1;
                let end = start + after_open.len();
                self.remaining = b"";
                self.pos = end;
                return Some(Token::UnclosedLiteral(start..end));
            };
            let Some((_, after_quote)) = after_open.split_at_checked(quote_pos) else {
                debug_assert!(
                    false,
                    "quote_pos ({quote_pos}) came from position() on after_open (len {})",
                    after_open.len()
                );
                return None;
            };
            let mut remainder = after_quote.strip_prefix(b"'").unwrap_or(after_quote);
            // Search for nested quotes, like 'o''clock', 'a''', or 'b''''c'.
            // Because consuming `quote_pos` and its `'`, stripping pairs of `b"''"`
            // from `remainder` naturally consumes all remaining quotes when the total quote run is
            // odd (`1 + 2k`, closing the literal), and leaves a single leading `'` in `remainder`
            // when the total quote run is even (`1 + 2(k - 1) + 1`, keeping the next `step()` call
            // in quoted literal mode).
            let mut nested_quote_count = 0;
            while let Some(s) = remainder.strip_prefix(b"''") {
                nested_quote_count += 1;
                remainder = s;
            }
            if remainder.starts_with(b"'") {
                nested_quote_count += 1;
            }
            // If we found nested quotes, include the necessary number of quotes in the literal
            // range. For example, return 'o''clock' as two tokens: o' and clock.
            let literal_start = self.pos + 1;
            let literal_end = literal_start + quote_pos + nested_quote_count;
            let consumed = 1 + after_open.len() - remainder.len();
            self.remaining = remainder;
            self.pos += consumed;
            return Some(Token::Literal(literal_start..literal_end));
        }

        // Case 2: Field symbol (run of identical ASCII letters, e.g. `yyyy`)
        let &b = self.remaining.first()?;
        if b.is_ascii_alphabetic() {
            let byte_len = self.remaining.iter().take_while(|&&c| c == b).count();
            let Some((_, remainder)) = self.remaining.split_at_checked(byte_len) else {
                debug_assert!(
                    false,
                    "byte_len ({byte_len}) came from take_while() on remaining (len {})",
                    self.remaining.len()
                );
                return None;
            };
            self.remaining = remainder;
            self.pos += byte_len;
            return Some(Token::Symbol(b as char, byte_len));
        }

        // Case 3: Unquoted literal (run of non-ASCII-alphabetic, non-quote bytes)
        let byte_len = self
            .remaining
            .iter()
            .take_while(|&&c| !c.is_ascii_alphabetic() && c != b'\'')
            .count();
        let Some((_, remainder)) = self.remaining.split_at_checked(byte_len) else {
            debug_assert!(
                false,
                "byte_len ({byte_len}) came from take_while() on remaining (len {})",
                self.remaining.len()
            );
            return None;
        };
        let range = self.pos..self.pos + byte_len;
        self.remaining = remainder;
        self.pos += byte_len;
        Some(Token::Literal(range))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum ResolvedToken<'a, S: ?Sized> {
        Symbol(char, usize),
        Literal(&'a S),
        UnclosedLiteral(&'a S),
    }

    fn next_str<'a>(
        input: &'a str,
        tokenizer: &mut Uts35DateTimePatternTokenizer<'a>,
    ) -> Option<ResolvedToken<'a, str>> {
        Some(match tokenizer.step()? {
            Token::Symbol(ch, len) => ResolvedToken::Symbol(ch, len),
            Token::Literal(range) => ResolvedToken::Literal(&input[range]),
            Token::UnclosedLiteral(range) => ResolvedToken::UnclosedLiteral(&input[range]),
        })
    }

    fn next_bytes<'a>(
        input: &'a [u8],
        tokenizer: &mut Uts35DateTimePatternTokenizer<'a>,
    ) -> Option<ResolvedToken<'a, [u8]>> {
        Some(match tokenizer.step()? {
            Token::Symbol(ch, len) => ResolvedToken::Symbol(ch, len),
            Token::Literal(range) => ResolvedToken::Literal(&input[range]),
            Token::UnclosedLiteral(range) => ResolvedToken::UnclosedLiteral(&input[range]),
        })
    }

    #[test]
    fn test_basic() {
        let input = "abb.😀.ccc";
        let mut tokenizer = Uts35DateTimePatternTokenizer::new(input.as_bytes());
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Symbol('a', 1))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Symbol('b', 2))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal(".😀."))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Symbol('c', 3))
        );
        assert_eq!(next_str(input, &mut tokenizer), None);
    }

    #[test]
    fn test_quotes() {
        let input = "..a''bb'c'.d..";
        let mut tokenizer = Uts35DateTimePatternTokenizer::new(input.as_bytes());
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal(".."))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Symbol('a', 1))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("'"))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Symbol('b', 2))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("c"))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("."))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Symbol('d', 1))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal(".."))
        );
        assert_eq!(next_str(input, &mut tokenizer), None);
    }

    #[test]
    fn test_escaped_and_unclosed_quotes() {
        let input = "hh .'o''clock' 'a'''. 'foo ";
        let mut tokenizer = Uts35DateTimePatternTokenizer::new(input.as_bytes());
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Symbol('h', 2))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal(" ."))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("o'"))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("clock"))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal(" "))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("a'"))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal(". "))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::UnclosedLiteral("foo "))
        );
        assert_eq!(next_str(input, &mut tokenizer), None);
    }

    #[test]
    fn test_quote_runs() {
        // Note: the expected literals concatenate to "'' 'x a''b"
        let input = "'''' '''x' 'a''''b'";
        let mut tokenizer = Uts35DateTimePatternTokenizer::new(input.as_bytes());
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("'"))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("'"))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal(" "))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("'"))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("x"))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal(" "))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("a''"))
        );
        assert_eq!(
            next_str(input, &mut tokenizer),
            Some(ResolvedToken::Literal("b"))
        );
        assert_eq!(next_str(input, &mut tokenizer), None);
    }

    #[test]
    fn test_invalid_utf8_bytes() {
        let input = b"yy\xff'o''clock\xfe'M'unclosed\xff";
        let mut tokenizer = Uts35DateTimePatternTokenizer::new(input);
        assert_eq!(
            next_bytes(input, &mut tokenizer),
            Some(ResolvedToken::Symbol('y', 2))
        );
        assert_eq!(
            next_bytes(input, &mut tokenizer),
            Some(ResolvedToken::Literal(b"\xff".as_slice()))
        );
        assert_eq!(
            next_bytes(input, &mut tokenizer),
            Some(ResolvedToken::Literal(b"o'".as_slice()))
        );
        assert_eq!(
            next_bytes(input, &mut tokenizer),
            Some(ResolvedToken::Literal(b"clock\xfe".as_slice()))
        );
        assert_eq!(
            next_bytes(input, &mut tokenizer),
            Some(ResolvedToken::Symbol('M', 1))
        );
        assert_eq!(
            next_bytes(input, &mut tokenizer),
            Some(ResolvedToken::UnclosedLiteral(b"unclosed\xff".as_slice()))
        );
        assert_eq!(next_bytes(input, &mut tokenizer), None);
    }
}

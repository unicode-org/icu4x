// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Character-level tokenizer for `MessageFormat` 2 source text.
//!
//! Produces a flat stream of [`Token`] values, with mode tracking for
//! pattern-text versus expression contexts. The parser drives mode transitions
//! explicitly via [`Lexer::push_mode`] / [`Lexer::pop_mode`] when it needs to;
//! the lexer also auto-switches modes on emitted braces so a simple
//! iterator-style drive works for well-formed input.
//!
//! Grammar source: `spec/message.abnf` at WG rev
//! `dd86e42e10d1d0c9c4401d0781cdd87ee7166366`.

use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::messageformat::error::ParseError;

/// One lexical token produced by [`Lexer::next_token`].
///
/// `Text` and `QuotedLiteral` borrow from the source string when no escape
/// sequences are present and allocate only when unescaping is needed.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'src> {
    /// A run of pattern text. Unescaped.
    Text(Cow<'src, str>),
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `{{` — opens a quoted-pattern.
    DoubleOpenBrace,
    /// `}}` — closes a quoted-pattern.
    DoubleCloseBrace,
    /// `$` (variable sigil in an expression)
    Dollar,
    /// `:` (function sigil in an expression)
    Colon,
    /// `@` (attribute sigil in an expression)
    At,
    /// `#` (markup open/standalone sigil)
    Hash,
    /// `/` (markup close / self-close indicator)
    Slash,
    /// `=` (option assignment)
    Equals,
    /// `*` (catchall variant key)
    Star,
    /// `|` (lone pipe — only emitted when the parser explicitly avoids
    /// the quoted-literal path; normally a `|...|` sequence is folded
    /// into [`Token::QuotedLiteral`]).
    Pipe,
    /// An unquoted name (one or more `name-char`s). Used for both identifiers
    /// and unquoted literals; disambiguation is parser-level.
    Name(&'src str),
    /// A `|...|` quoted literal. Escape sequences (`\\`, `\{`, `\}`, `\|`)
    /// have been processed.
    QuotedLiteral(Cow<'src, str>),
    /// An unquoted number literal per the MF2 ABNF (`["-"] (0 / 1-9 *DIGIT)
    /// ["." 1*DIGIT] [e/E [+/-] 1*DIGIT]`). Always borrowed from source.
    NumberLiteral(&'src str),
    /// `.input` declaration keyword.
    InputKeyword,
    /// `.local` declaration keyword.
    LocalKeyword,
    /// `.match` matcher keyword.
    MatchKeyword,
}

/// Lexer mode — controls which tokens the lexer looks for next.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexMode {
    /// Default. Reads text runs until a `{` or end of input.
    Pattern,
    /// Inside a `{{...}}` quoted pattern. Same as [`LexMode::Pattern`]
    /// except `}}` ends the mode.
    QuotedPattern,
    /// Inside an expression `{...}` or markup `{#...}`/`{/...}`.
    Expression,
    /// Outside braces but inside a declaration or matcher (after `.match`,
    /// `.local`, or as a variant key list). Skips whitespace; recognizes
    /// `$`, `*`, `=`, `|...|`, names, and `{{` (which pushes
    /// [`LexMode::QuotedPattern`]); `{` pushes [`LexMode::Expression`].
    Structural,
}

/// Hand-written character-level lexer for MF2 source text.
#[derive(Debug)]
pub struct Lexer<'src> {
    src: &'src str,
    pos: usize,
    modes: Vec<LexMode>,
    /// Whether at least one whitespace or bidi-control byte was consumed
    /// immediately before the most recently emitted expression-mode token.
    /// The parser consults this to enforce the ABNF's `s` (required
    /// whitespace) production between operand/function/attribute parts.
    last_ws: bool,
}

impl<'src> Lexer<'src> {
    /// Construct a lexer starting in [`LexMode::Pattern`].
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            pos: 0,
            modes: vec![LexMode::Pattern],
            last_ws: false,
        }
    }

    /// Construct a lexer with an explicit initial mode. Useful for tests.
    pub fn with_mode(src: &'src str, mode: LexMode) -> Self {
        Self {
            src,
            pos: 0,
            modes: vec![mode],
            last_ws: false,
        }
    }

    /// Whether at least one whitespace or bidi-control byte was consumed
    /// immediately before the most recently emitted expression-mode token.
    pub fn last_token_had_leading_whitespace(&self) -> bool {
        self.last_ws
    }

    /// Current top-of-stack mode.
    pub fn mode(&self) -> LexMode {
        self.modes.last().copied().unwrap_or(LexMode::Pattern)
    }

    /// Push a new mode onto the mode stack.
    pub fn push_mode(&mut self, mode: LexMode) {
        self.modes.push(mode);
    }

    /// Pop the top mode. Returns `None` if the stack is already empty.
    pub fn pop_mode(&mut self) -> Option<LexMode> {
        self.modes.pop()
    }

    /// Byte offset of the next character to be read.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Rewind the lexer to a previously-recorded byte offset. Invalidates no
    /// mode stack; callers use this to restore a position after lookahead.
    pub fn seek(&mut self, pos: usize) {
        self.pos = pos;
        self.last_ws = false;
    }

    /// Returns `true` when all input has been consumed.
    pub fn is_at_end(&self) -> bool {
        self.pos >= self.src.len()
    }

    /// Produce the next token.
    ///
    /// Returns `Ok(None)` when all input in the current mode has been consumed.
    /// Emitting an open-brace token auto-pushes the appropriate nested mode;
    /// emitting a close-brace token auto-pops.
    pub fn next_token(&mut self) -> Result<Option<Token<'src>>, ParseError> {
        match self.mode() {
            LexMode::Pattern => self.next_pattern_token(false),
            LexMode::QuotedPattern => self.next_pattern_token(true),
            LexMode::Expression => self.next_expression_token(),
            LexMode::Structural => self.next_structural_token(),
        }
    }

    /// Check whether the current position begins with the given literal prefix.
    /// Used by the parser to dispatch between simple and complex messages.
    pub fn starts_with_str(&self, prefix: &str) -> bool {
        self.rest().starts_with(prefix)
    }

    /// Skip any whitespace or bidi-control characters at the current position.
    ///
    /// Used by the parser to honor the ABNF's `o` (optional whitespace)
    /// between declarations, keywords, and expressions. Inside pattern text,
    /// whitespace is significant and must not be skipped this way.
    pub fn skip_whitespace(&mut self) {
        self.skip_ws_and_bidi();
    }

    /// Like [`skip_whitespace`] but reports whether an ABNF `s` production
    /// (`*bidi ws o`) was matched — i.e. at least one non-bidi whitespace
    /// character was present. Per spec (message.abnf) bidi controls alone
    /// do not satisfy required-whitespace.
    ///
    /// [`skip_whitespace`]: Self::skip_whitespace
    pub fn consume_whitespace(&mut self) -> bool {
        let mut saw_ws = false;
        while let Some(c) = self.peek() {
            if is_whitespace(c) {
                saw_ws = true;
                self.pos += c.len_utf8();
            } else if is_bidi(c) {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
        saw_ws
    }

    /// Try to consume a declaration keyword (`.input` / `.local` / `.match`)
    /// at the current position, after any leading whitespace or bidi marks.
    ///
    /// Returns `Ok(Some(Token::…Keyword))` on a successful match, `Ok(None)`
    /// when the current position does not begin with a recognized keyword,
    /// and an error only on malformed input at the keyword site.
    pub fn try_consume_keyword(&mut self) -> Result<Option<Token<'src>>, ParseError> {
        let start = self.pos;
        self.skip_ws_and_bidi();
        let rest = self.rest();
        for (kw, tok) in [
            (".input", Token::InputKeyword),
            (".local", Token::LocalKeyword),
            (".match", Token::MatchKeyword),
        ] {
            if let Some(after) = rest.strip_prefix(kw) {
                if after.chars().next().is_none_or(|c| !is_name_char(c)) {
                    self.pos += kw.len();
                    return Ok(Some(tok));
                }
            }
        }
        // Not a keyword — restore position to before the skip.
        self.pos = start;
        Ok(None)
    }

    // ---------- internals ----------

    fn rest(&self) -> &'src str {
        &self.src[self.pos..]
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn peek2(&self) -> Option<char> {
        let mut it = self.rest().chars();
        it.next()?;
        it.next()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn starts_with(&self, s: &str) -> bool {
        self.rest().starts_with(s)
    }

    fn syntax(&self, message: &'static str) -> ParseError {
        ParseError::Syntax {
            offset: self.pos,
            message,
        }
    }

    fn skip_ws_and_bidi(&mut self) {
        while let Some(c) = self.peek() {
            if is_whitespace(c) || is_bidi(c) {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
    }

    /// Consume optional whitespace (`o` = `*(ws / bidi)`) and report whether
    /// an ABNF `s` production (`*bidi ws o`) was satisfied. Shared path for
    /// the inter-token whitespace tracking used by expression and structural
    /// modes.
    fn consume_whitespace_with_bidi_suffix(&mut self) -> bool {
        let mut saw_ws = false;
        while let Some(c) = self.peek() {
            if is_whitespace(c) {
                saw_ws = true;
                self.pos += c.len_utf8();
            } else if is_bidi(c) {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
        saw_ws
    }

    fn next_pattern_token(&mut self, is_quoted: bool) -> Result<Option<Token<'src>>, ParseError> {
        if self.is_at_end() {
            return Ok(None);
        }

        // Quoted-pattern close: `}}` pops the mode.
        if is_quoted && self.starts_with("}}") {
            self.pos += 2;
            self.pop_mode();
            return Ok(Some(Token::DoubleCloseBrace));
        }

        match self.peek() {
            Some('{') => {
                if self.peek2() == Some('{') {
                    self.pos += 2;
                    self.push_mode(LexMode::QuotedPattern);
                    Ok(Some(Token::DoubleOpenBrace))
                } else {
                    self.pos += 1;
                    self.push_mode(LexMode::Expression);
                    Ok(Some(Token::OpenBrace))
                }
            }
            Some('}') => {
                // Bare `}` in a pattern is a syntax error in well-formed
                // messages; quoted-pattern `}}` is handled above.
                Err(self.syntax("unmatched `}` in pattern"))
            }
            Some(_) => self.read_text_run(is_quoted).map(Some),
            None => Ok(None),
        }
    }

    fn read_text_run(&mut self, is_quoted: bool) -> Result<Token<'src>, ParseError> {
        let start = self.pos;
        let mut owned: Option<String> = None;

        loop {
            match self.peek() {
                None => break,
                Some('{') | Some('}') => {
                    if is_quoted && self.starts_with("}}") {
                        break;
                    }
                    if self.peek() == Some('{') {
                        break;
                    }
                    if self.peek() == Some('}') && !(is_quoted && self.starts_with("}}")) {
                        // Bare `}` ends the run so the top-level dispatcher
                        // can report the syntax error.
                        break;
                    }
                    break;
                }
                Some('\\') => {
                    // Per spec ABNF `escaped-char = backslash ( backslash /
                    // "{" / "|" / "}" )` — all four are valid in pattern text.
                    let escape_start = self.pos;
                    self.bump();
                    let esc = self.peek().ok_or(ParseError::Syntax {
                        offset: escape_start,
                        message: "dangling backslash",
                    })?;
                    if !matches!(esc, '\\' | '{' | '}' | '|') {
                        return Err(ParseError::Syntax {
                            offset: self.pos,
                            message: "invalid escape in pattern text",
                        });
                    }
                    // Commit to owned mode and push the unescaped char.
                    if owned.is_none() {
                        owned = Some(self.src[start..escape_start].to_string());
                    }
                    self.bump();
                    if let Some(s) = owned.as_mut() {
                        s.push(esc);
                    }
                }
                Some(c) => {
                    // ABNF `text-char` / `simple-start-char` exclude U+0000.
                    if c == '\0' {
                        return Err(self.syntax("NUL is not allowed in pattern text"));
                    }
                    if let Some(s) = owned.as_mut() {
                        s.push(c);
                    }
                    self.bump();
                }
            }
        }

        Ok(match owned {
            Some(s) => Token::Text(Cow::Owned(s)),
            None => Token::Text(Cow::Borrowed(&self.src[start..self.pos])),
        })
    }

    fn next_expression_token(&mut self) -> Result<Option<Token<'src>>, ParseError> {
        self.last_ws = self.consume_whitespace_with_bidi_suffix();
        let Some(c) = self.peek() else {
            return Ok(None);
        };
        match c {
            '}' => {
                self.pos += 1;
                self.pop_mode();
                Ok(Some(Token::CloseBrace))
            }
            '{' => {
                self.pos += 1;
                self.push_mode(LexMode::Expression);
                Ok(Some(Token::OpenBrace))
            }
            '$' => {
                self.pos += 1;
                Ok(Some(Token::Dollar))
            }
            ':' => {
                self.pos += 1;
                Ok(Some(Token::Colon))
            }
            '@' => {
                self.pos += 1;
                Ok(Some(Token::At))
            }
            '#' => {
                self.pos += 1;
                Ok(Some(Token::Hash))
            }
            '/' => {
                self.pos += 1;
                Ok(Some(Token::Slash))
            }
            '=' => {
                self.pos += 1;
                Ok(Some(Token::Equals))
            }
            '*' => {
                self.pos += 1;
                Ok(Some(Token::Star))
            }
            '|' => self.read_quoted_literal().map(Some),
            '-' => Ok(Some(self.read_number_or_unquoted())),
            c if c.is_ascii_digit() => Ok(Some(self.read_number_or_unquoted())),
            c if is_name_char(c) => self.read_name().map(|n| Some(Token::Name(n))),
            _ => Err(self.syntax("unexpected character in expression")),
        }
    }

    fn next_structural_token(&mut self) -> Result<Option<Token<'src>>, ParseError> {
        self.last_ws = self.consume_whitespace_with_bidi_suffix();
        if self.is_at_end() {
            return Ok(None);
        }
        if self.starts_with("{{") {
            self.pos += 2;
            self.push_mode(LexMode::QuotedPattern);
            return Ok(Some(Token::DoubleOpenBrace));
        }
        let Some(c) = self.peek() else {
            return Ok(None);
        };
        match c {
            '{' => {
                self.pos += 1;
                self.push_mode(LexMode::Expression);
                Ok(Some(Token::OpenBrace))
            }
            '$' => {
                self.pos += 1;
                Ok(Some(Token::Dollar))
            }
            '*' => {
                self.pos += 1;
                Ok(Some(Token::Star))
            }
            '=' => {
                self.pos += 1;
                Ok(Some(Token::Equals))
            }
            '|' => self.read_quoted_literal().map(Some),
            '-' => Ok(Some(self.read_number_or_unquoted())),
            c if c.is_ascii_digit() => Ok(Some(self.read_number_or_unquoted())),
            c if is_name_char(c) => self.read_name().map(|n| Some(Token::Name(n))),
            _ => Err(self.syntax("unexpected character in declaration or matcher")),
        }
    }

    fn read_name(&mut self) -> Result<&'src str, ParseError> {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if is_name_char(c) {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
        if self.pos == start {
            return Err(self.syntax("expected name"));
        }
        Ok(&self.src[start..self.pos])
    }

    /// When the next character is an ASCII digit, emit either a
    /// `NumberLiteral` (when the token matches the MF2 number-literal ABNF)
    /// or a `Name` (when it matches only the unquoted-literal rule). Per
    /// spec both productions are valid `literal` positions; downstream
    /// consumers treat non-numeric unquoted literals as `bad-operand` at
    /// runtime for numeric functions.
    fn read_number_or_unquoted(&mut self) -> Token<'src> {
        let start = self.pos;
        // Read the maximal name-char run. ASCII digits, `.` and `-` are all
        // name-chars, so exponent / fractional forms (`1.5`, `1e2`) are
        // included even though `-` does not otherwise start a name.
        while let Some(c) = self.peek() {
            if !is_name_char(c) {
                break;
            }
            self.bump();
        }
        let slice = &self.src[start..self.pos];
        if is_valid_number_literal(slice) {
            Token::NumberLiteral(slice)
        } else {
            Token::Name(slice)
        }
    }

    fn read_quoted_literal(&mut self) -> Result<Token<'src>, ParseError> {
        // Opening `|`.
        debug_assert_eq!(self.peek(), Some('|'));
        self.pos += 1;
        let content_start = self.pos;
        let mut owned: Option<String> = None;

        loop {
            match self.peek() {
                None => return Err(self.syntax("unterminated quoted literal")),
                Some('|') => {
                    let end = self.pos;
                    self.pos += 1;
                    return Ok(match owned {
                        Some(s) => Token::QuotedLiteral(Cow::Owned(s)),
                        None => Token::QuotedLiteral(Cow::Borrowed(&self.src[content_start..end])),
                    });
                }
                Some('\\') => {
                    let escape_start = self.pos;
                    self.bump();
                    let esc = self.peek().ok_or(ParseError::Syntax {
                        offset: escape_start,
                        message: "dangling backslash",
                    })?;
                    if !matches!(esc, '\\' | '{' | '}' | '|') {
                        return Err(ParseError::Syntax {
                            offset: self.pos,
                            message: "invalid escape in quoted literal",
                        });
                    }
                    if owned.is_none() {
                        owned = Some(self.src[content_start..escape_start].to_string());
                    }
                    self.bump();
                    if let Some(s) = owned.as_mut() {
                        s.push(esc);
                    }
                }
                Some(c) => {
                    // ABNF `quoted-char` excludes U+0000.
                    if c == '\0' {
                        return Err(self.syntax("NUL is not allowed in quoted literal"));
                    }
                    if let Some(s) = owned.as_mut() {
                        s.push(c);
                    }
                    self.bump();
                }
            }
        }
    }
}

// ---------- character classification ----------

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n' | '\u{3000}')
}

fn is_bidi(c: char) -> bool {
    matches!(
        c,
        '\u{061C}' | '\u{200E}' | '\u{200F}' | '\u{2066}'..='\u{2069}'
    )
}

/// `name-start` classifier — ASCII alpha, `+`, `_`, and the Unicode ranges
/// enumerated in the MF2 ABNF `name-start` rule.
fn is_name_start(c: char) -> bool {
    if c.is_ascii_alphabetic() {
        return true;
    }
    matches!(c, '+' | '_')
        || matches!(c as u32,
            0x00A1..=0x061B
            | 0x061D..=0x167F
            | 0x1681..=0x1FFF
            | 0x200B..=0x200D
            | 0x2010..=0x2027
            | 0x2030..=0x205E
            | 0x2060..=0x2065
            | 0x206A..=0x2FFF
            | 0x3001..=0xD7FF
            | 0xE000..=0xFDCF
            | 0xFDF0..=0xFFFD
            | 0x10000..=0x1FFFD
            | 0x20000..=0x2FFFD
            | 0x30000..=0x3FFFD
            | 0x40000..=0x4FFFD
            | 0x50000..=0x5FFFD
            | 0x60000..=0x6FFFD
            | 0x70000..=0x7FFFD
            | 0x80000..=0x8FFFD
            | 0x90000..=0x9FFFD
            | 0xA0000..=0xAFFFD
            | 0xB0000..=0xBFFFD
            | 0xC0000..=0xCFFFD
            | 0xD0000..=0xDFFFD
            | 0xE0000..=0xEFFFD
            | 0xF0000..=0xFFFFD
            | 0x100000..=0x10FFFD)
}

/// Does `s` match the MF2 number-literal ABNF exactly?
pub(crate) fn is_valid_number_literal(s: &str) -> bool {
    let mut it = s.chars().peekable();
    // Optional leading minus.
    if it.peek() == Some(&'-') {
        it.next();
    }
    // Integer part: `0` or `1-9 *DIGIT`.
    match it.next() {
        Some('0') => {}
        Some(c) if ('1'..='9').contains(&c) => {
            while let Some(&c) = it.peek() {
                if c.is_ascii_digit() {
                    it.next();
                } else {
                    break;
                }
            }
        }
        _ => return false,
    }
    // Optional fractional part.
    if it.peek() == Some(&'.') {
        it.next();
        let mut any = false;
        while let Some(&c) = it.peek() {
            if c.is_ascii_digit() {
                any = true;
                it.next();
            } else {
                break;
            }
        }
        if !any {
            return false;
        }
    }
    // Optional exponent.
    if matches!(it.peek(), Some(&'e') | Some(&'E')) {
        it.next();
        if matches!(it.peek(), Some(&'+') | Some(&'-')) {
            it.next();
        }
        let mut any = false;
        while let Some(&c) = it.peek() {
            if c.is_ascii_digit() {
                any = true;
                it.next();
            } else {
                break;
            }
        }
        if !any {
            return false;
        }
    }
    it.next().is_none()
}

fn is_name_char(c: char) -> bool {
    is_name_start(c) || c.is_ascii_digit() || c == '-' || c == '.'
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(src: &str) -> Vec<Token<'_>> {
        let mut lx = Lexer::new(src);
        let mut toks = Vec::new();
        while let Some(t) = lx.next_token().expect("no syntax error") {
            toks.push(t);
        }
        toks
    }

    #[test]
    fn plain_text() {
        let toks = lex("hello");
        assert_eq!(toks, vec![Token::Text(Cow::Borrowed("hello"))]);
    }

    #[test]
    fn empty_input() {
        assert_eq!(lex(""), Vec::<Token<'_>>::new());
    }

    #[test]
    fn variable_expression() {
        let toks = lex("{$x}");
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("x"),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn text_then_variable() {
        let toks = lex("Hello, {$user}!");
        assert_eq!(
            toks,
            vec![
                Token::Text(Cow::Borrowed("Hello, ")),
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("user"),
                Token::CloseBrace,
                Token::Text(Cow::Borrowed("!")),
            ]
        );
    }

    #[test]
    fn function_expression() {
        let toks = lex("{$count :integer}");
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("count"),
                Token::Colon,
                Token::Name("integer"),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn function_with_option() {
        let toks = lex("{$d :datetime weekday=long}");
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("d"),
                Token::Colon,
                Token::Name("datetime"),
                Token::Name("weekday"),
                Token::Equals,
                Token::Name("long"),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn quoted_literal_simple() {
        let toks = lex("{|hello world|}");
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::QuotedLiteral(Cow::Borrowed("hello world")),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn quoted_literal_with_escapes() {
        // |a \| b| → `a | b`
        let toks = lex(r"{|a \| b|}");
        match &toks[1] {
            Token::QuotedLiteral(Cow::Owned(s)) => assert_eq!(s, "a | b"),
            other => panic!("expected owned quoted literal, got {other:?}"),
        }
    }

    #[test]
    fn markup_tokens() {
        let toks = lex("{#b}bold{/b}{#br /}");
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Hash,
                Token::Name("b"),
                Token::CloseBrace,
                Token::Text(Cow::Borrowed("bold")),
                Token::OpenBrace,
                Token::Slash,
                Token::Name("b"),
                Token::CloseBrace,
                Token::OpenBrace,
                Token::Hash,
                Token::Name("br"),
                Token::Slash,
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn text_with_escapes() {
        // Pattern text: \\, \{, \}
        let toks = lex(r"a \{ b \} c \\ d");
        match &toks[0] {
            Token::Text(Cow::Owned(s)) => assert_eq!(s, "a { b } c \\ d"),
            other => panic!("expected owned text, got {other:?}"),
        }
    }

    #[test]
    fn double_braces_quoted_pattern() {
        let toks = lex("{{Hello, {$u}!}}");
        assert_eq!(
            toks,
            vec![
                Token::DoubleOpenBrace,
                Token::Text(Cow::Borrowed("Hello, ")),
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("u"),
                Token::CloseBrace,
                Token::Text(Cow::Borrowed("!")),
                Token::DoubleCloseBrace,
            ]
        );
    }

    #[test]
    fn whitespace_skipped_in_expression() {
        let toks = lex("{   $x   :string   }");
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("x"),
                Token::Colon,
                Token::Name("string"),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn attribute_tokens() {
        let toks = lex("{$x :number @translate=no}");
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("x"),
                Token::Colon,
                Token::Name("number"),
                Token::At,
                Token::Name("translate"),
                Token::Equals,
                Token::Name("no"),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn star_key() {
        let mut lx = Lexer::with_mode("*", LexMode::Expression);
        assert_eq!(lx.next_token().unwrap(), Some(Token::Star));
    }

    #[test]
    fn bidi_controls_are_whitespace_in_expressions() {
        // FSI / PDI around a name are stripped in expression context.
        let src = "{\u{2068}$x\u{2069}}";
        let toks = lex(src);
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("x"),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn unicode_name_greek() {
        // Greek letters are in the name-start range 0x3001..=0xD7FF.
        let src = "{$αβγ}";
        let toks = lex(src);
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("αβγ"),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn unicode_name_cjk() {
        let src = "{$名前}";
        let toks = lex(src);
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("名前"),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn namespaced_function() {
        let toks = lex("{:ns:fn}");
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Colon,
                Token::Name("ns"),
                Token::Colon,
                Token::Name("fn"),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn keyword_input() {
        let mut lx = Lexer::new(".input {$x :integer}");
        assert_eq!(lx.try_consume_keyword().unwrap(), Some(Token::InputKeyword));
        // The parser is responsible for skipping inter-token whitespace.
        lx.skip_whitespace();
        assert_eq!(lx.next_token().unwrap(), Some(Token::OpenBrace));
    }

    #[test]
    fn keyword_local_and_match() {
        let mut lx = Lexer::new(".local $y = {$x} .match $y * {{_}}");
        assert_eq!(lx.try_consume_keyword().unwrap(), Some(Token::LocalKeyword));
    }

    #[test]
    fn keyword_prefix_is_not_keyword() {
        // `.inputXYZ` must NOT match `.input` because the follow-char is a name-char.
        let mut lx = Lexer::new(".inputXYZ");
        assert_eq!(lx.try_consume_keyword().unwrap(), None);
    }

    #[test]
    fn dangling_backslash_is_error() {
        let mut lx = Lexer::new("a \\");
        let err = loop {
            match lx.next_token() {
                Ok(None) => panic!("expected syntax error"),
                Ok(Some(_)) => continue,
                Err(e) => break e,
            }
        };
        assert!(matches!(err, ParseError::Syntax { .. }));
    }

    #[test]
    fn unmatched_close_brace_is_error() {
        let mut lx = Lexer::new("oops}");
        // First token consumes "oops".
        let _ = lx.next_token().unwrap();
        let err = lx.next_token().unwrap_err();
        assert!(matches!(err, ParseError::Syntax { .. }));
    }

    #[test]
    fn unterminated_quoted_literal_is_error() {
        let mut lx = Lexer::new("{|abc");
        let _ = lx.next_token().unwrap(); // OpenBrace
        let err = lx.next_token().unwrap_err();
        assert!(matches!(err, ParseError::Syntax { .. }));
    }

    #[test]
    fn unicode_name_supplementary_plane() {
        // Gothic letter (U+10330) — in the supplementary plane, covered
        // by the 0x10000..=0x1FFFD name-start range.
        let src = "{$\u{10330}\u{10331}}";
        let toks = lex(src);
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("\u{10330}\u{10331}"),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn name_rejects_noncharacter_plane_boundary() {
        // U+FDD0 is a non-character in the gap 0xFDD0..=0xFDEF excluded by
        // the name-start ABNF. It must not begin a name.
        assert!(!is_name_start('\u{FDD0}'));
    }

    #[test]
    fn bom_in_text_passes_through() {
        // U+FEFF is not whitespace in MF2; it becomes part of pattern text.
        let src = "\u{FEFF}hi";
        let toks = lex(src);
        match &toks[0] {
            Token::Text(cow) => assert_eq!(&**cow, "\u{FEFF}hi"),
            other => panic!("expected text token, got {other:?}"),
        }
    }

    #[test]
    fn ideographic_space_skipped_in_expression() {
        // U+3000 ideographic space is classified as whitespace in MF2.
        let src = "{\u{3000}$x\u{3000}}";
        let toks = lex(src);
        assert_eq!(
            toks,
            vec![
                Token::OpenBrace,
                Token::Dollar,
                Token::Name("x"),
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn negative_number_literal() {
        let mut lx = Lexer::with_mode("-42", LexMode::Expression);
        let t = lx.next_token().unwrap().unwrap();
        assert_eq!(t, Token::NumberLiteral("-42"));
    }

    #[test]
    fn number_literal_scientific_and_fractional() {
        assert!(is_valid_number_literal("0"));
        assert!(is_valid_number_literal("-0"));
        assert!(is_valid_number_literal("123"));
        assert!(is_valid_number_literal("1.5"));
        assert!(is_valid_number_literal("-1.5e10"));
        assert!(is_valid_number_literal("2.5E-3"));
        assert!(is_valid_number_literal("7e+2"));
    }

    #[test]
    fn number_literal_rejects_leading_zero_and_trailing_dot() {
        // ABNF forbids `00`, `.5`, `5.`, and bare exponents.
        assert!(!is_valid_number_literal("00"));
        assert!(!is_valid_number_literal("01"));
        assert!(!is_valid_number_literal(".5"));
        assert!(!is_valid_number_literal("5."));
        assert!(!is_valid_number_literal("1e"));
        assert!(!is_valid_number_literal("1e+"));
        assert!(!is_valid_number_literal(""));
        assert!(!is_valid_number_literal("-"));
    }

    #[test]
    fn pattern_text_preserves_combined_grapheme() {
        // Family emoji with ZWJ sequence must pass through byte-for-byte.
        let src = "Hello \u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F466}!";
        let toks = lex(src);
        match &toks[0] {
            Token::Text(cow) => assert_eq!(&**cow, src),
            other => panic!("expected text token, got {other:?}"),
        }
    }

    #[test]
    fn quoted_literal_preserves_unicode() {
        // Quoted literals only process \\, \|, \{, \} — other content
        // (including RTL scripts) is passed through verbatim.
        let toks = lex("{|שלום עולם|}");
        match &toks[1] {
            Token::QuotedLiteral(cow) => assert_eq!(&**cow, "שלום עולם"),
            other => panic!("expected quoted literal, got {other:?}"),
        }
    }

    #[test]
    fn private_use_area_is_valid_name_start() {
        // U+E000 is inside the E000..=FDCF range of name-start; MF2 allows
        // PUA names because the spec intentionally admits them.
        assert!(is_name_start('\u{E000}'));
    }

    #[test]
    fn ascii_digit_is_not_name_start() {
        // Digits are name-char but NOT name-start per the ABNF.
        assert!(!is_name_start('0'));
        assert!(!is_name_start('9'));
    }

    #[test]
    fn hyphen_is_not_name_start() {
        // `-` is a name-char but must not start a name (ambiguous with
        // negative-number prefix).
        assert!(!is_name_start('-'));
    }
}

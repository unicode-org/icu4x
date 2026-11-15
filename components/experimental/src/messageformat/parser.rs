// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parser for MF2 simple messages, following `spec/message.abnf` of LDML 48.2.

use core::fmt;
use core::ops::Range;
use utf8_iter::ErrorReportingUtf8Chars;
use writeable::Writeable;

/// A part of a message.
///
/// The events from an [`Event::ExpressionStart`] to the next [`Event::ExpressionEnd`]
/// describe one expression.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum Event<'a> {
    /// Text to output verbatim.
    Text(&'a str),
    /// The `{` that opens an expression.
    ExpressionStart,
    /// The operand of the expression.
    Operand(Value<'a>),
    /// The `}` that closes an expression.
    ExpressionEnd,
}

/// A literal or a variable.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) enum Value<'a> {
    Literal(Literal<'a>),
    /// The name of a variable, without `$` and bidi marks.
    Variable(&'a str),
}

/// The value of a literal.
///
/// Quoted and unquoted literals with the same value are not distinguished.
/// The value is written with [`Writeable`], because an escape sequence makes it
/// differ from the source.
// Equality would compare the source, not the value, so it exists only for tests.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct Literal<'a> {
    /// The source of an unquoted literal, or of a quoted literal between its `|`.
    /// Every backslash in it starts an escape sequence that the parser has checked.
    raw: &'a str,
}

impl Writeable for Literal<'_> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let mut rest = self.raw;
        while let Some((unescaped, escape)) = rest.split_once('\\') {
            sink.write_str(unescaped)?;
            // The escaped character stands for itself. Taking it here also keeps the
            // second backslash of `\\` from being read as the start of an escape.
            let mut chars = escape.chars();
            match chars.next() {
                Some(escaped) => sink.write_char(escaped)?,
                None => debug_assert!(false, "{:?} ends with a backslash", self.raw),
            }
            rest = chars.as_str();
        }
        sink.write_str(rest)
    }

    fn writeable_borrow(&self) -> Option<&str> {
        (!self.raw.contains('\\')).then_some(self.raw)
    }
}

writeable::impl_display_with_writeable!(Literal<'_>);

/// An error found while parsing a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ParseError {
    /// Byte offset of the offending input, or the source length for [`ParseErrorKind::Eof`].
    pub(crate) offset: usize,
    pub(crate) kind: ParseErrorKind,
}

/// The kind of a [`ParseError`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParseErrorKind {
    /// A character that is not allowed at this position.
    UnexpectedChar(char),
    /// A byte sequence that is not valid UTF-8.
    InvalidUtf8,
    /// The message ended inside an expression or an escape sequence.
    Eof,
    /// Parsing reached syntax outside the supported subset.
    /// The remaining input has not been validated.
    Unsupported,
}

/// Yields the events of a simple message, each with the byte range of the source it came from.
///
/// The source does not need to be validated as UTF-8 beforehand: the parser validates
/// every byte it reads.
///
/// Events are yielded as the message is read, so an error invalidates the whole message,
/// including events yielded before it. Consumers must iterate to completion to validate
/// the message. After an error or the last event, `next` returns `None`.
pub(crate) struct Parser<'a> {
    source: &'a [u8],
    /// The unread end of `source`.
    rest: &'a [u8],
    state: State,
}

/// Where the parser is in the message, named after the last event it yielded.
enum State {
    /// In a pattern, outside of any expression.
    Pattern,
    ExpressionStart,
    Operand,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            rest: source,
            state: State::Pattern,
        }
    }

    /// Byte offset of the next unread byte.
    fn offset(&self) -> usize {
        self.source.len() - self.rest.len()
    }

    /// Decodes the next character, which stays unread until `Peek::consume` is called.
    fn peek(&mut self) -> Result<Option<Peek<'_, 'a>>, ParseError> {
        let mut chars = ErrorReportingUtf8Chars::new(self.rest);
        match chars.next() {
            None => Ok(None),
            Some(Ok(c)) => Ok(Some(Peek {
                c,
                rest: chars.as_slice(),
                parser: self,
            })),
            Some(Err(_)) => Err(self.invalid_utf8()),
        }
    }

    fn unexpected(&self, c: char) -> ParseError {
        ParseError {
            offset: self.offset(),
            kind: ParseErrorKind::UnexpectedChar(c),
        }
    }

    fn invalid_utf8(&self) -> ParseError {
        ParseError {
            offset: self.offset(),
            kind: ParseErrorKind::InvalidUtf8,
        }
    }

    fn eof(&self) -> ParseError {
        ParseError {
            offset: self.offset(),
            kind: ParseErrorKind::Eof,
        }
    }

    fn unsupported(&self) -> ParseError {
        ParseError {
            offset: self.offset(),
            kind: ParseErrorKind::Unsupported,
        }
    }

    fn next_event(&mut self) -> Result<Option<(Event<'a>, Range<usize>)>, ParseError> {
        match self.state {
            State::Pattern => self.parse_pattern_part(),
            State::ExpressionStart => self.parse_operand().map(Some),
            State::Operand => self.parse_expression_end().map(Some),
        }
    }

    // pattern = *(text-char / escaped-char / placeholder)
    // One run of text-char, one escaped-char, or the "{" of a placeholder.
    fn parse_pattern_part(&mut self) -> Result<Option<(Event<'a>, Range<usize>)>, ParseError> {
        if self.offset() == 0 {
            self.check_simple_start()?;
        }
        let start = self.offset();
        let event = match self.rest {
            [] => return Ok(None),
            [b'\\', rest @ ..] => {
                self.rest = rest;
                Event::Text(self.parse_escaped_char()?)
            }
            [b'{', rest @ ..] => {
                self.rest = rest;
                self.state = State::ExpressionStart;
                Event::ExpressionStart
            }
            [b @ (b'}' | b'\0'), ..] => return Err(self.unexpected(char::from(*b))),
            _ => self.parse_text()?,
        };
        Ok(Some((event, start..self.offset())))
    }

    // simple-message = o [simple-start pattern]
    // simple-start-char excludes `.`, so a first non-whitespace `.` or `{{`
    // can only begin a complex-message.
    fn check_simple_start(&self) -> Result<(), ParseError> {
        let mut lookahead = Self {
            source: self.source,
            rest: self.rest,
            state: State::Pattern,
        };
        lookahead.skip_o()?;
        if lookahead.rest.starts_with(b".") || lookahead.rest.starts_with(b"{{") {
            return Err(lookahead.unsupported());
        }
        Ok(())
    }

    // A run of text-char.
    fn parse_text(&mut self) -> Result<Event<'a>, ParseError> {
        let start = self.offset();
        // The bytes that end a run are ASCII, and UTF-8 never uses ASCII bytes inside
        // a multi-byte character, so the end of the run can be found without decoding.
        let len = self
            .rest
            .iter()
            .position(|&b| matches!(b, b'\\' | b'{' | b'}' | b'\0'))
            .unwrap_or(self.rest.len());
        let (run, rest) = self.rest.split_at_checked(len).unwrap_or_else(|| {
            debug_assert!(false, "a run of {len} bytes with {} left", self.rest.len());
            // Consuming everything guarantees that parsing still terminates.
            (self.rest, &[])
        });
        self.rest = rest;
        let text = core::str::from_utf8(run).map_err(|e| ParseError {
            offset: start + e.valid_up_to(),
            kind: ParseErrorKind::InvalidUtf8,
        })?;
        Ok(Event::Text(text))
    }

    // escaped-char = backslash ( backslash / "{" / "|" / "}" )
    // The backslash has been consumed. Returns the escaped character.
    fn parse_escaped_char(&mut self) -> Result<&'static str, ParseError> {
        let Some(p) = self.peek()? else {
            return Err(self.eof());
        };
        let escaped = match p.c {
            '\\' => "\\",
            '{' => "{",
            '|' => "|",
            '}' => "}",
            _ => return Err(p.unexpected()),
        };
        p.consume();
        Ok(escaped)
    }

    // literal-expression  = "{" o literal [s function] *(s attribute) o "}"
    // variable-expression = "{" o variable [s function] *(s attribute) o "}"
    // The `o literal` or `o variable` after the "{".
    fn parse_operand(&mut self) -> Result<(Event<'a>, Range<usize>), ParseError> {
        self.skip_o()?;
        let start = self.offset();
        let operand = match self.peek()? {
            Some(p) if p.c == '$' => {
                p.consume();
                Value::Variable(self.parse_name()?)
            }
            Some(p) if p.c == '|' => {
                p.consume();
                Value::Literal(self.parse_quoted_literal()?)
            }
            // The start of a function-expression or markup.
            Some(p) if matches!(p.c, ':' | '#' | '/') => return Err(self.unsupported()),
            _ => Value::Literal(self.parse_unquoted_literal()?),
        };
        self.state = State::Operand;
        Ok((Event::Operand(operand), start..self.offset()))
    }

    // The `[s function] *(s attribute) o "}"` after an operand,
    // without function and attributes.
    fn parse_expression_end(&mut self) -> Result<(Event<'a>, Range<usize>), ParseError> {
        let saw_ws = self.skip_o()?;
        let start = self.offset();
        match self.peek()? {
            Some(p) if p.c == '}' => {
                p.consume();
                self.state = State::Pattern;
                Ok((Event::ExpressionEnd, start..self.offset()))
            }
            // s = *bidi ws o, so bidi marks alone do not separate a function or attribute.
            Some(p) if saw_ws && matches!(p.c, ':' | '@') => Err(self.unsupported()),
            Some(p) => Err(p.unexpected()),
            None => Err(self.eof()),
        }
    }

    // quoted-literal = "|" *(quoted-char / escaped-char) "|"
    // The opening "|" has been consumed.
    fn parse_quoted_literal(&mut self) -> Result<Literal<'a>, ParseError> {
        let start = self.rest;
        let len = loop {
            let len = start.len() - self.rest.len();
            let Some(p) = self.peek()? else {
                return Err(self.eof());
            };
            match p.c {
                '|' => {
                    p.consume();
                    break len;
                }
                '\\' => {
                    p.consume();
                    self.parse_escaped_char()?;
                }
                '\0' => return Err(p.unexpected()),
                _ => p.consume(),
            }
        };
        Ok(Literal {
            raw: Self::decoded(start, len),
        })
    }

    // unquoted-literal = 1*name-char
    fn parse_unquoted_literal(&mut self) -> Result<Literal<'a>, ParseError> {
        let start = self.rest;
        match self.peek()? {
            Some(p) if is_name_char(p.c) => p.consume(),
            Some(p) => return Err(p.unexpected()),
            None => return Err(self.eof()),
        }
        while let Some(p) = self.peek()?.filter(|p| is_name_char(p.c)) {
            p.consume();
        }
        Ok(Literal {
            raw: Self::decoded(start, start.len() - self.rest.len()),
        })
    }

    // name = [bidi] name-start *name-char [bidi]
    // Returns the name without the bidi marks.
    fn parse_name(&mut self) -> Result<&'a str, ParseError> {
        if let Some(p) = self.peek()?.filter(|p| is_bidi(p.c)) {
            p.consume();
        }
        let start = self.rest;
        match self.peek()? {
            Some(p) if is_name_start(p.c) => p.consume(),
            Some(p) => return Err(p.unexpected()),
            None => return Err(self.eof()),
        }
        while let Some(p) = self.peek()?.filter(|p| is_name_char(p.c)) {
            p.consume();
        }
        let name = Self::decoded(start, start.len() - self.rest.len());
        if let Some(p) = self.peek()?.filter(|p| is_bidi(p.c)) {
            p.consume();
        }
        Ok(name)
    }

    /// The first `len` bytes of `input` as text. They must be whole characters
    /// that `peek` has decoded.
    fn decoded(input: &'a [u8], len: usize) -> &'a str {
        input
            .get(..len)
            .and_then(|bytes| core::str::from_utf8(bytes).ok())
            .unwrap_or_else(|| {
                debug_assert!(false, "{len} bytes of {input:?} are not decoded characters");
                ""
            })
    }

    // o = *(ws / bidi)
    // Returns whether any ws was skipped.
    fn skip_o(&mut self) -> Result<bool, ParseError> {
        let mut saw_ws = false;
        while let Some(p) = self.peek()? {
            if is_ws(p.c) {
                saw_ws = true;
            } else if !is_bidi(p.c) {
                break;
            }
            p.consume();
        }
        Ok(saw_ws)
    }
}

/// A decoded character that the parser has not consumed yet.
struct Peek<'p, 'a> {
    parser: &'p mut Parser<'a>,
    c: char,
    /// The input after `c`.
    rest: &'a [u8],
}

impl Peek<'_, '_> {
    /// Advances the parser past the character.
    fn consume(self) {
        self.parser.rest = self.rest;
    }

    /// The error for the character not being allowed at this position.
    fn unexpected(self) -> ParseError {
        self.parser.unexpected(self.c)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<(Event<'a>, Range<usize>), ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next_event().transpose();
        if let Some(Err(_)) = result {
            self.rest = &[];
            self.state = State::Pattern;
        }
        result
    }
}

// ws = SP / HTAB / CR / LF / %x3000
fn is_ws(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n' | '\u{3000}')
}

// bidi = %x061C / %x200E / %x200F / %x2066-2069
fn is_bidi(c: char) -> bool {
    matches!(
        c,
        '\u{061C}' | '\u{200E}' | '\u{200F}' | '\u{2066}'..='\u{2069}'
    )
}

// name-start
fn is_name_start(c: char) -> bool {
    match c {
        'a'..='z'
        | 'A'..='Z'
        | '+'
        | '_'
        | '\u{A1}'..='\u{61B}'
        | '\u{61D}'..='\u{167F}'
        | '\u{1681}'..='\u{1FFF}'
        | '\u{200B}'..='\u{200D}'
        | '\u{2010}'..='\u{2027}'
        | '\u{2030}'..='\u{205E}'
        | '\u{2060}'..='\u{2065}'
        | '\u{206A}'..='\u{2FFF}'
        | '\u{3001}'..='\u{D7FF}'
        | '\u{E000}'..='\u{FDCF}'
        | '\u{FDF0}'..='\u{FFFD}' => true,
        // %x10000-1FFFD through %x100000-10FFFD: every supplementary plane
        // except its last two code points, which are noncharacters.
        '\u{10000}'.. => u32::from(c) & 0xFFFF < 0xFFFE,
        _ => false,
    }
}

// name-char = name-start / DIGIT / "-" / "."
fn is_name_char(c: char) -> bool {
    is_name_start(c) || matches!(c, '0'..='9' | '-' | '.')
}

#[cfg(test)]
mod tests {
    use super::*;
    use Event::*;
    use ParseErrorKind::*;
    use writeable::assert_writeable_eq;

    type Events<'a> = Vec<(Event<'a>, Range<usize>)>;

    fn parse(source: &str) -> Result<Events<'_>, ParseError> {
        Parser::new(source.as_bytes()).collect()
    }

    fn error(offset: usize, kind: ParseErrorKind) -> Result<Events<'static>, ParseError> {
        Err(ParseError { offset, kind })
    }

    fn var(name: &str) -> Event<'_> {
        Operand(Value::Variable(name))
    }

    fn lit(raw: &str) -> Event<'_> {
        Operand(Value::Literal(Literal { raw }))
    }

    /// The events of a message that is a single expression without whitespace,
    /// whose operand is `len` bytes long in the source.
    fn expression(operand: Event<'_>, len: usize) -> Events<'_> {
        vec![
            (ExpressionStart, 0..1),
            (operand, 1..len + 1),
            (ExpressionEnd, len + 1..len + 2),
        ]
    }

    #[test]
    fn empty_message() {
        assert_eq!(parse(""), Ok(vec![]));
    }

    #[test]
    fn text_and_variable() {
        assert_eq!(
            parse("Hello {$name}!"),
            Ok(vec![
                (Text("Hello "), 0..6),
                (ExpressionStart, 6..7),
                (var("name"), 7..12),
                (ExpressionEnd, 12..13),
                (Text("!"), 13..14),
            ])
        );
    }

    #[test]
    fn adjacent_expressions() {
        assert_eq!(
            parse("{$a}{|b|}"),
            Ok(vec![
                (ExpressionStart, 0..1),
                (var("a"), 1..3),
                (ExpressionEnd, 3..4),
                (ExpressionStart, 4..5),
                (lit("b"), 5..8),
                (ExpressionEnd, 8..9),
            ])
        );
    }

    #[test]
    fn escapes_are_separate_text_events() {
        assert_eq!(
            parse("a\\\\b\\{c\\|d\\}"),
            Ok(vec![
                (Text("a"), 0..1),
                (Text("\\"), 1..3),
                (Text("b"), 3..4),
                (Text("{"), 4..6),
                (Text("c"), 6..7),
                (Text("|"), 7..9),
                (Text("d"), 9..10),
                (Text("}"), 10..12),
            ])
        );
    }

    #[test]
    fn invalid_escape() {
        assert_eq!(parse("\\a"), error(1, UnexpectedChar('a')));
    }

    #[test]
    fn trailing_backslash() {
        assert_eq!(parse("ab\\"), error(3, Eof));
    }

    #[test]
    fn pipe_in_text_is_text() {
        assert_eq!(parse("a|b|"), Ok(vec![(Text("a|b|"), 0..4)]));
    }

    #[test]
    fn non_ascii_text_and_name() {
        assert_eq!(
            parse("こんにちは {$名前}"),
            Ok(vec![
                (Text("こんにちは "), 0..16),
                (ExpressionStart, 16..17),
                (var("名前"), 17..24),
                (ExpressionEnd, 24..25),
            ])
        );
    }

    #[test]
    fn whitespace_and_bidi_inside_braces() {
        assert_eq!(
            parse("{ \u{2066}$x\u{2069}\t}"),
            Ok(vec![
                (ExpressionStart, 0..1),
                (var("x"), 5..10),
                (ExpressionEnd, 11..12),
            ])
        );
        assert_eq!(
            parse("{\u{3000}|a|\r\n}"),
            Ok(vec![
                (ExpressionStart, 0..1),
                (lit("a"), 4..7),
                (ExpressionEnd, 9..10),
            ])
        );
    }

    #[test]
    fn bidi_marks_around_name() {
        assert_eq!(
            parse("{$\u{200E}x\u{200F}}"),
            Ok(vec![
                (ExpressionStart, 0..1),
                (var("x"), 1..9),
                (ExpressionEnd, 9..10),
            ])
        );
    }

    #[test]
    fn multiple_bidi_marks_after_name() {
        // The name takes one mark; the rest is whitespace before the `}`.
        assert_eq!(
            parse("{$x\u{200E}\u{200F}\u{2069}}"),
            Ok(vec![
                (ExpressionStart, 0..1),
                (var("x"), 1..6),
                (ExpressionEnd, 12..13),
            ])
        );
    }

    #[test]
    fn two_bidi_marks_after_dollar() {
        assert_eq!(
            parse("{$\u{200E}\u{200E}x}"),
            error(5, UnexpectedChar('\u{200E}'))
        );
    }

    #[test]
    fn leading_and_trailing_whitespace_is_text() {
        assert_eq!(
            parse("  hello\u{3000}"),
            Ok(vec![(Text("  hello\u{3000}"), 0..10)])
        );
    }

    #[test]
    fn whitespace_only_message() {
        assert_eq!(parse(" \t"), Ok(vec![(Text(" \t"), 0..2)]));
    }

    #[test]
    fn leading_dot_is_unsupported() {
        assert_eq!(parse(".input {$x}"), error(0, Unsupported));
        assert_eq!(
            parse(" \u{200E}.local $x = {1} {{}}"),
            error(4, Unsupported)
        );
        // Malformed, but past the supported subset: not diagnosed as a syntax error yet.
        assert_eq!(parse(".foo"), error(0, Unsupported));
    }

    #[test]
    fn leading_quoted_pattern_is_unsupported() {
        assert_eq!(parse("{{hi}}"), error(0, Unsupported));
        assert_eq!(parse("\t{{hi}}"), error(1, Unsupported));
    }

    #[test]
    fn dot_after_first_char_is_text() {
        assert_eq!(parse("a.b"), Ok(vec![(Text("a.b"), 0..3)]));
        assert_eq!(
            parse("{$x}."),
            Ok(vec![
                (ExpressionStart, 0..1),
                (var("x"), 1..3),
                (ExpressionEnd, 3..4),
                (Text("."), 4..5),
            ])
        );
    }

    #[test]
    fn double_brace_after_start() {
        assert_eq!(parse("x {{"), error(3, UnexpectedChar('{')));
    }

    #[test]
    fn unclosed_expression() {
        assert_eq!(parse("hi {"), error(4, Eof));
        assert_eq!(parse("{$x"), error(3, Eof));
        assert_eq!(parse("{|a|"), error(4, Eof));
        assert_eq!(parse("{42 "), error(4, Eof));
    }

    #[test]
    fn stray_close_brace() {
        assert_eq!(parse("a}b"), error(1, UnexpectedChar('}')));
    }

    #[test]
    fn empty_expression() {
        assert_eq!(parse("{}"), error(1, UnexpectedChar('}')));
        assert_eq!(parse("{ }"), error(2, UnexpectedChar('}')));
    }

    #[test]
    fn invalid_names() {
        assert_eq!(parse("{$}"), error(2, UnexpectedChar('}')));
        assert_eq!(parse("{$ x}"), error(2, UnexpectedChar(' ')));
        assert_eq!(parse("{$x!}"), error(3, UnexpectedChar('!')));
        assert_eq!(parse("{^}"), error(1, UnexpectedChar('^')));
    }

    #[test]
    fn name_start_boundaries() {
        for name in ["+", "_", "\u{A1}", "\u{10FFFD}"] {
            let source = format!("{{${name}}}");
            assert_eq!(
                parse(&source),
                Ok(expression(var(name), name.len() + 1)),
                "{source:?}"
            );
        }
        for c in ['1', '-', '.', '\u{A0}', '\u{2028}', '\u{FDD0}', '\u{1FFFE}'] {
            let source = format!("{{${c}}}");
            assert_eq!(parse(&source), error(2, UnexpectedChar(c)), "{source:?}");
        }
    }

    #[test]
    fn name_char_continuation() {
        assert_eq!(parse("{$a1.-}"), Ok(expression(var("a1.-"), 5)));
        assert_eq!(parse("{$a\u{FFFE}}"), error(3, UnexpectedChar('\u{FFFE}')));
    }

    #[test]
    fn quoted_literal() {
        assert_eq!(parse("{|hi|}"), Ok(expression(lit("hi"), 4)));
    }

    #[test]
    fn empty_quoted_literal() {
        assert_eq!(parse("{||}"), Ok(expression(lit(""), 2)));
    }

    #[test]
    fn quoted_literal_keeps_syntax_characters_and_whitespace() {
        assert_eq!(
            parse("{|a {b}\n$c :d @e|}"),
            Ok(expression(lit("a {b}\n$c :d @e"), 16))
        );
    }

    #[test]
    fn non_ascii_quoted_literal() {
        assert_eq!(parse("{|名前|}"), Ok(expression(lit("名前"), 8)));
    }

    #[test]
    fn quoted_literal_with_escapes_keeps_its_source() {
        assert_eq!(
            parse("{|a\\\\b\\|c\\{d\\}|}"),
            Ok(expression(lit("a\\\\b\\|c\\{d\\}"), 14))
        );
    }

    #[test]
    fn literal_value_drops_escaping_backslashes() {
        assert_writeable_eq!(
            Literal {
                raw: "a\\\\b\\|c\\{d\\}"
            },
            "a\\b|c{d}"
        );
        assert_writeable_eq!(Literal { raw: "\\\\\\\\" }, "\\\\");
        assert_writeable_eq!(Literal { raw: "\\|" }, "|");
    }

    #[test]
    fn parsed_literal_has_unescaped_value() {
        let mut parser = Parser::new(b"{|a\\|b|}");
        assert_eq!(parser.next(), Some(Ok((ExpressionStart, 0..1))));
        let Some(Ok((Operand(Value::Literal(literal)), span))) = parser.next() else {
            panic!("expected a literal operand");
        };
        assert_eq!(span, 1..7);
        assert_writeable_eq!(literal, "a|b");
    }

    #[test]
    fn literal_value_is_borrowed_without_escapes() {
        assert_eq!(Literal { raw: "a {b}" }.writeable_borrow(), Some("a {b}"));
        assert_eq!(Literal { raw: "" }.writeable_borrow(), Some(""));
        assert_eq!(Literal { raw: "a\\|b" }.writeable_borrow(), None);
    }

    #[test]
    fn invalid_escape_in_quoted_literal() {
        assert_eq!(parse("{|a\\x|}"), error(4, UnexpectedChar('x')));
    }

    #[test]
    fn unclosed_quoted_literal() {
        assert_eq!(parse("{|a"), error(3, Eof));
        assert_eq!(parse("{|a\\"), error(4, Eof));
        // The escaped `|` does not close the literal.
        assert_eq!(parse("{|a\\|}"), error(6, Eof));
    }

    #[test]
    fn nul_in_quoted_literal() {
        assert_eq!(parse("{|a\0|}"), error(3, UnexpectedChar('\0')));
    }

    #[test]
    fn unquoted_literals() {
        for raw in ["42", "-1.5", "a.b-c", "+", "0E-1", "名前", "\u{B7}"] {
            let source = format!("{{{raw}}}");
            assert_eq!(
                parse(&source),
                Ok(expression(lit(raw), raw.len())),
                "{source:?}"
            );
        }
    }

    #[test]
    fn unquoted_literal_ends_at_first_other_char() {
        assert_eq!(parse("{4 2}"), error(3, UnexpectedChar('2')));
        assert_eq!(parse("{42|x|}"), error(3, UnexpectedChar('|')));
        assert_eq!(parse("{a$b}"), error(2, UnexpectedChar('$')));
        assert_eq!(parse("{|a|b}"), error(4, UnexpectedChar('b')));
    }

    #[test]
    fn nul_in_text() {
        assert_eq!(parse("a\0b"), error(1, UnexpectedChar('\0')));
    }

    #[test]
    fn invalid_utf8() {
        for (source, offset) in [
            // Not a UTF-8 byte, in text.
            (&b"a\xFFb"[..], 1),
            // A character cut off by the end of the message.
            (b"ab\xE3\x81", 2),
            // An overlong encoding of `/`.
            (b"\xC0\xAF", 0),
            // An encoded surrogate.
            (b"\xED\xA0\x80", 0),
            // In leading whitespace, inside an expression, in a name and after a backslash.
            (b" \xFF", 1),
            (b"{\xFF$x}", 1),
            (b"{$x\xFF}", 3),
            (b"\\\xFF", 1),
            // In a quoted literal, after a backslash in it, and in an unquoted literal.
            (b"{|a\xFF|}", 3),
            (b"{|\\\xFF|}", 3),
            (b"{4\xFF}", 2),
        ] {
            assert_eq!(
                Parser::new(source).collect::<Result<Events, _>>(),
                error(offset, InvalidUtf8),
                "{source:?}"
            );
        }
    }

    #[test]
    fn unsupported_constructs() {
        for (source, offset) in [
            ("{:fn}", 1),
            ("{$x :fn}", 4),
            ("{$x @a}", 4),
            ("{|a| :fn}", 5),
            ("{42 @a}", 4),
            ("{#b}", 1),
            ("{/b}", 1),
        ] {
            assert_eq!(parse(source), error(offset, Unsupported), "{source:?}");
        }
    }

    #[test]
    fn bidi_only_is_not_required_whitespace() {
        assert_eq!(parse("{$x:fn}"), error(3, UnexpectedChar(':')));
        assert_eq!(parse("{$x\u{200E}:fn}"), error(6, UnexpectedChar(':')));
        assert_eq!(parse("{$x\u{200E}@a}"), error(6, UnexpectedChar('@')));
        assert_eq!(parse("{|a|\u{200E}:fn}"), error(7, UnexpectedChar(':')));
    }

    #[test]
    fn error_after_valid_prefix() {
        let mut parser = Parser::new(b"Hello {$name} }");
        assert_eq!(parser.next(), Some(Ok((Text("Hello "), 0..6))));
        assert_eq!(parser.next(), Some(Ok((ExpressionStart, 6..7))));
        assert_eq!(parser.next(), Some(Ok((var("name"), 7..12))));
        assert_eq!(parser.next(), Some(Ok((ExpressionEnd, 12..13))));
        assert_eq!(parser.next(), Some(Ok((Text(" "), 13..14))));
        assert_eq!(
            parser.next(),
            Some(Err(ParseError {
                offset: 14,
                kind: UnexpectedChar('}')
            }))
        );
        assert_eq!(parser.next(), None);
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn error_inside_expression() {
        let mut parser = Parser::new(b"{$x !}");
        assert_eq!(parser.next(), Some(Ok((ExpressionStart, 0..1))));
        assert_eq!(parser.next(), Some(Ok((var("x"), 1..3))));
        assert_eq!(
            parser.next(),
            Some(Err(ParseError {
                offset: 4,
                kind: UnexpectedChar('!')
            }))
        );
        assert_eq!(parser.next(), None);
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn none_after_end() {
        let mut parser = Parser::new(b"a");
        assert_eq!(parser.next(), Some(Ok((Text("a"), 0..1))));
        assert_eq!(parser.next(), None);
        assert_eq!(parser.next(), None);
    }

    // Fixtures from unicode-org/message-format-wg at tag LDML48.2,
    // commit 7f142fb4f1f5ea6ab1eb34ce2b87e918ca9fd331, test/tests/syntax.json and bidi.json.
    // Only fixtures within the supported subset are included.
    #[test]
    fn conformance_valid() {
        assert_eq!(parse(""), Ok(vec![]));
        assert_eq!(parse("a"), Ok(vec![(Text("a"), 0..1)]));
        assert_eq!(parse("hello"), Ok(vec![(Text("hello"), 0..5)]));
        assert_eq!(parse("\\\\"), Ok(vec![(Text("\\"), 0..2)]));
        assert_eq!(
            parse("\u{A} hello\u{9}"),
            Ok(vec![(Text("\n hello\t"), 0..8)])
        );
        assert_eq!(
            parse("hello {world}"),
            Ok(vec![
                (Text("hello "), 0..6),
                (ExpressionStart, 6..7),
                (lit("world"), 7..12),
                (ExpressionEnd, 12..13),
            ])
        );
        assert_eq!(
            parse("hello {|world|}"),
            Ok(vec![
                (Text("hello "), 0..6),
                (ExpressionStart, 6..7),
                (lit("world"), 7..14),
                (ExpressionEnd, 14..15),
            ])
        );
        assert_eq!(
            parse("hello {$place}"),
            Ok(vec![
                (Text("hello "), 0..6),
                (ExpressionStart, 6..7),
                (var("place"), 7..13),
                (ExpressionEnd, 13..14),
            ])
        );
        assert_eq!(
            parse("hello {$place-.}"),
            Ok(vec![
                (Text("hello "), 0..6),
                (ExpressionStart, 6..7),
                (var("place-."), 7..15),
                (ExpressionEnd, 15..16),
            ])
        );
        assert_eq!(parse("{$x}"), Ok(expression(var("x"), 2)));
        assert_eq!(
            parse("{ $x}"),
            Ok(vec![
                (ExpressionStart, 0..1),
                (var("x"), 2..4),
                (ExpressionEnd, 4..5),
            ])
        );
        assert_eq!(
            parse("{$x }"),
            Ok(vec![
                (ExpressionStart, 0..1),
                (var("x"), 1..3),
                (ExpressionEnd, 4..5),
            ])
        );
        for raw in ["a", "0", "-0", "11", "0.12", "0e1", "0E-1", "+", "-", "🥔"] {
            let source = format!("{{{raw}}}");
            assert_eq!(
                parse(&source),
                Ok(expression(lit(raw), raw.len())),
                "{source:?}"
            );
        }
        assert_eq!(
            parse("hello { world\t\n}"),
            Ok(vec![
                (Text("hello "), 0..6),
                (ExpressionStart, 6..7),
                (lit("world"), 8..13),
                (ExpressionEnd, 15..16),
            ])
        );
        assert_eq!(parse("{||}"), Ok(expression(lit(""), 2)));
        assert_eq!(parse("{|a|}"), Ok(expression(lit("a"), 3)));
        assert_eq!(parse("{|\\\\|}"), Ok(expression(lit("\\\\"), 4)));
        assert_eq!(
            parse("{|a\\\\\\{\\|\\}|}"),
            Ok(expression(lit("a\\\\\\{\\|\\}"), 11))
        );
        assert_eq!(
            parse("{$one} and {$two}"),
            Ok(vec![
                (ExpressionStart, 0..1),
                (var("one"), 1..5),
                (ExpressionEnd, 5..6),
                (Text(" and "), 6..11),
                (ExpressionStart, 11..12),
                (var("two"), 12..16),
                (ExpressionEnd, 16..17),
            ])
        );
        assert_eq!(
            parse("  \u{61C} Hello world!"),
            Ok(vec![(Text("  \u{61C} Hello world!"), 0..17)])
        );
        assert_eq!(
            parse("{\u{200E} hello \u{200F}}"),
            Ok(vec![
                (ExpressionStart, 0..1),
                (lit("hello"), 5..10),
                (ExpressionEnd, 14..15),
            ])
        );
    }

    // Fixtures from unicode-org/message-format-wg at tag LDML48.2,
    // commit 7f142fb4f1f5ea6ab1eb34ce2b87e918ca9fd331, test/tests/syntax-errors.json.
    // Only fixtures whose error lies within the supported subset are included,
    // so each one is diagnosed as a syntax error rather than `Unsupported`.
    #[test]
    fn conformance_syntax_errors() {
        for (source, offset, kind) in [
            ("{", 1, Eof),
            ("}", 0, UnexpectedChar('}')),
            ("{}", 1, UnexpectedChar('}')),
            ("{|foo| #markup}", 7, UnexpectedChar('#')),
            ("empty { } placeholder", 8, UnexpectedChar('}')),
            ("missing space {42:func}", 17, UnexpectedChar(':')),
            ("missing space {|foo|:func}", 20, UnexpectedChar(':')),
            ("missing space {|foo|@bar}", 20, UnexpectedChar('@')),
            ("missing space {+reserved@bar}", 24, UnexpectedChar('@')),
            ("missing space {&private@bar}", 15, UnexpectedChar('&')),
            ("bad {\\u0000placeholder}", 5, UnexpectedChar('\\')),
            ("bad {$placeholder option}", 18, UnexpectedChar('o')),
            ("no {placeholder end", 16, UnexpectedChar('e')),
            ("no {$placeholder end", 17, UnexpectedChar('e')),
            ("no {|placeholder| end", 18, UnexpectedChar('e')),
            ("no {|literal} end", 17, Eof),
            ("no {|literal or placeholder end", 31, Eof),
            ("{^}", 1, UnexpectedChar('^')),
            ("{!}", 1, UnexpectedChar('!')),
            ("{%}", 1, UnexpectedChar('%')),
            ("{*}", 1, UnexpectedChar('*')),
            ("{<}", 1, UnexpectedChar('<')),
            ("{>}", 1, UnexpectedChar('>')),
            ("{?}", 1, UnexpectedChar('?')),
            ("{~}", 1, UnexpectedChar('~')),
            ("{&}", 1, UnexpectedChar('&')),
            ("{\u{FDD0}}", 1, UnexpectedChar('\u{FDD0}')),
            ("{\u{FFFE}}", 1, UnexpectedChar('\u{FFFE}')),
            ("{!|a|}", 1, UnexpectedChar('!')),
            ("foo {&private}", 5, UnexpectedChar('&')),
            ("hello {|4.2| %number}", 13, UnexpectedChar('%')),
            ("hello {|foo| !number}", 13, UnexpectedChar('!')),
            ("hello {?number}", 7, UnexpectedChar('?')),
            ("hello {$foo ~xyzz }", 12, UnexpectedChar('~')),
            ("hello {$x   <xyzz   }", 12, UnexpectedChar('<')),
            ("{  !xyzz   }", 3, UnexpectedChar('!')),
        ] {
            assert_eq!(parse(source), error(offset, kind), "{source:?}");
        }
    }
}

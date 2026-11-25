// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parser for MF2 simple messages, following `spec/message.abnf` of LDML 48.2.

use core::fmt;
use core::ops::Range;
use utf8_iter::ErrorReportingUtf8Chars;
use writeable::{LengthHint, Writeable};

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
    /// The function of the expression, without its options.
    Function(Identifier<'a>),
    /// An option of the expression's function.
    OptionPair {
        name: Identifier<'a>,
        value: Value<'a>,
    },
    /// An attribute of the expression.
    Attribute {
        name: Identifier<'a>,
        value: Option<Literal<'a>>,
    },
    /// The `}` that closes an expression.
    ExpressionEnd,
}

/// The name of a function, option or attribute. Neither part contains bidi marks.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct Identifier<'a> {
    pub(crate) namespace: Option<&'a str>,
    pub(crate) name: &'a str,
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

    fn writeable_length_hint(&self) -> LengthHint {
        // An escape sequence is two bytes of the source and one byte of the value.
        LengthHint::between(self.raw.len().div_ceil(2), self.raw.len())
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
#[derive(Clone, Copy)]
enum State {
    /// In a pattern, outside of any expression.
    Pattern,
    ExpressionStart,
    Operand,
    /// After a function or one of its options.
    Function,
    Attribute,
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
            State::ExpressionStart => self.parse_expression_start().map(Some),
            State::Operand | State::Function | State::Attribute => {
                self.parse_expression_part().map(Some)
            }
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
    // function-expression = "{" o function *(s attribute) o "}"
    // The `o literal`, `o variable` or `o function` after the "{".
    fn parse_expression_start(&mut self) -> Result<(Event<'a>, Range<usize>), ParseError> {
        self.skip_o()?;
        let start = self.offset();
        let event = match self.peek()? {
            Some(p) if p.c == ':' => {
                p.consume();
                self.parse_function()?
            }
            // The start of markup.
            Some(p) if matches!(p.c, '#' | '/') => return Err(self.unsupported()),
            _ => {
                let operand = self.parse_value()?;
                self.state = State::Operand;
                Event::Operand(operand)
            }
        };
        Ok((event, start..self.offset()))
    }

    // What can follow in an expression:
    // after an operand,             [s function] *(s attribute) o "}"
    // after a function or option,   *(s option) *(s attribute) o "}"
    // after an attribute,           *(s attribute) o "}"
    fn parse_expression_part(&mut self) -> Result<(Event<'a>, Range<usize>), ParseError> {
        let state = self.state;
        // s = *bidi ws o, so bidi marks alone do not separate the parts.
        let saw_ws = self.skip_o()?;
        let start = self.offset();
        let Some(p) = self.peek()? else {
            return Err(self.eof());
        };
        let event = match (p.c, state) {
            ('}', _) => {
                p.consume();
                self.state = State::Pattern;
                Event::ExpressionEnd
            }
            (':', State::Operand) if saw_ws => {
                p.consume();
                self.parse_function()?
            }
            ('@', _) if saw_ws => {
                p.consume();
                self.parse_attribute()?
            }
            (_, State::Function) if saw_ws => self.parse_option()?,
            _ => return Err(p.unexpected()),
        };
        Ok((event, start..self.offset()))
    }

    // function = ":" identifier *(s option)
    // The ":" has been consumed. The options are yielded as separate events.
    fn parse_function(&mut self) -> Result<Event<'a>, ParseError> {
        let name = self.parse_identifier()?;
        self.state = State::Function;
        Ok(Event::Function(name))
    }

    // option = identifier o "=" o (literal / variable)
    fn parse_option(&mut self) -> Result<Event<'a>, ParseError> {
        let name = self.parse_identifier()?;
        self.skip_o()?;
        match self.peek()? {
            Some(p) if p.c == '=' => p.consume(),
            Some(p) => return Err(p.unexpected()),
            None => return Err(self.eof()),
        }
        self.skip_o()?;
        let value = self.parse_value()?;
        Ok(Event::OptionPair { name, value })
    }

    // attribute = "@" identifier [o "=" o literal]
    // The "@" has been consumed.
    fn parse_attribute(&mut self) -> Result<Event<'a>, ParseError> {
        let name = self.parse_identifier()?;
        // Without a "=", the whitespace after the identifier is not part of the
        // attribute: it may be the `s` that the next attribute requires.
        let after_name = self.rest;
        self.skip_o()?;
        let value = match self.peek()? {
            Some(p) if p.c == '=' => {
                p.consume();
                self.skip_o()?;
                Some(self.parse_literal()?)
            }
            _ => {
                self.rest = after_name;
                None
            }
        };
        self.state = State::Attribute;
        Ok(Event::Attribute { name, value })
    }

    // identifier = [namespace ":"] name
    // namespace  = name
    fn parse_identifier(&mut self) -> Result<Identifier<'a>, ParseError> {
        let name = self.parse_name()?;
        match self.peek()? {
            Some(p) if p.c == ':' => {
                p.consume();
                Ok(Identifier {
                    namespace: Some(name),
                    name: self.parse_name()?,
                })
            }
            _ => Ok(Identifier {
                namespace: None,
                name,
            }),
        }
    }

    // literal / variable
    fn parse_value(&mut self) -> Result<Value<'a>, ParseError> {
        match self.peek()? {
            Some(p) if p.c == '$' => {
                p.consume();
                Ok(Value::Variable(self.parse_name()?))
            }
            _ => Ok(Value::Literal(self.parse_literal()?)),
        }
    }

    // literal = quoted-literal / unquoted-literal
    fn parse_literal(&mut self) -> Result<Literal<'a>, ParseError> {
        match self.peek()? {
            Some(p) if p.c == '|' => {
                p.consume();
                self.parse_quoted_literal()
            }
            _ => self.parse_unquoted_literal(),
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
        Operand(literal(raw))
    }

    fn literal(raw: &str) -> Value<'_> {
        Value::Literal(Literal { raw })
    }

    /// An identifier written as `name` or `namespace:name`.
    fn id(identifier: &str) -> Identifier<'_> {
        match identifier.split_once(':') {
            Some((namespace, name)) => Identifier {
                namespace: Some(namespace),
                name,
            },
            None => Identifier {
                namespace: None,
                name: identifier,
            },
        }
    }

    fn function(name: &str) -> Event<'_> {
        Function(id(name))
    }

    fn option<'a>(name: &'a str, value: Value<'a>) -> Event<'a> {
        let name = id(name);
        OptionPair { name, value }
    }

    fn attribute<'a>(name: &'a str, value: Option<&'a str>) -> Event<'a> {
        let name = id(name);
        let value = value.map(|raw| Literal { raw });
        Attribute { name, value }
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

    /// The events of a message that is a single expression,
    /// with the given parts between the `{` and the `}` at offset `close`.
    fn expression_of(parts: Events<'_>, close: usize) -> Events<'_> {
        let mut events = vec![(ExpressionStart, 0..1)];
        events.extend(parts);
        events.push((ExpressionEnd, close..close + 1));
        events
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
        assert_eq!(parse("{:f"), error(3, Eof));
        assert_eq!(parse("{$x :f k=v"), error(10, Eof));
        assert_eq!(parse("{$x @a "), error(7, Eof));
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
    fn quoted_literal_keeps_bidi_marks() {
        // Unlike around a name, bidi marks inside the `|` are part of the value.
        assert_eq!(
            parse("{|\u{200E}a\u{200F}|}"),
            Ok(expression(lit("\u{200E}a\u{200F}"), 9))
        );
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
    fn literal_value_with_multi_byte_text_and_escapes() {
        assert_writeable_eq!(Literal { raw: "名前\\|a" }, "名前|a");
        assert_writeable_eq!(Literal { raw: "\\{名\\}" }, "{名}");
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
            // In an identifier, in an option value and after an attribute.
            (b"{:f\xFF}", 3),
            (b"{:f k=\xFF}", 6),
            (b"{$x @a \xFF}", 7),
        ] {
            assert_eq!(
                Parser::new(source).collect::<Result<Events, _>>(),
                error(offset, InvalidUtf8),
                "{source:?}"
            );
        }
    }

    #[test]
    fn function_after_quoted_literal() {
        assert_eq!(
            parse("{|a| :f}"),
            Ok(expression_of(
                vec![(lit("a"), 1..4), (function("f"), 5..7)],
                7
            ))
        );
    }

    #[test]
    fn bidi_marks_in_identifier() {
        assert_eq!(
            parse("{:\u{200E}ns\u{200F}:\u{200E}f\u{200F}}"),
            Ok(expression_of(vec![(function("ns:f"), 1..18)], 18))
        );
    }

    #[test]
    fn non_ascii_identifiers() {
        assert_eq!(
            parse("{:名前 キー=値}"),
            Ok(expression_of(
                vec![
                    (function("名前"), 1..8),
                    (option("キー", literal("値")), 9..19)
                ],
                19
            ))
        );
    }

    #[test]
    fn invalid_identifiers() {
        assert_eq!(parse("{:}"), error(2, UnexpectedChar('}')));
        assert_eq!(parse("{: f}"), error(2, UnexpectedChar(' ')));
        assert_eq!(parse("{:1}"), error(2, UnexpectedChar('1')));
        assert_eq!(parse("{::f}"), error(2, UnexpectedChar(':')));
        assert_eq!(parse("{:f:}"), error(4, UnexpectedChar('}')));
        assert_eq!(parse("{:f::g}"), error(4, UnexpectedChar(':')));
        assert_eq!(parse("{:a:b:c}"), error(5, UnexpectedChar(':')));
    }

    #[test]
    fn second_function() {
        assert_eq!(parse("{:f :g}"), error(4, UnexpectedChar(':')));
        assert_eq!(parse("{$x :f :g}"), error(7, UnexpectedChar(':')));
    }

    #[test]
    fn option_with_quoted_literal_or_variable() {
        let options = vec![
            (function("f"), 1..3),
            (option("a", literal("b c")), 4..11),
            (option("d", Value::Variable("x")), 12..16),
        ];
        assert_eq!(parse("{:f a=|b c| d=$x}"), Ok(expression_of(options, 16)));
    }

    #[test]
    fn whitespace_and_bidi_around_option_equals() {
        assert_eq!(
            parse("{:f k \u{200E}=\t v}"),
            Ok(expression_of(
                vec![(function("f"), 1..3), (option("k", literal("v")), 4..13)],
                13
            ))
        );
    }

    #[test]
    fn option_requires_whitespace() {
        assert_eq!(parse("{:f k=|a|l=w}"), error(9, UnexpectedChar('l')));
        assert_eq!(parse("{:f\u{200E}k=v}"), error(6, UnexpectedChar('k')));
    }

    #[test]
    fn option_without_equals_or_value() {
        assert_eq!(parse("{:f k}"), error(5, UnexpectedChar('}')));
        assert_eq!(parse("{:f k v}"), error(6, UnexpectedChar('v')));
        assert_eq!(parse("{:f k=}"), error(6, UnexpectedChar('}')));
        assert_eq!(parse("{:f k"), error(5, Eof));
        assert_eq!(parse("{:f k="), error(6, Eof));
    }

    #[test]
    fn option_without_function() {
        assert_eq!(parse("{$x k=v}"), error(4, UnexpectedChar('k')));
        assert_eq!(parse("{42 k=v}"), error(4, UnexpectedChar('k')));
    }

    #[test]
    fn whitespace_and_bidi_around_attribute_equals() {
        assert_eq!(
            parse("{$x @a \u{200E}=\t 1}"),
            Ok(expression_of(
                vec![(var("x"), 1..3), (attribute("a", Some("1")), 4..14)],
                14
            ))
        );
    }

    #[test]
    fn whitespace_after_attribute_without_value_is_not_in_its_span() {
        assert_eq!(
            parse("{$x @a \u{200E}}"),
            Ok(expression_of(
                vec![(var("x"), 1..3), (attribute("a", None), 4..6)],
                10
            ))
        );
    }

    #[test]
    fn attributes_after_function_and_option() {
        let parts = vec![
            (var("x"), 1..3),
            (function("f"), 4..6),
            (option("k", literal("v")), 7..10),
            (attribute("a", None), 11..13),
            (attribute("ns:b", Some("1")), 14..21),
        ];
        assert_eq!(
            parse("{$x :f k=v @a @ns:b=1}"),
            Ok(expression_of(parts, 21))
        );
    }

    #[test]
    fn duplicate_attributes_are_yielded() {
        let parts = vec![
            (function("f"), 1..3),
            (attribute("a", None), 4..6),
            (attribute("a", Some("1")), 7..11),
        ];
        assert_eq!(parse("{:f @a @a=1}"), Ok(expression_of(parts, 11)));
    }

    #[test]
    fn attribute_value_must_be_a_literal() {
        assert_eq!(parse("{$x @a=$y}"), error(7, UnexpectedChar('$')));
        assert_eq!(parse("{$x @a=@b}"), error(7, UnexpectedChar('@')));
        assert_eq!(parse("{$x @a=}"), error(7, UnexpectedChar('}')));
        assert_eq!(parse("{$x @a="), error(7, Eof));
    }

    #[test]
    fn attribute_requires_whitespace() {
        assert_eq!(parse("{$x@a}"), error(3, UnexpectedChar('@')));
        assert_eq!(parse("{:f@a}"), error(3, UnexpectedChar('@')));
        assert_eq!(parse("{$x @a@b}"), error(6, UnexpectedChar('@')));
        assert_eq!(parse("{$x @a=1@b}"), error(8, UnexpectedChar('@')));
    }

    #[test]
    fn only_attributes_after_an_attribute() {
        assert_eq!(parse("{:f @a k=v}"), error(7, UnexpectedChar('k')));
        assert_eq!(parse("{$x @a :f}"), error(7, UnexpectedChar(':')));
    }

    #[test]
    fn attribute_without_operand_or_function() {
        assert_eq!(parse("{@a}"), error(1, UnexpectedChar('@')));
        assert_eq!(parse("{ @a }"), error(2, UnexpectedChar('@')));
    }

    #[test]
    fn markup_is_unsupported() {
        for (source, offset) in [("{#b}", 1), ("{/b}", 1), ("{ #b}", 2)] {
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
    // commit 7f142fb4f1f5ea6ab1eb34ce2b87e918ca9fd331, test/tests/syntax.json and bidi.json.
    #[test]
    fn conformance_valid_expressions() {
        #[rustfmt::skip]
        let fixtures = [
            ("{a :f}", vec![(lit("a"), 1..2), (function("f"), 3..5)], 5),
            ("{a :u:f}", vec![(lit("a"), 1..2), (function("u:f"), 3..7)], 7),
            ("{$x :f}", vec![(var("x"), 1..3), (function("f"), 4..6)], 6),
            ("{$x :u:f}", vec![(var("x"), 1..3), (function("u:f"), 4..8)], 8),
            ("{:u:f}", vec![(function("u:f"), 1..5)], 5),
            ("{:f}", vec![(function("f"), 1..3)], 3),
            ("{ :f}", vec![(function("f"), 2..4)], 4),
            ("{:f }", vec![(function("f"), 1..3)], 4),
            ("{a @c}", vec![(lit("a"), 1..2), (attribute("c", None), 3..5)], 5),
            ("{$x @c}", vec![(var("x"), 1..3), (attribute("c", None), 4..6)], 6),
            ("{:f @c}", vec![(function("f"), 1..3), (attribute("c", None), 4..6)], 6),
            ("{:f k=v}", vec![(function("f"), 1..3), (option("k", literal("v")), 4..7)], 7),
            ("{:f k =v}", vec![(function("f"), 1..3), (option("k", literal("v")), 4..8)], 8),
            ("{:f k= v}", vec![(function("f"), 1..3), (option("k", literal("v")), 4..8)], 8),
            ("{:f k = v}", vec![(function("f"), 1..3), (option("k", literal("v")), 4..9)], 9),
            ("{a @c=d}", vec![(lit("a"), 1..2), (attribute("c", Some("d")), 3..7)], 7),
            ("{a @c =d}", vec![(lit("a"), 1..2), (attribute("c", Some("d")), 3..8)], 8),
            ("{a @c= d}", vec![(lit("a"), 1..2), (attribute("c", Some("d")), 3..8)], 8),
            ("{a @c = d}", vec![(lit("a"), 1..2), (attribute("c", Some("d")), 3..9)], 9),
            (
                "{42 @foo=|bar|}",
                vec![(lit("42"), 1..3), (attribute("foo", Some("bar")), 4..14)],
                14,
            ),
            (
                "{42 @foo @bar=13}",
                vec![
                    (lit("42"), 1..3),
                    (attribute("foo", None), 4..8),
                    (attribute("bar", Some("13")), 9..16),
                ],
                16,
            ),
            (
                "{1 \u{200E} :number \u{200F}}",
                vec![(lit("1"), 1..2), (function("number"), 7..14)],
                18,
            ),
            (
                "{1 :number minimumFractionDigits\u{200F}=\u{200E}1 }",
                vec![
                    (lit("1"), 1..2),
                    (function("number"), 3..10),
                    (option("minimumFractionDigits", literal("1")), 11..40),
                ],
                41,
            ),
            (
                "{1 :number @locale\u{200F}=\u{200E}en }",
                vec![
                    (lit("1"), 1..2),
                    (function("number"), 3..10),
                    (attribute("locale", Some("en")), 11..27),
                ],
                28,
            ),
        ];
        for (source, parts, close) in fixtures {
            assert_eq!(parse(source), Ok(expression_of(parts, close)), "{source:?}");
        }
    }

    // A fixture from test/tests/data-model-errors.json at the same commit.
    // Duplicate option names are for the data model to reject.
    #[test]
    fn duplicate_option_names_are_not_a_syntax_error() {
        assert_eq!(
            parse("bad {:placeholder option=x option=x}"),
            Ok(vec![
                (Text("bad "), 0..4),
                (ExpressionStart, 4..5),
                (function("placeholder"), 5..17),
                (option("option", literal("x")), 18..26),
                (option("option", literal("x")), 27..35),
                (ExpressionEnd, 35..36),
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
            ("missing space {:func@bar}", 20, UnexpectedChar('@')),
            ("missing space {:func @bar@baz}", 25, UnexpectedChar('@')),
            ("missing space {:func @bar=42@baz}", 28, UnexpectedChar('@')),
            ("bad {:} placeholder", 6, UnexpectedChar('}')),
            (
                "no-equal {|42| :number minimumFractionDigits 2}",
                45,
                UnexpectedChar('2'),
            ),
            ("bad {:placeholder option=}", 25, UnexpectedChar('}')),
            ("bad {:placeholder option value}", 25, UnexpectedChar('v')),
            ("bad {:placeholder option:value}", 30, UnexpectedChar('}')),
            ("bad {:placeholder option}", 24, UnexpectedChar('}')),
            ("bad {:placeholder:}", 18, UnexpectedChar('}')),
            ("bad {::placeholder}", 6, UnexpectedChar(':')),
            ("bad {:placeholder::foo}", 18, UnexpectedChar(':')),
            ("bad {:placeholder option:=x}", 25, UnexpectedChar('=')),
            ("bad {:placeholder :option=x}", 18, UnexpectedChar(':')),
            ("bad {:placeholder option::x=y}", 25, UnexpectedChar(':')),
            ("bad {:placeholder @attribute=}", 29, UnexpectedChar('}')),
            (
                "bad {:placeholder @attribute=@foo}",
                29,
                UnexpectedChar('@'),
            ),
            (
                "bad {:placeholder @attribute=$foo}",
                29,
                UnexpectedChar('$'),
            ),
            ("{ @misplaced = attribute }", 2, UnexpectedChar('@')),
            ("no {:placeholder end", 20, Eof),
            ("foo {?reserved @a @b=c}", 5, UnexpectedChar('?')),
        ] {
            assert_eq!(parse(source), error(offset, kind), "{source:?}");
        }
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parser for MF2 simple messages, following `spec/message.abnf` of LDML 48.2.

use core::ops::Range;

/// A part of a message, borrowed from its source.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Event<'a> {
    /// Text to output verbatim.
    Text(&'a str),
    /// A `{$name}` placeholder. The name excludes `$` and bidi marks.
    Variable(&'a str),
}

/// An error found while parsing a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ParseError {
    /// Byte offset of the offending character, or the source length for [`ParseErrorKind::Eof`].
    pub(crate) offset: usize,
    pub(crate) kind: ParseErrorKind,
}

/// The kind of a [`ParseError`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParseErrorKind {
    /// A character that is not allowed at this position.
    UnexpectedChar(char),
    /// The message ended inside a placeholder or an escape sequence.
    Eof,
    /// Parsing reached syntax outside the supported subset.
    /// The remaining input has not been validated.
    Unsupported,
}

/// Yields the events of a simple message, each with the byte range of the source it came from.
///
/// Events are yielded as the message is read, so an error invalidates the whole message,
/// including events yielded before it. Consumers must iterate to completion to validate
/// the message. After an error or the last event, `next` returns `None`.
pub(crate) struct Parser<'a> {
    source: &'a str,
    /// Byte offset of the next unread character. Always on a char boundary.
    pos: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }

    fn rest(&self) -> &'a str {
        self.source.get(self.pos..).unwrap_or_default()
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn unexpected(&self, c: char) -> ParseError {
        ParseError {
            offset: self.pos,
            kind: ParseErrorKind::UnexpectedChar(c),
        }
    }

    fn eof(&self) -> ParseError {
        ParseError {
            offset: self.pos,
            kind: ParseErrorKind::Eof,
        }
    }

    fn unsupported(&self) -> ParseError {
        ParseError {
            offset: self.pos,
            kind: ParseErrorKind::Unsupported,
        }
    }

    fn next_event(&mut self) -> Result<Option<(Event<'a>, Range<usize>)>, ParseError> {
        if self.pos == 0 {
            self.check_simple_start()?;
        }
        match self.peek() {
            None => Ok(None),
            Some('\\') => self.escape().map(Some),
            Some('{') => self.placeholder().map(Some),
            Some(c @ ('}' | '\0')) => Err(self.unexpected(c)),
            Some(_) => Ok(Some(self.text())),
        }
    }

    // simple-message = o [simple-start pattern]
    // simple-start-char excludes `.`, so a first non-whitespace `.` or `{{`
    // can only begin a complex-message.
    fn check_simple_start(&self) -> Result<(), ParseError> {
        let start = self.source.trim_start_matches(|c| is_ws(c) || is_bidi(c));
        if start.starts_with('.') || start.starts_with("{{") {
            return Err(ParseError {
                offset: self.source.len() - start.len(),
                kind: ParseErrorKind::Unsupported,
            });
        }
        Ok(())
    }

    // A run of text-char.
    fn text(&mut self) -> (Event<'a>, Range<usize>) {
        let start = self.pos;
        let rest = self.rest();
        let len = rest.find(['\\', '{', '}', '\0']).unwrap_or(rest.len());
        self.pos += len;
        (
            Event::Text(rest.get(..len).unwrap_or_default()),
            start..self.pos,
        )
    }

    // escaped-char = backslash ( backslash / "{" / "|" / "}" )
    fn escape(&mut self) -> Result<(Event<'a>, Range<usize>), ParseError> {
        let start = self.pos;
        self.pos += 1;
        match self.peek() {
            Some('\\' | '{' | '|' | '}') => {
                // All four escapable characters are one byte long.
                let text = self.rest().get(..1).unwrap_or_default();
                self.pos += 1;
                Ok((Event::Text(text), start..self.pos))
            }
            Some(c) => Err(self.unexpected(c)),
            None => Err(self.eof()),
        }
    }

    // variable-expression = "{" o variable [s function] *(s attribute) o "}"
    // without function and attributes.
    fn placeholder(&mut self) -> Result<(Event<'a>, Range<usize>), ParseError> {
        let start = self.pos;
        self.pos += 1;
        self.skip_o();
        match self.peek() {
            Some('$') => self.pos += 1,
            // The start of a literal-expression, function-expression or markup.
            Some(c) if matches!(c, '|' | ':' | '#' | '/') || is_name_char(c) => {
                return Err(self.unsupported());
            }
            Some(c) => return Err(self.unexpected(c)),
            None => return Err(self.eof()),
        }
        let name = self.name()?;
        let saw_ws = self.skip_o();
        match self.peek() {
            Some('}') => {
                self.pos += 1;
                Ok((Event::Variable(name), start..self.pos))
            }
            // s = *bidi ws o, so bidi marks alone do not separate a function or attribute.
            Some(':' | '@') if saw_ws => Err(self.unsupported()),
            Some(c) => Err(self.unexpected(c)),
            None => Err(self.eof()),
        }
    }

    // name = [bidi] name-start *name-char [bidi]
    // The trailing bidi mark is left to the `o` that always follows a name here.
    fn name(&mut self) -> Result<&'a str, ParseError> {
        if let Some(c) = self.peek().filter(|&c| is_bidi(c)) {
            self.pos += c.len_utf8();
        }
        let start = self.pos;
        match self.peek() {
            Some(c) if is_name_start(c) => self.pos += c.len_utf8(),
            Some(c) => return Err(self.unexpected(c)),
            None => return Err(self.eof()),
        }
        while let Some(c) = self.peek().filter(|&c| is_name_char(c)) {
            self.pos += c.len_utf8();
        }
        Ok(self.source.get(start..self.pos).unwrap_or_default())
    }

    // o = *(ws / bidi)
    // Returns whether any ws was skipped.
    fn skip_o(&mut self) -> bool {
        let mut saw_ws = false;
        while let Some(c) = self.peek() {
            if is_ws(c) {
                saw_ws = true;
            } else if !is_bidi(c) {
                break;
            }
            self.pos += c.len_utf8();
        }
        saw_ws
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<(Event<'a>, Range<usize>), ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next_event().transpose();
        if let Some(Err(_)) = result {
            self.pos = self.source.len();
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

    type Events<'a> = Vec<(Event<'a>, Range<usize>)>;

    fn parse(source: &str) -> Result<Events<'_>, ParseError> {
        Parser::new(source).collect()
    }

    fn error(offset: usize, kind: ParseErrorKind) -> Result<Events<'static>, ParseError> {
        Err(ParseError { offset, kind })
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
                (Variable("name"), 6..13),
                (Text("!"), 13..14),
            ])
        );
    }

    #[test]
    fn adjacent_placeholders() {
        assert_eq!(
            parse("{$a}{$b}"),
            Ok(vec![(Variable("a"), 0..4), (Variable("b"), 4..8)])
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
    fn non_ascii_text_and_name() {
        assert_eq!(
            parse("こんにちは {$名前}"),
            Ok(vec![
                (Text("こんにちは "), 0..16),
                (Variable("名前"), 16..25)
            ])
        );
    }

    #[test]
    fn whitespace_and_bidi_inside_braces() {
        assert_eq!(
            parse("{ \u{2066}$x\u{2069}\t}"),
            Ok(vec![(Variable("x"), 0..12)])
        );
        assert_eq!(parse("{\u{3000}$x\r\n}"), Ok(vec![(Variable("x"), 0..9)]));
    }

    #[test]
    fn bidi_marks_around_name() {
        assert_eq!(
            parse("{$\u{200E}x\u{200F}}"),
            Ok(vec![(Variable("x"), 0..10)])
        );
    }

    #[test]
    fn multiple_bidi_marks_after_name() {
        assert_eq!(
            parse("{$x\u{200E}\u{200F}\u{2069}}"),
            Ok(vec![(Variable("x"), 0..13)])
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
            Ok(vec![(Variable("x"), 0..4), (Text("."), 4..5)])
        );
    }

    #[test]
    fn double_brace_after_start() {
        assert_eq!(parse("x {{"), error(3, UnexpectedChar('{')));
    }

    #[test]
    fn unclosed_placeholder() {
        assert_eq!(parse("{$x"), error(3, Eof));
        assert_eq!(parse("hi {"), error(4, Eof));
    }

    #[test]
    fn stray_close_brace() {
        assert_eq!(parse("a}b"), error(1, UnexpectedChar('}')));
    }

    #[test]
    fn empty_placeholder() {
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
        assert_eq!(parse("{$+}"), Ok(vec![(Variable("+"), 0..4)]));
        assert_eq!(parse("{$_}"), Ok(vec![(Variable("_"), 0..4)]));
        assert_eq!(parse("{$\u{A1}}"), Ok(vec![(Variable("\u{A1}"), 0..5)]));
        assert_eq!(
            parse("{$\u{10FFFD}}"),
            Ok(vec![(Variable("\u{10FFFD}"), 0..7)])
        );
        for c in ['1', '-', '.', '\u{A0}', '\u{2028}', '\u{FDD0}', '\u{1FFFE}'] {
            let source = format!("{{${c}}}");
            assert_eq!(parse(&source), error(2, UnexpectedChar(c)), "{source:?}");
        }
    }

    #[test]
    fn name_char_continuation() {
        assert_eq!(parse("{$a1.-}"), Ok(vec![(Variable("a1.-"), 0..7)]));
        assert_eq!(parse("{$a\u{FFFE}}"), error(3, UnexpectedChar('\u{FFFE}')));
    }

    #[test]
    fn nul_in_text() {
        assert_eq!(parse("a\0b"), error(1, UnexpectedChar('\0')));
    }

    #[test]
    fn unsupported_constructs() {
        for (source, offset) in [
            ("{a}", 1),
            ("{0}", 1),
            ("{|a|}", 1),
            ("{:fn}", 1),
            ("{$x :fn}", 4),
            ("{$x @a}", 4),
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
    }

    #[test]
    fn error_after_valid_prefix() {
        let mut parser = Parser::new("Hello {$name} }");
        assert_eq!(parser.next(), Some(Ok((Text("Hello "), 0..6))));
        assert_eq!(parser.next(), Some(Ok((Variable("name"), 6..13))));
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
    fn none_after_end() {
        let mut parser = Parser::new("a");
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
            parse("hello {$place}"),
            Ok(vec![(Text("hello "), 0..6), (Variable("place"), 6..14)])
        );
        assert_eq!(
            parse("hello {$place-.}"),
            Ok(vec![(Text("hello "), 0..6), (Variable("place-."), 6..16)])
        );
        assert_eq!(parse("{$x}"), Ok(vec![(Variable("x"), 0..4)]));
        assert_eq!(parse("{ $x}"), Ok(vec![(Variable("x"), 0..5)]));
        assert_eq!(parse("{$x }"), Ok(vec![(Variable("x"), 0..5)]));
        assert_eq!(
            parse("{$one} and {$two}"),
            Ok(vec![
                (Variable("one"), 0..6),
                (Text(" and "), 6..11),
                (Variable("two"), 11..17),
            ])
        );
        assert_eq!(
            parse("  \u{61C} Hello world!"),
            Ok(vec![(Text("  \u{61C} Hello world!"), 0..17)])
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
            ("empty { } placeholder", 8, UnexpectedChar('}')),
            ("bad {\\u0000placeholder}", 5, UnexpectedChar('\\')),
            ("bad {$placeholder option}", 18, UnexpectedChar('o')),
            ("no {$placeholder end", 17, UnexpectedChar('e')),
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
            ("foo {&private}", 5, UnexpectedChar('&')),
            ("hello {?number}", 7, UnexpectedChar('?')),
            ("hello {$foo ~xyzz }", 12, UnexpectedChar('~')),
            ("hello {$x   <xyzz   }", 12, UnexpectedChar('<')),
            ("{  !xyzz   }", 3, UnexpectedChar('!')),
        ] {
            assert_eq!(parse(source), error(offset, kind), "{source:?}");
        }
    }
}

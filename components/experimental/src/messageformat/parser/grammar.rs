// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Recursive-descent grammar for `MessageFormat` 2.
//!
//! Drives the [`super::lexer::Lexer`] via a one-token lookahead cache,
//! producing an [`crate::messageformat::ast::Message`]. All errors carry
//! a precise byte offset from the lexer.
//!
//! The grammar covers the ABNF at `spec/message.abnf`:
//!
//! - Simple messages: `Hello, {$u}!`
//! - Complex messages with `.input` / `.local` declarations, `{{...}}`
//!   quoted-pattern bodies, and `.match` matchers with variants.
//! - Markup elements (open, close, self-closing).
//! - Function annotations with options and attributes.

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::lexer::{LexMode, Lexer, Token};
use crate::messageformat::ast::{
    Arg, AttributeValue, Attributes, CatchallKey, Declaration, Expression, FunctionRef, Literal,
    Markup, MarkupKind, Message, OptionMap, OptionValue, Pattern, PatternElement, True, Variable,
    Variant, VariantKey,
};
use crate::messageformat::error::{ParseError, ValidationError};

/// Normalize a parsed identifier or variable name to NFC.
///
/// Per spec (syntax.md §Names): "Two names are considered equal if they are
/// canonically equivalent strings ... after NFC has been applied." We
/// normalize at parse time so downstream `BTreeMap` lookups (declarations,
/// options) compare correctly.
///
/// When the `compiled_data` feature is off the normalizer isn't available;
/// the spec permits the "as-if normalized" fast path, so we return the input
/// unchanged. In practice almost all source text is already NFC.
fn nfc_name(s: &str) -> Box<str> {
    #[cfg(feature = "compiled_data")]
    {
        use icu_normalizer::ComposingNormalizer;
        let normalizer = ComposingNormalizer::new_nfc();
        normalizer.normalize(s).into_owned().into_boxed_str()
    }
    #[cfg(not(feature = "compiled_data"))]
    {
        s.to_string().into_boxed_str()
    }
}

/// Parse an MF2 source string into a [`Message`].
pub fn parse_message(source: &str) -> Result<Message, ParseError> {
    let mut p = Parser::new(source);
    let msg = p.parse_message_root()?;
    p.finish()?;
    Ok(msg)
}

struct Parser<'src> {
    lexer: Lexer<'src>,
    peeked: Option<Option<Token<'src>>>,
}

#[derive(Copy, Clone)]
enum PatternEnd {
    Eof,
    DoubleCloseBrace,
}

impl<'src> Parser<'src> {
    fn new(src: &'src str) -> Self {
        Self {
            lexer: Lexer::new(src),
            peeked: None,
        }
    }

    // --- token-level helpers ---

    fn peek(&mut self) -> Result<Option<&Token<'src>>, ParseError> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next_token()?);
        }
        Ok(self.peeked.as_ref().and_then(|o| o.as_ref()))
    }

    /// Did whitespace precede the next-emitted token from the lexer?
    ///
    /// Valid only after a successful [`Self::peek`]. Returns `false` at EOF
    /// or when no whitespace was consumed immediately before the token.
    fn last_token_had_leading_whitespace(&self) -> bool {
        self.lexer.last_token_had_leading_whitespace()
    }

    fn bump(&mut self) -> Result<Option<Token<'src>>, ParseError> {
        if let Some(t) = self.peeked.take() {
            return Ok(t);
        }
        self.lexer.next_token()
    }

    fn position(&self) -> usize {
        self.lexer.position()
    }

    fn syntax(&self, message: &'static str) -> ParseError {
        ParseError::Syntax {
            offset: self.position(),
            message,
        }
    }

    /// Skip whitespace at the lexer-character level. Only valid when the peek
    /// cache is empty.
    fn skip_whitespace(&mut self) {
        if self.peeked.is_none() {
            self.lexer.skip_whitespace();
        }
    }

    /// Enforce the ABNF's `s` production: at least one whitespace character
    /// is required here. Errors if none is present.
    fn require_whitespace(&mut self, msg: &'static str) -> Result<(), ParseError> {
        debug_assert!(
            self.peeked.is_none(),
            "require_whitespace called with cached lookahead"
        );
        if !self.lexer.consume_whitespace() {
            return Err(self.syntax(msg));
        }
        Ok(())
    }

    fn try_keyword(&mut self) -> Result<Option<Token<'src>>, ParseError> {
        debug_assert!(
            self.peeked.is_none(),
            "try_keyword called with cached lookahead"
        );
        self.lexer.try_consume_keyword()
    }

    fn expect(&mut self, expected: &Token<'static>, msg: &'static str) -> Result<(), ParseError> {
        match self.bump()? {
            Some(ref t) if t == expected => Ok(()),
            _ => Err(self.syntax(msg)),
        }
    }

    // --- entry points ---

    fn parse_message_root(&mut self) -> Result<Message, ParseError> {
        // Peek past leading optional-whitespace (ABNF `o`) to decide between
        // simple and complex messages without consuming the whitespace — a
        // simple message whose first text-char is a whitespace or bidi control
        // must preserve it in the output pattern (see syntax.json#7).
        let save = self.lexer.position();
        self.lexer.skip_whitespace();
        let is_complex = self.lexer.starts_with_str(".input")
            || self.lexer.starts_with_str(".local")
            || self.lexer.starts_with_str(".match")
            || self.lexer.starts_with_str("{{");

        if is_complex {
            // Keep the whitespace consumed — declarations and quoted-pattern
            // openers tolerate (and expect) prior `o`.
            self.parse_complex_message()
        } else {
            // Restore: simple messages include leading whitespace in the pattern.
            self.lexer.seek(save);
            self.parse_simple_message()
        }
    }

    fn finish(&mut self) -> Result<(), ParseError> {
        // Allow trailing whitespace only after a complex body. For simple
        // messages, the pattern already consumed all input.
        if self.peeked.take().flatten().is_some() {
            return Err(self.syntax("unexpected trailing token"));
        }
        self.lexer.skip_whitespace();
        if !self.lexer.is_at_end() {
            return Err(self.syntax("unexpected trailing input"));
        }
        Ok(())
    }

    // --- simple message ---

    fn parse_simple_message(&mut self) -> Result<Message, ParseError> {
        // ABNF `simple-start = simple-start-char / escaped-char / placeholder`
        // — the first non-whitespace character of a simple-message MUST NOT
        // be `.`, since that is reserved for the keyword prefix of a complex
        // message. Upstream fixture `syntax-errors#0` asserts this behavior.
        if self.lexer.starts_with_str(".") {
            return Err(self.syntax("a simple message must not start with `.`"));
        }
        let pattern = self.parse_pattern_until(PatternEnd::Eof)?;
        Ok(Message::Pattern {
            declarations: Vec::new(),
            pattern,
        })
    }

    fn parse_pattern_until(&mut self, end: PatternEnd) -> Result<Pattern, ParseError> {
        let mut out: Pattern = Vec::new();
        loop {
            let kind = match self.peek()? {
                None => PeekKind::Eof,
                Some(Token::Text(_)) => PeekKind::Text,
                Some(Token::OpenBrace) => PeekKind::OpenBrace,
                Some(Token::DoubleCloseBrace) => PeekKind::DoubleCloseBrace,
                Some(_) => return Err(self.syntax("unexpected token in pattern")),
            };
            match (kind, end) {
                (PeekKind::Eof, PatternEnd::Eof) => return Ok(out),
                (PeekKind::Eof, PatternEnd::DoubleCloseBrace) => {
                    return Err(self.syntax("expected `}}`"));
                }
                (PeekKind::DoubleCloseBrace, PatternEnd::DoubleCloseBrace) => {
                    self.bump()?;
                    return Ok(out);
                }
                (PeekKind::DoubleCloseBrace, PatternEnd::Eof) => {
                    return Err(self.syntax("unexpected `}}`"));
                }
                (PeekKind::Text, _) => {
                    if let Some(Token::Text(s)) = self.bump()? {
                        out.push(PatternElement::Text(s.into_owned()));
                    }
                }
                (PeekKind::OpenBrace, _) => {
                    self.bump()?;
                    out.push(self.parse_after_open_brace()?);
                }
            }
        }
    }

    fn parse_after_open_brace(&mut self) -> Result<PatternElement, ParseError> {
        // OpenBrace has been consumed; lexer is in Expression mode.
        match self.peek()? {
            Some(Token::Hash) | Some(Token::Slash) => {
                Ok(PatternElement::Markup(self.parse_markup_body()?))
            }
            _ => Ok(PatternElement::Expression(self.parse_expression_body()?)),
        }
    }

    // --- expressions ---

    fn parse_expression_body(&mut self) -> Result<Expression, ParseError> {
        let arg = match self.peek()? {
            Some(Token::Dollar) => Some(Arg::Variable(self.parse_variable()?)),
            Some(Token::QuotedLiteral(_))
            | Some(Token::Name(_))
            | Some(Token::NumberLiteral(_)) => Some(Arg::Literal(self.parse_literal()?)),
            Some(Token::Colon) => None,
            _ => return Err(self.syntax("expected literal, variable, or function")),
        };

        let mut function = None;
        if let Some(Token::Colon) = self.peek()? {
            // Spec ABNF `*-expression = ... [s function]` — when an operand
            // is present, required whitespace precedes `:`.
            if arg.is_some() && !self.last_token_had_leading_whitespace() {
                return Err(self.syntax("expected whitespace before `:`"));
            }
            function = Some(self.parse_function_ref()?);
        }

        let attributes = self.parse_attributes()?;

        self.expect(&Token::CloseBrace, "expected `}`")?;

        if arg.is_none() && function.is_none() {
            return Err(self.syntax("expression must have an operand or function"));
        }

        Ok(Expression::new(arg, function, attributes))
    }

    fn parse_markup_body(&mut self) -> Result<Markup, ParseError> {
        let (mut kind, allow_self_close) = match self.bump()? {
            Some(Token::Hash) => (MarkupKind::Open, true),
            Some(Token::Slash) => (MarkupKind::Close, false),
            _ => return Err(self.syntax("expected `#` or `/` in markup")),
        };
        let name = self.parse_identifier()?;
        let options = self.parse_options()?;
        let attributes = self.parse_attributes()?;
        if allow_self_close {
            if let Some(Token::Slash) = self.peek()? {
                self.bump()?;
                kind = MarkupKind::Standalone;
            }
        }
        self.expect(&Token::CloseBrace, "expected `}`")?;
        Ok(Markup::new(kind, name, options, attributes))
    }

    fn parse_function_ref(&mut self) -> Result<FunctionRef, ParseError> {
        self.expect(&Token::Colon, "expected `:`")?;
        let name = self.parse_identifier()?;
        let options = self.parse_options()?;
        Ok(FunctionRef::new(name, options))
    }

    fn parse_options(&mut self) -> Result<OptionMap, ParseError> {
        let mut out = OptionMap::new();
        loop {
            if !matches!(self.peek()?, Some(Token::Name(_))) {
                break;
            }
            // ABNF `function = ":" identifier *(s option)` and
            // `markup-open = "#" identifier *(s option) ...`: each option
            // requires required-whitespace before its name. Lexer
            // greediness covers adjacent unquoted values (e.g. `k=v l=w`
            // needs the space to separate names), but a preceding
            // quoted-literal or variable option value would otherwise let
            // a bare name through: `{:f k=|v|l=w}` must be a syntax error.
            if !self.last_token_had_leading_whitespace() {
                return Err(self.syntax("expected whitespace before option"));
            }
            let name = self.parse_identifier()?;
            self.expect(&Token::Equals, "expected `=`")?;
            let value = self.parse_option_value()?;
            if out.contains_key(&name) {
                return Err(ParseError::DataModel(
                    ValidationError::DuplicateOptionName { name },
                ));
            }
            out.insert(name, value);
        }
        Ok(out)
    }

    fn parse_option_value(&mut self) -> Result<OptionValue, ParseError> {
        match self.peek()? {
            Some(Token::Dollar) => Ok(OptionValue::Variable(self.parse_variable()?)),
            Some(Token::QuotedLiteral(_))
            | Some(Token::Name(_))
            | Some(Token::NumberLiteral(_)) => Ok(OptionValue::Literal(self.parse_literal()?)),
            _ => Err(self.syntax("expected option value")),
        }
    }

    fn parse_attributes(&mut self) -> Result<Attributes, ParseError> {
        let mut out = Attributes::new();
        loop {
            if !matches!(self.peek()?, Some(Token::At)) {
                break;
            }
            // Each attribute requires at least one whitespace char before
            // the leading `@` (ABNF `*(s attribute)`).
            if !self.last_token_had_leading_whitespace() {
                return Err(self.syntax("expected whitespace before `@`"));
            }
            self.bump()?;
            let name = self.parse_identifier()?;
            let value = match self.peek()? {
                Some(Token::Equals) => {
                    self.bump()?;
                    AttributeValue::Literal(self.parse_literal()?)
                }
                _ => AttributeValue::Present(True),
            };
            out.insert(name, value);
        }
        Ok(out)
    }

    // --- leaves ---

    fn parse_identifier(&mut self) -> Result<Box<str>, ParseError> {
        let first = match self.bump()? {
            Some(Token::Name(n)) => n,
            _ => return Err(self.syntax("expected identifier")),
        };
        // Reject identifiers that do not start with a name-start per UAX#31.
        if !starts_with_name_start(first) {
            return Err(self.syntax("identifier must start with a letter or `_`"));
        }
        if let Some(Token::Colon) = self.peek()? {
            self.bump()?;
            let second = match self.bump()? {
                Some(Token::Name(n)) => n,
                _ => return Err(self.syntax("expected name after namespace separator")),
            };
            if !starts_with_name_start(second) {
                return Err(self.syntax("namespaced name must start with a letter or `_`"));
            }
            let mut s = String::with_capacity(first.len() + 1 + second.len());
            s.push_str(first);
            s.push(':');
            s.push_str(second);
            Ok(nfc_name(&s))
        } else {
            Ok(nfc_name(first))
        }
    }

    fn parse_literal(&mut self) -> Result<Literal, ParseError> {
        match self.bump()? {
            Some(Token::QuotedLiteral(s)) => Ok(Literal::new(s.into_owned().into_boxed_str())),
            Some(Token::Name(s)) => Ok(Literal::new(s.to_string().into_boxed_str())),
            Some(Token::NumberLiteral(s)) => Ok(Literal::new(s.to_string().into_boxed_str())),
            _ => Err(self.syntax("expected literal")),
        }
    }

    fn parse_variable(&mut self) -> Result<Variable, ParseError> {
        self.expect(&Token::Dollar, "expected `$`")?;
        match self.bump()? {
            Some(Token::Name(n)) => {
                if !starts_with_name_start(n) {
                    return Err(self.syntax("variable name must start with a letter or `_`"));
                }
                Ok(Variable::new(nfc_name(n)))
            }
            _ => Err(self.syntax("expected variable name")),
        }
    }

    // --- complex messages ---

    fn parse_complex_message(&mut self) -> Result<Message, ParseError> {
        let mut declarations = Vec::new();

        loop {
            self.skip_whitespace();
            let kw = self.try_keyword()?;
            match kw {
                Some(Token::InputKeyword) => {
                    declarations.push(self.parse_input_declaration()?);
                }
                Some(Token::LocalKeyword) => {
                    declarations.push(self.parse_local_declaration()?);
                }
                Some(Token::MatchKeyword) => {
                    return self.parse_matcher(declarations);
                }
                Some(_) => return Err(self.syntax("internal: unexpected keyword token")),
                None => break,
            }
        }

        self.skip_whitespace();
        let pattern = self.parse_quoted_pattern()?;
        Ok(Message::Pattern {
            declarations,
            pattern,
        })
    }

    fn parse_input_declaration(&mut self) -> Result<Declaration, ParseError> {
        // Spec: `input-declaration = input o variable-expression` — the
        // whitespace between `.input` and the opening brace is OPTIONAL.
        self.skip_whitespace();
        self.expect(&Token::OpenBrace, "expected `{` after `.input`")?;
        // Lexer now in Expression mode.
        if !matches!(self.peek()?, Some(Token::Dollar)) {
            return Err(self.syntax(".input value must be a variable expression"));
        }
        let var = self.parse_variable()?;
        let function = match self.peek()? {
            Some(Token::Colon) => Some(self.parse_function_ref()?),
            _ => None,
        };
        let attributes = self.parse_attributes()?;
        self.expect(&Token::CloseBrace, "expected `}`")?;
        Ok(Declaration::Input {
            name: var.name.clone(),
            value: Expression::new(Some(Arg::Variable(var)), function, attributes),
        })
    }

    fn parse_local_declaration(&mut self) -> Result<Declaration, ParseError> {
        self.require_whitespace("`.local` must be followed by whitespace")?;
        self.lexer.push_mode(LexMode::Structural);
        let var = self.parse_variable()?;
        self.expect(&Token::Equals, "expected `=` in `.local`")?;
        self.expect(&Token::OpenBrace, "expected `{` in `.local` value")?;
        // Lexer now in Expression (pushed atop Structural).
        let expr = self.parse_expression_body()?;
        // Expression body consumes `}` which pops Expression. Pop Structural.
        self.lexer.pop_mode();
        Ok(Declaration::Local {
            name: var.name,
            value: expr,
        })
    }

    fn parse_quoted_pattern(&mut self) -> Result<Pattern, ParseError> {
        self.expect(&Token::DoubleOpenBrace, "expected `{{`")?;
        self.parse_pattern_until(PatternEnd::DoubleCloseBrace)
    }

    fn parse_matcher(&mut self, declarations: Vec<Declaration>) -> Result<Message, ParseError> {
        self.require_whitespace("`.match` must be followed by whitespace")?;
        self.lexer.push_mode(LexMode::Structural);

        let mut selectors = Vec::new();
        let mut first = true;
        while matches!(self.peek()?, Some(Token::Dollar)) {
            // Between selectors, the ABNF requires whitespace
            // (`match-statement s variant *(o variant)` — but within
            // `1*(s selector)` each selector after the first needs `s`).
            if !first && !self.last_token_had_leading_whitespace() {
                return Err(self.syntax("expected whitespace between selectors"));
            }
            selectors.push(self.parse_variable()?);
            first = false;
        }
        if selectors.is_empty() {
            return Err(self.syntax("`.match` requires at least one selector"));
        }

        let mut variants = Vec::new();
        let mut first_variant = true;
        while self.peek()?.is_some() {
            // ABNF `matcher = match-statement s variant *(o variant)`:
            // the FIRST variant requires `s` (whitespace) after the
            // selector list; subsequent variants only need `o` (optional).
            if first_variant && !self.last_token_had_leading_whitespace() {
                return Err(self.syntax("expected whitespace before first variant"));
            }
            variants.push(self.parse_variant()?);
            first_variant = false;
        }
        if variants.is_empty() {
            return Err(self.syntax("`.match` requires at least one variant"));
        }

        self.lexer.pop_mode();

        Ok(Message::Select {
            declarations,
            selectors,
            variants,
        })
    }

    fn parse_variant(&mut self) -> Result<Variant, ParseError> {
        let mut keys = Vec::new();
        let mut first = true;
        loop {
            match self.peek()? {
                Some(Token::Star) => {
                    if !first && !self.last_token_had_leading_whitespace() {
                        return Err(self.syntax("expected whitespace between keys"));
                    }
                    self.bump()?;
                    keys.push(VariantKey::Catchall(CatchallKey::default()));
                }
                Some(Token::QuotedLiteral(_))
                | Some(Token::Name(_))
                | Some(Token::NumberLiteral(_)) => {
                    if !first && !self.last_token_had_leading_whitespace() {
                        return Err(self.syntax("expected whitespace between keys"));
                    }
                    keys.push(VariantKey::Literal(self.parse_literal()?));
                }
                Some(Token::DoubleOpenBrace) => break,
                _ => return Err(self.syntax("expected variant key or `{{`")),
            }
            first = false;
        }
        if keys.is_empty() {
            return Err(self.syntax("variant requires at least one key"));
        }
        let pattern = self.parse_quoted_pattern()?;
        Ok(Variant {
            keys,
            value: pattern,
        })
    }
}

enum PeekKind {
    Eof,
    Text,
    OpenBrace,
    DoubleCloseBrace,
}

fn starts_with_name_start(s: &str) -> bool {
    let Some(c) = s.chars().next() else {
        return false;
    };
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::parse_message;
    use crate::messageformat::ast::{
        Arg, AttributeValue, Declaration, Expression, FunctionRef, Literal, Markup, MarkupKind,
        Message, OptionValue, PatternElement, True, VariantKey,
    };
    use crate::messageformat::error::ParseError;

    fn assert_syntax_at(src: &str, offset: usize) {
        match parse_message(src) {
            Err(ParseError::Syntax { offset: got, .. }) => assert_eq!(got, offset, "src = {src:?}"),
            other => panic!("expected Syntax error at {offset}, got {other:?}"),
        }
    }

    // ---- simple messages ----

    #[test]
    fn text_only() {
        let msg = parse_message("Hello, world!").unwrap();
        match msg {
            Message::Pattern {
                declarations,
                pattern,
            } => {
                assert!(declarations.is_empty());
                assert_eq!(pattern.len(), 1);
                assert!(matches!(&pattern[0], PatternElement::Text(t) if t == "Hello, world!"));
            }
            _ => panic!("expected pattern message"),
        }
    }

    #[test]
    fn text_then_variable_then_text() {
        let msg = parse_message("Hello, {$user}!").unwrap();
        let Message::Pattern { pattern, .. } = msg else {
            panic!("expected pattern")
        };
        assert_eq!(pattern.len(), 3);
        match &pattern[1] {
            PatternElement::Expression(Expression {
                arg: Some(Arg::Variable(v)),
                function: None,
                ..
            }) => assert_eq!(&*v.name, "user"),
            other => panic!("unexpected second element: {other:?}"),
        }
    }

    #[test]
    fn variable_with_function_and_options() {
        let msg = parse_message("{$count :integer minimumFractionDigits=0}").unwrap();
        let Message::Pattern { pattern, .. } = msg else {
            panic!("expected pattern")
        };
        let PatternElement::Expression(expr) = &pattern[0] else {
            panic!("expected expression element")
        };
        assert!(matches!(&expr.arg, Some(Arg::Variable(v)) if &*v.name == "count"));
        let FunctionRef {
            name, options: _, ..
        } = expr.function.as_ref().unwrap();
        assert_eq!(name.as_ref(), "integer");
        let fr = expr.function.as_ref().unwrap();
        let val = fr.options.get("minimumFractionDigits").unwrap();
        match val {
            OptionValue::Literal(Literal { value, .. }) => assert_eq!(value.as_ref(), "0"),
            other => panic!("unexpected option value: {other:?}"),
        }
    }

    #[test]
    fn numeric_literal_arg() {
        let msg = parse_message("{42 :number}").unwrap();
        let Message::Pattern { pattern, .. } = msg else {
            panic!("expected pattern")
        };
        let PatternElement::Expression(expr) = &pattern[0] else {
            panic!("expected expression")
        };
        match &expr.arg {
            Some(Arg::Literal(Literal { value, .. })) => assert_eq!(value.as_ref(), "42"),
            other => panic!("expected literal, got {other:?}"),
        }
    }

    #[test]
    fn quoted_literal_arg() {
        let msg = parse_message("{|hello world| :string}").unwrap();
        let Message::Pattern { pattern, .. } = msg else {
            panic!("expected pattern")
        };
        let PatternElement::Expression(expr) = &pattern[0] else {
            panic!("expected expression")
        };
        match &expr.arg {
            Some(Arg::Literal(Literal { value, .. })) => {
                assert_eq!(value.as_ref(), "hello world");
            }
            other => panic!("expected literal, got {other:?}"),
        }
    }

    #[test]
    fn function_only_expression() {
        let msg = parse_message("{:now}").unwrap();
        let Message::Pattern { pattern, .. } = msg else {
            panic!("expected pattern")
        };
        let PatternElement::Expression(expr) = &pattern[0] else {
            panic!("expected expression")
        };
        assert!(expr.arg.is_none());
        assert_eq!(expr.function.as_ref().unwrap().name.as_ref(), "now");
    }

    #[test]
    fn namespaced_function_identifier() {
        let msg = parse_message("{$x :ns:custom opt=val}").unwrap();
        let Message::Pattern { pattern, .. } = msg else {
            panic!("expected pattern")
        };
        let PatternElement::Expression(expr) = &pattern[0] else {
            panic!("expected expression")
        };
        assert_eq!(expr.function.as_ref().unwrap().name.as_ref(), "ns:custom");
    }

    #[test]
    fn attribute_present_and_literal() {
        let msg = parse_message("{$x :number @translate=no @private}").unwrap();
        let Message::Pattern { pattern, .. } = msg else {
            panic!("expected pattern")
        };
        let PatternElement::Expression(expr) = &pattern[0] else {
            panic!("expected expression")
        };
        assert!(matches!(
            expr.attributes.get("translate"),
            Some(AttributeValue::Literal(Literal { value, .. })) if value.as_ref() == "no"
        ));
        assert!(matches!(
            expr.attributes.get("private"),
            Some(AttributeValue::Present(True))
        ));
    }

    // ---- markup ----

    #[test]
    fn markup_open_close() {
        let msg = parse_message("{#b}bold{/b}").unwrap();
        let Message::Pattern { pattern, .. } = msg else {
            panic!("expected pattern")
        };
        assert!(matches!(
            &pattern[0],
            PatternElement::Markup(Markup { kind: MarkupKind::Open, name, .. }) if name.as_ref() == "b"
        ));
        assert!(matches!(&pattern[1], PatternElement::Text(t) if t == "bold"));
        assert!(matches!(
            &pattern[2],
            PatternElement::Markup(Markup { kind: MarkupKind::Close, name, .. }) if name.as_ref() == "b"
        ));
    }

    #[test]
    fn markup_standalone() {
        let msg = parse_message("{#br /}").unwrap();
        let Message::Pattern { pattern, .. } = msg else {
            panic!("expected pattern")
        };
        assert!(matches!(
            &pattern[0],
            PatternElement::Markup(Markup { kind: MarkupKind::Standalone, name, .. }) if name.as_ref() == "br"
        ));
    }

    #[test]
    fn markup_with_options_and_attributes() {
        let msg = parse_message("{#link href=|/home| @title=Home}go home{/link}").unwrap();
        let Message::Pattern { pattern, .. } = msg else {
            panic!("expected pattern")
        };
        let PatternElement::Markup(m) = &pattern[0] else {
            panic!("expected markup")
        };
        assert_eq!(m.name.as_ref(), "link");
        let OptionValue::Literal(lit) = m.options.get("href").unwrap() else {
            panic!("expected literal option")
        };
        assert_eq!(lit.value.as_ref(), "/home");
        let Some(AttributeValue::Literal(al)) = m.attributes.get("title") else {
            panic!("expected literal attribute")
        };
        assert_eq!(al.value.as_ref(), "Home");
    }

    // ---- complex messages ----

    #[test]
    fn input_declaration_and_quoted_pattern() {
        let src = ".input {$count :integer}\n{{You have {$count} messages.}}";
        let msg = parse_message(src).unwrap();
        let Message::Pattern {
            declarations,
            pattern,
        } = msg
        else {
            panic!("expected pattern")
        };
        assert_eq!(declarations.len(), 1);
        let Declaration::Input { name, value } = &declarations[0] else {
            panic!("expected input decl")
        };
        assert_eq!(name.as_ref(), "count");
        assert!(matches!(&value.arg, Some(Arg::Variable(v)) if &*v.name == "count"));
        assert_eq!(value.function.as_ref().unwrap().name.as_ref(), "integer");
        assert_eq!(pattern.len(), 3);
    }

    #[test]
    fn local_declaration() {
        let src = ".local $greeting = {|Hello|}\n{{{$greeting}, world!}}";
        let msg = parse_message(src).unwrap();
        let Message::Pattern { declarations, .. } = msg else {
            panic!("expected pattern")
        };
        assert_eq!(declarations.len(), 1);
        let Declaration::Local { name, value } = &declarations[0] else {
            panic!("expected local decl")
        };
        assert_eq!(name.as_ref(), "greeting");
        match &value.arg {
            Some(Arg::Literal(l)) => assert_eq!(l.value.as_ref(), "Hello"),
            other => panic!("unexpected arg: {other:?}"),
        }
    }

    #[test]
    fn quoted_pattern_only() {
        let msg = parse_message("{{Hello, {$u}!}}").unwrap();
        let Message::Pattern {
            declarations,
            pattern,
        } = msg
        else {
            panic!("expected pattern")
        };
        assert!(declarations.is_empty());
        assert_eq!(pattern.len(), 3);
    }

    #[test]
    fn matcher_single_selector() {
        let src = ".input {$count :integer}\n\
                   .match $count\n\
                   0   {{You have no items.}}\n\
                   one {{You have {$count} item.}}\n\
                   *   {{You have {$count} items.}}";
        let msg = parse_message(src).unwrap();
        let Message::Select {
            declarations,
            selectors,
            variants,
        } = msg
        else {
            panic!("expected select message")
        };
        assert_eq!(declarations.len(), 1);
        assert_eq!(selectors.len(), 1);
        assert_eq!(selectors[0].name.as_ref(), "count");
        assert_eq!(variants.len(), 3);
        match &variants[0].keys[0] {
            VariantKey::Literal(l) => assert_eq!(l.value.as_ref(), "0"),
            other => panic!("unexpected key: {other:?}"),
        }
        assert!(matches!(&variants[2].keys[0], VariantKey::Catchall(_)));
    }

    #[test]
    fn matcher_multi_selector() {
        let src = ".input {$a :integer} .input {$b :integer} .match $a $b\n\
                   0 0 {{both zero}}\n\
                   * * {{default}}";
        let msg = parse_message(src).unwrap();
        let Message::Select {
            selectors,
            variants,
            ..
        } = msg
        else {
            panic!("expected select")
        };
        assert_eq!(selectors.len(), 2);
        assert_eq!(variants.len(), 2);
        assert_eq!(variants[0].keys.len(), 2);
        assert_eq!(variants[1].keys.len(), 2);
    }

    #[test]
    fn escape_in_pattern() {
        // Literal `{` in pattern via \{.
        let msg = parse_message(r"a \{ b").unwrap();
        let Message::Pattern { pattern, .. } = msg else {
            panic!("expected pattern")
        };
        assert!(matches!(&pattern[0], PatternElement::Text(t) if t == "a { b"));
    }

    // ---- error cases ----

    #[test]
    fn unclosed_expression_is_error() {
        assert!(matches!(
            parse_message("{$x"),
            Err(ParseError::Syntax { .. })
        ));
    }

    #[test]
    fn unmatched_close_brace_is_error() {
        assert!(matches!(
            parse_message("oops}"),
            Err(ParseError::Syntax { .. })
        ));
    }

    #[test]
    fn matcher_requires_fallback_structure() {
        // Parser does not enforce the all-catchall requirement — that's the
        // validator's job. Here we only verify this parses.
        let msg = parse_message(".input {$x :integer}\n.match $x\n1 {{one}}").unwrap();
        assert!(matches!(msg, Message::Select { .. }));
    }

    #[test]
    fn input_must_be_variable() {
        // Literal .input values are a Data Model error eventually, but the
        // syntax layer rejects them up front per the ABNF.
        assert!(matches!(
            parse_message(".input {|oops|}\n{{_}}"),
            Err(ParseError::Syntax { .. })
        ));
    }

    #[test]
    fn precise_error_offset() {
        // `{@}` — `@` starts an attribute but there's no preceding
        // annotation or operand; the attribute parser then errors expecting
        // a name after `@`.
        assert_syntax_at("{@}", 2);
    }

    #[test]
    fn input_keyword_requires_whitespace() {
        // `.input{$x}` (no space) violates the ABNF `s` rule.
        match parse_message(".input{$x}") {
            Err(ParseError::Syntax { .. }) => {}
            other => panic!("expected syntax error, got {other:?}"),
        }
    }

    #[test]
    fn match_keyword_requires_whitespace() {
        match parse_message(".input {$x :string}\n.match$x\nfoo {{a}}\n* {{b}}") {
            Err(ParseError::Syntax { .. }) => {}
            other => panic!("expected syntax error, got {other:?}"),
        }
    }

    #[test]
    fn local_keyword_requires_whitespace() {
        match parse_message(".local$x = {|v|}\n{{hi}}") {
            Err(ParseError::Syntax { .. }) => {}
            other => panic!("expected syntax error, got {other:?}"),
        }
    }

    #[test]
    fn whitespace_after_keyword_still_accepted() {
        // Normal single-space form still works.
        assert!(parse_message(".input {$x :string}\n{{ok}}").is_ok());
    }

    #[test]
    fn dangling_keyword_with_no_body() {
        assert!(matches!(
            parse_message(".input"),
            Err(ParseError::Syntax { .. })
        ));
    }

    #[test]
    fn trailing_content_after_complex_body() {
        assert!(matches!(
            parse_message("{{hello}} garbage"),
            Err(ParseError::Syntax { .. })
        ));
    }
}

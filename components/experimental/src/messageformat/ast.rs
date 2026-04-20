// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Abstract syntax tree for `MessageFormat` 2.
//!
//! The types in this module match the interchange JSON Schema at
//! `spec/data-model/message.json` in the Unicode `MessageFormat` Working Group
//! repository, rev `dd86e42e10d1d0c9c4401d0781cdd87ee7166366`.
//!
//! When the `serde` feature is enabled, every public type round-trips through
//! that JSON Schema. `Box<str>` is used instead of `String` wherever a field
//! is conceptually an immutable identifier.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

// ---------------------------------------------------------------------------
// Top-level message
// ---------------------------------------------------------------------------

/// A parsed `MessageFormat` 2 message.
///
/// Either a single-pattern message or a [`.match`]-driven selection between
/// multiple variants. Round-trips through the spec's JSON interchange format
/// when `serde` is enabled.
///
/// [`.match`]: https://www.unicode.org/reports/tr35/tr35-73/tr35-messageFormat.html#matcher
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum Message {
    /// A message whose body is a single pattern.
    #[cfg_attr(feature = "serde", serde(rename = "message"))]
    Pattern {
        /// Declarations that bind or annotate variables referenced in the pattern.
        declarations: Vec<Declaration>,
        /// The single pattern rendered for every formatting.
        pattern: Pattern,
    },
    /// A message whose body is a matcher.
    #[cfg_attr(feature = "serde", serde(rename = "select"))]
    Select {
        /// Declarations that bind or annotate variables referenced in selectors or variants.
        declarations: Vec<Declaration>,
        /// The ordered list of selector variables.
        selectors: Vec<Variable>,
        /// The ordered list of variants. Spec requires an all-catchall variant.
        variants: Vec<Variant>,
    },
}

impl Message {
    /// Parse an MF2 source string into a [`Message`].
    pub fn parse(source: &str) -> Result<Self, super::ParseError> {
        super::parser::grammar::parse_message(source)
    }

    /// Parse and validate an MF2 source string in one step, returning a
    /// [`super::ValidatedMessage`] on success.
    pub fn parse_and_validate(source: &str) -> Result<super::ValidatedMessage, super::BuildError> {
        let msg = Self::parse(source)?;
        let vm = super::ValidatedMessage::try_from(msg)?;
        Ok(vm)
    }

    /// Serialize this message back to valid MF2 source text.
    ///
    /// The output parses back to an AST that is structurally equal to `self`
    /// (round-trip), but is not guaranteed to be byte-identical to whatever
    /// source produced `self`: literals are always quoted with `|...|` unless
    /// they match the ABNF `name` production, quoted patterns (`{{...}}`) are
    /// emitted whenever a complex-message form is required, and whitespace
    /// normalizes to single spaces.
    pub fn to_source(&self) -> String {
        let mut out = String::new();
        write_message(&mut out, self);
        out
    }
}

// ---------------------------------------------------------------------------
// Serializer (pub fn Message::to_source())
// ---------------------------------------------------------------------------

fn write_message(out: &mut String, m: &Message) {
    match m {
        Message::Pattern {
            declarations,
            pattern,
        } => {
            if declarations.is_empty() && !pattern_needs_quoting(pattern) {
                write_pattern(out, pattern);
            } else {
                for d in declarations {
                    write_declaration(out, d);
                    out.push('\n');
                }
                out.push_str("{{");
                write_pattern(out, pattern);
                out.push_str("}}");
            }
        }
        Message::Select {
            declarations,
            selectors,
            variants,
        } => {
            for d in declarations {
                write_declaration(out, d);
                out.push('\n');
            }
            out.push_str(".match");
            for sel in selectors {
                out.push_str(" $");
                out.push_str(&sel.name);
            }
            for v in variants {
                out.push('\n');
                for (i, k) in v.keys.iter().enumerate() {
                    if i > 0 {
                        out.push(' ');
                    }
                    write_variant_key(out, k);
                }
                out.push_str(" {{");
                write_pattern(out, &v.value);
                out.push_str("}}");
            }
        }
    }
}

/// Whether a bare simple-message cannot represent this pattern and we must
/// emit a `{{...}}` quoted pattern instead. Simple-start-char excludes
/// `.`, SP, HTAB, CR, LF; if the pattern begins with raw text whose first
/// char is one of these, we need the quoted form because the escape set
/// only covers `\`, `{`, `}`, `|`.
fn pattern_needs_quoting(pattern: &[PatternElement]) -> bool {
    match pattern.first() {
        Some(PatternElement::Text(t)) => matches!(
            t.chars().next(),
            Some('.') | Some(' ') | Some('\t') | Some('\n') | Some('\r')
        ),
        _ => false,
    }
}

fn write_pattern(out: &mut String, pattern: &Pattern) {
    for el in pattern {
        match el {
            PatternElement::Text(t) => write_text(out, t),
            PatternElement::Expression(e) => write_expression(out, e),
            PatternElement::Markup(m) => write_markup(out, m),
        }
    }
}

fn write_text(out: &mut String, t: &str) {
    for c in t.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            _ => out.push(c),
        }
    }
}

fn write_declaration(out: &mut String, d: &Declaration) {
    match d {
        Declaration::Input { name: _, value } => {
            out.push_str(".input ");
            write_expression(out, value);
        }
        Declaration::Local { name, value } => {
            out.push_str(".local $");
            out.push_str(name);
            out.push_str(" = ");
            write_expression(out, value);
        }
    }
}

fn write_expression(out: &mut String, e: &Expression) {
    out.push('{');
    let mut wrote_any = false;
    if let Some(arg) = &e.arg {
        match arg {
            Arg::Literal(l) => write_literal(out, l),
            Arg::Variable(v) => {
                out.push('$');
                out.push_str(&v.name);
            }
        }
        wrote_any = true;
    }
    if let Some(f) = &e.function {
        if wrote_any {
            out.push(' ');
        }
        write_function_ref(out, f);
        wrote_any = true;
    }
    for (k, v) in &e.attributes {
        if wrote_any {
            out.push(' ');
        }
        write_attribute(out, k, v);
        wrote_any = true;
    }
    out.push('}');
}

fn write_function_ref(out: &mut String, f: &FunctionRef) {
    out.push(':');
    out.push_str(&f.name);
    for (k, v) in &f.options {
        out.push(' ');
        out.push_str(k);
        out.push('=');
        write_option_value(out, v);
    }
}

fn write_option_value(out: &mut String, v: &OptionValue) {
    match v {
        OptionValue::Literal(l) => write_literal(out, l),
        OptionValue::Variable(var) => {
            out.push('$');
            out.push_str(&var.name);
        }
    }
}

fn write_literal(out: &mut String, l: &Literal) {
    let v = &*l.value;
    if is_unquoted_safe(v) {
        out.push_str(v);
    } else {
        out.push('|');
        for c in v.chars() {
            match c {
                '\\' => out.push_str("\\\\"),
                '|' => out.push_str("\\|"),
                '{' => out.push_str("\\{"),
                '}' => out.push_str("\\}"),
                _ => out.push(c),
            }
        }
        out.push('|');
    }
}

/// Can this string be emitted unquoted (as an ABNF `name` or `number-literal`)?
fn is_unquoted_safe(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    is_name(s) || is_number_literal(s)
}

fn is_name(s: &str) -> bool {
    let mut cs = s.chars();
    let Some(first) = cs.next() else {
        return false;
    };
    if !is_name_start(first) {
        return false;
    }
    cs.all(is_name_char)
}

fn is_name_start(c: char) -> bool {
    // Matches the ABNF `name-start` rule. A safe conservative subset is
    // enough here — when in doubt we fall back to quoted form.
    c.is_ascii_alphabetic() || c == '+' || c == '_'
}

fn is_name_char(c: char) -> bool {
    is_name_start(c) || c.is_ascii_digit() || c == '-' || c == '.'
}

/// Subset of JSON number literals. Matches: optional `-`, integer part with
/// no leading zero (unless the integer is `0`), optional fraction, optional
/// `[eE][+-]?\d+` exponent.
fn is_number_literal(s: &str) -> bool {
    let mut cs = s.chars().peekable();
    if cs.peek() == Some(&'-') {
        cs.next();
    }
    match cs.next() {
        Some('0') => {}
        Some(c) if c.is_ascii_digit() => {
            while cs.peek().is_some_and(|d| d.is_ascii_digit()) {
                cs.next();
            }
        }
        _ => return false,
    }
    if cs.peek() == Some(&'.') {
        cs.next();
        if !cs.peek().is_some_and(|d| d.is_ascii_digit()) {
            return false;
        }
        while cs.peek().is_some_and(|d| d.is_ascii_digit()) {
            cs.next();
        }
    }
    if matches!(cs.peek(), Some(&'e') | Some(&'E')) {
        cs.next();
        if matches!(cs.peek(), Some(&'+') | Some(&'-')) {
            cs.next();
        }
        if !cs.peek().is_some_and(|d| d.is_ascii_digit()) {
            return false;
        }
        while cs.peek().is_some_and(|d| d.is_ascii_digit()) {
            cs.next();
        }
    }
    cs.next().is_none()
}

fn write_attribute(out: &mut String, name: &str, v: &AttributeValue) {
    out.push('@');
    out.push_str(name);
    match v {
        AttributeValue::Literal(l) => {
            out.push('=');
            write_literal(out, l);
        }
        AttributeValue::Present(_) => {}
    }
}

fn write_markup(out: &mut String, m: &Markup) {
    out.push('{');
    match m.kind {
        MarkupKind::Open | MarkupKind::Standalone => out.push('#'),
        MarkupKind::Close => out.push('/'),
    }
    out.push_str(&m.name);
    for (k, v) in &m.options {
        out.push(' ');
        out.push_str(k);
        out.push('=');
        write_option_value(out, v);
    }
    for (k, v) in &m.attributes {
        out.push(' ');
        write_attribute(out, k, v);
    }
    if matches!(m.kind, MarkupKind::Standalone) {
        out.push('/');
    }
    out.push('}');
}

fn write_variant_key(out: &mut String, k: &VariantKey) {
    match k {
        VariantKey::Literal(l) => write_literal(out, l),
        VariantKey::Catchall(_) => out.push('*'),
    }
}

// ---------------------------------------------------------------------------
// Declarations
// ---------------------------------------------------------------------------

/// A declaration that binds or annotates a variable.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum Declaration {
    /// `.input {$name ...}` — annotates an input variable.
    ///
    /// The validator enforces that `value.arg` is a [`Arg::Variable`].
    #[cfg_attr(feature = "serde", serde(rename = "input"))]
    Input {
        /// The variable name (no `$` prefix).
        name: Box<str>,
        /// The annotated expression; its `arg` must be a variable of the same name.
        value: Expression,
    },
    /// `.local $name = {expression}` — binds a local variable.
    #[cfg_attr(feature = "serde", serde(rename = "local"))]
    Local {
        /// The variable name (no `$` prefix).
        name: Box<str>,
        /// The value expression.
        value: Expression,
    },
}

// ---------------------------------------------------------------------------
// Patterns
// ---------------------------------------------------------------------------

/// A pattern is an ordered list of text, expressions, and markup elements.
pub type Pattern = Vec<PatternElement>;

/// One element of a [`Pattern`].
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PatternElement {
    /// A run of literal text.
    Text(String),
    /// An expression placeholder (literal, variable, or function).
    Expression(Expression),
    /// A markup element (open, standalone, or close).
    Markup(Markup),
}

// ---------------------------------------------------------------------------
// Expression
// ---------------------------------------------------------------------------

/// An expression placeholder: `{arg}`, `{arg :fn ...}`, or `{:fn ...}`.
///
/// Spec requires that at least one of `arg` or `function` be present; the
/// validator enforces this.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Expression {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    _tag: ExpressionTag,
    /// The operand, if any. `None` when the expression is a bare `{:fn ...}`.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub arg: Option<Arg>,
    /// The function annotation, if any.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub function: Option<FunctionRef>,
    /// Attributes attached to the expression. Metadata only.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Attributes::is_empty")
    )]
    pub attributes: Attributes,
}

impl Expression {
    /// Construct an expression from its components.
    pub fn new(arg: Option<Arg>, function: Option<FunctionRef>, attributes: Attributes) -> Self {
        Self {
            _tag: ExpressionTag::Expression,
            arg,
            function,
            attributes,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum ExpressionTag {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "expression"))]
    Expression,
}

// ---------------------------------------------------------------------------
// Markup
// ---------------------------------------------------------------------------

/// An open, standalone, or close markup element inside a pattern.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Markup {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    _tag: MarkupTag,
    /// Which form of markup this is.
    pub kind: MarkupKind,
    /// The markup identifier (optionally namespaced: e.g. `"u:id"`).
    pub name: Box<str>,
    /// Options attached to the markup element.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "OptionMap::is_empty")
    )]
    pub options: OptionMap,
    /// Attributes attached to the markup element. Metadata only.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Attributes::is_empty")
    )]
    pub attributes: Attributes,
}

impl Markup {
    /// Construct a markup element from its components.
    pub fn new(
        kind: MarkupKind,
        name: Box<str>,
        options: OptionMap,
        attributes: Attributes,
    ) -> Self {
        Self {
            _tag: MarkupTag::Markup,
            kind,
            name,
            options,
            attributes,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum MarkupTag {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "markup"))]
    Markup,
}

/// Which form of markup an element represents.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum MarkupKind {
    /// `{#tag}` — opens a markup scope.
    Open,
    /// `{#tag /}` — self-closing.
    Standalone,
    /// `{/tag}` — closes a previously-opened scope.
    Close,
}

// ---------------------------------------------------------------------------
// FunctionRef
// ---------------------------------------------------------------------------

/// A function annotation attached to an expression: `:identifier option=value ...`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionRef {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    _tag: FunctionTag,
    /// The function identifier (optionally namespaced, e.g. `"number"` or `"ns:custom"`).
    pub name: Box<str>,
    /// The function's options map.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "OptionMap::is_empty")
    )]
    pub options: OptionMap,
}

impl FunctionRef {
    /// Construct a function annotation.
    pub fn new(name: Box<str>, options: OptionMap) -> Self {
        Self {
            _tag: FunctionTag::Function,
            name,
            options,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum FunctionTag {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "function"))]
    Function,
}

// ---------------------------------------------------------------------------
// Literal / Variable / Arg / OptionValue
// ---------------------------------------------------------------------------

/// A literal value — an inline string operand.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Literal {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    _tag: LiteralTag,
    /// The literal's string value (unescaped).
    pub value: Box<str>,
}

impl Literal {
    /// Construct a literal from its string value.
    pub fn new(value: Box<str>) -> Self {
        Self {
            _tag: LiteralTag::Literal,
            value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum LiteralTag {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "literal"))]
    Literal,
}

/// A reference to a variable: the name side of `$foo`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variable {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    _tag: VariableTag,
    /// The variable name (no leading `$`).
    pub name: Box<str>,
}

impl Variable {
    /// Construct a variable reference.
    pub fn new(name: Box<str>) -> Self {
        Self {
            _tag: VariableTag::Variable,
            name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum VariableTag {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "variable"))]
    Variable,
}

/// An expression operand: a literal or a variable reference.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Arg {
    /// An inline literal.
    Literal(Literal),
    /// A variable reference.
    Variable(Variable),
}

/// Option map for function and markup elements.
///
/// Keys must be unique within a single map; the validator catches duplicates
/// (the JSON object form already enforces this at parse time).
pub type OptionMap = BTreeMap<Box<str>, OptionValue>;

/// Value of an option entry: a literal or a variable reference.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OptionValue {
    /// An inline literal.
    Literal(Literal),
    /// A variable reference (resolved at format time).
    Variable(Variable),
}

// ---------------------------------------------------------------------------
// Attributes
// ---------------------------------------------------------------------------

/// Attributes attached to an expression or markup. Metadata only — never
/// observable in formatted output or in function handler calls.
pub type Attributes = BTreeMap<Box<str>, AttributeValue>;

/// Value of an attribute entry: a literal, or bare presence (`@flag`).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AttributeValue {
    /// `@name=|literal|`
    Literal(Literal),
    /// `@name` — serializes as JSON `true`.
    Present(True),
}

/// Zero-sized marker that serializes as JSON `true`. Used by [`AttributeValue::Present`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct True;

#[cfg(feature = "serde")]
impl serde::Serialize for True {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bool(true)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for True {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        match bool::deserialize(d)? {
            true => Ok(True),
            false => Err(serde::de::Error::custom("attribute value must be `true`")),
        }
    }
}

// ---------------------------------------------------------------------------
// Variants
// ---------------------------------------------------------------------------

/// A single variant of a `.match` message.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Variant {
    /// Keys to match against each selector; one per selector (in order).
    pub keys: Vec<VariantKey>,
    /// The pattern rendered when this variant is selected.
    pub value: Pattern,
}

/// A key in a [`Variant`] — either a concrete literal or the catchall `*`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VariantKey {
    /// A literal key — exact (possibly NFC-compared) match against the selector value.
    Literal(Literal),
    /// A catchall `*` key.
    Catchall(CatchallKey),
}

/// The catchall key `*`. May optionally carry a name for diagnostics.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CatchallKey {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    _tag: CatchallTag,
    /// Optional display name — useful only for error messages.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub value: Option<Box<str>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum CatchallTag {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "*"))]
    Catchall,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    fn assert_round_trip(src: &str) -> serde_json::Value {
        let original: serde_json::Value = serde_json::from_str(src).expect("source parses");
        let msg: Message = serde_json::from_value(original.clone()).expect("deserializes");
        let reserialized: serde_json::Value = serde_json::to_value(&msg).expect("serializes");
        assert_eq!(
            reserialized, original,
            "re-serialized JSON does not match original"
        );
        reserialized
    }

    #[test]
    fn simple_pattern_message() {
        assert_round_trip(
            r#"{
                "type": "message",
                "declarations": [],
                "pattern": ["Hello, world!"]
            }"#,
        );
    }

    #[test]
    fn pattern_with_expression() {
        let v = assert_round_trip(
            r#"{
                "type": "message",
                "declarations": [],
                "pattern": [
                    "Hello, ",
                    {
                        "type": "expression",
                        "arg": {"type": "variable", "name": "user"}
                    },
                    "!"
                ]
            }"#,
        );
        assert_eq!(v["pattern"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn select_message_with_declarations() {
        assert_round_trip(
            r#"{
                "type": "select",
                "declarations": [
                    {
                        "type": "input",
                        "name": "count",
                        "value": {
                            "type": "expression",
                            "arg": {"type": "variable", "name": "count"},
                            "function": {"type": "function", "name": "integer"}
                        }
                    }
                ],
                "selectors": [{"type": "variable", "name": "count"}],
                "variants": [
                    {
                        "keys": [{"type": "literal", "value": "one"}],
                        "value": ["one item"]
                    },
                    {
                        "keys": [{"type": "*"}],
                        "value": ["many items"]
                    }
                ]
            }"#,
        );
    }

    #[test]
    fn expression_with_options_and_attributes() {
        assert_round_trip(
            r#"{
                "type": "message",
                "declarations": [],
                "pattern": [
                    {
                        "type": "expression",
                        "arg": {"type": "variable", "name": "amount"},
                        "function": {
                            "type": "function",
                            "name": "number",
                            "options": {
                                "minimumFractionDigits": {"type": "literal", "value": "2"},
                                "style": {"type": "variable", "name": "style"}
                            }
                        },
                        "attributes": {
                            "translate": {"type": "literal", "value": "no"},
                            "private": true
                        }
                    }
                ]
            }"#,
        );
    }

    #[test]
    fn markup_in_pattern() {
        assert_round_trip(
            r#"{
                "type": "message",
                "declarations": [],
                "pattern": [
                    {"type": "markup", "kind": "open", "name": "b"},
                    "bold",
                    {"type": "markup", "kind": "close", "name": "b"},
                    {"type": "markup", "kind": "standalone", "name": "br"}
                ]
            }"#,
        );
    }

    #[test]
    fn catchall_with_name() {
        assert_round_trip(
            r#"{
                "type": "select",
                "declarations": [
                    {
                        "type": "input",
                        "name": "x",
                        "value": {
                            "type": "expression",
                            "arg": {"type": "variable", "name": "x"},
                            "function": {"type": "function", "name": "string"}
                        }
                    }
                ],
                "selectors": [{"type": "variable", "name": "x"}],
                "variants": [
                    {
                        "keys": [{"type": "*", "value": "else"}],
                        "value": ["fallback"]
                    }
                ]
            }"#,
        );
    }

    #[test]
    fn function_only_expression() {
        assert_round_trip(
            r#"{
                "type": "message",
                "declarations": [],
                "pattern": [
                    {
                        "type": "expression",
                        "function": {"type": "function", "name": "today"}
                    }
                ]
            }"#,
        );
    }

    #[test]
    fn literal_argument_expression() {
        assert_round_trip(
            r#"{
                "type": "message",
                "declarations": [],
                "pattern": [
                    {
                        "type": "expression",
                        "arg": {"type": "literal", "value": "42"},
                        "function": {"type": "function", "name": "number"}
                    }
                ]
            }"#,
        );
    }

    #[test]
    fn tag_mismatch_rejects() {
        // A literal with the wrong discriminator must fail.
        let src = r#"{"type": "not-a-literal", "value": "x"}"#;
        let err = serde_json::from_str::<Literal>(src).unwrap_err();
        // The error mentions the unrecognized tag.
        assert!(
            err.to_string().contains("not-a-literal") || err.to_string().contains("variant"),
            "unexpected error: {err}"
        );
    }
}

#[cfg(test)]
mod ast_ctor_tests {
    use super::*;

    #[test]
    fn literal_new_roundtrips_value() {
        let lit = Literal::new(Box::from("hello"));
        assert_eq!(lit.value.as_ref(), "hello");
    }

    #[test]
    fn variable_new_roundtrips_name() {
        let v = Variable::new(Box::from("count"));
        assert_eq!(v.name.as_ref(), "count");
    }

    #[test]
    fn function_ref_new_stores_name_and_options() {
        let mut opts: OptionMap = BTreeMap::new();
        opts.insert(
            Box::from("style"),
            OptionValue::Literal(Literal::new(Box::from("percent"))),
        );
        let f = FunctionRef::new(Box::from("number"), opts);
        assert_eq!(f.name.as_ref(), "number");
        assert_eq!(f.options.len(), 1);
    }

    #[test]
    fn expression_new_accepts_none_both() {
        // This is the empty expression — the validator rejects it, but the
        // constructor itself must allow it so the validator can see it.
        let e = Expression::new(None, None, Attributes::new());
        assert!(e.arg.is_none());
        assert!(e.function.is_none());
        assert!(e.attributes.is_empty());
    }

    #[test]
    fn catchall_key_default_has_no_name() {
        let k = CatchallKey::default();
        assert!(k.value.is_none());
    }

    #[test]
    fn variant_key_equality_literal_vs_catchall() {
        let lit = VariantKey::Literal(Literal::new(Box::from("1")));
        let star = VariantKey::Catchall(CatchallKey::default());
        assert_ne!(lit, star);
    }

    #[test]
    fn arg_variants_distinct() {
        let a = Arg::Literal(Literal::new(Box::from("x")));
        let b = Arg::Variable(Variable::new(Box::from("x")));
        assert_ne!(a, b);
    }

    #[test]
    fn parse_and_validate_ok_for_valid_source() {
        let vm = Message::parse_and_validate("Hello, {$u}!").expect("valid");
        matches!(vm.as_message(), Message::Pattern { .. });
    }

    #[test]
    fn parse_and_validate_surfaces_validation_error() {
        let err = Message::parse_and_validate(".input {$x :integer}\n.match $x\n1 {{one}}")
            .expect_err("missing fallback");
        assert!(matches!(
            err,
            super::super::BuildError::Validation(
                super::super::ValidationError::MissingFallbackVariant
            )
        ));
    }

    #[test]
    fn parse_and_validate_surfaces_parse_error() {
        let err = Message::parse_and_validate("{$x").expect_err("syntax");
        assert!(matches!(err, super::super::BuildError::Parse(_)));
    }

    #[test]
    fn markup_new_stores_metadata() {
        let mk = Markup::new(
            MarkupKind::Open,
            Box::from("b"),
            OptionMap::new(),
            Attributes::new(),
        );
        assert_eq!(mk.kind, MarkupKind::Open);
        assert_eq!(mk.name.as_ref(), "b");
        assert!(mk.options.is_empty());
    }

    #[test]
    fn attribute_value_present_and_literal() {
        let mut attrs: Attributes = BTreeMap::new();
        attrs.insert(Box::from("flag"), AttributeValue::Present(True));
        attrs.insert(
            Box::from("info"),
            AttributeValue::Literal(Literal::new(Box::from("x"))),
        );
        assert_eq!(attrs.len(), 2);
        assert!(matches!(
            attrs.get("flag"),
            Some(AttributeValue::Present(_))
        ));
    }
}

// Round-trip tests for `Message::to_source()`. The serializer contract is
// that `parse(msg.to_source()) == msg` for any `msg` produced by `parse` — we
// do not require byte-identical output.
#[cfg(test)]
mod to_source_tests {
    use super::*;

    fn assert_round_trip(src: &str) {
        let parsed = Message::parse(src).expect("source parses");
        let emitted = parsed.to_source();
        let reparsed = Message::parse(&emitted).unwrap_or_else(|e| {
            panic!("reparse failed: {e:?}\n  orig: {src:?}\n  emitted: {emitted:?}")
        });
        assert_eq!(parsed, reparsed, "\norig:    {src:?}\nemitted: {emitted:?}");
    }

    #[test]
    fn plain_text() {
        assert_round_trip("Hello, world!");
    }

    #[test]
    fn text_with_escapes() {
        assert_round_trip("a\\\\b\\{c\\}d");
    }

    #[test]
    fn simple_variable() {
        assert_round_trip("Hi {$name}!");
    }

    #[test]
    fn variable_with_function() {
        assert_round_trip("Total: {$count :number}");
    }

    #[test]
    fn function_with_options() {
        assert_round_trip("{$n :number minimumFractionDigits=2 useGrouping=always}");
    }

    #[test]
    fn literal_expression() {
        assert_round_trip("{|hello world|}");
    }

    #[test]
    fn literal_expression_with_bar_and_backslash() {
        assert_round_trip("{|pipe\\|and\\\\back|}");
    }

    #[test]
    fn bare_function_expression() {
        assert_round_trip("Today is {:datetime}");
    }

    #[test]
    fn variable_option_value() {
        assert_round_trip("{$x :number minimumFractionDigits=$digits}");
    }

    #[test]
    fn markup_open_close() {
        assert_round_trip("Click {#link href=|/a|}here{/link}");
    }

    #[test]
    fn markup_standalone() {
        assert_round_trip("See {#img src=|x.png| /} next.");
    }

    #[test]
    fn attribute_bare_and_value() {
        assert_round_trip("{$x :number @flag @info=|note|}");
    }

    #[test]
    fn input_declaration() {
        assert_round_trip(".input {$x :number}\n{{You have {$x}.}}");
    }

    #[test]
    fn local_declaration() {
        assert_round_trip(".local $y = {$x :number}\n{{value is {$y}}}");
    }

    #[test]
    fn multiple_declarations() {
        assert_round_trip(
            ".input {$n :number}\n.local $m = {$n :number style=percent}\n{{x={$n} y={$m}}}",
        );
    }

    #[test]
    fn select_message() {
        assert_round_trip(
            ".input {$count :integer}\n\
             .match $count\n\
             0 {{You have no messages.}}\n\
             one {{You have {$count} message.}}\n\
             * {{You have {$count} messages.}}",
        );
    }

    #[test]
    fn select_multiple_selectors() {
        assert_round_trip(
            ".input {$a :integer}\n\
             .input {$b :integer}\n\
             .match $a $b\n\
             1 1 {{one and one}}\n\
             * * {{default}}",
        );
    }

    #[test]
    fn quoted_pattern_preserved_for_leading_dot() {
        // Leading `.` in pattern text forces complex-message / quoted form.
        assert_round_trip("{{.com domain}}");
    }

    #[test]
    fn quoted_pattern_preserved_for_leading_space() {
        assert_round_trip("{{ padded}}");
    }

    #[test]
    fn number_literal_argument() {
        assert_round_trip("count: {42 :number}");
    }

    #[test]
    fn negative_number_literal() {
        assert_round_trip("delta: {-3.14 :number}");
    }

    #[test]
    fn empty_pattern_via_declaration() {
        // Declaration forces complex form; the pattern itself is empty.
        assert_round_trip(".local $x = {|v|}\n{{}}");
    }

    #[test]
    fn namespaced_function() {
        assert_round_trip("{$x :ns:custom}");
    }

    #[test]
    fn text_with_pipe_unescaped_in_text_context() {
        // `|` is valid text-char per ABNF; must not be escaped outside quoted literals.
        assert_round_trip("a|b|c");
    }

    #[test]
    fn unquoted_literal_renders_unquoted() {
        let parsed = Message::parse("{|foo|}").unwrap();
        let out = parsed.to_source();
        // `foo` satisfies `name` → emitted unquoted.
        assert_eq!(out, "{foo}");
        // …and that output parses back to the same AST.
        assert_eq!(Message::parse(&out).unwrap(), parsed);
    }

    #[test]
    fn quoted_literal_with_special_chars() {
        let parsed = Message::parse("{|hello world|}").unwrap();
        let out = parsed.to_source();
        assert_eq!(out, "{|hello world|}");
    }

    #[test]
    fn number_helpers() {
        assert!(is_number_literal("0"));
        assert!(is_number_literal("-0"));
        assert!(is_number_literal("42"));
        assert!(is_number_literal("-3.14"));
        assert!(is_number_literal("1e9"));
        assert!(is_number_literal("1.5E-3"));
        assert!(!is_number_literal(""));
        assert!(!is_number_literal("00"));
        assert!(!is_number_literal("."));
        assert!(!is_number_literal("1."));
        assert!(!is_number_literal("1e"));
        assert!(!is_number_literal("abc"));
    }

    #[test]
    fn name_helpers() {
        assert!(is_name("foo"));
        assert!(is_name("foo_bar"));
        assert!(is_name("foo-bar"));
        assert!(is_name("f1"));
        assert!(!is_name(""));
        assert!(!is_name("1foo"));
        assert!(!is_name("-foo"));
        assert!(!is_name("hello world"));
    }
}

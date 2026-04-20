// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! [`MessageFormatter`] and builder — the public entry point for formatting
//! MF2 messages.
//!
//! Supports both [`super::ast::Message::Pattern`] and
//! [`super::ast::Message::Select`] messages, with built-in and draft handlers
//! resolved via the [`super::FunctionRegistry`].

use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt;

use icu_locale_core::{locale, Locale};

use super::ast::{Message, Pattern, PatternElement};
use super::bidi::{self, BidiIsolation, BidiStrategy, Direction};
use super::error::{BuildError, FormatError};
use super::function::FunctionRegistry;
use super::input::InputValues;
use super::parts::FormattedPart;
use super::resolver::Resolver;
use super::selector::{pick_variant, NamedSelector};
use super::validator::ValidatedMessage;

/// A configured `MessageFormat` 2 formatter.
///
/// Construct via [`MessageFormatter::builder`]. A formatter is cheap to
/// clone — the registry uses `Arc` internally — and can be shared across
/// threads.
#[derive(Debug, Clone)]
pub struct MessageFormatter {
    message: ValidatedMessage,
    registry: FunctionRegistry,
    locale: Locale,
    direction: Direction,
    bidi_isolation: BidiIsolation,
}

impl MessageFormatter {
    /// Start building a new [`MessageFormatter`].
    pub fn builder() -> MessageFormatterBuilder {
        MessageFormatterBuilder::default()
    }

    /// Format this message with the given inputs to a `String`, collecting
    /// any emitted [`FormatError`]s.
    pub fn format_to_string<V: InputValues + ?Sized>(
        &self,
        inputs: &V,
    ) -> (String, Vec<FormatError>) {
        let mut out = String::new();
        let errors = self.format(inputs, &mut out).unwrap_or_else(|_| Vec::new());
        (out, errors)
    }

    /// Format into an arbitrary [`fmt::Write`] sink.
    ///
    /// Returns the list of emitted format errors (possibly empty). The
    /// `Err` arm is reserved for writer I/O failures — never for MF2
    /// semantic errors, which are always reported through the `Ok` list.
    pub fn format<V, W>(&self, inputs: &V, out: &mut W) -> Result<Vec<FormatError>, fmt::Error>
    where
        V: InputValues + ?Sized,
        W: fmt::Write + ?Sized,
    {
        let opts = WriteOpts {
            base_direction: self.direction,
            bidi_isolation: self.bidi_strategy(),
        };
        match self.message.as_message() {
            Message::Pattern {
                declarations,
                pattern,
            } => {
                let mut r = Resolver::new(
                    declarations,
                    inputs,
                    &self.registry,
                    &self.locale,
                    self.direction,
                );
                write_pattern(&mut r, pattern, out, opts)?;
                Ok(r.into_errors())
            }
            Message::Select {
                declarations,
                selectors,
                variants,
            } => {
                let mut r = Resolver::new(
                    declarations,
                    inputs,
                    &self.registry,
                    &self.locale,
                    self.direction,
                );
                let resolved_selectors: Vec<Option<NamedSelector>> = selectors
                    .iter()
                    .map(|var| {
                        let rv = r.resolve_variable_pub(&var.name);
                        if rv.is_fallback() {
                            // When the selector's fallback was caused by a
                            // function-level error (UnknownFunction,
                            // BadOperand, BadOption, …), spec requires an
                            // additional Bad Selector error. A plain
                            // UnresolvedVariable fallback is already enough.
                            if rv.is_function_error_fallback() {
                                r.push_error(FormatError::BadSelector {
                                    name: var.name.clone(),
                                });
                            }
                            None
                        } else if let Some(sel) = rv.selector() {
                            let fname: Box<str> = rv
                                .selector_function()
                                .map(Box::from)
                                .unwrap_or_else(|| Box::from(""));
                            Some((fname, Arc::clone(sel)))
                        } else {
                            r.push_error(FormatError::BadSelector {
                                name: var.name.clone(),
                            });
                            None
                        }
                    })
                    .collect();
                let mut sel_errors: Vec<FormatError> = Vec::new();
                let chosen = pick_variant(&resolved_selectors, variants, &mut sel_errors);
                for e in sel_errors {
                    r.push_error(e);
                }
                write_pattern(&mut r, &chosen.value, out, opts)?;
                Ok(r.into_errors())
            }
        }
    }

    /// Format this message with the given inputs into a structured list of
    /// [`FormattedPart`]s. Mirrors `formatToParts` in the JavaScript
    /// reference implementation.
    pub fn format_to_parts<V: InputValues + ?Sized>(
        &self,
        inputs: &V,
    ) -> (Vec<FormattedPart>, Vec<FormatError>) {
        let mut parts: Vec<FormattedPart> = Vec::new();
        let opts = WriteOpts {
            base_direction: self.direction,
            bidi_isolation: self.bidi_strategy(),
        };
        let errs = match self.message.as_message() {
            Message::Pattern {
                declarations,
                pattern,
            } => {
                let mut r = Resolver::new(
                    declarations,
                    inputs,
                    &self.registry,
                    &self.locale,
                    self.direction,
                );
                collect_pattern_parts(&mut r, pattern, &mut parts, opts);
                r.into_errors()
            }
            Message::Select {
                declarations,
                selectors,
                variants,
            } => {
                let mut r = Resolver::new(
                    declarations,
                    inputs,
                    &self.registry,
                    &self.locale,
                    self.direction,
                );
                let resolved_selectors: Vec<Option<NamedSelector>> = selectors
                    .iter()
                    .map(|var| {
                        let rv = r.resolve_variable_pub(&var.name);
                        if rv.is_fallback() {
                            // When the selector's fallback was caused by a
                            // function-level error (UnknownFunction,
                            // BadOperand, BadOption, …), spec requires an
                            // additional Bad Selector error. A plain
                            // UnresolvedVariable fallback is already enough.
                            if rv.is_function_error_fallback() {
                                r.push_error(FormatError::BadSelector {
                                    name: var.name.clone(),
                                });
                            }
                            None
                        } else if let Some(sel) = rv.selector() {
                            let fname: Box<str> = rv
                                .selector_function()
                                .map(Box::from)
                                .unwrap_or_else(|| Box::from(""));
                            Some((fname, Arc::clone(sel)))
                        } else {
                            r.push_error(FormatError::BadSelector {
                                name: var.name.clone(),
                            });
                            None
                        }
                    })
                    .collect();
                let mut sel_errors: Vec<FormatError> = Vec::new();
                let chosen = pick_variant(&resolved_selectors, variants, &mut sel_errors);
                for e in sel_errors {
                    r.push_error(e);
                }
                collect_pattern_parts(&mut r, &chosen.value, &mut parts, opts);
                r.into_errors()
            }
        };
        (parts, errs)
    }

    /// Borrow the underlying validated message.
    pub fn validated_message(&self) -> &ValidatedMessage {
        &self.message
    }

    /// Borrow the function registry.
    pub fn registry(&self) -> &FunctionRegistry {
        &self.registry
    }

    /// Borrow the formatter's locale.
    pub fn locale(&self) -> &Locale {
        &self.locale
    }

    /// The formatter's base directionality.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// The configured bidi-isolation strategy. Use
    /// [`BidiIsolation::is_enabled`] (or pattern-match the enum directly)
    /// if you just need a yes/no signal.
    pub fn bidi_isolation(&self) -> &BidiIsolation {
        &self.bidi_isolation
    }

    /// Resolve the configured [`BidiIsolation`] to a strategy reference, or
    /// `None` if isolation is disabled. Used by the formatting loops.
    fn bidi_strategy(&self) -> Option<&dyn BidiStrategy> {
        match &self.bidi_isolation {
            BidiIsolation::None => None,
            BidiIsolation::Default => Some(&bidi::DEFAULT_BIDI),
            BidiIsolation::Custom(s) => Some(s.as_ref()),
        }
    }
}

#[derive(Clone, Copy)]
struct WriteOpts<'a> {
    base_direction: Direction,
    /// `None` = no isolation (fast path); `Some` delegates to the strategy,
    /// covering both the default and any caller-supplied strategy through
    /// one code path.
    bidi_isolation: Option<&'a dyn BidiStrategy>,
}

fn push_bidi_isolation_parts(out: &mut Vec<FormattedPart>, isolate: &str) {
    for ch in isolate.chars() {
        out.push(FormattedPart::BidiIsolation { value: ch });
    }
}

fn collect_pattern_parts<V>(
    resolver: &mut Resolver<'_, V>,
    pattern: &Pattern,
    out: &mut Vec<FormattedPart>,
    opts: WriteOpts<'_>,
) where
    V: InputValues + ?Sized,
{
    for element in pattern {
        match element {
            PatternElement::Text(s) => {
                if !s.is_empty() {
                    out.push(FormattedPart::text(s.clone()));
                }
            }
            PatternElement::Expression(expr) => {
                let v = resolver.resolve_expression(expr);
                if let Some(strategy) = opts.bidi_isolation {
                    let (prefix, suffix) = strategy.isolate(
                        opts.base_direction,
                        v.direction(),
                        v.direction_explicit(),
                    );
                    push_bidi_isolation_parts(out, &prefix);
                    out.push(FormattedPart::Expression {
                        kind: v.part_kind().into(),
                        value: v.text().into(),
                        id: v.u_id().map(Into::into),
                        direction: v.direction(),
                    });
                    push_bidi_isolation_parts(out, &suffix);
                    continue;
                }
                out.push(FormattedPart::Expression {
                    kind: v.part_kind().into(),
                    value: v.text().into(),
                    id: v.u_id().map(Into::into),
                    direction: v.direction(),
                });
            }
            PatternElement::Markup(m) => {
                let mut resolved = resolver.resolve_options_pub(&m.options);
                let id = resolved
                    .remove("u:id")
                    .map(|v| Into::<Box<str>>::into(v.text()));
                // Per spec u-namespace.md:51-52, u:dir on markup is a Bad
                // Option error and the option is ignored.
                if resolved.remove("u:dir").is_some() {
                    resolver.push_error(FormatError::FunctionError {
                        function: m.name.clone(),
                        error: super::error::FunctionError::BadOption {
                            name: "u:dir".into(),
                        },
                    });
                }
                let options: alloc::collections::BTreeMap<Box<str>, String> = resolved
                    .into_iter()
                    .map(|(k, v)| (k, v.text().into()))
                    .collect();
                out.push(FormattedPart::Markup {
                    kind: m.kind,
                    name: m.name.clone(),
                    options,
                    id,
                    direction: None,
                });
            }
        }
    }
}

fn write_pattern<V, W>(
    resolver: &mut Resolver<'_, V>,
    pattern: &Pattern,
    out: &mut W,
    opts: WriteOpts<'_>,
) -> Result<(), fmt::Error>
where
    V: InputValues + ?Sized,
    W: fmt::Write + ?Sized,
{
    for element in pattern {
        match element {
            PatternElement::Text(s) => out.write_str(s)?,
            PatternElement::Expression(expr) => {
                let v = resolver.resolve_expression(expr);
                if let Some(strategy) = opts.bidi_isolation {
                    let (prefix, suffix) = strategy.isolate(
                        opts.base_direction,
                        v.direction(),
                        v.direction_explicit(),
                    );
                    out.write_str(&prefix)?;
                    out.write_str(v.text())?;
                    out.write_str(&suffix)?;
                } else {
                    out.write_str(v.text())?;
                }
            }
            PatternElement::Markup(m) => {
                // Markup produces no text, but we still resolve options to
                // surface option errors (e.g. the spec-required Bad Option
                // when `u:dir` is supplied on markup).
                let mut resolved = resolver.resolve_options_pub(&m.options);
                if resolved.remove("u:dir").is_some() {
                    resolver.push_error(FormatError::FunctionError {
                        function: m.name.clone(),
                        error: super::error::FunctionError::BadOption {
                            name: "u:dir".into(),
                        },
                    });
                }
            }
        }
    }
    Ok(())
}

/// Builder for [`MessageFormatter`]. Constructed via
/// [`MessageFormatter::builder`].
#[derive(Debug, Default)]
pub struct MessageFormatterBuilder {
    input: Option<BuilderInput>,
    registry: Option<FunctionRegistry>,
    locale: Option<Locale>,
    direction: Option<Direction>,
    bidi_isolation: Option<BidiIsolation>,
}

#[derive(Debug)]
enum BuilderInput {
    Source(String),
    Validated(ValidatedMessage),
}

impl MessageFormatterBuilder {
    /// Supply MF2 source text. Parsing and validation run at [`Self::build`].
    pub fn source(mut self, src: impl Into<String>) -> Self {
        self.input = Some(BuilderInput::Source(src.into()));
        self
    }

    /// Set the locale used for number formatting, plural rules, and future
    /// locale-sensitive functions.
    ///
    /// **Required.** [`Self::build`] returns [`BuildError::MissingLocale`]
    /// if no locale was supplied — this prevents callers from silently
    /// shipping root-locale output. If root-locale behavior is genuinely
    /// desired, call [`Self::locale_undetermined`] instead.
    pub fn locale(mut self, locale: Locale) -> Self {
        self.locale = Some(locale);
        self
    }

    /// Explicitly opt in to undetermined-locale (`und`) behavior. Equivalent
    /// to `.locale(locale!("und"))` but self-documents the intent for
    /// future readers. Locale-sensitive functions will emit root-locale
    /// output; this is almost never what you want for user-facing messages.
    pub fn locale_undetermined(mut self) -> Self {
        self.locale = Some(locale!("und"));
        self
    }

    /// Set the message's base directionality. Defaults to [`Direction::Ltr`].
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }

    /// Set the bidi-isolation strategy for expression output. Defaults to
    /// [`BidiIsolation::Default`]. Accepts any value convertible into a
    /// [`BidiIsolation`]:
    ///
    /// - [`BidiIsolation`] itself (including
    ///   [`BidiIsolation::Custom`] for a caller-supplied
    ///   [`super::BidiStrategy`]);
    /// - `bool` — `true` → [`BidiIsolation::Default`], `false` →
    ///   [`BidiIsolation::None`];
    /// - `Arc<dyn BidiStrategy>` — shorthand for
    ///   [`BidiIsolation::Custom`];
    /// - the unit structs [`super::DefaultBidiStrategy`] and
    ///   [`super::NoneBidiStrategy`].
    pub fn bidi_isolation(mut self, mode: impl Into<BidiIsolation>) -> Self {
        self.bidi_isolation = Some(mode.into());
        self
    }

    /// Supply a pre-validated message.
    pub fn message(mut self, msg: ValidatedMessage) -> Self {
        self.input = Some(BuilderInput::Validated(msg));
        self
    }

    /// Replace the function registry wholesale. If not called, the builder
    /// uses [`FunctionRegistry::default_registry`].
    pub fn functions(mut self, registry: FunctionRegistry) -> Self {
        self.registry = Some(registry);
        self
    }

    /// Register a single custom function. Creates a default registry if
    /// none has been supplied yet.
    pub fn function<F>(mut self, name: impl Into<Box<str>>, handler: F) -> Self
    where
        F: super::function::FunctionHandler + 'static,
    {
        let reg = self
            .registry
            .get_or_insert_with(FunctionRegistry::default_registry);
        reg.register(name, handler);
        self
    }

    /// Finalize the builder and produce a [`MessageFormatter`].
    pub fn build(self) -> Result<MessageFormatter, BuildError> {
        let message = match self.input.ok_or(BuildError::NoMessage)? {
            BuilderInput::Source(s) => Message::parse_and_validate(&s)?,
            BuilderInput::Validated(m) => m,
        };
        let registry = self
            .registry
            .unwrap_or_else(FunctionRegistry::default_registry);
        let locale = self.locale.ok_or(BuildError::MissingLocale)?;
        // When the caller did not set an explicit base direction, derive it
        // from the locale via LocaleDirectionality per formatting.md — RTL
        // locales get an RTL message base direction.
        let direction = self
            .direction
            .unwrap_or_else(|| super::resolver::locale_direction(&locale));
        let bidi_isolation = self.bidi_isolation.unwrap_or_default();
        Ok(MessageFormatter {
            message,
            registry,
            locale,
            direction,
            bidi_isolation,
        })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messageformat::error::FunctionError;

    fn fmt(src: &str, inputs: &[(&str, &str)]) -> (String, Vec<FormatError>) {
        let f = MessageFormatter::builder()
            .source(src)
            .locale_undetermined()
            .build()
            .unwrap();
        f.format_to_string(&inputs)
    }

    #[test]
    fn plain_text_message() {
        let (s, errs) = fmt("Hello, world!", &[]);
        assert_eq!(s, "Hello, world!");
        assert!(errs.is_empty());
    }

    #[test]
    fn simple_variable_substitution() {
        let (s, errs) = fmt("Hello, {$user}!", &[("user", "Ada")]);
        assert_eq!(s, "Hello, Ada!");
        assert!(errs.is_empty());
    }

    #[test]
    fn string_function_passthrough() {
        // `:string` leaves direction unset (spec allows; tested via u-options
        // #9). Default Bidi Strategy wraps unknown-direction values in
        // FSI+PDI.
        let (s, errs) = fmt("Hi, {$who :string}!", &[("who", "Grace")]);
        assert_eq!(s, "Hi, \u{2068}Grace\u{2069}!");
        assert!(errs.is_empty());
    }

    #[test]
    fn literal_argument() {
        let (s, errs) = fmt("{|constant| :string}", &[]);
        assert_eq!(s, "\u{2068}constant\u{2069}");
        assert!(errs.is_empty());
    }

    #[test]
    fn unresolved_variable_fallback() {
        let (s, errs) = fmt("Hello, {$missing}!", &[]);
        assert_eq!(s, "Hello, \u{2068}{$missing}\u{2069}!");
        assert_eq!(
            errs,
            alloc::vec![FormatError::UnresolvedVariable {
                name: "missing".into()
            }]
        );
    }

    #[test]
    fn unknown_function_fallback_on_variable() {
        let (s, errs) = fmt("{$x :unknown}", &[("x", "value")]);
        assert_eq!(s, "\u{2068}{$x}\u{2069}");
        assert_eq!(
            errs,
            alloc::vec![FormatError::UnknownFunction {
                name: "unknown".into()
            }]
        );
    }

    #[test]
    fn unknown_function_fallback_on_literal() {
        let (s, errs) = fmt("{|hi| :unknown}", &[]);
        assert_eq!(s, "\u{2068}{|hi|}\u{2069}");
        assert_eq!(
            errs,
            alloc::vec![FormatError::UnknownFunction {
                name: "unknown".into()
            }]
        );
    }

    #[test]
    fn unknown_function_fallback_bare() {
        let (s, errs) = fmt("{:unknown}", &[]);
        assert_eq!(s, "\u{2068}{:unknown}\u{2069}");
        assert_eq!(
            errs,
            alloc::vec![FormatError::UnknownFunction {
                name: "unknown".into()
            }]
        );
    }

    #[test]
    fn declaration_local_resolved() {
        // `{|Hello|}` is a literal operand with no function annotation, so
        // its direction is unknown — the Default Bidi Strategy wraps it
        // with FSI+PDI per formatting.md:871-874. `$who` is an input
        // variable that inherits the locale direction, so no wrap.
        let src = ".local $g = {|Hello|}\n{{{$g}, {$who}!}}";
        let (s, errs) = fmt(src, &[("who", "Ada")]);
        assert_eq!(s, "\u{2068}Hello\u{2069}, Ada!");
        assert!(errs.is_empty());
    }

    #[test]
    fn declaration_input_resolved() {
        let src = ".input {$who :string}\n{{Hi, {$who}!}}";
        let (s, errs) = fmt(src, &[("who", "Ada")]);
        assert_eq!(s, "Hi, \u{2068}Ada\u{2069}!");
        assert!(errs.is_empty());
    }

    #[test]
    fn variable_cached_once() {
        // Resolving `$x` twice in the same pattern hits the cache on the
        // second reference; behavior is indistinguishable from recomputing
        // for pure inputs, but this exercises the caching path.
        let (s, errs) = fmt("{$x}/{$x}", &[("x", "Y")]);
        assert_eq!(s, "Y/Y");
        assert!(errs.is_empty());
    }

    #[test]
    fn fallback_literal_quoted_with_escaped_pipe() {
        let (s, _errs) = fmt(r"{|a\|b| :unknown}", &[]);
        assert_eq!(s, "\u{2068}{|a\\|b|}\u{2069}");
    }

    #[test]
    fn markup_emits_no_text_in_format_to_string() {
        let (s, errs) = fmt("a{#b}bold{/b}c", &[]);
        assert_eq!(s, "aboldc");
        assert!(errs.is_empty());
    }

    // ---- selection ----

    #[test]
    fn select_string_exact_match() {
        let src = ".input {$color :string}\n\
                   .match $color\n\
                   red {{rouge}}\n\
                   blue {{bleu}}\n\
                   * {{autre}}";
        let (s, errs) = fmt(src, &[("color", "blue")]);
        assert_eq!(s, "bleu");
        assert!(errs.is_empty());
    }

    #[test]
    fn select_string_falls_back_to_catchall() {
        let src = ".input {$color :string}\n\
                   .match $color\n\
                   red {{rouge}}\n\
                   blue {{bleu}}\n\
                   * {{autre}}";
        let (s, errs) = fmt(src, &[("color", "green")]);
        assert_eq!(s, "autre");
        assert!(errs.is_empty());
    }

    #[test]
    fn select_nfc_equivalent_keys_match() {
        // The literal key uses combining-diaeresis form; the input uses
        // precomposed form. NFC normalization makes them equal.
        let src = ".input {$name :string}\n\
                   .match $name\n\
                   |A\u{0308}| {{diaeresis}}\n\
                   * {{other}}";
        let (s, errs) = fmt(src, &[("name", "\u{00C4}")]);
        assert_eq!(s, "diaeresis");
        assert!(errs.is_empty());
    }

    #[test]
    fn select_unresolved_selector_forces_catchall() {
        // `$missing` doesn't resolve → the inner UnresolvedVariable and
        // downstream BadOperand errors are already reported; no BadSelector
        // is added because the selector's failure is already surfaced.
        // Catchall variant wins.
        let src = ".local $missing = {$unbound :string}\n\
                   .match $missing\n\
                   red {{rouge}}\n\
                   * {{fallback}}";
        let (s, errs) = fmt(src, &[]);
        assert_eq!(s, "fallback");
        assert!(
            errs.iter()
                .any(|e| matches!(e, FormatError::UnresolvedVariable { .. })),
            "expected UnresolvedVariable among {errs:?}",
        );
    }

    #[test]
    fn select_multi_selector_literal_beats_catchall() {
        let src = ".input {$a :string} .input {$b :string}\n\
                   .match $a $b\n\
                   x y {{x-y}}\n\
                   x * {{x-any}}\n\
                   * * {{any-any}}";
        let (s, errs) = fmt(src, &[("a", "x"), ("b", "y")]);
        assert_eq!(s, "x-y");
        assert!(errs.is_empty());
    }

    #[test]
    fn select_partial_match_picks_catchall_on_second_axis() {
        let src = ".input {$a :string} .input {$b :string}\n\
                   .match $a $b\n\
                   x y {{x-y}}\n\
                   x * {{x-any}}\n\
                   * * {{any-any}}";
        let (s, errs) = fmt(src, &[("a", "x"), ("b", "z")]);
        assert_eq!(s, "x-any");
        assert!(errs.is_empty());
    }

    // ---- draft datetime (unstable) ----

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn datetime_handler_registers_under_unstable() {
        let f = MessageFormatter::builder()
            .source("On {$d :datetime} we met.")
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("d", "2026-04-19T12:00:00")];
        let (out, errs) = f.format_to_string(&inputs);
        assert_eq!(out, "On 2026 M04 19 12:00 we met.");
        assert!(errs.is_empty());
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn date_time_parts_carry_correct_kind() {
        let f = MessageFormatter::builder()
            .source("{$d :date}/{$t :time}/{$dt :datetime}")
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[
            ("d", "2026-04-19"),
            ("t", "12:00:00"),
            ("dt", "2026-04-19T12:00:00"),
        ];
        let (parts, errs) = f.format_to_parts(&inputs);
        assert!(errs.is_empty());
        let kinds: Vec<&str> = parts
            .iter()
            .filter_map(|p| match p {
                FormattedPart::Expression { kind, .. } => Some(kind.as_ref()),
                _ => None,
            })
            .collect();
        assert_eq!(kinds, alloc::vec!["date", "time", "datetime"]);
    }

    // ---- format_to_parts ----

    #[test]
    fn parts_plain_text() {
        let f = MessageFormatter::builder()
            .source("Hello, world!")
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[];
        let (parts, errs) = f.format_to_parts(&inputs);
        assert!(errs.is_empty());
        assert_eq!(
            parts,
            alloc::vec![FormattedPart::Text {
                value: "Hello, world!".into(),
            }]
        );
    }

    #[test]
    fn parts_text_plus_expression() {
        let f = MessageFormatter::builder()
            .source("Hello, {$user}!")
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("user", "Ada")];
        let (parts, errs) = f.format_to_parts(&inputs);
        assert!(errs.is_empty());
        assert_eq!(parts.len(), 3);
        assert_eq!(
            parts[0],
            FormattedPart::Text {
                value: "Hello, ".into()
            }
        );
        let FormattedPart::Expression {
            kind,
            value,
            id,
            direction,
        } = &parts[1]
        else {
            panic!("expected Expression part, got {:?}", parts[1]);
        };
        assert_eq!(kind.as_ref(), "string");
        assert_eq!(value, "Ada");
        assert!(id.is_none());
        // Input variables inherit the formatter locale's direction (LTR for
        // the default `und`). Structured parts surface this for consumers.
        assert_eq!(*direction, Some(Direction::Ltr));
        assert_eq!(parts[2], FormattedPart::Text { value: "!".into() });
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn parts_number_kind() {
        use crate::messageformat::input::OwnedInputs;
        let f = MessageFormatter::builder()
            .source("{$n :number}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 42_i64);
        let (parts, errs) = f.format_to_parts(&inputs);
        assert!(errs.is_empty());
        assert_eq!(parts.len(), 1);
        let FormattedPart::Expression { kind, value, .. } = &parts[0] else {
            panic!()
        };
        assert_eq!(kind.as_ref(), "number");
        assert_eq!(value, "42");
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn parts_integer_kind() {
        use crate::messageformat::input::OwnedInputs;
        let f = MessageFormatter::builder()
            .source("{$n :integer}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 42_i64);
        let (parts, _) = f.format_to_parts(&inputs);
        let FormattedPart::Expression { kind, .. } = &parts[0] else {
            panic!()
        };
        assert_eq!(kind.as_ref(), "integer");
    }

    #[test]
    fn parts_u_id_propagates() {
        let f = MessageFormatter::builder()
            .source("{$x :string u:id=my-id}")
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("x", "hi")];
        let (parts, _) = f.format_to_parts(&inputs);
        assert_eq!(parts.len(), 3);
        assert!(matches!(
            parts[0],
            FormattedPart::BidiIsolation { value: '\u{2068}' }
        ));
        let FormattedPart::Expression { id, .. } = &parts[1] else {
            panic!()
        };
        assert_eq!(id.as_deref(), Some("my-id"));
        assert!(matches!(
            parts[2],
            FormattedPart::BidiIsolation { value: '\u{2069}' }
        ));
    }

    #[test]
    fn parts_markup_emitted() {
        let f = MessageFormatter::builder()
            .source("a{#b}bold{/b}c")
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[];
        let (parts, _) = f.format_to_parts(&inputs);
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0], FormattedPart::Text { value: "a".into() });
        assert!(matches!(
            &parts[1],
            FormattedPart::Markup {
                kind: crate::messageformat::ast::MarkupKind::Open,
                name,
                ..
            } if name.as_ref() == "b"
        ));
        assert_eq!(
            parts[2],
            FormattedPart::Text {
                value: "bold".into()
            }
        );
        assert!(matches!(
            &parts[3],
            FormattedPart::Markup {
                kind: crate::messageformat::ast::MarkupKind::Close,
                name,
                ..
            } if name.as_ref() == "b"
        ));
        assert_eq!(parts[4], FormattedPart::Text { value: "c".into() });
    }

    #[test]
    fn parts_surface_direction_without_isolates() {
        // Structured parts mirror the same bidi isolation behavior as
        // string formatting, surfacing the isolate controls explicitly.
        let f = MessageFormatter::builder()
            .source("a {$x :string u:dir=rtl} b")
            .direction(Direction::Ltr)
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("x", "xxx")];
        let (parts, _) = f.format_to_parts(&inputs);
        assert_eq!(parts.len(), 5);
        let FormattedPart::Expression {
            value, direction, ..
        } = &parts[2]
        else {
            panic!()
        };
        assert_eq!(value, "xxx");
        assert_eq!(*direction, Some(Direction::Rtl));
        assert!(matches!(
            parts[1],
            FormattedPart::BidiIsolation { value: '\u{2067}' }
        ));
        assert!(matches!(
            parts[3],
            FormattedPart::BidiIsolation { value: '\u{2069}' }
        ));
    }

    #[test]
    fn parts_fallback_kind_on_unresolved() {
        let f = MessageFormatter::builder()
            .source("Hi, {$ghost}!")
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[];
        let (parts, errs) = f.format_to_parts(&inputs);
        assert_eq!(errs.len(), 1);
        assert_eq!(parts.len(), 5);
        assert!(matches!(
            parts[1],
            FormattedPart::BidiIsolation { value: '\u{2068}' }
        ));
        let FormattedPart::Expression { kind, value, .. } = &parts[2] else {
            panic!()
        };
        assert_eq!(kind.as_ref(), "fallback");
        assert_eq!(value, "{$ghost}");
        assert!(matches!(
            parts[3],
            FormattedPart::BidiIsolation { value: '\u{2069}' }
        ));
    }

    // ---- u: namespace + bidi ----

    #[test]
    fn u_dir_ltr_in_rtl_message_wraps_with_lri() {
        let formatter = MessageFormatter::builder()
            .source("a {$x :string u:dir=ltr} b")
            .direction(Direction::Rtl)
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("x", "world")];
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "a \u{2066}world\u{2069} b");
        assert!(errs.is_empty());
    }

    #[test]
    fn u_dir_rtl_in_ltr_message_wraps_with_rli() {
        let formatter = MessageFormatter::builder()
            .source("a {$x :string u:dir=rtl} b")
            .direction(Direction::Ltr)
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("x", "שלום")];
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "a \u{2067}שלום\u{2069} b");
        assert!(errs.is_empty());
    }

    #[test]
    fn u_dir_auto_always_isolates_with_fsi() {
        let formatter = MessageFormatter::builder()
            .source("{$x :string u:dir=auto}")
            .direction(Direction::Ltr)
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("x", "?")];
        let (out, _errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "\u{2068}?\u{2069}");
    }

    #[test]
    fn u_dir_inherit_does_not_isolate() {
        let formatter = MessageFormatter::builder()
            .source("{$x :string u:dir=inherit}")
            .direction(Direction::Rtl)
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("x", "xxx")];
        let (out, _errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "xxx");
    }

    #[test]
    fn matching_direction_still_wraps_under_explicit_u_dir() {
        // Per spec u-namespace.md:50, an explicit u:dir always isolates
        // (with LRI / RLI), even when it matches the base direction.
        let formatter = MessageFormatter::builder()
            .source("{$x :string u:dir=ltr}")
            .direction(Direction::Ltr)
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("x", "world")];
        let (out, _errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "\u{2066}world\u{2069}");
    }

    #[test]
    fn bidi_isolation_disabled() {
        let formatter = MessageFormatter::builder()
            .source("{$x :string u:dir=ltr}")
            .direction(Direction::Rtl)
            .bidi_isolation(false)
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("x", "world")];
        let (out, _errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "world");
    }

    #[test]
    fn u_dir_invalid_value_is_ignored() {
        let formatter = MessageFormatter::builder()
            .source("{$x :string u:dir=sideways}")
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("x", "y")];
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "\u{2068}y\u{2069}");
        assert_eq!(errs.len(), 1);
        assert!(matches!(
            &errs[0],
            FormatError::FunctionError {
                function,
                error: FunctionError::BadOption { name }
            } if function.as_ref() == "string" && name.as_ref() == "u:dir"
        ));
    }

    #[test]
    fn u_id_stripped_from_options_and_does_not_affect_output() {
        // u:id is silently preserved on the ResolvedValue but contributes
        // nothing to text output — the :string handler sees an empty
        // options map. Direction is unknown → FSI+PDI per Default Bidi.
        let formatter = MessageFormatter::builder()
            .source("{$x :string u:id=my-id}")
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("x", "hello")];
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "\u{2068}hello\u{2069}");
        assert!(errs.is_empty());
    }

    // ---- :number and :integer ----

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_formats_with_locale_grouping() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("You have {$n :number} messages")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 12_345_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "You have 12,345 messages");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_select_plural_one() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source(
                ".input {$n :integer}\n\
                 .match $n\n\
                 0 {{zero items}}\n\
                 one {{one item}}\n\
                 * {{{$n} items}}",
            )
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 1_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "one item");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_select_exact_zero_beats_plural_other() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source(
                ".input {$n :integer}\n\
                 .match $n\n\
                 0 {{no items}}\n\
                 one {{one item}}\n\
                 * {{{$n} items}}",
            )
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 0_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "no items");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_select_catchall() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source(
                ".input {$n :integer}\n\
                 .match $n\n\
                 0 {{zero}}\n\
                 one {{one}}\n\
                 * {{many}}",
            )
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 5_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "many");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn integer_rounds_half_expand() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("value is {$n :integer}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_str("n", "3.7");
        let (out, errs) = formatter.format_to_string(&inputs);
        // :integer rounds (not truncates) per ECMA-402 default halfExpand.
        assert_eq!(out, "value is 4");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn percent_multiplies_by_100_and_appends_sign() {
        use crate::messageformat::input::OwnedInputs;
        let f = MessageFormatter::builder()
            .source("{$x :percent maximumFractionDigits=1}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_str("x", "0.1234");
        let (out, errs) = f.format_to_string(&inputs);
        assert_eq!(out, "12.3%");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn percent_uses_locale_percent_pattern() {
        use crate::messageformat::input::OwnedInputs;
        let f = MessageFormatter::builder()
            .source("{$x :percent maximumFractionDigits=1}")
            .locale(locale!("tr"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_str("x", "0.1234");
        let (out, errs) = f.format_to_string(&inputs);
        assert_eq!(out, "%12,3");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn percent_selects_by_scaled_plural() {
        use crate::messageformat::input::OwnedInputs;
        // 1 :percent → 100%, so plural category is "other" in English.
        let src = ".local $pct = {$n :percent}\n\
                   .match $pct\n\
                   one {{one}}\n\
                   * {{other}}";
        let f = MessageFormatter::builder()
            .source(src)
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 1_i64);
        let (out, errs) = f.format_to_string(&inputs);
        assert_eq!(out, "other");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn percent_composition_keeps_unscaled_numeric_value() {
        let f = MessageFormatter::builder()
            .source(".local $n = {0.01 :percent} {{{$n :percent}}}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[];
        let (out, errs) = f.format_to_string(&inputs);
        assert_eq!(out, "1%");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn currency_formats_usd_in_en() {
        use crate::messageformat::input::OwnedInputs;
        let f = MessageFormatter::builder()
            .source("{$p :currency currency=USD}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_str("p", "12345.67");
        let (out, errs) = f.format_to_string(&inputs);
        assert!(out.contains("12,345.67"), "got: {out}");
        assert!(out.contains('$'), "got: {out}");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn currency_requires_currency_option() {
        let f = MessageFormatter::builder()
            .source("{$p :currency}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("p", "10")];
        let (_out, errs) = f.format_to_string(&inputs);
        assert!(matches!(
            &errs[0],
            FormatError::FunctionError {
                error: FunctionError::BadOperand,
                ..
            }
        ));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn currency_rejects_malformed_code() {
        let f = MessageFormatter::builder()
            .source("{$p :currency currency=XX}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("p", "10")];
        let (_out, errs) = f.format_to_string(&inputs);
        assert!(matches!(
            &errs[0],
            FormatError::FunctionError {
                error: FunctionError::BadOption { name },
                ..
            } if name.as_ref() == "currency"
        ));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn offset_subtract_for_plural_selection() {
        use crate::messageformat::input::OwnedInputs;
        let src = ".input {$n :integer}\n\
                   .local $others = {$n :offset subtract=1}\n\
                   .match $others\n\
                   0   {{only you}}\n\
                   one {{and one other}}\n\
                   *   {{and {$others} others}}";
        let f = MessageFormatter::builder()
            .source(src)
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 2_i64);
        let (out, errs) = f.format_to_string(&inputs);
        assert_eq!(out, "and one other");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn offset_requires_exactly_one_option() {
        let f = MessageFormatter::builder()
            .source("{$n :offset}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("n", "5")];
        let (_out, errs) = f.format_to_string(&inputs);
        assert!(matches!(
            &errs[0],
            FormatError::FunctionError {
                error: FunctionError::BadOption { .. },
                ..
            }
        ));
    }

    #[test]
    fn markup_exposes_options_id_and_rejects_u_dir() {
        // Per spec u-namespace.md:51-52, u:dir on markup emits Bad Option
        // and is ignored; u:id is still surfaced. Other options pass through.
        let f = MessageFormatter::builder()
            .source("{#link href=|/x| u:id=main u:dir=ltr}click{/link}")
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[];
        let (parts, errs) = f.format_to_parts(&inputs);
        assert_eq!(errs.len(), 1);
        assert!(matches!(
            &errs[0],
            FormatError::FunctionError {
                function,
                error: FunctionError::BadOption { name },
            } if function.as_ref() == "link" && name.as_ref() == "u:dir"
        ));
        // Find the open-markup part and inspect its contents.
        let open = parts
            .iter()
            .find(|p| {
                matches!(
                    p,
                    FormattedPart::Markup {
                        kind: crate::messageformat::ast::MarkupKind::Open,
                        ..
                    }
                )
            })
            .expect("missing open-markup part");
        let FormattedPart::Markup {
            name,
            options,
            id,
            direction,
            ..
        } = open
        else {
            unreachable!()
        };
        assert_eq!(name.as_ref(), "link");
        assert_eq!(options.get("href").map(String::as_str), Some("/x"));
        assert!(!options.contains_key("u:id"));
        assert!(!options.contains_key("u:dir"));
        assert_eq!(id.as_deref(), Some("main"));
        assert_eq!(*direction, None);
    }

    #[test]
    fn markup_variable_option_resolves() {
        let f = MessageFormatter::builder()
            .source("{#a href=$url}ok{/a}")
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("url", "/docs")];
        let (parts, errs) = f.format_to_parts(&inputs);
        assert!(errs.is_empty());
        let FormattedPart::Markup { options, .. } = parts
            .iter()
            .find(|p| {
                matches!(
                    p,
                    FormattedPart::Markup {
                        kind: crate::messageformat::ast::MarkupKind::Open,
                        ..
                    }
                )
            })
            .unwrap()
        else {
            unreachable!()
        };
        assert_eq!(options.get("href").map(String::as_str), Some("/docs"));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn negative_number_literal_as_option_value() {
        // ABNF number-literal allows a leading minus.
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number minimumIntegerDigits=3}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", -7_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "-007");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn decimal_and_exponent_number_literal_selection() {
        // Decimal and exponent number literals are valid variant keys.
        let formatter = MessageFormatter::builder()
            .source(
                ".input {$n :number}\n\
                 .match $n\n\
                 1.5 {{half}}\n\
                 1e2 {{hundred}}\n\
                 * {{other}}",
            )
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs1: &[(&str, &str)] = &[("n", "1.5")];
        let (out1, _) = formatter.format_to_string(&inputs1);
        assert_eq!(out1, "half");
        let inputs2: &[(&str, &str)] = &[("n", "100")];
        let (out2, _) = formatter.format_to_string(&inputs2);
        assert_eq!(out2, "hundred");
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_use_grouping_never() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number useGrouping=never}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 12_345_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "12345");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_sign_display_always() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number signDisplay=always}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 42_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "+42");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_min_fraction_digits_pads_with_zeros() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number minimumFractionDigits=2}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 3_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "3.00");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_max_fraction_digits_rounds() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number maximumFractionDigits=2}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_str("n", "3.14159");
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "3.14");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_min_integer_digits_pads_left() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number minimumIntegerDigits=3}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 5_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "005");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_rounding_mode_floor() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number maximumFractionDigits=0 roundingMode=floor}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_str("n", "3.7");
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "3");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_max_significant_digits_rounds() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number maximumSignificantDigits=3}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 12_345_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "12,300");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_min_significant_digits_pads() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number minimumSignificantDigits=5}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 12_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "12.000");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_rounding_increment_applies() {
        use crate::messageformat::input::OwnedInputs;
        // roundingIncrement=5, maxFractionDigits=2 → round to nearest 0.05.
        let formatter = MessageFormatter::builder()
            .source("{$n :number maximumFractionDigits=2 roundingIncrement=5}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_str("n", "1.23");
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "1.25");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_trailing_zero_display_strip_if_integer() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number minimumFractionDigits=2 trailingZeroDisplay=stripIfInteger}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 3_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "3");
        assert!(errs.is_empty());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_invalid_rounding_increment_errors() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number roundingIncrement=3}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 1_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "\u{2068}{$n}\u{2069}");
        assert!(matches!(
            &errs[0],
            FormatError::FunctionError {
                error: FunctionError::BadOption { name },
                ..
            } if name.as_ref() == "roundingIncrement"
        ));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_bad_option_value_yields_fallback() {
        use crate::messageformat::input::OwnedInputs;
        let formatter = MessageFormatter::builder()
            .source("{$n :number signDisplay=bogus}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 1_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "\u{2068}{$n}\u{2069}");
        assert!(matches!(
            &errs[0],
            FormatError::FunctionError {
                function,
                error: FunctionError::BadOption { name }
            } if function.as_ref() == "number" && name.as_ref() == "signDisplay"
        ));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_select_bad_variant_key_surfaces_error() {
        use crate::messageformat::input::OwnedInputs;
        // `dog` is neither a number literal nor a plural category —
        // the :number selector must surface a Bad Variant Key and the
        // catchall variant should win.
        let formatter = MessageFormatter::builder()
            .source(
                ".input {$n :integer}\n\
                 .match $n\n\
                 dog {{invalid}}\n\
                 * {{fallback}}",
            )
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs = OwnedInputs::new().with_number("n", 2_i64);
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "fallback");
        assert_eq!(errs.len(), 1);
        assert!(matches!(
            &errs[0],
            FormatError::FunctionError {
                function,
                error: FunctionError::BadVariantKey { key },
            } if function.as_ref() == "integer" && key.as_ref() == "dog"
        ));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_bad_operand_yields_fallback() {
        let formatter = MessageFormatter::builder()
            .source("n={$n :number}")
            .locale(locale!("en"))
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("n", "abc")];
        let (out, errs) = formatter.format_to_string(&inputs);
        assert_eq!(out, "n=\u{2068}{$n}\u{2069}");
        assert!(matches!(errs[0], FormatError::FunctionError { .. }));
    }

    #[test]
    fn select_transitive_local_declaration() {
        // `$derived` is a local that references `$raw` (which has the
        // function annotation). Selecting on `$derived` works because the
        // selector propagates through the local chain.
        let src = ".input {$raw :string}\n\
                   .local $derived = {$raw}\n\
                   .match $derived\n\
                   foo {{matched-foo}}\n\
                   * {{default}}";
        let (s, errs) = fmt(src, &[("raw", "foo")]);
        assert_eq!(s, "matched-foo");
        assert!(errs.is_empty());
    }

    #[test]
    fn function_handler_error_yields_fallback() {
        // Register a handler that always fails, then format a message that
        // invokes it.
        #[derive(Debug)]
        struct Broken;
        impl crate::messageformat::function::FunctionHandler for Broken {
            fn format(
                &self,
                _ctx: &crate::messageformat::function::FunctionContext<'_>,
                _op: Option<&crate::messageformat::value::ResolvedValue>,
                _o: &crate::messageformat::function::FunctionOptions,
            ) -> Result<crate::messageformat::value::ResolvedValue, FunctionError> {
                Err(FunctionError::UnsupportedOperation)
            }
        }
        let f = MessageFormatter::builder()
            .source("{$x :broken}")
            .function("broken", Broken)
            .locale_undetermined()
            .build()
            .unwrap();
        let inputs: &[(&str, &str)] = &[("x", "v")];
        let (s, errs) = f.format_to_string(&inputs);
        assert_eq!(s, "\u{2068}{$x}\u{2069}");
        assert_eq!(errs.len(), 1);
        assert!(matches!(
            &errs[0],
            FormatError::FunctionError {
                function,
                error: FunctionError::UnsupportedOperation,
            } if function.as_ref() == "broken"
        ));
    }

    #[test]
    fn builder_requires_message() {
        let err = MessageFormatter::builder().build().unwrap_err();
        assert_eq!(err, BuildError::NoMessage);
    }

    #[test]
    fn builder_requires_locale() {
        let err = MessageFormatter::builder()
            .source("Hello")
            .build()
            .unwrap_err();
        assert_eq!(err, BuildError::MissingLocale);
    }

    #[test]
    fn builder_undetermined_locale_opt_in() {
        let f = MessageFormatter::builder()
            .source("Hello")
            .locale_undetermined()
            .build()
            .unwrap();
        assert_eq!(f.locale().to_string(), "und");
    }

    #[test]
    fn no_message_takes_precedence_over_missing_locale() {
        // Pins the check order inside `.build()` — the input check runs
        // first, so a fully-empty builder surfaces `NoMessage`, not
        // `MissingLocale`.
        let err = MessageFormatter::builder().build().unwrap_err();
        assert_eq!(err, BuildError::NoMessage);
    }

    #[test]
    fn builder_propagates_parse_errors() {
        let err = MessageFormatter::builder()
            .source("{$x")
            .locale_undetermined()
            .build()
            .unwrap_err();
        assert!(matches!(err, BuildError::Parse(_)));
    }

    #[test]
    fn builder_propagates_validation_errors() {
        let err = MessageFormatter::builder()
            .source(".input {$x :integer}\n.match $x\n1 {{one}}")
            .locale_undetermined()
            .build()
            .unwrap_err();
        assert!(matches!(err, BuildError::Validation(_)));
    }
}

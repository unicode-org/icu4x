// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Expression and variable resolution.
//!
//! The resolver walks declarations and expressions, consults [`InputValues`]
//! for variable bindings, invokes [`FunctionHandler`]s from the
//! [`FunctionRegistry`], and emits [`FormatError`]s without ever aborting.
//! Each variable name is evaluated at most once per format call (call-by-need
//! per the spec) via an internal cache.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use icu_locale_core::Locale;

use super::ast::{Arg, Declaration, Expression, OptionMap, OptionValue};
use super::bidi::Direction;
use super::error::{FormatError, FunctionError};
use super::function::{FunctionContext, FunctionOptions, FunctionRegistry};
use super::input::{InputValue, InputValues};
use super::value::ResolvedValue;

/// Resolver state for a single `format()` call.
pub(crate) struct Resolver<'a, V: InputValues + ?Sized> {
    declarations: &'a [Declaration],
    inputs: &'a V,
    registry: &'a FunctionRegistry,
    locale: &'a Locale,
    base_direction: Direction,
    cache: BTreeMap<Box<str>, ResolvedValue>,
    in_progress: Vec<Box<str>>,
    errors: Vec<FormatError>,
}

impl<'a, V: InputValues + ?Sized> Resolver<'a, V> {
    pub(crate) fn new(
        declarations: &'a [Declaration],
        inputs: &'a V,
        registry: &'a FunctionRegistry,
        locale: &'a Locale,
        base_direction: Direction,
    ) -> Self {
        Self {
            declarations,
            inputs,
            registry,
            locale,
            base_direction,
            cache: BTreeMap::new(),
            in_progress: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Consume the resolver and return the accumulated errors.
    pub(crate) fn into_errors(self) -> Vec<FormatError> {
        self.errors
    }

    /// Resolve a variable reference. Exposed for the formatter's selector
    /// resolution path.
    pub(crate) fn resolve_variable_pub(&mut self, name: &str) -> ResolvedValue {
        self.resolve_variable(name)
    }

    /// Record a [`FormatError`] without resolving anything.
    pub(crate) fn push_error(&mut self, err: FormatError) {
        self.errors.push(err);
    }

    /// Resolve a top-level expression in a pattern.
    pub(crate) fn resolve_expression(&mut self, expr: &Expression) -> ResolvedValue {
        let operand = match &expr.arg {
            Some(Arg::Variable(v)) => Some(self.resolve_variable(&v.name)),
            // Literal operands carry no inherent direction — unknown until a
            // function annotation overrides. This preserves the spec's
            // Default Bidi Strategy FSI+PDI wrap for `{literal}` placeholders.
            Some(Arg::Literal(l)) => Some(ResolvedValue::new(l.value.clone())),
            None => None,
        };
        self.apply_function(expr, operand)
    }

    /// Apply `expr`'s function annotation (if any) to an already-resolved
    /// operand. Used directly for `.input` declarations, which bypass the
    /// recursive `resolve_variable` call on the self-referencing arg.
    fn apply_function(
        &mut self,
        expr: &Expression,
        operand: Option<ResolvedValue>,
    ) -> ResolvedValue {
        let Some(func_ref) = &expr.function else {
            // Bare operand: propagate fallback or return as-is.
            if let Some(ref op) = operand {
                if op.is_fallback() {
                    return self.fallback_for_expression(expr);
                }
            }
            return operand.unwrap_or_else(|| self.fallback_for_expression(expr));
        };

        let Some(handler) = self.registry.get(&func_ref.name) else {
            self.errors.push(FormatError::UnknownFunction {
                name: func_ref.name.clone(),
            });
            return self.fallback_for_expression(expr);
        };

        // Handlers are responsible for accepting or rejecting fallback
        // operands. Most (`:number`, `:integer`, `:test:*`, `:datetime` …)
        // reject them with `BadOperand`; `:string` tolerates any operand
        // and the spec expects it to succeed on an unresolved input (so
        // `.match` picks the catchall without an additional `BadSelector`).
        // Either way, any underlying cause (`UnresolvedVariable`, …) is
        // already recorded by operand resolution.
        // Enforce spec literal-only options per function (e.g. `:number`
        // requires `select` to be a literal per number.md §Number Selection).
        // When a required-literal option is supplied as a variable, emit
        // BadOption and drop the option so the function formats without
        // selection support.
        let literal_only = literal_only_options(func_ref.name.as_ref());
        let mut disable_selection = false;
        for name in literal_only {
            if matches!(func_ref.options.get(*name), Some(OptionValue::Variable(_))) {
                self.errors.push(FormatError::FunctionError {
                    function: func_ref.name.clone(),
                    error: FunctionError::BadOption {
                        name: Box::from(*name),
                    },
                });
                if *name == "select" {
                    disable_selection = true;
                }
            }
        }
        // Per spec (number.md §Number Selection): `select` MUST be a literal
        // on the expression itself. If it's instead inherited from the
        // operand's resolved options, emit Bad Option and disable selection.
        if matches!(func_ref.name.as_ref(), "number" | "integer")
            && !func_ref.options.contains_key("select")
        {
            if let Some(op) = operand.as_ref() {
                if op.resolved_options().contains_key("select") {
                    self.errors.push(FormatError::FunctionError {
                        function: func_ref.name.clone(),
                        error: FunctionError::BadOption {
                            name: "select".into(),
                        },
                    });
                    disable_selection = true;
                }
            }
        }
        let mut options = self.resolve_options(&func_ref.options);
        for name in literal_only {
            if matches!(func_ref.options.get(*name), Some(OptionValue::Variable(_))) {
                options.remove(*name);
            }
        }

        // Strip u: options before calling the handler. u:id is opaque
        // metadata; u:dir overrides the value's direction; u:locale overrides
        // the formatting locale for this expression only. Invalid values
        // emit BadOption; `u:dir` is ignored per u-namespace.md, while
        // invalid `u:locale` still forces the expression to fall back.
        //
        // Spec: u-namespace.md defines `u:id`, `u:dir`, and (Draft) `u:locale`.
        // Other u:* options reach the handler unchanged.
        let u_id = match options.remove("u:id") {
            Some(v) => {
                // Spec: u:id value must be a literal or a variable resolving
                // to a string without error. A fallback resolved value means
                // resolution errored; that's a Bad Option per u-namespace.md.
                if v.is_fallback() {
                    self.errors.push(FormatError::FunctionError {
                        function: func_ref.name.clone(),
                        error: FunctionError::BadOption {
                            name: "u:id".into(),
                        },
                    });
                    None
                } else {
                    Some(v.text().into())
                }
            }
            None => None,
        };
        let u_dir = match options.remove("u:dir") {
            Some(v) => match Direction::from_option(v.text()) {
                Some(d) => Some(d),
                None => {
                    self.errors.push(FormatError::FunctionError {
                        function: func_ref.name.clone(),
                        error: FunctionError::BadOption {
                            name: "u:dir".into(),
                        },
                    });
                    None
                }
            },
            None => None,
        };
        let u_locale_override = match options.remove("u:locale") {
            Some(v) => match parse_u_locale(v.text()) {
                Some(loc) => Some(loc),
                None => {
                    self.errors.push(FormatError::FunctionError {
                        function: func_ref.name.clone(),
                        error: FunctionError::BadOption {
                            name: "u:locale".into(),
                        },
                    });
                    return self.fallback_for_expression(expr);
                }
            },
            None => None,
        };

        let mut ctx = FunctionContext::new(self.locale).with_base_direction(self.base_direction);
        if let Some(loc) = u_locale_override {
            ctx = ctx.with_locale_override(loc);
        }
        let mut value = match handler.format(&ctx, operand.as_ref(), &options) {
            Ok(v) => v,
            Err(err) => {
                self.errors.push(FormatError::FunctionError {
                    function: func_ref.name.clone(),
                    error: err,
                });
                return self.fallback_for_expression(expr);
            }
        };
        if let Some(id) = u_id {
            value = value.with_u_id(id);
        }
        if let Some(dir) = u_dir {
            value = value.with_direction(dir);
        }
        // Else: leave the direction as whatever the function attached, or
        // `None` (unknown) if it didn't. The Default Bidi Strategy wraps
        // unknown-direction values with FSI+PDI per spec formatting.md:871.
        if disable_selection {
            value = value.without_selector();
        }
        // Tag the value with the function name so the matcher can attribute
        // any selector errors (e.g. Bad Variant Key) to this function.
        if value.selector().is_some() {
            value = value.with_selector_function(func_ref.name.clone());
        }
        value
    }

    fn resolve_variable(&mut self, name: &str) -> ResolvedValue {
        if let Some(cached) = self.cache.get(name) {
            return cached.clone();
        }
        let name_boxed: Box<str> = Box::from(name);

        // Cycle guard — happens when a `.local` references itself directly
        // or through a chain. The validator can't always catch this, so we
        // defend at runtime.
        if self.in_progress.contains(&name_boxed) {
            let fb = ResolvedValue::fallback(format!("{{${name}}}"));
            self.errors.push(FormatError::UnresolvedVariable {
                name: name_boxed.clone(),
            });
            return fb;
        }

        let resolved = if let Some(decl) = self.find_declaration(name) {
            match decl {
                Declaration::Input { value, .. } => {
                    // For `.input`, the declaration's arg is `$name` itself;
                    // resolve it directly against the input map to avoid
                    // the recursive self-reference through `resolve_variable`.
                    // When the input is missing, surface `UnresolvedVariable`
                    // but still run any function annotation on the resulting
                    // fallback operand — the spec requires that `:fn` applied
                    // to an unresolved input produce a `BadOperand` error on
                    // top of the underlying cause, which in turn lets a
                    // downstream `.match` emit `BadSelector`.
                    let operand = match self.inputs.get(name) {
                        Some(v) => Some(input_to_resolved(&v, self.locale)),
                        None => {
                            self.errors.push(FormatError::UnresolvedVariable {
                                name: name_boxed.clone(),
                            });
                            Some(ResolvedValue::fallback(format!("{{${name}}}")))
                        }
                    };
                    self.apply_function(value, operand)
                }
                Declaration::Local { value, .. } => {
                    self.in_progress.push(name_boxed.clone());
                    let r = self.resolve_expression(value);
                    self.in_progress.pop();
                    r
                }
            }
        } else if let Some(v) = self.inputs.get(name) {
            input_to_resolved(&v, self.locale)
        } else {
            self.errors.push(FormatError::UnresolvedVariable {
                name: name_boxed.clone(),
            });
            ResolvedValue::fallback(format!("{{${name}}}"))
        };
        self.cache.insert(name_boxed, resolved.clone());
        resolved
    }

    fn find_declaration(&self, name: &str) -> Option<&'a Declaration> {
        self.declarations
            .iter()
            .find(|d| declaration_name(d) == name)
    }

    /// Expose option resolution to the formatter so markup elements can
    /// also resolve their option values. Emits `UnresolvedVariable` into
    /// the error list on unbound `$var` references.
    pub(crate) fn resolve_options_pub(&mut self, opts: &OptionMap) -> FunctionOptions {
        self.resolve_options(opts)
    }

    fn resolve_options(&mut self, opts: &OptionMap) -> FunctionOptions {
        let mut out = FunctionOptions::new();
        for (k, v) in opts {
            let value = match v {
                OptionValue::Literal(l) => ResolvedValue::new(l.value.clone()),
                OptionValue::Variable(var) => self.resolve_variable(&var.name),
            };
            out.insert(k.clone(), value);
        }
        out
    }

    fn fallback_for_expression(&self, expr: &Expression) -> ResolvedValue {
        // Per spec:
        //   {|literal| :fn ...} → {|literal|}
        //   {unquoted :fn ...} → {|unquoted|}  (our parser loses the quoted
        //       flag, so we conservatively always emit quoted form)
        //   {$name :fn ...}    → {$name}
        //   {:fn ...}          → {:fn}
        let s = match (&expr.arg, &expr.function) {
            (Some(Arg::Literal(l)), _) => format!("{{|{}|}}", escape_literal(&l.value)),
            (Some(Arg::Variable(v)), _) => format!("{{${}}}", v.name),
            (None, Some(f)) => format!("{{:{}}}", f.name),
            (None, None) => String::from("{}"),
        };
        // If a function annotation is present, the fallback was produced by
        // the function pipeline — tag it so selectors surface a Bad Selector
        // error on top of the underlying cause.
        if expr.function.is_some() {
            ResolvedValue::fallback_from_function_error(s)
        } else {
            ResolvedValue::fallback(s)
        }
    }
}

/// Option names that must be supplied as literals (not variables) per spec.
fn literal_only_options(function: &str) -> &'static [&'static str] {
    match function {
        "number" | "integer" | "percent" => &["select"],
        "date" => &["fields", "length"],
        "time" => &["precision", "timeZoneStyle"],
        "datetime" => &["dateFields", "dateLength", "timePrecision", "timeZoneStyle"],
        _ => &[],
    }
}

fn escape_literal(value: &str) -> String {
    // Escape `\` and `|` per the MF2 quoted-literal escape rules.
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '\\' | '|' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

fn declaration_name(decl: &Declaration) -> &str {
    match decl {
        Declaration::Input { name, .. } | Declaration::Local { name, .. } => name.as_ref(),
    }
}

fn input_to_resolved(v: &InputValue<'_>, locale: &Locale) -> ResolvedValue {
    let dir = locale_direction(locale);
    match v {
        InputValue::Number(n) => {
            let text = format_number_default(locale, n);
            ResolvedValue::new(text)
                .with_numeric(n.clone())
                .with_inferred_direction(dir)
        }
        InputValue::Currency { value, currency } => {
            let text = format_number_default(locale, value);
            ResolvedValue::new(text)
                .with_numeric(value.clone())
                .with_resolved_option("currency", *currency)
                .with_inferred_direction(dir)
        }
        InputValue::Unit { value, unit } => {
            let text = format_number_default(locale, value);
            ResolvedValue::new(text)
                .with_numeric(value.clone())
                .with_resolved_option("unit", *unit)
                .with_inferred_direction(dir)
        }
        #[cfg(all(feature = "unstable", feature = "compiled_data"))]
        InputValue::Date(_)
        | InputValue::Time(_)
        | InputValue::DateTime(_)
        | InputValue::ZonedDateTime(_)
        | InputValue::ZonedTime(_) => {
            let text = v.to_display_string();
            ResolvedValue::new(text.clone())
                .with_datetime_source(text)
                .with_inferred_direction(dir)
        }
        _ => ResolvedValue::new(v.to_display_string()).with_inferred_direction(dir),
    }
}

#[cfg(feature = "compiled_data")]
fn format_number_default(locale: &Locale, n: &fixed_decimal::Decimal) -> String {
    use icu_decimal::{options::DecimalFormatterOptions, DecimalFormatter};
    use writeable::Writeable;
    if let Ok(fmter) = DecimalFormatter::try_new(locale.into(), DecimalFormatterOptions::default())
    {
        fmter.format(n).write_to_string().into_owned()
    } else {
        n.to_string()
    }
}

#[cfg(not(feature = "compiled_data"))]
fn format_number_default(_locale: &Locale, n: &fixed_decimal::Decimal) -> String {
    n.to_string()
}

/// Parse a `u:locale` option value. Per u-namespace.md the value is a
/// single locale identifier or a whitespace-separated list (first valid
/// wins). Returns `None` on parse failure, triggering a `BadOption` error
/// at the call site.
fn parse_u_locale(value: &str) -> Option<Locale> {
    use core::str::FromStr;
    for tag in value.split_whitespace() {
        if let Ok(loc) = Locale::from_str(tag) {
            return Some(loc);
        }
    }
    None
}

/// Look up the direction of `locale` via `LocaleDirectionality`. Falls back
/// to LTR when the locale has no associated script or the direction is
/// otherwise undetermined (e.g. `und`).
pub(crate) fn locale_direction(locale: &Locale) -> Direction {
    #[cfg(feature = "compiled_data")]
    {
        use icu_locale::{Direction as LocDir, LocaleDirectionality};
        const LD: LocaleDirectionality = LocaleDirectionality::new_common();
        match LD.get(&locale.id) {
            Some(LocDir::RightToLeft) => Direction::Rtl,
            _ => Direction::Ltr,
        }
    }
    #[cfg(not(feature = "compiled_data"))]
    {
        let _ = locale;
        Direction::Ltr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locale_core::locale;

    #[test]
    fn parse_u_locale_single_tag() {
        let loc = parse_u_locale("en-US").expect("valid BCP47");
        assert_eq!(loc.id.language.to_string(), "en");
    }

    #[test]
    fn parse_u_locale_whitespace_list_takes_first_valid() {
        // Per u-namespace.md, the value is a space-separated priority list;
        // first valid wins.
        let loc = parse_u_locale("!!!invalid fr-FR de-DE").expect("second tag ok");
        assert_eq!(loc.id.language.to_string(), "fr");
    }

    #[test]
    fn parse_u_locale_all_invalid_returns_none() {
        // Both tags malformed per BCP47 — triggers BadOption at call site.
        assert!(parse_u_locale("!!! ???").is_none());
    }

    #[test]
    fn parse_u_locale_empty_string_returns_none() {
        assert!(parse_u_locale("").is_none());
    }

    #[test]
    fn parse_u_locale_with_script_and_region() {
        let loc = parse_u_locale("zh-Hant-TW").expect("valid");
        assert_eq!(loc.id.language.to_string(), "zh");
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn locale_direction_ltr() {
        assert_eq!(locale_direction(&locale!("en")), Direction::Ltr);
        assert_eq!(locale_direction(&locale!("fr-FR")), Direction::Ltr);
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn locale_direction_rtl() {
        assert_eq!(locale_direction(&locale!("ar")), Direction::Rtl);
        assert_eq!(locale_direction(&locale!("he")), Direction::Rtl);
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn locale_direction_und_defaults_to_ltr() {
        // LocaleDirectionality returns None for `und`; spec silently maps
        // that to LTR (matches the base-direction fallback in formatter.rs).
        assert_eq!(locale_direction(&locale!("und")), Direction::Ltr);
    }
}

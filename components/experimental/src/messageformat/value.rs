// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Resolved values — the "formattable" intermediate representation returned
//! by function handlers and the resolver.
//!
//! Carries displayable text, an optional [`SelectorImpl`] for `.match`
//! selection, an optional numeric operand for downstream numeric functions,
//! and directionality information (`u:dir`).

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;

use fixed_decimal::Decimal;

use super::bidi::Direction;
use super::selector::SelectorImpl;

/// A value that has been resolved to a displayable string, optionally
/// annotated with a [`SelectorImpl`] for `.match` selection and/or a
/// typed numeric operand for further function composition.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ResolvedValue {
    text: Box<str>,
    is_fallback: bool,
    /// Set when this fallback was caused by a function-level error
    /// (`UnknownFunction`, `BadOperand`, `BadOption`, …) rather than a plain
    /// `UnresolvedVariable`. Drives whether a selector resolving to this
    /// value should add a [`FormatError::BadSelector`] on top of the
    /// underlying cause.
    fallback_from_function_error: bool,
    selector: Option<Arc<dyn SelectorImpl>>,
    selector_function: Option<Box<str>>,
    numeric: Option<Decimal>,
    u_id: Option<Box<str>>,
    direction: Option<Direction>,
    /// True iff the direction was set via an explicit `u:dir` option (not
    /// inferred from locale). Drives the `isolate` flag in the Default Bidi
    /// Strategy per formatting.md:857-859.
    direction_explicit: bool,
    /// Preserved ISO source for chained `:date` / `:time` / `:datetime` (draft).
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    date_time_source: Option<Box<str>>,
    /// Resolved options that downstream functions inherit per
    /// `spec/functions/number.md`: _options on the expression take priority
    /// over any options of the operand_. Keyed by option name (without any
    /// namespace prefix); values are the resolved text form.
    resolved_options: BTreeMap<Box<str>, Box<str>>,
    part_kind: Box<str>,
}

impl ResolvedValue {
    /// Build a resolved value from its displayable text. No selector, no
    /// numeric operand.
    pub fn new(text: impl Into<Box<str>>) -> Self {
        Self {
            text: text.into(),
            is_fallback: false,
            fallback_from_function_error: false,
            selector: None,
            selector_function: None,
            numeric: None,
            u_id: None,
            direction: None,
            direction_explicit: false,
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            date_time_source: None,
            resolved_options: BTreeMap::new(),
            part_kind: Box::from("string"),
        }
    }

    /// Build a fallback representation of the form `{$name}`, `{|literal|}`,
    /// `{:fn}`, etc., as defined by the spec. Fallbacks are never
    /// selectable.
    pub fn fallback(text: impl Into<Box<str>>) -> Self {
        Self {
            text: text.into(),
            is_fallback: true,
            fallback_from_function_error: false,
            selector: None,
            selector_function: None,
            numeric: None,
            u_id: None,
            direction: None,
            direction_explicit: false,
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            date_time_source: None,
            resolved_options: BTreeMap::new(),
            part_kind: Box::from("fallback"),
        }
    }

    /// Construct a fallback value caused by a function-level error. Selector
    /// resolution will surface this as a [`super::FormatError::BadSelector`]
    /// in addition to the underlying function error.
    pub fn fallback_from_function_error(text: impl Into<Box<str>>) -> Self {
        let mut fb = Self::fallback(text);
        fb.fallback_from_function_error = true;
        fb
    }

    /// Whether this fallback was caused by a function-level error.
    pub fn is_function_error_fallback(&self) -> bool {
        self.fallback_from_function_error
    }

    /// Attach a selector to this value. Returns `self` for chaining.
    pub fn with_selector(mut self, selector: Arc<dyn SelectorImpl>) -> Self {
        self.selector = Some(selector);
        self
    }

    /// Strip any attached selector. Used when a function-level error
    /// (e.g. literal-only `select` supplied as a variable) must disable
    /// selection support while formatting proceeds normally.
    pub fn without_selector(mut self) -> Self {
        self.selector = None;
        self.selector_function = None;
        self
    }

    /// Attach a numeric operand. Returns `self` for chaining.
    pub fn with_numeric(mut self, n: Decimal) -> Self {
        self.numeric = Some(n);
        self
    }

    /// The displayable text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Whether this value is a fallback representation.
    pub fn is_fallback(&self) -> bool {
        self.is_fallback
    }

    /// Borrow the attached selector, if any.
    pub fn selector(&self) -> Option<&Arc<dyn SelectorImpl>> {
        self.selector.as_ref()
    }

    /// Tag this value with the name of the function that produced its
    /// selector. Used by the matcher to attribute `Bad Variant Key` errors.
    pub fn with_selector_function(mut self, name: impl Into<Box<str>>) -> Self {
        self.selector_function = Some(name.into());
        self
    }

    /// Name of the function that produced this value's selector, if any.
    pub fn selector_function(&self) -> Option<&str> {
        self.selector_function.as_deref()
    }

    /// Borrow the attached numeric operand, if any.
    pub fn numeric(&self) -> Option<&Decimal> {
        self.numeric.as_ref()
    }

    /// Attach a `u:id` tag — opaque identifier that the formatter
    /// preserves but does not display.
    pub fn with_u_id(mut self, id: Box<str>) -> Self {
        self.u_id = Some(id);
        self
    }

    /// Borrow the `u:id` tag, if any.
    pub fn u_id(&self) -> Option<&str> {
        self.u_id.as_deref()
    }

    /// Override this value's directionality (from a `u:dir` option).
    /// Marks the direction as explicit, which causes the Default Bidi
    /// Strategy to isolate even when the direction matches the base.
    pub fn with_direction(mut self, dir: Direction) -> Self {
        self.direction = Some(dir);
        self.direction_explicit = true;
        self
    }

    /// Attach an inferred direction (e.g. from the formatter's locale). Does
    /// not set the explicit-isolation flag.
    pub(crate) fn with_inferred_direction(mut self, dir: Direction) -> Self {
        self.direction = Some(dir);
        self.direction_explicit = false;
        self
    }

    /// Borrow the placeholder direction, if set. `None` means inherit.
    pub fn direction(&self) -> Option<Direction> {
        self.direction
    }

    /// Whether the placeholder's direction was set by an explicit `u:dir`
    /// option. Internal — drives the Default Bidi Strategy `isolate` flag.
    pub(crate) fn direction_explicit(&self) -> bool {
        self.direction_explicit
    }

    /// Attach the implementation-defined source value used by draft
    /// date/time handlers so downstream annotations can continue operating
    /// on the original operand rather than on localized display text.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    pub(crate) fn with_datetime_source(mut self, source: impl Into<Box<str>>) -> Self {
        self.date_time_source = Some(source.into());
        self
    }

    /// Borrow the preserved date/time source, if present.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    pub(crate) fn datetime_source(&self) -> Option<&str> {
        self.date_time_source.as_deref()
    }

    /// Attach a resolved option. Used by number-family handlers to expose
    /// options to downstream operand-inheriting annotations (see the
    /// spec example in `functions/number.md` "resolved options" note).
    pub fn with_resolved_option(
        mut self,
        name: impl Into<Box<str>>,
        value: impl Into<Box<str>>,
    ) -> Self {
        self.resolved_options.insert(name.into(), value.into());
        self
    }

    /// Replace the resolved-options map wholesale.
    pub fn with_resolved_options(mut self, opts: BTreeMap<Box<str>, Box<str>>) -> Self {
        self.resolved_options = opts;
        self
    }

    /// Borrow the resolved options map.
    pub fn resolved_options(&self) -> &BTreeMap<Box<str>, Box<str>> {
        &self.resolved_options
    }

    /// Override this value's part-kind label (e.g., from `"string"` to
    /// `"number"`). Used by [`format_to_parts`] to tag structured output.
    ///
    /// [`format_to_parts`]: super::MessageFormatter::format_to_parts
    pub fn with_part_kind(mut self, kind: impl Into<Box<str>>) -> Self {
        self.part_kind = kind.into();
        self
    }

    /// Borrow the part kind (default `"string"`, `"fallback"` for
    /// fallback values).
    pub fn part_kind(&self) -> &str {
        &self.part_kind
    }
}

impl From<&str> for ResolvedValue {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ResolvedValue {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<Box<str>> for ResolvedValue {
    fn from(s: Box<str>) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messageformat::error::FunctionError;

    #[derive(Debug)]
    struct DummySelector;
    impl SelectorImpl for DummySelector {
        fn rank(&self, _key: &str) -> Result<Option<usize>, FunctionError> {
            Ok(None)
        }
    }

    #[test]
    fn new_is_not_fallback_and_default_part_kind() {
        let v = ResolvedValue::new("hello");
        assert_eq!(v.text(), "hello");
        assert!(!v.is_fallback());
        assert_eq!(v.part_kind(), "string");
        assert!(v.selector().is_none());
        assert!(v.numeric().is_none());
        assert!(v.u_id().is_none());
        assert!(v.direction().is_none());
        assert!(!v.is_function_error_fallback());
        assert!(v.resolved_options().is_empty());
    }

    #[test]
    fn fallback_sets_flag_and_part_kind() {
        let v = ResolvedValue::fallback("{$x}");
        assert!(v.is_fallback());
        assert_eq!(v.part_kind(), "fallback");
        assert!(!v.is_function_error_fallback());
    }

    #[test]
    fn fallback_from_function_error_flags_cause() {
        let v = ResolvedValue::fallback_from_function_error("{:fn}");
        assert!(v.is_fallback());
        assert!(v.is_function_error_fallback());
        assert_eq!(v.part_kind(), "fallback");
    }

    #[test]
    fn with_selector_then_without_clears() {
        let sel = Arc::new(DummySelector) as Arc<dyn SelectorImpl>;
        let v = ResolvedValue::new("v")
            .with_selector(Arc::clone(&sel))
            .with_selector_function("number");
        assert!(v.selector().is_some());
        assert_eq!(v.selector_function(), Some("number"));
        let stripped = v.without_selector();
        assert!(stripped.selector().is_none());
        assert!(stripped.selector_function().is_none());
    }

    #[test]
    fn with_numeric_stores_decimal() {
        let d: Decimal = 5_i64.into();
        let v = ResolvedValue::new("5").with_numeric(d.clone());
        assert_eq!(v.numeric(), Some(&d));
    }

    #[test]
    fn with_u_id_stores_label() {
        let v = ResolvedValue::new("x").with_u_id(Box::from("my-id"));
        assert_eq!(v.u_id(), Some("my-id"));
    }

    #[test]
    fn with_direction_marks_explicit() {
        let v = ResolvedValue::new("x").with_direction(Direction::Rtl);
        assert_eq!(v.direction(), Some(Direction::Rtl));
        assert!(v.direction_explicit());
    }

    #[test]
    fn with_inferred_direction_not_explicit() {
        let v = ResolvedValue::new("x").with_inferred_direction(Direction::Ltr);
        assert_eq!(v.direction(), Some(Direction::Ltr));
        assert!(!v.direction_explicit());
    }

    #[test]
    fn explicit_overrides_inferred_then_inferred_demotes() {
        // Chained: explicit then inferred — final flag should reflect the
        // last setter, which is inferred (i.e. not explicit).
        let v = ResolvedValue::new("x")
            .with_direction(Direction::Rtl)
            .with_inferred_direction(Direction::Ltr);
        assert_eq!(v.direction(), Some(Direction::Ltr));
        assert!(!v.direction_explicit());
    }

    #[test]
    fn resolved_options_accumulate_and_replace() {
        let v = ResolvedValue::new("x")
            .with_resolved_option("minimumFractionDigits", "2")
            .with_resolved_option("maximumFractionDigits", "4");
        assert_eq!(v.resolved_options().len(), 2);
        assert_eq!(
            v.resolved_options()
                .get(&Box::from("minimumFractionDigits"))
                .map(|b| b.as_ref()),
            Some("2")
        );
        let mut replacement = BTreeMap::new();
        replacement.insert(Box::<str>::from("style"), Box::<str>::from("percent"));
        let replaced = v.with_resolved_options(replacement);
        assert_eq!(replaced.resolved_options().len(), 1);
        assert!(replaced
            .resolved_options()
            .contains_key(&Box::<str>::from("style")));
    }

    #[test]
    fn with_part_kind_overrides_default() {
        let v = ResolvedValue::new("42").with_part_kind("number");
        assert_eq!(v.part_kind(), "number");
    }

    #[test]
    fn from_str_impls() {
        let a: ResolvedValue = "lit".into();
        assert_eq!(a.text(), "lit");
        assert!(!a.is_fallback());
        let b: ResolvedValue = String::from("owned").into();
        assert_eq!(b.text(), "owned");
        let c: ResolvedValue = Box::<str>::from("boxed").into();
        assert_eq!(c.text(), "boxed");
    }
}

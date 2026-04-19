// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Function handlers and the registry.
//!
//! Defines the [`FunctionHandler`] trait, the [`FunctionRegistry`] container,
//! and [`FunctionContext`] (which carries the format locale and numeric
//! operand plumbing). Built-in handlers cover `:string`, `:number`,
//! `:integer`, `:currency`, `:percent`, and offset math for selection. Draft
//! `:date` / `:time` / `:datetime` / `:unit` handlers are gated behind
//! `unstable` + `compiled_data`.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use alloc::format;
#[cfg(feature = "compiled_data")]
use alloc::string::String;
use alloc::sync::Arc;
#[cfg(feature = "compiled_data")]
use alloc::vec::Vec;

#[cfg(feature = "compiled_data")]
use fixed_decimal::Decimal;
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use icu_calendar::{Date, Iso};
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use icu_datetime::fieldsets::builder::{DateFields as IcuDateFields, FieldSetBuilder, ZoneStyle};
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use icu_datetime::options::{Length, TimePrecision};
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use icu_datetime::preferences::{CalendarAlgorithm, HourCycle, NumberingSystem};
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use icu_datetime::{DateTimeFormatter, DateTimeFormatterPreferences, NoCalendarFormatter};
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use icu_locale_core::extensions::unicode::Value as UnicodeValue;
use icu_locale_core::Locale;
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use icu_time::zone::{models, IanaParser, TimeZone, TimeZoneInfo, UtcOffset};
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use icu_time::{DateTime, Time, ZonedDateTime, ZonedTime};

use super::error::FunctionError;
use super::selector::SelectorImpl;
use super::value::ResolvedValue;

/// Options passed to a [`FunctionHandler`]. Keys are option identifiers
/// (possibly namespaced, e.g. `u:dir`); values have already been resolved
/// from their literal or variable form.
pub type FunctionOptions = BTreeMap<Box<str>, ResolvedValue>;

/// Merge an operand's resolved options into the expression's options. The
/// expression's options take priority (spec: "options on the expression
/// take priority over any options of the operand"). Returns a fresh map.
#[cfg(feature = "compiled_data")]
pub(crate) fn merge_operand_options(
    operand: Option<&ResolvedValue>,
    expr_options: &FunctionOptions,
    skip: &[&str],
) -> FunctionOptions {
    let mut out: FunctionOptions = expr_options.clone();
    if let Some(op) = operand {
        for (k, v) in op.resolved_options() {
            if skip.iter().any(|s| *s == k.as_ref()) {
                continue;
            }
            if !out.contains_key(k.as_ref()) {
                out.insert(k.clone(), ResolvedValue::new(v.clone()));
            }
        }
    }
    out
}

/// Context passed to every [`FunctionHandler`] call.
///
/// Carries the active [`Locale`] and the message's base directionality.
/// A per-expression `u:locale` override, when supplied, is surfaced via
/// [`FunctionContext::locale`] so handlers see the overridden locale
/// transparently.
#[non_exhaustive]
#[derive(Debug)]
pub struct FunctionContext<'a> {
    locale: &'a Locale,
    base_direction: super::bidi::Direction,
    locale_override: Option<Locale>,
}

impl<'a> FunctionContext<'a> {
    /// Construct a context. Used by the formatter; third parties can build
    /// one for out-of-band handler invocations. Defaults the base direction
    /// to [`super::bidi::Direction::Ltr`] and leaves the locale override
    /// unset.
    pub fn new(locale: &'a Locale) -> Self {
        Self {
            locale,
            base_direction: super::bidi::Direction::Ltr,
            locale_override: None,
        }
    }

    /// Set the message's base directionality.
    pub fn with_base_direction(mut self, dir: super::bidi::Direction) -> Self {
        self.base_direction = dir;
        self
    }

    /// Attach a per-expression locale override (e.g. from `u:locale`).
    /// When set, [`FunctionContext::locale`] returns this locale rather
    /// than the formatter-level locale.
    pub fn with_locale_override(mut self, locale: Locale) -> Self {
        self.locale_override = Some(locale);
        self
    }

    /// The effective locale — the `u:locale` override if set, otherwise
    /// the formatter-level locale.
    pub fn locale(&self) -> &Locale {
        self.locale_override.as_ref().unwrap_or(self.locale)
    }

    /// The message's base directionality.
    pub fn base_direction(&self) -> super::bidi::Direction {
        self.base_direction
    }
}

/// A handler implementing a single MF2 function (e.g. `:string`, `:number`).
pub trait FunctionHandler: core::fmt::Debug + Send + Sync {
    /// Apply this function to `operand` and `options` under `ctx`, producing
    /// a [`ResolvedValue`].
    ///
    /// Returning an `Err` causes the formatter to emit a
    /// [`super::FormatError::FunctionError`] and substitute the spec-defined
    /// fallback for the expression.
    fn format(
        &self,
        ctx: &FunctionContext<'_>,
        operand: Option<&ResolvedValue>,
        options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError>;
}

/// A map from function identifier → handler.
#[derive(Debug, Clone)]
pub struct FunctionRegistry {
    handlers: BTreeMap<Box<str>, Arc<dyn FunctionHandler>>,
}

impl FunctionRegistry {
    /// An empty registry.
    pub fn new() -> Self {
        Self {
            handlers: BTreeMap::new(),
        }
    }

    /// A registry populated with the current built-ins: `:string`; with
    /// `compiled_data`, `:number`, `:integer`, `:percent`, `:currency`, and
    /// `:offset` (which require baked decimal and plural-rule data); and
    /// with `unstable + compiled_data`, draft `:unit`, `:date`, `:time`,
    /// and `:datetime` handlers.
    pub fn default_registry() -> Self {
        let mut r = Self::new();
        r.register("string", StringHandler);
        #[cfg(feature = "compiled_data")]
        {
            r.register(
                "number",
                NumberHandler {
                    kind: NumberKind::Number,
                },
            );
            r.register(
                "integer",
                NumberHandler {
                    kind: NumberKind::Integer,
                },
            );
            r.register(
                "percent",
                NumberHandler {
                    kind: NumberKind::Percent,
                },
            );
            r.register("currency", CurrencyHandler);
            r.register("offset", OffsetHandler);
        }
        #[cfg(all(feature = "unstable", feature = "compiled_data"))]
        {
            r.register("unit", UnitHandler);
            r.register("date", DateTimeHandler { kind: "date" });
            r.register("time", DateTimeHandler { kind: "time" });
            r.register("datetime", DateTimeHandler { kind: "datetime" });
        }
        r
    }

    /// Insert (or overwrite) a handler under `name`.
    pub fn register<F: FunctionHandler + 'static>(&mut self, name: impl Into<Box<str>>, f: F) {
        self.handlers.insert(name.into(), Arc::new(f));
    }

    /// Look up a handler by identifier.
    pub fn get(&self, name: &str) -> Option<&Arc<dyn FunctionHandler>> {
        self.handlers.get(name)
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::default_registry()
    }
}

// ---------------------------------------------------------------------------
// :string
// ---------------------------------------------------------------------------

/// `:string` — format and select on string operands.
#[derive(Debug, Default)]
struct StringHandler;

impl FunctionHandler for StringHandler {
    fn format(
        &self,
        _ctx: &FunctionContext<'_>,
        operand: Option<&ResolvedValue>,
        _options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError> {
        match operand {
            Some(v) => {
                let text = v.text();
                let selector: Arc<dyn SelectorImpl> = Arc::new(StringSelector {
                    value_nfc: normalize_nfc(text),
                });
                Ok(ResolvedValue::new(text).with_selector(selector))
            }
            None => Err(FunctionError::BadOperand),
        }
    }
}

#[derive(Debug)]
struct StringSelector {
    value_nfc: Box<str>,
}

impl SelectorImpl for StringSelector {
    fn rank(&self, key: &str) -> Result<Option<usize>, FunctionError> {
        let normalized = normalize_nfc(key);
        if normalized.as_ref() == self.value_nfc.as_ref() {
            Ok(Some(0))
        } else {
            Ok(None)
        }
    }
}

#[cfg(feature = "compiled_data")]
fn normalize_nfc(s: &str) -> Box<str> {
    use alloc::string::String;
    use icu_normalizer::ComposingNormalizer;
    let normalizer = ComposingNormalizer::new_nfc();
    let normalized: String = normalizer.normalize(s).into_owned();
    normalized.into_boxed_str()
}

#[cfg(not(feature = "compiled_data"))]
fn normalize_nfc(s: &str) -> Box<str> {
    Box::from(s)
}

// ---------------------------------------------------------------------------
// :number / :integer
// ---------------------------------------------------------------------------

/// `:number`, `:integer`, and `:percent` — format numeric operands and
/// (via a selector) drive plural/ordinal/exact variant selection.
///
/// `:integer` truncates to integer digits before formatting.
/// `:percent` multiplies the operand by 100 and formats with the locale’s
/// CLDR percent pattern (symbol placement and spacing).
/// Per spec, the stable option set is a superset of ECMA-402
/// `Intl.NumberFormat` options.
#[cfg(feature = "compiled_data")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumberKind {
    Number,
    Integer,
    Percent,
}

#[cfg(feature = "compiled_data")]
impl NumberKind {
    fn part_kind(self) -> &'static str {
        match self {
            NumberKind::Number => "number",
            NumberKind::Integer => "integer",
            NumberKind::Percent => "percent",
        }
    }
}

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
struct NumberHandler {
    kind: NumberKind,
}

#[cfg(feature = "compiled_data")]
impl FunctionHandler for NumberHandler {
    fn format(
        &self,
        ctx: &FunctionContext<'_>,
        operand: Option<&ResolvedValue>,
        options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError> {
        let operand = operand.ok_or(FunctionError::BadOperand)?;
        let raw_value = resolve_numeric(operand)?;
        let mut value = raw_value.clone();

        // Per number.md: options on the operand carry forward unless
        // overridden on this expression. :integer and :percent additionally
        // discard certain operand options per spec.
        let skip: &[&str] = match self.kind {
            NumberKind::Integer => &[
                "minimumFractionDigits",
                "maximumFractionDigits",
                "minimumSignificantDigits",
            ],
            NumberKind::Percent => &["minimumIntegerDigits", "roundingIncrement", "select"],
            NumberKind::Number => &[],
        };

        // Per number.md §Number Selection: `select` MUST be a literal on
        // the expression itself. Inheriting it from an operand's resolved
        // options is a Bad Option error — we emit one via the BadOption
        // variant and disable selection for this expression.
        let select_from_operand = matches!(self.kind, NumberKind::Number | NumberKind::Integer)
            && !options.contains_key("select")
            && operand.resolved_options().contains_key("select");

        // Don't inherit `select` into the merged map if doing so would break
        // the literal-only constraint.
        let mut skip_vec: Vec<&str> = skip.to_vec();
        if select_from_operand {
            skip_vec.push("select");
        }
        let merged = merge_operand_options(Some(operand), options, &skip_vec);

        let mut parsed = NumberOptions::parse(&merged)?;
        match self.kind {
            NumberKind::Integer => {
                // :integer is :number with maximumFractionDigits=0 and
                // minimumFractionDigits=0 forced, and significant-digit
                // options cleared per spec.
                parsed.min_fraction_digits = Some(0);
                parsed.max_fraction_digits = Some(0);
                parsed.min_significant_digits = None;
                parsed.max_significant_digits = None;
            }
            NumberKind::Percent => {
                // :percent multiplies the operand by 100 before formatting
                // and selection. All option interpretation operates on the
                // scaled value.
                value.multiply_pow10(2);
                value.trim_start();
            }
            NumberKind::Number => {}
        }

        apply_digit_options(&mut value, &parsed);
        if let Some(sd) = parsed.sign_display {
            value = value.with_sign_display(sd);
        }

        // :percent always selects via `plural` per spec; `:number` /
        // `:integer` honor the `select` option.
        let select_mode = match self.kind {
            NumberKind::Percent => SelectMode::Plural,
            _ => parse_select_option(&merged)?,
        };

        let formatted = if self.kind == NumberKind::Percent {
            format_percent(ctx.locale(), &value, parsed.grouping)?
        } else {
            format_decimal(ctx.locale(), &value, parsed.grouping)?
        };

        let selector: Arc<dyn SelectorImpl> = Arc::new(NumberSelector::build(
            ctx.locale(),
            value.clone(),
            select_mode,
        )?);

        let locale_dir = super::resolver::locale_direction(ctx.locale());
        let mut out = ResolvedValue::new(formatted)
            .with_numeric(match self.kind {
                // :percent formats and selects on the scaled value, but the
                // resolved numeric payload remains equal to the operand so
                // downstream composition does not re-scale unexpectedly.
                NumberKind::Percent => raw_value,
                NumberKind::Number | NumberKind::Integer => value,
            })
            .with_inferred_direction(locale_dir)
            .with_part_kind(self.kind.part_kind());
        if !select_from_operand {
            out = out.with_selector(selector);
        }
        // Expose resolved options so downstream functions (e.g. :offset on
        // a :number-typed operand) can inherit them per spec.
        for (name, rv) in &merged {
            if !name.starts_with("u:") {
                out = out.with_resolved_option(name.clone(), Box::<str>::from(rv.text()));
            }
        }
        Ok(out)
    }
}

#[cfg(feature = "compiled_data")]
fn resolve_numeric(operand: &ResolvedValue) -> Result<Decimal, FunctionError> {
    use core::str::FromStr;
    if let Some(n) = operand.numeric() {
        return Ok(n.clone());
    }
    // Text operands must match the MF2 number-literal ABNF. Spec (number.md
    // §Operands): non-numeric inputs produce a Bad Operand error, including
    // syntactically-invalid number strings like `00`, `042`, `1.e3`, `+1`.
    let text = operand.text();
    if !super::parser::lexer::is_valid_number_literal(text) {
        return Err(FunctionError::BadOperand);
    }
    // MF2 ABNF allows `e+n` exponent; fixed_decimal::Decimal::from_str rejects
    // a literal `+` sign. Strip it before parsing, then trim leading zeros
    // that the parser leaves when combining `0.42` with a positive exponent.
    let cleaned = strip_positive_exponent(text);
    let mut value = Decimal::from_str(cleaned.as_ref()).map_err(|_| FunctionError::BadOperand)?;
    value.trim_start();
    Ok(value)
}

/// Remove a literal `+` from an exponent (`1.5e+2` → `1.5e2`). Leaves the
/// input unchanged when no such pattern exists.
#[cfg(feature = "compiled_data")]
fn strip_positive_exponent(s: &str) -> alloc::borrow::Cow<'_, str> {
    let bytes = s.as_bytes();
    if let Some(e) = bytes.iter().position(|b| *b == b'e' || *b == b'E') {
        if bytes.get(e + 1) == Some(&b'+') {
            let mut out = String::with_capacity(s.len() - 1);
            out.push_str(&s[..=e]);
            out.push_str(&s[e + 2..]);
            return alloc::borrow::Cow::Owned(out);
        }
    }
    alloc::borrow::Cow::Borrowed(s)
}

#[cfg(feature = "compiled_data")]
fn format_decimal(
    locale: &Locale,
    value: &Decimal,
    grouping: Option<icu_decimal::options::GroupingStrategy>,
) -> Result<String, FunctionError> {
    use icu_decimal::{options::DecimalFormatterOptions, DecimalFormatter};
    use writeable::Writeable;
    let mut opts = DecimalFormatterOptions::default();
    opts.grouping_strategy = grouping;
    let fmter = DecimalFormatter::try_new(locale.into(), opts)
        .map_err(|_| FunctionError::UnsupportedOperation)?;
    Ok(fmter.format(value).write_to_string().into_owned())
}

/// Formats a value for `:percent` using CLDR percent patterns and the locale’s
/// percent sign (including placement, e.g. Turkish prefix `%`).
#[cfg(feature = "compiled_data")]
fn format_percent(
    locale: &Locale,
    value: &Decimal,
    grouping: Option<icu_decimal::options::GroupingStrategy>,
) -> Result<String, FunctionError> {
    use crate::dimension::percent::formatter::{PercentFormatter, PercentFormatterPreferences};
    use crate::dimension::percent::options::PercentFormatterOptions;
    use icu_decimal::{options::DecimalFormatterOptions, DecimalFormatter};
    use writeable::Writeable;

    let prefs: PercentFormatterPreferences = locale.clone().into();
    let mut decimal_opts = DecimalFormatterOptions::default();
    decimal_opts.grouping_strategy = grouping;
    let decimal_formatter = DecimalFormatter::try_new(locale.into(), decimal_opts)
        .map_err(|_| FunctionError::UnsupportedOperation)?;
    let percent_formatter = PercentFormatter::try_new_with_decimal_formatter(
        prefs,
        decimal_formatter,
        PercentFormatterOptions::default(),
    )
    .map_err(|_| FunctionError::UnsupportedOperation)?;
    Ok(percent_formatter
        .format(value)
        .write_to_string()
        .into_owned())
}

/// Parsed subset of the spec's `:number` option bag. Unsupported options
/// (e.g. `unit`, compact `notation`) will be added incrementally.
#[cfg(feature = "compiled_data")]
#[derive(Debug, Default)]
struct NumberOptions {
    sign_display: Option<fixed_decimal::SignDisplay>,
    grouping: Option<icu_decimal::options::GroupingStrategy>,
    min_fraction_digits: Option<u8>,
    max_fraction_digits: Option<u8>,
    min_integer_digits: Option<u8>,
    min_significant_digits: Option<u8>,
    max_significant_digits: Option<u8>,
    rounding_mode: Option<fixed_decimal::SignedRoundingMode>,
    /// `roundingIncrement` per spec: the increment is applied at the
    /// `maximumFractionDigits` position. Stored as (base, shift) where
    /// `base ∈ {1, 2, 5, 25}` and the effective position equals
    /// `-max_fraction_digits + shift`.
    rounding_increment: Option<(fixed_decimal::RoundingIncrement, i16)>,
    rounding_priority: RoundingPriority,
    trailing_zero_display: TrailingZeroDisplay,
}

/// Spec-defined values for the `roundingPriority` option.
#[cfg(feature = "compiled_data")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum RoundingPriority {
    #[default]
    Auto,
    MorePrecision,
    LessPrecision,
}

/// Spec-defined values for the `trailingZeroDisplay` option.
#[cfg(feature = "compiled_data")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum TrailingZeroDisplay {
    #[default]
    Auto,
    StripIfInteger,
}

#[cfg(feature = "compiled_data")]
impl NumberOptions {
    fn parse(opts: &FunctionOptions) -> Result<Self, FunctionError> {
        use fixed_decimal::{SignDisplay, SignedRoundingMode, UnsignedRoundingMode};
        use icu_decimal::options::GroupingStrategy;
        let mut n = NumberOptions::default();

        if let Some(v) = opts.get("signDisplay") {
            n.sign_display = Some(match v.text() {
                "auto" => SignDisplay::Auto,
                "never" => SignDisplay::Never,
                "always" => SignDisplay::Always,
                "exceptZero" => SignDisplay::ExceptZero,
                "negative" => SignDisplay::Negative,
                _ => {
                    return Err(FunctionError::BadOption {
                        name: "signDisplay".into(),
                    })
                }
            });
        }

        if let Some(v) = opts.get("useGrouping") {
            n.grouping = Some(match v.text() {
                "auto" => GroupingStrategy::Auto,
                "always" => GroupingStrategy::Always,
                "never" => GroupingStrategy::Never,
                "min2" => GroupingStrategy::Min2,
                _ => {
                    return Err(FunctionError::BadOption {
                        name: "useGrouping".into(),
                    })
                }
            });
        }

        n.min_fraction_digits = parse_digit_count(opts, "minimumFractionDigits", 0, 100)?;
        n.max_fraction_digits = parse_digit_count(opts, "maximumFractionDigits", 0, 100)?;
        n.min_integer_digits = parse_digit_count(opts, "minimumIntegerDigits", 1, 21)?;
        n.min_significant_digits = parse_digit_count(opts, "minimumSignificantDigits", 1, 21)?;
        n.max_significant_digits = parse_digit_count(opts, "maximumSignificantDigits", 1, 21)?;

        if let (Some(lo), Some(hi)) = (n.min_significant_digits, n.max_significant_digits) {
            if lo > hi {
                return Err(FunctionError::BadOption {
                    name: "minimumSignificantDigits".into(),
                });
            }
        }

        if let Some(v) = opts.get("roundingPriority") {
            n.rounding_priority = match v.text() {
                "auto" => RoundingPriority::Auto,
                "morePrecision" => RoundingPriority::MorePrecision,
                "lessPrecision" => RoundingPriority::LessPrecision,
                _ => {
                    return Err(FunctionError::BadOption {
                        name: "roundingPriority".into(),
                    })
                }
            };
        }

        if let Some(v) = opts.get("trailingZeroDisplay") {
            n.trailing_zero_display = match v.text() {
                "auto" => TrailingZeroDisplay::Auto,
                "stripIfInteger" => TrailingZeroDisplay::StripIfInteger,
                _ => {
                    return Err(FunctionError::BadOption {
                        name: "trailingZeroDisplay".into(),
                    })
                }
            };
        }

        if let Some(v) = opts.get("roundingIncrement") {
            let inc: u16 = v.text().parse().map_err(|_| FunctionError::BadOption {
                name: "roundingIncrement".into(),
            })?;
            let max_frac = n.max_fraction_digits.unwrap_or(0);
            n.rounding_increment = Some(decompose_rounding_increment(inc, max_frac)?);
        }

        if let Some(v) = opts.get("roundingMode") {
            n.rounding_mode = Some(match v.text() {
                "ceil" => SignedRoundingMode::Ceil,
                "floor" => SignedRoundingMode::Floor,
                "expand" => SignedRoundingMode::Unsigned(UnsignedRoundingMode::Expand),
                "trunc" => SignedRoundingMode::Unsigned(UnsignedRoundingMode::Trunc),
                "halfCeil" => SignedRoundingMode::HalfCeil,
                "halfFloor" => SignedRoundingMode::HalfFloor,
                "halfExpand" => SignedRoundingMode::Unsigned(UnsignedRoundingMode::HalfExpand),
                "halfTrunc" => SignedRoundingMode::Unsigned(UnsignedRoundingMode::HalfTrunc),
                "halfEven" => SignedRoundingMode::Unsigned(UnsignedRoundingMode::HalfEven),
                _ => {
                    return Err(FunctionError::BadOption {
                        name: "roundingMode".into(),
                    })
                }
            });
        }

        // Cross-field validity: min must not exceed max for fraction digits.
        if let (Some(lo), Some(hi)) = (n.min_fraction_digits, n.max_fraction_digits) {
            if lo > hi {
                return Err(FunctionError::BadOption {
                    name: "minimumFractionDigits".into(),
                });
            }
        }

        Ok(n)
    }
}

#[cfg(feature = "compiled_data")]
fn parse_digit_count(
    opts: &FunctionOptions,
    name: &str,
    min: u8,
    max: u8,
) -> Result<Option<u8>, FunctionError> {
    let Some(v) = opts.get(name) else {
        return Ok(None);
    };
    let parsed: u8 = v
        .text()
        .parse()
        .map_err(|_| FunctionError::BadOption { name: name.into() })?;
    if parsed < min || parsed > max {
        return Err(FunctionError::BadOption { name: name.into() });
    }
    Ok(Some(parsed))
}

/// Decompose the spec's `roundingIncrement` value (one of 1, 2, 5, 10, 20,
/// 25, 50, 100, 200, 250, 500, 1000, 2000, 2500, 5000) into a
/// `(RoundingIncrement, shift)` pair. The effective rounding position is
/// `-max_fraction_digits + shift`.
#[cfg(feature = "compiled_data")]
fn decompose_rounding_increment(
    value: u16,
    _max_frac: u8,
) -> Result<(fixed_decimal::RoundingIncrement, i16), FunctionError> {
    use fixed_decimal::RoundingIncrement as RI;
    let bad = || FunctionError::BadOption {
        name: "roundingIncrement".into(),
    };
    let (base, base_val): (RI, u16) = if value % 25 == 0 && is_power_of_10(value / 25) {
        (RI::MultiplesOf25, 25)
    } else if value % 5 == 0 && is_power_of_10(value / 5) {
        (RI::MultiplesOf5, 5)
    } else if value % 2 == 0 && is_power_of_10(value / 2) {
        (RI::MultiplesOf2, 2)
    } else if is_power_of_10(value) {
        (RI::MultiplesOf1, 1)
    } else {
        return Err(bad());
    };
    let factor = value / base_val;
    let shift = log10(factor).ok_or_else(bad)?;
    // Spec-defined valid increments cap at 5000; shift ≤ 3.
    if shift > 3 {
        return Err(bad());
    }
    Ok((base, shift as i16))
}

#[cfg(feature = "compiled_data")]
fn is_power_of_10(mut v: u16) -> bool {
    if v == 0 {
        return false;
    }
    while v > 1 {
        if v % 10 != 0 {
            return false;
        }
        v /= 10;
    }
    true
}

#[cfg(feature = "compiled_data")]
fn log10(mut v: u16) -> Option<u32> {
    let mut n = 0u32;
    while v > 1 {
        if v % 10 != 0 {
            return None;
        }
        v /= 10;
        n += 1;
    }
    Some(n)
}

#[cfg(feature = "compiled_data")]
fn apply_digit_options(value: &mut Decimal, opts: &NumberOptions) {
    use fixed_decimal::{SignedRoundingMode, UnsignedRoundingMode};
    let mode = opts.rounding_mode.unwrap_or(SignedRoundingMode::Unsigned(
        UnsignedRoundingMode::HalfExpand,
    ));

    // Decide whether fraction-digit or significant-digit options drive
    // rounding. Per spec §roundingPriority: `auto` (default) uses significant
    // if set, otherwise fraction; `morePrecision` / `lessPrecision` pick the
    // side that produces more / fewer digits.
    let frac_pos = opts.max_fraction_digits.map(|f| -i16::from(f));
    let sig_pos = opts
        .max_significant_digits
        .map(|n| leading_magnitude(value) - i16::from(n) + 1);
    let round_pos = match (frac_pos, sig_pos, opts.rounding_priority) {
        (None, None, _) => None,
        (Some(f), None, _) => Some(f),
        (None, Some(s), _) => Some(s),
        (Some(_), Some(s), RoundingPriority::Auto) => Some(s),
        // `morePrecision` → keep more digits → smaller position wins.
        (Some(f), Some(s), RoundingPriority::MorePrecision) => Some(f.min(s)),
        // `lessPrecision` → keep fewer digits → larger position wins.
        (Some(f), Some(s), RoundingPriority::LessPrecision) => Some(f.max(s)),
    };

    if let Some(pos) = round_pos {
        if let Some((inc, shift)) = opts.rounding_increment {
            let adj = pos + shift;
            value.round_with_mode_and_increment(adj, mode, inc);
        } else {
            value.round_with_mode(pos, mode);
        }
    }

    // minimumFractionDigits / minimumSignificantDigits: pad trailing zeros
    // so at least that many fraction / significant digits are present.
    // When neither is set, strip trailing fractional zeros to match
    // ECMA-402's default rendering (which the spec inherits for :number).
    if let Some(min_frac) = opts.min_fraction_digits {
        let pos: i16 = -i16::from(min_frac);
        value.pad_end(pos);
    } else if opts.min_significant_digits.is_none() {
        value.pad_end(0);
    }
    if let Some(min_sig) = opts.min_significant_digits {
        let pos = leading_magnitude(value) - i16::from(min_sig) + 1;
        if pos <= 0 {
            value.pad_end(pos);
        }
    }

    // trailingZeroDisplay=stripIfInteger: if the rounded value has no
    // non-zero fractional component, strip trailing zeros in the fraction.
    if opts.trailing_zero_display == TrailingZeroDisplay::StripIfInteger
        && value.nonzero_magnitude_end() >= 0
    {
        value.pad_end(0);
    }

    // minimumIntegerDigits: pad integer digits on the left.
    if let Some(min_int) = opts.min_integer_digits {
        value.pad_start(i16::from(min_int));
    }
}

/// Magnitude of the leading (most significant) nonzero digit. For 0 we fall
/// back to magnitude 0, so a significant-digit window keeps the units place.
#[cfg(feature = "compiled_data")]
fn leading_magnitude(value: &Decimal) -> i16 {
    if value.is_zero() {
        0
    } else {
        value.nonzero_magnitude_start()
    }
}

#[cfg(feature = "compiled_data")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectMode {
    Plural,
    Ordinal,
    Exact,
}

#[cfg(feature = "compiled_data")]
fn parse_select_option(options: &FunctionOptions) -> Result<SelectMode, FunctionError> {
    let Some(v) = options.get("select") else {
        return Ok(SelectMode::Plural);
    };
    match v.text() {
        "plural" => Ok(SelectMode::Plural),
        "ordinal" => Ok(SelectMode::Ordinal),
        "exact" => Ok(SelectMode::Exact),
        _ => Err(FunctionError::BadOption {
            name: "select".into(),
        }),
    }
}

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
struct NumberSelector {
    value: Decimal,
    category: Option<icu_plurals::PluralCategory>,
    integer_text: Box<str>,
}

#[cfg(feature = "compiled_data")]
impl NumberSelector {
    fn build(locale: &Locale, value: Decimal, mode: SelectMode) -> Result<Self, FunctionError> {
        use icu_plurals::PluralRules;
        let category = match mode {
            SelectMode::Exact => None,
            SelectMode::Plural => Some(
                PluralRules::try_new_cardinal(locale.into())
                    .map_err(|_| FunctionError::UnsupportedOperation)?
                    .category_for(&value),
            ),
            SelectMode::Ordinal => Some(
                PluralRules::try_new_ordinal(locale.into())
                    .map_err(|_| FunctionError::UnsupportedOperation)?
                    .category_for(&value),
            ),
        };
        // Pre-compute the canonical text form of the value for exact
        // comparison; the key `"1"` should match value `1`, `"0"` should
        // match `0`, etc.
        use writeable::Writeable;
        let integer_text: Box<str> = value.write_to_string().into_owned().into_boxed_str();
        Ok(Self {
            value,
            category,
            integer_text,
        })
    }
}

#[cfg(feature = "compiled_data")]
impl SelectorImpl for NumberSelector {
    fn rank(&self, key: &str) -> Result<Option<usize>, FunctionError> {
        use core::str::FromStr;
        // 1. Exact numeric match (rank 0).
        let parsed = Decimal::from_str(key).ok();
        if let Some(p) = &parsed {
            if *p == self.value {
                return Ok(Some(0));
            }
        }
        // 2. Fallback: string equality against the canonical text form,
        //    to honor keys like `0` that spec examples use.
        if key == self.integer_text.as_ref() {
            return Ok(Some(0));
        }
        // 3. Plural category name (rank 1).
        if is_plural_category_name(key) {
            if let Some(cat) = self.category {
                if plural_category_name(cat) == key {
                    return Ok(Some(1));
                }
            }
            return Ok(None);
        }
        // 4. Key is neither a number literal nor a plural category name —
        //    this is a Bad Variant Key per spec.
        if parsed.is_none() {
            return Err(FunctionError::BadVariantKey {
                key: Box::from(key),
            });
        }
        Ok(None)
    }
}

#[cfg(feature = "compiled_data")]
fn is_plural_category_name(s: &str) -> bool {
    matches!(s, "zero" | "one" | "two" | "few" | "many" | "other")
}

#[cfg(feature = "compiled_data")]
fn plural_category_name(cat: icu_plurals::PluralCategory) -> &'static str {
    use icu_plurals::PluralCategory;
    match cat {
        PluralCategory::Zero => "zero",
        PluralCategory::One => "one",
        PluralCategory::Two => "two",
        PluralCategory::Few => "few",
        PluralCategory::Many => "many",
        PluralCategory::Other => "other",
    }
}

// ---------------------------------------------------------------------------
// :currency
// ---------------------------------------------------------------------------

/// `:currency` — format a numeric operand as a currency value.
///
/// Requires a `currency` option when the operand is a plain numeric value.
/// Uses `icu_experimental::dimension::currency::CurrencyFormatter` under
/// the hood, so this handler is only registered when `compiled_data` is on.
#[cfg(feature = "compiled_data")]
#[derive(Debug)]
struct CurrencyHandler;

#[cfg(feature = "compiled_data")]
impl FunctionHandler for CurrencyHandler {
    fn format(
        &self,
        ctx: &FunctionContext<'_>,
        operand: Option<&ResolvedValue>,
        options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError> {
        use crate::dimension::currency::formatter::CurrencyFormatter;
        use crate::dimension::currency::options::Width;
        use crate::dimension::currency::CurrencyCode;
        use tinystr::TinyAsciiStr;

        let operand = operand.ok_or(FunctionError::BadOperand)?;
        let mut value = resolve_numeric(operand)?;

        // Inherit resolved options from the operand (e.g. `currency=EUR` set
        // via an upstream `:currency` annotation) before validating.
        let merged = merge_operand_options(Some(operand), options, &[]);

        // Required: `currency`. Must be 3 ASCII letters (case-insensitive).
        let currency_opt = merged.get("currency").ok_or(FunctionError::BadOperand)?;
        let currency_text = currency_opt.text().to_uppercase();
        if currency_text.len() != 3 || !currency_text.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(FunctionError::BadOption {
                name: "currency".into(),
            });
        }
        let currency_code = CurrencyCode(TinyAsciiStr::<3>::try_from_str(&currency_text).map_err(
            |_| FunctionError::BadOption {
                name: "currency".into(),
            },
        )?);

        // currencyDisplay → Width. `name` is not supported by icu_experimental
        // currency; implementations MAY alias unsupported values per spec.
        let width = match merged.get("currencyDisplay").map(|v| v.text()) {
            Some("narrowSymbol") => Width::Narrow,
            Some("symbol") | Some("name") | Some("code") | Some("never") | None => Width::Short,
            Some(_) => {
                return Err(FunctionError::BadOption {
                    name: "currencyDisplay".into(),
                })
            }
        };

        // currencySign: standard (default) or accounting. ICU4X's currency
        // pipeline has no separate accounting-sign backend, so both values
        // currently produce the standard rendering. The spec (number.md) lets
        // implementations render unsupported sign styles in an
        // implementation-defined way — it does NOT require `BadOption`, so
        // accepting `accounting` without a distinct visual is conformant.
        match merged.get("currencySign").map(|v| v.text()) {
            Some("standard") | Some("accounting") | None => {}
            Some(_) => {
                return Err(FunctionError::BadOption {
                    name: "currencySign".into(),
                })
            }
        }

        // Digit / rounding options mirror :number, minus `minimumIntegerDigits`
        // default-1 semantics (already the default).
        let mut parsed = NumberOptions::parse(&merged)?;
        // `fractionDigits` replaces min/max FractionDigits on :currency.
        if let Some(fd) = merged.get("fractionDigits") {
            match fd.text() {
                "auto" => {}
                other => {
                    let n: u8 = other.parse().map_err(|_| FunctionError::BadOption {
                        name: "fractionDigits".into(),
                    })?;
                    if n > 100 {
                        return Err(FunctionError::BadOption {
                            name: "fractionDigits".into(),
                        });
                    }
                    parsed.min_fraction_digits = Some(n);
                    parsed.max_fraction_digits = Some(n);
                }
            }
        }
        apply_digit_options(&mut value, &parsed);

        let formatter = CurrencyFormatter::try_new(ctx.locale().into(), width.into())
            .map_err(|_| FunctionError::UnsupportedOperation)?;
        use writeable::Writeable;
        let formatted = formatter
            .format_fixed_decimal(&value, &currency_code)
            .write_to_string()
            .into_owned();

        // Per spec `number.md §The :currency function`, :currency is a
        // formatter only — no Selection section, so no selector is attached.
        // `.match` on a :currency value produces a Bad Selector error.
        let locale_dir = super::resolver::locale_direction(ctx.locale());
        let mut out = ResolvedValue::new(formatted)
            .with_numeric(value)
            .with_inferred_direction(locale_dir)
            .with_part_kind("currency");
        for (name, rv) in &merged {
            if !name.starts_with("u:") {
                out = out.with_resolved_option(name.clone(), Box::<str>::from(rv.text()));
            }
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// :offset
// ---------------------------------------------------------------------------

/// `:offset` — add or subtract an integer from the operand before selection
/// and formatting. Exactly one of `add` / `subtract` must be supplied.
#[cfg(feature = "compiled_data")]
#[derive(Debug)]
struct OffsetHandler;

#[cfg(feature = "compiled_data")]
impl FunctionHandler for OffsetHandler {
    fn format(
        &self,
        ctx: &FunctionContext<'_>,
        operand: Option<&ResolvedValue>,
        options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError> {
        use core::str::FromStr;
        let operand = operand.ok_or(FunctionError::BadOperand)?;
        let base = resolve_numeric(operand)?;

        let add = options.get("add").map(|v| v.text());
        let sub = options.get("subtract").map(|v| v.text());
        let (delta_text, sign) = match (add, sub) {
            (Some(a), None) => (a, 1i32),
            (None, Some(s)) => (s, -1i32),
            _ => {
                return Err(FunctionError::BadOption {
                    name: if add.is_some() {
                        "add".into()
                    } else {
                        "subtract".into()
                    },
                })
            }
        };

        // Per spec, `add` / `subtract` values MUST match the `digit-size-option`
        // ABNF: `"0" / ("1"-"9") [DIGIT]` — a non-negative integer in 0..=99.
        // Negative or fractional deltas are a Bad Option error, not a silent
        // success or an UnsupportedOperation.
        if !is_digit_size_option(delta_text) {
            return Err(FunctionError::BadOption {
                name: if sign > 0 {
                    "add".into()
                } else {
                    "subtract".into()
                },
            });
        }
        let delta = Decimal::from_str(delta_text).map_err(|_| FunctionError::BadOption {
            name: if sign > 0 {
                "add".into()
            } else {
                "subtract".into()
            },
        })?;
        let mut value =
            add_decimals(&base, &delta, sign).ok_or(FunctionError::UnsupportedOperation)?;

        // Inherit numeric-formatting options from the operand (e.g.
        // signDisplay=always propagated from a :number ancestor). :offset's
        // own `add` / `subtract` options are per-call and never flow outward.
        let merged = merge_operand_options(Some(operand), options, &["add", "subtract"]);
        let parsed = NumberOptions::parse(&merged)?;
        apply_digit_options(&mut value, &parsed);
        let grouping = parsed.grouping;
        let mut formatted = format_decimal(ctx.locale(), &value, grouping)?;
        if parsed.sign_display.is_some() {
            // Sign-display prefix handled via value when built; format_decimal
            // honors the fixed_decimal sign. Re-format with sign applied.
            let mut signed_value = value.clone();
            if let Some(sd) = parsed.sign_display {
                signed_value = signed_value.with_sign_display(sd);
            }
            formatted = format_decimal(ctx.locale(), &signed_value, grouping)?;
        }
        let selector: Arc<dyn SelectorImpl> = Arc::new(NumberSelector::build(
            ctx.locale(),
            value.clone(),
            SelectMode::Plural,
        )?);
        let locale_dir = super::resolver::locale_direction(ctx.locale());
        let mut out = ResolvedValue::new(formatted)
            .with_numeric(value)
            .with_selector(selector)
            .with_inferred_direction(locale_dir)
            .with_part_kind("number");
        for (name, rv) in &merged {
            if !name.starts_with("u:") && name.as_ref() != "add" && name.as_ref() != "subtract" {
                out = out.with_resolved_option(name.clone(), Box::<str>::from(rv.text()));
            }
        }
        Ok(out)
    }
}

/// Match the MF2 `digit-size-option` ABNF: `"0" / (("1"-"9") [DIGIT])`.
/// Accepts `0`, `1`..`99` with no leading zero. Rejects negatives,
/// fractions, exponents, and values outside `0..=99`.
#[cfg(feature = "compiled_data")]
fn is_digit_size_option(s: &str) -> bool {
    match s.len() {
        1 => s.as_bytes()[0].is_ascii_digit(),
        2 => {
            let b = s.as_bytes();
            (b'1'..=b'9').contains(&b[0]) && b[1].is_ascii_digit()
        }
        _ => false,
    }
}

/// Minimal i64 decimal arithmetic for `:offset`. Returns `None` on overflow
/// or on inputs that cannot be represented as i64 (non-integer, or out of
/// range). Sufficient for the "small integer adjustment" use case the spec
/// describes. Non-integer deltas produce `None` → `UnsupportedOperation`.
#[cfg(feature = "compiled_data")]
fn add_decimals(base: &Decimal, delta: &Decimal, sign: i32) -> Option<Decimal> {
    use writeable::Writeable;
    let base_int: i64 = base.write_to_string().parse().ok()?;
    let delta_int: i64 = delta.write_to_string().parse().ok()?;
    let signed_delta = delta_int.checked_mul(i64::from(sign))?;
    let sum = base_int.checked_add(signed_delta)?;
    Some(Decimal::from(sum))
}

// ---------------------------------------------------------------------------
// Draft: :unit (unstable feature)
// ---------------------------------------------------------------------------

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
#[derive(Debug)]
struct UnitHandler;

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
impl FunctionHandler for UnitHandler {
    fn format(
        &self,
        ctx: &FunctionContext<'_>,
        operand: Option<&ResolvedValue>,
        options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError> {
        use crate::dimension::units::formatter::UnitsFormatter;
        use crate::dimension::units::options::{UnitsFormatterOptions, Width};
        use crate::measure::measureunit::MeasureUnit;

        let operand = operand.ok_or(FunctionError::BadOperand)?;
        if operand.is_fallback() {
            return Err(FunctionError::BadOperand);
        }

        let merged = merge_operand_options(Some(operand), options, &[]);
        let unit = merged
            .get("unit")
            .map(|v| v.text())
            .ok_or(FunctionError::BadOperand)?;

        MeasureUnit::try_from_str(unit).map_err(|_| FunctionError::BadOption {
            name: "unit".into(),
        })?;

        if let Some(usage) = merged.get("usage").map(|v| v.text()) {
            if !is_well_formed_identifier(usage) {
                return Err(FunctionError::BadOption {
                    name: "usage".into(),
                });
            }
            return Err(FunctionError::UnsupportedOperation);
        }

        let width = match merged.get("unitDisplay").map(|v| v.text()) {
            Some("long") => Width::Long,
            Some("short") | None => Width::Short,
            Some("narrow") => Width::Narrow,
            Some(_) => {
                return Err(FunctionError::BadOption {
                    name: "unitDisplay".into(),
                })
            }
        };

        let mut value = resolve_numeric(operand)?;
        let parsed = NumberOptions::parse(&merged)?;
        apply_digit_options(&mut value, &parsed);
        if let Some(sd) = parsed.sign_display {
            value = value.with_sign_display(sd);
        }

        let formatter =
            UnitsFormatter::try_new(ctx.locale().into(), unit, UnitsFormatterOptions { width })
                .map_err(|_| FunctionError::UnsupportedOperation)?;

        use writeable::Writeable;
        let formatted = formatter
            .format_fixed_decimal(&value)
            .write_to_string()
            .into_owned();

        let locale_dir = super::resolver::locale_direction(ctx.locale());
        let mut out = ResolvedValue::new(formatted)
            .with_numeric(value)
            .with_inferred_direction(locale_dir)
            .with_part_kind("unit");
        for (name, rv) in &merged {
            if !name.starts_with("u:") {
                out = out.with_resolved_option(name.clone(), Box::<str>::from(rv.text()));
            }
        }
        Ok(out)
    }
}

// ---------------------------------------------------------------------------
// Draft: :date / :time / :datetime (unstable feature)
// ---------------------------------------------------------------------------

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
#[derive(Debug)]
struct DateTimeHandler {
    kind: &'static str,
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
impl FunctionHandler for DateTimeHandler {
    fn format(
        &self,
        ctx: &FunctionContext<'_>,
        operand: Option<&ResolvedValue>,
        options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError> {
        let operand = operand.ok_or(FunctionError::BadOperand)?;
        if operand.is_fallback() {
            return Err(FunctionError::BadOperand);
        }

        let source_text = operand.datetime_source().unwrap_or_else(|| operand.text());
        let merged = merge_datetime_operand_options(Some(operand), options);
        let parsed = parse_datetime_options(self.kind, ctx.locale(), &merged)?;
        let locale_dir = super::resolver::locale_direction(ctx.locale());

        let formatted = match self.kind {
            "date" => format_date_operand(ctx.locale(), source_text, &parsed)?,
            "time" => format_time_operand(ctx.locale(), source_text, &parsed)?,
            "datetime" => format_datetime_operand(ctx.locale(), source_text, &parsed)?,
            _ => return Err(FunctionError::UnsupportedOperation),
        };

        let mut out = ResolvedValue::new(formatted)
            .with_datetime_source(source_text)
            .with_inferred_direction(locale_dir)
            .with_part_kind(self.kind);
        for (name, rv) in &merged {
            if !name.starts_with("u:") {
                out = out.with_resolved_option(name.clone(), Box::<str>::from(rv.text()));
            }
        }
        Ok(out)
    }
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn merge_datetime_operand_options(
    operand: Option<&ResolvedValue>,
    expr_options: &FunctionOptions,
) -> FunctionOptions {
    let mut out = expr_options.clone();
    if let Some(op) = operand {
        for (k, v) in op.resolved_options() {
            if matches!(k.as_ref(), "timeZone" | "calendar" | "numberingSystem")
                && !out.contains_key(k.as_ref())
            {
                out.insert(k.clone(), ResolvedValue::new(v.clone()));
            }
        }
    }
    out
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
#[derive(Debug, Clone, Copy)]
enum ParsedTimeZoneOverride {
    Default,
    Input,
    Offset(UtcOffset),
    Named(TimeZone),
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
#[derive(Debug, Clone, Copy)]
enum ParsedTimeZoneStyle {
    Long,
    Short,
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
#[derive(Debug, Clone)]
struct ParsedDateTimeOptions {
    date_fields: IcuDateFields,
    length: Length,
    time_precision: TimePrecision,
    time_zone_style: Option<ParsedTimeZoneStyle>,
    time_zone: ParsedTimeZoneOverride,
    prefs: DateTimeFormatterPreferences,
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_datetime_options(
    kind: &str,
    locale: &Locale,
    options: &FunctionOptions,
) -> Result<ParsedDateTimeOptions, FunctionError> {
    let mut parsed = ParsedDateTimeOptions {
        date_fields: IcuDateFields::YMD,
        length: Length::Medium,
        time_precision: TimePrecision::Minute,
        time_zone_style: None,
        time_zone: ParsedTimeZoneOverride::Default,
        prefs: locale.into(),
    };

    for (name, value) in options {
        if name.starts_with("u:") {
            continue;
        }
        match (kind, name.as_ref()) {
            ("datetime", "dateFields") | ("date", "fields") => {
                parsed.date_fields = parse_date_fields(value.text())
                    .ok_or_else(|| FunctionError::BadOption { name: name.clone() })?;
            }
            ("datetime", "dateLength") | ("date", "length") => {
                parsed.length = parse_length(value.text())
                    .ok_or_else(|| FunctionError::BadOption { name: name.clone() })?;
            }
            ("datetime", "timePrecision") | ("time", "precision") => {
                parsed.time_precision = parse_time_precision(value.text())
                    .ok_or_else(|| FunctionError::BadOption { name: name.clone() })?;
            }
            ("datetime" | "time", "timeZoneStyle") => {
                parsed.time_zone_style = Some(
                    parse_time_zone_style(value.text())
                        .ok_or_else(|| FunctionError::BadOption { name: name.clone() })?,
                );
            }
            ("datetime" | "time", "hour12") => {
                parsed.prefs.hour_cycle = Some(match value.text() {
                    "true" => HourCycle::Clock12,
                    "false" => HourCycle::Clock24,
                    _ => {
                        return Err(FunctionError::BadOption { name: name.clone() });
                    }
                });
            }
            ("date" | "time" | "datetime", "timeZone") => {
                parsed.time_zone = parse_time_zone_override(value.text())
                    .ok_or_else(|| FunctionError::BadOption { name: name.clone() })?;
            }
            ("date" | "time" | "datetime", "calendar") => {
                let value = UnicodeValue::try_from_str(value.text())
                    .map_err(|_| FunctionError::BadOption { name: name.clone() })?;
                parsed.prefs.calendar_algorithm = Some(
                    CalendarAlgorithm::try_from(&value)
                        .map_err(|_| FunctionError::BadOption { name: name.clone() })?,
                );
            }
            ("date" | "time" | "datetime", "numberingSystem") => {
                let value = UnicodeValue::try_from_str(value.text())
                    .map_err(|_| FunctionError::BadOption { name: name.clone() })?;
                parsed.prefs.numbering_system = Some(
                    NumberingSystem::try_from(&value)
                        .map_err(|_| FunctionError::BadOption { name: name.clone() })?,
                );
            }
            _ => {
                return Err(FunctionError::BadOption { name: name.clone() });
            }
        }
    }

    Ok(parsed)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_date_fields(value: &str) -> Option<IcuDateFields> {
    Some(match value {
        "weekday" => IcuDateFields::E,
        "day-weekday" => IcuDateFields::DE,
        "month-day" => IcuDateFields::MD,
        "month-day-weekday" => IcuDateFields::MDE,
        "year-month-day" => IcuDateFields::YMD,
        "year-month-day-weekday" => IcuDateFields::YMDE,
        _ => return None,
    })
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_length(value: &str) -> Option<Length> {
    Some(match value {
        "long" => Length::Long,
        "medium" => Length::Medium,
        "short" => Length::Short,
        _ => return None,
    })
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_time_precision(value: &str) -> Option<TimePrecision> {
    Some(match value {
        "hour" => TimePrecision::Hour,
        "minute" => TimePrecision::Minute,
        "second" => TimePrecision::Second,
        _ => return None,
    })
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_time_zone_style(value: &str) -> Option<ParsedTimeZoneStyle> {
    Some(match value {
        "long" => ParsedTimeZoneStyle::Long,
        "short" => ParsedTimeZoneStyle::Short,
        _ => return None,
    })
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_time_zone_override(value: &str) -> Option<ParsedTimeZoneOverride> {
    if value == "input" {
        return Some(ParsedTimeZoneOverride::Input);
    }
    if value == "UTC" {
        return Some(ParsedTimeZoneOverride::Offset(UtcOffset::zero()));
    }
    if let Ok(offset) = UtcOffset::try_from_str(value) {
        return Some(ParsedTimeZoneOverride::Offset(offset));
    }
    if is_well_formed_time_zone_name(value) {
        return Some(ParsedTimeZoneOverride::Named(TimeZone::from_iana_id(value)));
    }
    None
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn base_datetime_builder(
    kind: &str,
    options: &ParsedDateTimeOptions,
    include_zone: bool,
    named_zone: bool,
) -> Result<FieldSetBuilder, FunctionError> {
    let mut builder = FieldSetBuilder::new();
    match kind {
        "date" => {
            builder.date_fields = Some(options.date_fields);
            builder.length = Some(options.length);
        }
        "time" => {
            builder.time_precision = Some(options.time_precision);
        }
        "datetime" => {
            builder.date_fields = Some(options.date_fields);
            builder.length = Some(options.length);
            builder.time_precision = Some(options.time_precision);
        }
        _ => return Err(FunctionError::UnsupportedOperation),
    }
    if include_zone {
        builder.zone_style = Some(match (named_zone, options.time_zone_style) {
            (true, Some(ParsedTimeZoneStyle::Long)) => ZoneStyle::GenericLong,
            (true, Some(ParsedTimeZoneStyle::Short)) => ZoneStyle::GenericShort,
            (false, Some(ParsedTimeZoneStyle::Long)) => ZoneStyle::LocalizedOffsetLong,
            (false, Some(ParsedTimeZoneStyle::Short)) => ZoneStyle::LocalizedOffsetShort,
            (_, None) => return Err(FunctionError::UnsupportedOperation),
        });
    }
    Ok(builder)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_date_operand(
    _locale: &Locale,
    text: &str,
    options: &ParsedDateTimeOptions,
) -> Result<String, FunctionError> {
    use writeable::Writeable;

    let field_set = base_datetime_builder("date", options, false, false)?
        .build_date()
        .map_err(|_| FunctionError::UnsupportedOperation)?;
    let formatter = DateTimeFormatter::try_new(options.prefs, field_set)
        .map_err(|_| FunctionError::UnsupportedOperation)?;

    if let Ok(zdt) = parse_named_zoned_datetime(text) {
        ensure_named_zone_override(zdt.zone, options.time_zone)?;
        return Ok(formatter.format(&zdt.date).write_to_string().into_owned());
    }
    if let Ok(zdt) = parse_offset_zoned_datetime(text) {
        ensure_offset_zone_override(zdt.zone, options.time_zone)?;
        return Ok(formatter.format(&zdt.date).write_to_string().into_owned());
    }
    if let Ok(dt) = parse_datetime_value(text) {
        ensure_missing_input_zone(options.time_zone)?;
        return Ok(formatter.format(&dt.date).write_to_string().into_owned());
    }
    if let Ok(date) = parse_date_value(text) {
        ensure_missing_input_zone(options.time_zone)?;
        return Ok(formatter.format(&date).write_to_string().into_owned());
    }

    Err(FunctionError::BadOperand)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_time_operand(
    _locale: &Locale,
    text: &str,
    options: &ParsedDateTimeOptions,
) -> Result<String, FunctionError> {
    if let Ok(time) = parse_named_zoned_time(text) {
        ensure_named_zone_override(time.zone, options.time_zone)?;
        if options.time_zone_style.is_none() {
            return format_plain_time_value(time.time, options);
        }
        return format_named_time_value(time.time, time.zone, options);
    }
    if let Ok(time) = parse_offset_zoned_time(text) {
        ensure_offset_zone_override(time.zone, options.time_zone)?;
        if options.time_zone_style.is_none() {
            return format_plain_time_value(time.time, options);
        }
        return format_offset_time_value(time.time, time.zone, options);
    }
    if let Ok(zdt) = parse_named_zoned_datetime(text) {
        ensure_named_zone_override(zdt.zone, options.time_zone)?;
        if options.time_zone_style.is_none() {
            return format_plain_time_value(zdt.time, options);
        }
        return format_named_time_value(zdt.time, zdt.zone, options);
    }
    if let Ok(zdt) = parse_offset_zoned_datetime(text) {
        ensure_offset_zone_override(zdt.zone, options.time_zone)?;
        if options.time_zone_style.is_none() {
            return format_plain_time_value(zdt.time, options);
        }
        return format_offset_time_value(zdt.time, zdt.zone, options);
    }
    if let Ok(dt) = parse_datetime_value(text) {
        return match options.time_zone {
            ParsedTimeZoneOverride::Input => return Err(FunctionError::BadOperand),
            ParsedTimeZoneOverride::Named(zone) if options.time_zone_style.is_some() => {
                format_named_time_value(
                    dt.time,
                    zone.with_offset(None).with_zone_name_timestamp(
                        icu_time::zone::ZoneNameTimestamp::far_in_future(),
                    ),
                    options,
                )
            }
            ParsedTimeZoneOverride::Offset(offset) if options.time_zone_style.is_some() => {
                format_offset_time_value(dt.time, offset, options)
            }
            ParsedTimeZoneOverride::Default if options.time_zone_style.is_some() => {
                format_offset_time_value(dt.time, UtcOffset::zero(), options)
            }
            _ => format_plain_time_value(dt.time, options),
        };
    }
    if parse_date_value(text).is_ok() {
        return match options.time_zone {
            ParsedTimeZoneOverride::Input => Err(FunctionError::BadOperand),
            ParsedTimeZoneOverride::Named(zone) if options.time_zone_style.is_some() => {
                format_named_time_value(
                    Time::start_of_day(),
                    zone.with_offset(None).with_zone_name_timestamp(
                        icu_time::zone::ZoneNameTimestamp::far_in_future(),
                    ),
                    options,
                )
            }
            ParsedTimeZoneOverride::Offset(offset) if options.time_zone_style.is_some() => {
                format_offset_time_value(Time::start_of_day(), offset, options)
            }
            ParsedTimeZoneOverride::Default if options.time_zone_style.is_some() => {
                format_offset_time_value(Time::start_of_day(), UtcOffset::zero(), options)
            }
            _ => format_plain_time_value(Time::start_of_day(), options),
        };
    }
    if let Ok(time) = parse_time_value(text) {
        return match options.time_zone {
            ParsedTimeZoneOverride::Input => Err(FunctionError::BadOperand),
            ParsedTimeZoneOverride::Named(zone) if options.time_zone_style.is_some() => {
                format_named_time_value(
                    time,
                    zone.with_offset(None).with_zone_name_timestamp(
                        icu_time::zone::ZoneNameTimestamp::far_in_future(),
                    ),
                    options,
                )
            }
            ParsedTimeZoneOverride::Offset(offset) if options.time_zone_style.is_some() => {
                format_offset_time_value(time, offset, options)
            }
            ParsedTimeZoneOverride::Default if options.time_zone_style.is_some() => {
                format_offset_time_value(time, UtcOffset::zero(), options)
            }
            _ => format_plain_time_value(time, options),
        };
    }

    Err(FunctionError::BadOperand)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_datetime_operand(
    _locale: &Locale,
    text: &str,
    options: &ParsedDateTimeOptions,
) -> Result<String, FunctionError> {
    use writeable::Writeable;

    if let Ok(zdt) = parse_named_zoned_datetime(text) {
        ensure_named_zone_override(zdt.zone, options.time_zone)?;
        if options.time_zone_style.is_none() {
            let field_set = base_datetime_builder("datetime", options, false, false)?
                .build_date_and_time()
                .map_err(|_| FunctionError::UnsupportedOperation)?;
            let formatter = DateTimeFormatter::try_new(options.prefs, field_set)
                .map_err(|_| FunctionError::UnsupportedOperation)?;
            let dt = DateTime {
                date: zdt.date,
                time: zdt.time,
            };
            return Ok(formatter.format(&dt).write_to_string().into_owned());
        }
        let field_set =
            base_datetime_builder("datetime", options, options.time_zone_style.is_some(), true)?
                .build_zoned_date_and_time()
                .map_err(|_| FunctionError::UnsupportedOperation)?;
        let formatter = DateTimeFormatter::try_new(options.prefs, field_set)
            .map_err(|_| FunctionError::UnsupportedOperation)?;
        return Ok(formatter.format(&zdt).write_to_string().into_owned());
    }
    if let Ok(zdt) = parse_offset_zoned_datetime(text) {
        ensure_offset_zone_override(zdt.zone, options.time_zone)?;
        if options.time_zone_style.is_none() {
            let field_set = base_datetime_builder("datetime", options, false, false)?
                .build_date_and_time()
                .map_err(|_| FunctionError::UnsupportedOperation)?;
            let formatter = DateTimeFormatter::try_new(options.prefs, field_set)
                .map_err(|_| FunctionError::UnsupportedOperation)?;
            let dt = DateTime {
                date: zdt.date,
                time: zdt.time,
            };
            return Ok(formatter.format(&dt).write_to_string().into_owned());
        }
        return format_offset_datetime_value(zdt.date, zdt.time, zdt.zone, options);
    }
    if let Ok(dt) = parse_datetime_value(text) {
        match options.time_zone {
            ParsedTimeZoneOverride::Input => return Err(FunctionError::BadOperand),
            ParsedTimeZoneOverride::Named(zone) if options.time_zone_style.is_some() => {
                let zdt = ZonedDateTime {
                    date: dt.date,
                    time: dt.time,
                    zone: zone.with_offset(None).at_date_time(dt),
                };
                let field_set = base_datetime_builder("datetime", options, true, true)?
                    .build_zoned_date_and_time()
                    .map_err(|_| FunctionError::UnsupportedOperation)?;
                let formatter = DateTimeFormatter::try_new(options.prefs, field_set)
                    .map_err(|_| FunctionError::UnsupportedOperation)?;
                return Ok(formatter.format(&zdt).write_to_string().into_owned());
            }
            ParsedTimeZoneOverride::Offset(offset) if options.time_zone_style.is_some() => {
                return format_offset_datetime_value(dt.date, dt.time, offset, options);
            }
            ParsedTimeZoneOverride::Default if options.time_zone_style.is_some() => {
                return format_offset_datetime_value(dt.date, dt.time, UtcOffset::zero(), options);
            }
            _ => {
                let field_set = base_datetime_builder("datetime", options, false, false)?
                    .build_date_and_time()
                    .map_err(|_| FunctionError::UnsupportedOperation)?;
                let formatter = DateTimeFormatter::try_new(options.prefs, field_set)
                    .map_err(|_| FunctionError::UnsupportedOperation)?;
                return Ok(formatter.format(&dt).write_to_string().into_owned());
            }
        }
    }
    if let Ok(date) = parse_date_value(text) {
        let dt = DateTime {
            date,
            time: Time::start_of_day(),
        };
        match options.time_zone {
            ParsedTimeZoneOverride::Input => return Err(FunctionError::BadOperand),
            ParsedTimeZoneOverride::Named(zone) if options.time_zone_style.is_some() => {
                let zdt = ZonedDateTime {
                    date: dt.date,
                    time: dt.time,
                    zone: zone.with_offset(None).at_date_time(dt),
                };
                let field_set = base_datetime_builder("datetime", options, true, true)?
                    .build_zoned_date_and_time()
                    .map_err(|_| FunctionError::UnsupportedOperation)?;
                let formatter = DateTimeFormatter::try_new(options.prefs, field_set)
                    .map_err(|_| FunctionError::UnsupportedOperation)?;
                return Ok(formatter.format(&zdt).write_to_string().into_owned());
            }
            ParsedTimeZoneOverride::Offset(offset) if options.time_zone_style.is_some() => {
                return format_offset_datetime_value(dt.date, dt.time, offset, options);
            }
            ParsedTimeZoneOverride::Default if options.time_zone_style.is_some() => {
                return format_offset_datetime_value(dt.date, dt.time, UtcOffset::zero(), options);
            }
            _ => {
                let field_set = base_datetime_builder("datetime", options, false, false)?
                    .build_date_and_time()
                    .map_err(|_| FunctionError::UnsupportedOperation)?;
                let formatter = DateTimeFormatter::try_new(options.prefs, field_set)
                    .map_err(|_| FunctionError::UnsupportedOperation)?;
                return Ok(formatter.format(&dt).write_to_string().into_owned());
            }
        }
    }

    Err(FunctionError::BadOperand)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_plain_time_value(
    time: Time,
    options: &ParsedDateTimeOptions,
) -> Result<String, FunctionError> {
    use writeable::Writeable;

    let field_set = base_datetime_builder("time", options, false, false)?
        .build_time()
        .map_err(|_| FunctionError::UnsupportedOperation)?;
    let formatter = NoCalendarFormatter::try_new(options.prefs, field_set)
        .map_err(|_| FunctionError::UnsupportedOperation)?;
    Ok(formatter.format(&time).write_to_string().into_owned())
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_offset_time_value(
    time: Time,
    zone: UtcOffset,
    options: &ParsedDateTimeOptions,
) -> Result<String, FunctionError> {
    use writeable::Writeable;

    let field_set = base_datetime_builder("time", options, true, false)?
        .build_zoned_time()
        .map_err(|_| FunctionError::UnsupportedOperation)?;
    let formatter = NoCalendarFormatter::try_new(options.prefs, field_set)
        .map_err(|_| FunctionError::UnsupportedOperation)?;
    Ok(formatter
        .format(&ZonedTime {
            time,
            zone: offset_time_zone_info(zone),
        })
        .write_to_string()
        .into_owned())
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_named_time_value(
    time: Time,
    zone: TimeZoneInfo<models::AtTime>,
    options: &ParsedDateTimeOptions,
) -> Result<String, FunctionError> {
    use writeable::Writeable;

    let field_set = base_datetime_builder("time", options, true, true)?
        .build_zoned_time()
        .map_err(|_| FunctionError::UnsupportedOperation)?;
    let formatter = NoCalendarFormatter::try_new(options.prefs, field_set)
        .map_err(|_| FunctionError::UnsupportedOperation)?;
    Ok(formatter
        .format(&ZonedTime { time, zone })
        .write_to_string()
        .into_owned())
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_offset_datetime_value(
    date: Date<Iso>,
    time: Time,
    zone: UtcOffset,
    options: &ParsedDateTimeOptions,
) -> Result<String, FunctionError> {
    use writeable::Writeable;

    let field_set = base_datetime_builder("datetime", options, true, false)?
        .build_zoned_date_and_time()
        .map_err(|_| FunctionError::UnsupportedOperation)?;
    let formatter = DateTimeFormatter::try_new(options.prefs, field_set)
        .map_err(|_| FunctionError::UnsupportedOperation)?;
    Ok(formatter
        .format(&ZonedDateTime {
            date,
            time,
            zone: offset_datetime_zone_info(date, time, zone),
        })
        .write_to_string()
        .into_owned())
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn offset_time_zone_info(offset: UtcOffset) -> TimeZoneInfo<models::AtTime> {
    TimeZone::UNKNOWN
        .with_offset(Some(offset))
        .with_zone_name_timestamp(icu_time::zone::ZoneNameTimestamp::far_in_future())
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn offset_datetime_zone_info(
    date: Date<Iso>,
    time: Time,
    offset: UtcOffset,
) -> TimeZoneInfo<models::AtTime> {
    TimeZone::UNKNOWN
        .with_offset(Some(offset))
        .at_date_time(DateTime { date, time })
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_date_value(text: &str) -> Result<Date<Iso>, FunctionError> {
    text.parse().map_err(|_| FunctionError::BadOperand)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_time_value(text: &str) -> Result<Time, FunctionError> {
    text.parse().map_err(|_| FunctionError::BadOperand)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_datetime_value(text: &str) -> Result<DateTime<Iso>, FunctionError> {
    text.parse().map_err(|_| FunctionError::BadOperand)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_offset_zoned_datetime(text: &str) -> Result<ZonedDateTime<Iso, UtcOffset>, FunctionError> {
    ZonedDateTime::try_offset_only_from_str(text, Iso).map_err(|_| FunctionError::BadOperand)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_named_zoned_datetime(
    text: &str,
) -> Result<ZonedDateTime<Iso, TimeZoneInfo<models::AtTime>>, FunctionError> {
    if !text.contains('[') {
        return Err(FunctionError::BadOperand);
    }
    ZonedDateTime::try_strict_from_str(text, Iso, IanaParser::new())
        .or_else(|_| ZonedDateTime::try_location_only_from_str(text, Iso, IanaParser::new()))
        .map_err(|_| FunctionError::BadOperand)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_offset_zoned_time(text: &str) -> Result<ZonedTime<UtcOffset>, FunctionError> {
    let prefixed: alloc::borrow::Cow<'_, str> = if text.starts_with('T') {
        text.into()
    } else {
        format!("T{text}").into()
    };
    ZonedTime::try_offset_only_from_str(prefixed.as_ref()).map_err(|_| FunctionError::BadOperand)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn parse_named_zoned_time(
    text: &str,
) -> Result<ZonedTime<TimeZoneInfo<models::AtTime>>, FunctionError> {
    let prefixed: alloc::borrow::Cow<'_, str> = if text.starts_with('T') {
        text.into()
    } else {
        format!("T{text}").into()
    };
    if !prefixed.contains('[') {
        return Err(FunctionError::BadOperand);
    }
    ZonedTime::try_strict_from_str(prefixed.as_ref(), IanaParser::new())
        .or_else(|_| ZonedTime::try_location_only_from_str(prefixed.as_ref(), IanaParser::new()))
        .map_err(|_| FunctionError::BadOperand)
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn ensure_missing_input_zone(time_zone: ParsedTimeZoneOverride) -> Result<(), FunctionError> {
    if matches!(time_zone, ParsedTimeZoneOverride::Input) {
        Err(FunctionError::BadOperand)
    } else {
        Ok(())
    }
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn ensure_offset_zone_override(
    zone: UtcOffset,
    time_zone: ParsedTimeZoneOverride,
) -> Result<(), FunctionError> {
    match time_zone {
        ParsedTimeZoneOverride::Input => Ok(()),
        ParsedTimeZoneOverride::Default => Ok(()),
        ParsedTimeZoneOverride::Offset(expected) if expected == zone => Ok(()),
        ParsedTimeZoneOverride::Offset(_) | ParsedTimeZoneOverride::Named(_) => {
            Err(FunctionError::BadOption {
                name: "timeZone".into(),
            })
        }
    }
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn ensure_named_zone_override(
    zone: TimeZoneInfo<models::AtTime>,
    time_zone: ParsedTimeZoneOverride,
) -> Result<(), FunctionError> {
    match time_zone {
        ParsedTimeZoneOverride::Input => Ok(()),
        ParsedTimeZoneOverride::Default => Ok(()),
        ParsedTimeZoneOverride::Named(expected) if expected == zone.id() => Ok(()),
        ParsedTimeZoneOverride::Named(_) | ParsedTimeZoneOverride::Offset(_) => {
            Err(FunctionError::BadOption {
                name: "timeZone".into(),
            })
        }
    }
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn is_well_formed_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .split('-')
            .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_alphanumeric()))
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn is_well_formed_time_zone_name(value: &str) -> bool {
    !value.is_empty()
        && value.split('/').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '+'))
        })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locale_core::locale;

    fn und() -> Locale {
        locale!("und")
    }

    #[test]
    fn default_registry_has_string() {
        let r = FunctionRegistry::default_registry();
        assert!(r.get("string").is_some());
    }

    #[test]
    fn string_handler_passes_through() {
        let h = StringHandler;
        let operand = ResolvedValue::new("Ada");
        let out = h
            .format(
                &FunctionContext::new(&und()),
                Some(&operand),
                &FunctionOptions::new(),
            )
            .unwrap();
        assert_eq!(out.text(), "Ada");
    }

    #[test]
    fn string_handler_attaches_selector() {
        let h = StringHandler;
        let operand = ResolvedValue::new("Ada");
        let out = h
            .format(
                &FunctionContext::new(&und()),
                Some(&operand),
                &FunctionOptions::new(),
            )
            .unwrap();
        let sel = out.selector().expect(":string must expose a selector");
        assert_eq!(sel.rank("Ada"), Ok(Some(0)));
        assert_eq!(sel.rank("Bob"), Ok(None));
    }

    #[test]
    #[cfg(feature = "compiled_data")]
    fn string_selector_is_nfc_normalized() {
        let h = StringHandler;
        let operand = ResolvedValue::new("A\u{0308}");
        let out = h
            .format(
                &FunctionContext::new(&und()),
                Some(&operand),
                &FunctionOptions::new(),
            )
            .unwrap();
        let sel = out.selector().unwrap();
        assert_eq!(sel.rank("\u{00C4}"), Ok(Some(0)));
        assert_eq!(sel.rank("A\u{0308}"), Ok(Some(0)));
    }

    #[test]
    fn string_handler_requires_operand() {
        let h = StringHandler;
        let err = h
            .format(&FunctionContext::new(&und()), None, &FunctionOptions::new())
            .unwrap_err();
        assert_eq!(err, FunctionError::BadOperand);
    }

    #[test]
    fn register_custom_handler() {
        #[derive(Debug)]
        struct Echo(&'static str);
        impl FunctionHandler for Echo {
            fn format(
                &self,
                _ctx: &FunctionContext<'_>,
                _op: Option<&ResolvedValue>,
                _o: &FunctionOptions,
            ) -> Result<ResolvedValue, FunctionError> {
                Ok(ResolvedValue::new(self.0))
            }
        }
        let mut r = FunctionRegistry::new();
        r.register("ns:echo", Echo("hi"));
        let h = r.get("ns:echo").unwrap();
        let v = h
            .format(&FunctionContext::new(&und()), None, &FunctionOptions::new())
            .unwrap();
        assert_eq!(v.text(), "hi");
    }

    // ---- :number tests (gated on compiled_data) ----

    #[cfg(feature = "compiled_data")]
    #[test]
    fn default_registry_has_number_and_integer() {
        let r = FunctionRegistry::default_registry();
        assert!(r.get("number").is_some());
        assert!(r.get("integer").is_some());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_handler_formats_decimal() {
        let h = NumberHandler {
            kind: NumberKind::Number,
        };
        let operand = ResolvedValue::new("1234567").with_numeric(Decimal::from(1234567));
        let loc = locale!("en");
        let ctx_en = FunctionContext::new(&loc);
        let out = h
            .format(&ctx_en, Some(&operand), &FunctionOptions::new())
            .unwrap();
        // en uses `,` as the grouping separator.
        assert_eq!(out.text(), "1,234,567");
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_handler_rejects_non_numeric() {
        let h = NumberHandler {
            kind: NumberKind::Number,
        };
        let operand = ResolvedValue::new("not-a-number");
        let err = h
            .format(
                &FunctionContext::new(&und()),
                Some(&operand),
                &FunctionOptions::new(),
            )
            .unwrap_err();
        assert_eq!(err, FunctionError::BadOperand);
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_selector_exact_match() {
        let h = NumberHandler {
            kind: NumberKind::Number,
        };
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let loc = locale!("en");
        let ctx_en = FunctionContext::new(&loc);
        let out = h
            .format(&ctx_en, Some(&operand), &FunctionOptions::new())
            .unwrap();
        let sel = out.selector().unwrap();
        assert_eq!(sel.rank("1"), Ok(Some(0)));
        // English: category for 1 is "one".
        assert_eq!(sel.rank("one"), Ok(Some(1)));
        // "other" is the category for e.g. 5; 1 doesn't match.
        assert_eq!(sel.rank("other"), Ok(None));
        assert_eq!(sel.rank("zero"), Ok(None));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_selector_select_exact_only() {
        let h = NumberHandler {
            kind: NumberKind::Number,
        };
        let mut options = FunctionOptions::new();
        options.insert("select".into(), ResolvedValue::new("exact"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let loc = locale!("en");
        let ctx_en = FunctionContext::new(&loc);
        let out = h.format(&ctx_en, Some(&operand), &options).unwrap();
        let sel = out.selector().unwrap();
        assert_eq!(sel.rank("1"), Ok(Some(0)));
        // With select=exact, plural categories are not matched.
        assert_eq!(sel.rank("one"), Ok(None));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_bad_select_option() {
        let h = NumberHandler {
            kind: NumberKind::Number,
        };
        let mut options = FunctionOptions::new();
        options.insert("select".into(), ResolvedValue::new("bogus"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = h
            .format(&FunctionContext::new(&und()), Some(&operand), &options)
            .unwrap_err();
        assert!(matches!(err, FunctionError::BadOption { name } if name.as_ref() == "select"));
    }

    // ---- Draft datetime functions (unstable + compiled_data) ----

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn unstable_registry_has_datetime_handlers() {
        let r = FunctionRegistry::default_registry();
        assert!(r.get("unit").is_some());
        assert!(r.get("date").is_some());
        assert!(r.get("time").is_some());
        assert!(r.get("datetime").is_some());
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn datetime_handler_formats_localized_output() {
        let h = DateTimeHandler { kind: "datetime" };
        let operand = ResolvedValue::new("2026-04-19T12:34:56");
        let loc = locale!("en");
        let out = h
            .format(
                &FunctionContext::new(&loc),
                Some(&operand),
                &FunctionOptions::new(),
            )
            .unwrap();
        assert_eq!(out.text(), "Apr 19, 2026, 12:34\u{202F}PM");
        assert_eq!(out.part_kind(), "datetime");
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn date_handler_requires_operand() {
        let h = DateTimeHandler { kind: "date" };
        let err = h
            .format(&FunctionContext::new(&und()), None, &FunctionOptions::new())
            .unwrap_err();
        assert_eq!(err, FunctionError::BadOperand);
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn date_handler_rejects_non_iso_operand() {
        let h = DateTimeHandler { kind: "date" };
        let op = ResolvedValue::new("not-a-date");
        let err = h
            .format(
                &FunctionContext::new(&und()),
                Some(&op),
                &FunctionOptions::new(),
            )
            .unwrap_err();
        assert_eq!(err, FunctionError::BadOperand);
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn datetime_handler_accepts_iso_with_timezone() {
        let h = DateTimeHandler { kind: "datetime" };
        let loc = locale!("en");
        for s in [
            "2026-04-19T12:34:56",
            "2026-04-19T12:34:56Z",
            "2026-04-19T12:34:56+00:00",
            "2026-04-19T12:34:56-05:30",
            "2026-04-19T12:34:56.789Z",
        ] {
            let op = ResolvedValue::new(s);
            let out = h
                .format(
                    &FunctionContext::new(&loc),
                    Some(&op),
                    &FunctionOptions::new(),
                )
                .unwrap_or_else(|e| panic!("expected Ok for {s:?}, got {e:?}"));
            assert!(!out.text().is_empty(), "expected localized output for {s}");
            assert_eq!(out.part_kind(), "datetime");
        }
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn datetime_handler_rejects_bad_date_style_option() {
        let h = DateTimeHandler { kind: "date" };
        let op = ResolvedValue::new("2026-04-19");
        let mut options = FunctionOptions::new();
        options.insert("dateStyle".into(), ResolvedValue::new("huge"));
        let err = h
            .format(&FunctionContext::new(&und()), Some(&op), &options)
            .unwrap_err();
        assert!(matches!(
            err,
            FunctionError::BadOption { ref name } if name.as_ref() == "dateStyle"
        ));
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn datetime_handler_accepts_spec_options() {
        // Only the current spec option names are accepted — no legacy
        // ECMA-402 aliases like `dateStyle` / `timeStyle` / `hourCycle`.
        let h = DateTimeHandler { kind: "datetime" };
        let op = ResolvedValue::new("2026-04-19T12:00:00");
        let mut options = FunctionOptions::new();
        options.insert("dateLength".into(), ResolvedValue::new("medium"));
        options.insert("timePrecision".into(), ResolvedValue::new("second"));
        let loc = locale!("en");
        let out = h
            .format(&FunctionContext::new(&loc), Some(&op), &options)
            .unwrap();
        assert_eq!(out.text(), "Apr 19, 2026, 12:00:00\u{202F}PM");
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn datetime_handler_accepts_hour12_option() {
        let h = DateTimeHandler { kind: "datetime" };
        let op = ResolvedValue::new("2026-04-19T16:00:00");
        let mut options = FunctionOptions::new();
        options.insert("hour12".into(), ResolvedValue::new("false"));
        let loc = locale!("en");
        let out = h
            .format(&FunctionContext::new(&loc), Some(&op), &options)
            .unwrap();
        assert_eq!(out.text(), "Apr 19, 2026, 16:00");
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn datetime_handler_rejects_legacy_ecma_options() {
        let h = DateTimeHandler { kind: "datetime" };
        let op = ResolvedValue::new("2026-04-19T12:00:00");
        let mut options = FunctionOptions::new();
        options.insert("dateStyle".into(), ResolvedValue::new("medium"));
        let err = h
            .format(&FunctionContext::new(&und()), Some(&op), &options)
            .unwrap_err();
        assert!(matches!(
            err,
            FunctionError::BadOption { ref name } if name.as_ref() == "dateStyle"
        ));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn integer_handler_rounds_half_expand() {
        // :integer is :number with maximumFractionDigits=0. The default
        // rounding mode is halfExpand (ECMA-402), so 1.5 rounds away from
        // zero to 2.
        use core::str::FromStr;
        let h = NumberHandler {
            kind: NumberKind::Integer,
        };
        let d_frac = Decimal::from_str("1.5").unwrap();
        let operand = ResolvedValue::new("1.5").with_numeric(d_frac);
        let loc = locale!("en");
        let ctx_en = FunctionContext::new(&loc);
        let out = h
            .format(&ctx_en, Some(&operand), &FunctionOptions::new())
            .unwrap();
        assert_eq!(out.text(), "2");
    }

    // ---- is_digit_size_option ABNF conformance ----

    #[cfg(feature = "compiled_data")]
    #[test]
    fn is_digit_size_option_rejects_leading_zero() {
        // Spec ABNF: `"0" / ("1"-"9") [DIGIT]` — no leading zero except the
        // single-digit "0".
        assert!(is_digit_size_option("0"));
        assert!(is_digit_size_option("1"));
        assert!(is_digit_size_option("9"));
        assert!(is_digit_size_option("10"));
        assert!(is_digit_size_option("99"));
        assert!(!is_digit_size_option("00"));
        assert!(!is_digit_size_option("01"));
        assert!(!is_digit_size_option("100"));
        assert!(!is_digit_size_option("-1"));
        assert!(!is_digit_size_option(""));
        assert!(!is_digit_size_option("1a"));
    }

    // ---- NumberOptions::parse edge cases ----

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_options_parse_bad_sign_display() {
        let mut opts = FunctionOptions::new();
        opts.insert("signDisplay".into(), ResolvedValue::new("bogus"));
        let err = NumberOptions::parse(&opts).unwrap_err();
        assert!(matches!(err, FunctionError::BadOption { name } if name.as_ref() == "signDisplay"));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_options_parse_min_gt_max_fraction() {
        let mut opts = FunctionOptions::new();
        opts.insert("minimumFractionDigits".into(), ResolvedValue::new("5"));
        opts.insert("maximumFractionDigits".into(), ResolvedValue::new("2"));
        let err = NumberOptions::parse(&opts).unwrap_err();
        assert!(
            matches!(err, FunctionError::BadOption { name } if name.as_ref() == "minimumFractionDigits")
        );
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_options_parse_min_gt_max_significant() {
        let mut opts = FunctionOptions::new();
        opts.insert("minimumSignificantDigits".into(), ResolvedValue::new("5"));
        opts.insert("maximumSignificantDigits".into(), ResolvedValue::new("3"));
        let err = NumberOptions::parse(&opts).unwrap_err();
        assert!(
            matches!(err, FunctionError::BadOption { name } if name.as_ref() == "minimumSignificantDigits")
        );
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_options_parse_fraction_digits_out_of_range() {
        // parse_digit_count enforces max=100; 101 is rejected.
        let mut opts = FunctionOptions::new();
        opts.insert("maximumFractionDigits".into(), ResolvedValue::new("101"));
        let err = NumberOptions::parse(&opts).unwrap_err();
        assert!(
            matches!(err, FunctionError::BadOption { name } if name.as_ref() == "maximumFractionDigits")
        );
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_options_parse_integer_digits_min_is_1() {
        // min=1, max=21 — `0` should fail since parse_digit_count enforces min.
        let mut opts = FunctionOptions::new();
        opts.insert("minimumIntegerDigits".into(), ResolvedValue::new("0"));
        let err = NumberOptions::parse(&opts).unwrap_err();
        assert!(
            matches!(err, FunctionError::BadOption { name } if name.as_ref() == "minimumIntegerDigits")
        );
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_options_parse_all_rounding_modes_accepted() {
        for mode in [
            "ceil",
            "floor",
            "expand",
            "trunc",
            "halfCeil",
            "halfFloor",
            "halfExpand",
            "halfTrunc",
            "halfEven",
        ] {
            let mut opts = FunctionOptions::new();
            opts.insert("roundingMode".into(), ResolvedValue::new(mode));
            NumberOptions::parse(&opts).unwrap_or_else(|e| {
                panic!("roundingMode={mode} should be accepted, got {e:?}");
            });
        }
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_options_parse_bad_rounding_mode() {
        let mut opts = FunctionOptions::new();
        opts.insert("roundingMode".into(), ResolvedValue::new("randomly"));
        let err = NumberOptions::parse(&opts).unwrap_err();
        assert!(
            matches!(err, FunctionError::BadOption { name } if name.as_ref() == "roundingMode")
        );
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_options_parse_bad_use_grouping() {
        let mut opts = FunctionOptions::new();
        opts.insert("useGrouping".into(), ResolvedValue::new("sometimes"));
        let err = NumberOptions::parse(&opts).unwrap_err();
        assert!(matches!(err, FunctionError::BadOption { name } if name.as_ref() == "useGrouping"));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_options_parse_bad_rounding_priority() {
        let mut opts = FunctionOptions::new();
        opts.insert("roundingPriority".into(), ResolvedValue::new("asap"));
        let err = NumberOptions::parse(&opts).unwrap_err();
        assert!(
            matches!(err, FunctionError::BadOption { name } if name.as_ref() == "roundingPriority")
        );
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn number_options_parse_bad_trailing_zero_display() {
        let mut opts = FunctionOptions::new();
        opts.insert("trailingZeroDisplay".into(), ResolvedValue::new("drop"));
        let err = NumberOptions::parse(&opts).unwrap_err();
        assert!(
            matches!(err, FunctionError::BadOption { name } if name.as_ref() == "trailingZeroDisplay")
        );
    }

    // ---- :offset handler ----

    #[cfg(feature = "compiled_data")]
    #[test]
    fn offset_handler_adds() {
        let mut opts = FunctionOptions::new();
        opts.insert("add".into(), ResolvedValue::new("3"));
        let operand = ResolvedValue::new("10").with_numeric(Decimal::from(10));
        let loc = locale!("en");
        let out = OffsetHandler
            .format(&FunctionContext::new(&loc), Some(&operand), &opts)
            .unwrap();
        assert_eq!(out.text(), "13");
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn offset_handler_subtracts() {
        let mut opts = FunctionOptions::new();
        opts.insert("subtract".into(), ResolvedValue::new("7"));
        let operand = ResolvedValue::new("10").with_numeric(Decimal::from(10));
        let loc = locale!("en");
        let out = OffsetHandler
            .format(&FunctionContext::new(&loc), Some(&operand), &opts)
            .unwrap();
        assert_eq!(out.text(), "3");
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn offset_handler_requires_one_of_add_or_subtract() {
        // No options at all — Bad Option.
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = OffsetHandler
            .format(
                &FunctionContext::new(&und()),
                Some(&operand),
                &FunctionOptions::new(),
            )
            .unwrap_err();
        assert!(matches!(err, FunctionError::BadOption { .. }));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn offset_handler_rejects_both_add_and_subtract() {
        let mut opts = FunctionOptions::new();
        opts.insert("add".into(), ResolvedValue::new("1"));
        opts.insert("subtract".into(), ResolvedValue::new("2"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = OffsetHandler
            .format(&FunctionContext::new(&und()), Some(&operand), &opts)
            .unwrap_err();
        assert!(matches!(err, FunctionError::BadOption { .. }));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn offset_handler_rejects_fractional_delta() {
        let mut opts = FunctionOptions::new();
        opts.insert("add".into(), ResolvedValue::new("1.5"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = OffsetHandler
            .format(&FunctionContext::new(&und()), Some(&operand), &opts)
            .unwrap_err();
        assert!(matches!(err, FunctionError::BadOption { name } if name.as_ref() == "add"));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn offset_handler_rejects_negative_delta() {
        let mut opts = FunctionOptions::new();
        opts.insert("add".into(), ResolvedValue::new("-3"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = OffsetHandler
            .format(&FunctionContext::new(&und()), Some(&operand), &opts)
            .unwrap_err();
        assert!(matches!(err, FunctionError::BadOption { name } if name.as_ref() == "add"));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn offset_handler_requires_numeric_operand() {
        let mut opts = FunctionOptions::new();
        opts.insert("add".into(), ResolvedValue::new("1"));
        // Non-numeric string operand → BadOperand.
        let operand = ResolvedValue::new("hello");
        let err = OffsetHandler
            .format(&FunctionContext::new(&und()), Some(&operand), &opts)
            .unwrap_err();
        assert!(matches!(err, FunctionError::BadOperand));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn offset_result_is_plural_selectable() {
        // After :offset, result should still carry a :number-style selector
        // (plural mode) — so `.match ${x :offset add=1}` works.
        let mut opts = FunctionOptions::new();
        opts.insert("add".into(), ResolvedValue::new("1"));
        let operand = ResolvedValue::new("0").with_numeric(Decimal::from(0));
        let loc = locale!("en");
        let out = OffsetHandler
            .format(&FunctionContext::new(&loc), Some(&operand), &opts)
            .unwrap();
        let sel = out.selector().expect("selector attached");
        // 0 + 1 = 1 — English plural for 1 is "one".
        assert_eq!(sel.rank("one"), Ok(Some(1)));
    }

    // ---- :currency handler ----

    #[cfg(feature = "compiled_data")]
    #[test]
    fn currency_handler_formats_usd() {
        let mut opts = FunctionOptions::new();
        opts.insert("currency".into(), ResolvedValue::new("USD"));
        let operand = ResolvedValue::new("42").with_numeric(Decimal::from(42));
        let loc = locale!("en-US");
        let out = CurrencyHandler
            .format(&FunctionContext::new(&loc), Some(&operand), &opts)
            .unwrap();
        // en-US: "$42.00" — exact decimal places depend on CLDR currency data
        // but we can at least check that $ appears and the digits flow through.
        assert!(out.text().contains('$'), "got: {}", out.text());
        assert!(out.text().contains("42"));
        assert_eq!(out.part_kind(), "currency");
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn currency_handler_requires_currency_option() {
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = CurrencyHandler
            .format(
                &FunctionContext::new(&und()),
                Some(&operand),
                &FunctionOptions::new(),
            )
            .unwrap_err();
        // Without `currency=`, the handler reports BadOperand (per the
        // early `.ok_or(FunctionError::BadOperand)` on the resolved map).
        assert!(matches!(err, FunctionError::BadOperand));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn currency_handler_rejects_non_iso_code() {
        let mut opts = FunctionOptions::new();
        opts.insert("currency".into(), ResolvedValue::new("US$"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = CurrencyHandler
            .format(&FunctionContext::new(&und()), Some(&operand), &opts)
            .unwrap_err();
        assert!(matches!(err, FunctionError::BadOption { name } if name.as_ref() == "currency"));
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn currency_handler_rejects_bad_currency_display() {
        let mut opts = FunctionOptions::new();
        opts.insert("currency".into(), ResolvedValue::new("EUR"));
        opts.insert("currencyDisplay".into(), ResolvedValue::new("bogus"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = CurrencyHandler
            .format(&FunctionContext::new(&und()), Some(&operand), &opts)
            .unwrap_err();
        assert!(
            matches!(err, FunctionError::BadOption { name } if name.as_ref() == "currencyDisplay")
        );
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn currency_handler_rejects_bad_currency_sign() {
        let mut opts = FunctionOptions::new();
        opts.insert("currency".into(), ResolvedValue::new("EUR"));
        opts.insert("currencySign".into(), ResolvedValue::new("euphemism"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = CurrencyHandler
            .format(&FunctionContext::new(&und()), Some(&operand), &opts)
            .unwrap_err();
        assert!(
            matches!(err, FunctionError::BadOption { name } if name.as_ref() == "currencySign")
        );
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn currency_handler_has_no_selector() {
        // :currency is a formatter only — spec defines no selection section,
        // so `.match` on a currency expression emits BadSelector.
        let mut opts = FunctionOptions::new();
        opts.insert("currency".into(), ResolvedValue::new("EUR"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let loc = locale!("en");
        let out = CurrencyHandler
            .format(&FunctionContext::new(&loc), Some(&operand), &opts)
            .unwrap();
        assert!(out.selector().is_none());
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn unit_handler_formats_long_display() {
        let mut opts = FunctionOptions::new();
        opts.insert("unit".into(), ResolvedValue::new("meter"));
        opts.insert("unitDisplay".into(), ResolvedValue::new("long"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let loc = locale!("en-US");
        let out = UnitHandler
            .format(&FunctionContext::new(&loc), Some(&operand), &opts)
            .unwrap();
        assert_eq!(out.text(), "1 meter");
        assert_eq!(out.part_kind(), "unit");
        assert!(out.selector().is_none());
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn unit_handler_requires_unit_option() {
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = UnitHandler
            .format(
                &FunctionContext::new(&und()),
                Some(&operand),
                &FunctionOptions::new(),
            )
            .unwrap_err();
        assert!(matches!(err, FunctionError::BadOperand));
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn unit_handler_rejects_invalid_unit_identifier() {
        let mut opts = FunctionOptions::new();
        opts.insert("unit".into(), ResolvedValue::new("metre"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = UnitHandler
            .format(&FunctionContext::new(&und()), Some(&operand), &opts)
            .unwrap_err();
        assert!(matches!(err, FunctionError::BadOption { name } if name.as_ref() == "unit"));
    }

    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    #[test]
    fn unit_handler_reports_unsupported_usage_conversion() {
        let mut opts = FunctionOptions::new();
        opts.insert("unit".into(), ResolvedValue::new("meter"));
        opts.insert("usage".into(), ResolvedValue::new("road"));
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let err = UnitHandler
            .format(&FunctionContext::new(&und()), Some(&operand), &opts)
            .unwrap_err();
        assert_eq!(err, FunctionError::UnsupportedOperation);
    }

    // ---- :percent handler ----

    #[cfg(feature = "compiled_data")]
    #[test]
    fn percent_handler_scales_by_100() {
        let h = NumberHandler {
            kind: NumberKind::Percent,
        };
        // :percent presents the operand as a percentage — 0.5 → 50%.
        use core::str::FromStr;
        let operand = ResolvedValue::new("0.5").with_numeric(Decimal::from_str("0.5").unwrap());
        let loc = locale!("en");
        let out = h
            .format(
                &FunctionContext::new(&loc),
                Some(&operand),
                &FunctionOptions::new(),
            )
            .unwrap();
        assert!(out.text().contains('%'), "got: {}", out.text());
        assert!(out.text().contains("50"), "got: {}", out.text());
        assert_eq!(out.part_kind(), "percent");
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn percent_handler_attaches_plural_selector() {
        // :percent still supports selection via plural categories.
        let h = NumberHandler {
            kind: NumberKind::Percent,
        };
        let operand = ResolvedValue::new("1").with_numeric(Decimal::from(1));
        let loc = locale!("en");
        let out = h
            .format(
                &FunctionContext::new(&loc),
                Some(&operand),
                &FunctionOptions::new(),
            )
            .unwrap();
        assert!(out.selector().is_some());
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The [`InputValues`] trait and [`InputValue`] enum — the runtime map from
//! MF2 variable names to caller-supplied values.

use alloc::collections::BTreeMap;
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use alloc::format;
use alloc::string::{String, ToString};

use fixed_decimal::Decimal;
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use icu_calendar::{Date, Iso};
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use icu_time::zone::UtcOffset;
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
use icu_time::{DateTime, Time, ZonedDateTime, ZonedTime};

/// An external value supplied to the formatter for a given variable name.
///
/// Text-like, numeric, currency, and unit variants are always available.
/// Date-time variants are available under the `unstable` + `compiled_data`
/// features alongside the draft `:datetime` handler.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum InputValue<'a> {
    /// Absent / explicit null.
    Null,
    /// Boolean, rendered as `"true"` / `"false"`.
    Bool(bool),
    /// Borrowed string.
    String(&'a str),
    /// A finite decimal number. `:number` / `:integer` consume this directly.
    Number(Decimal),
    /// A numeric value bundled with a currency code.
    Currency { value: Decimal, currency: &'a str },
    /// A numeric value bundled with a unit identifier.
    Unit { value: Decimal, unit: &'a str },
    /// An ISO calendar date.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    Date(Date<Iso>),
    /// A wall-clock time.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    Time(Time),
    /// An ISO calendar date and time.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    DateTime(DateTime<Iso>),
    /// An ISO calendar date and time with an explicit UTC offset.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    ZonedDateTime(ZonedDateTime<Iso, UtcOffset>),
    /// A wall-clock time with an explicit UTC offset.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    ZonedTime(ZonedTime<UtcOffset>),
}

impl InputValue<'_> {
    /// Render this input as a plain text string. Numeric values use the
    /// [`Writeable`] impl of [`Decimal`] (decimal point, minus sign, etc.).
    ///
    /// [`Writeable`]: writeable::Writeable
    pub fn to_display_string(&self) -> String {
        match self {
            InputValue::Null => String::new(),
            InputValue::Bool(true) => "true".to_string(),
            InputValue::Bool(false) => "false".to_string(),
            InputValue::String(s) => s.to_string(),
            InputValue::Number(d) => d.to_string(),
            InputValue::Currency { value, .. } | InputValue::Unit { value, .. } => {
                value.to_string()
            }
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            InputValue::Date(date) => format_iso_date(*date),
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            InputValue::Time(time) => format_iso_time(*time),
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            InputValue::DateTime(date_time) => format_iso_datetime(date_time),
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            InputValue::ZonedDateTime(date_time) => format_iso_zoned_datetime(date_time),
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            InputValue::ZonedTime(time) => format_iso_zoned_time(time),
        }
    }

    /// If this value is numeric, borrow it.
    pub fn as_number(&self) -> Option<&Decimal> {
        match self {
            InputValue::Number(d)
            | InputValue::Currency { value: d, .. }
            | InputValue::Unit { value: d, .. } => Some(d),
            _ => None,
        }
    }
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_iso_date(date: Date<Iso>) -> String {
    format!(
        "{}-{:02}-{:02}",
        format_iso_year(date.year().extended_year()),
        date.month().ordinal,
        date.day_of_month().0
    )
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_iso_time(time: Time) -> String {
    if time.subsecond.number() == 0 {
        format!(
            "{:02}:{:02}:{:02}",
            time.hour.number(),
            time.minute.number(),
            time.second.number()
        )
    } else {
        let mut frac = format!("{:09}", time.subsecond.number());
        while frac.ends_with('0') {
            frac.pop();
        }
        format!(
            "{:02}:{:02}:{:02}.{}",
            time.hour.number(),
            time.minute.number(),
            time.second.number(),
            frac
        )
    }
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_iso_datetime(date_time: &DateTime<Iso>) -> String {
    format!(
        "{}T{}",
        format_iso_date(date_time.date),
        format_iso_time(date_time.time)
    )
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_iso_zoned_datetime(date_time: &ZonedDateTime<Iso, UtcOffset>) -> String {
    format!(
        "{}{}",
        format_iso_datetime(&DateTime {
            date: date_time.date,
            time: date_time.time,
        }),
        format_utc_offset(date_time.zone)
    )
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_iso_zoned_time(time: &ZonedTime<UtcOffset>) -> String {
    format!(
        "{}{}",
        format_iso_time(time.time),
        format_utc_offset(time.zone)
    )
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_iso_year(year: i32) -> String {
    if (0..=9999).contains(&year) {
        format!("{year:04}")
    } else {
        let sign = if year < 0 { '-' } else { '+' };
        format!("{sign}{:04}", year.unsigned_abs())
    }
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
fn format_utc_offset(offset: UtcOffset) -> String {
    if offset == UtcOffset::zero() {
        return "Z".to_string();
    }
    let total = offset.to_seconds();
    let sign = if total < 0 { '-' } else { '+' };
    let abs = total.abs();
    let hours = abs / 3600;
    let minutes = (abs % 3600) / 60;
    format!("{sign}{hours:02}:{minutes:02}")
}

/// Runtime source of MF2 variable values.
///
/// Implemented for `BTreeMap<String, String>` and any `&[(&str, &str)]`
/// slice for ergonomic construction; downstream callers can implement it
/// on their own config/context types.
pub trait InputValues {
    /// Resolve `name` to an [`InputValue`], or `None` if unbound.
    fn get(&self, name: &str) -> Option<InputValue<'_>>;
}

impl InputValues for BTreeMap<String, String> {
    fn get(&self, name: &str) -> Option<InputValue<'_>> {
        // Spec compares variable names as-if NFC-normalized on BOTH sides:
        // stored keys written in any canonical form must match queries in any
        // equivalent form.
        let normalized = nfc_key(name);
        if let Some(v) = BTreeMap::get(self, name) {
            return Some(InputValue::String(v.as_str()));
        }
        if let Some(v) = BTreeMap::get(self, normalized.as_ref()) {
            return Some(InputValue::String(v.as_str()));
        }
        self.iter()
            .find(|(k, _)| nfc_key(k.as_str()) == normalized)
            .map(|(_, v)| InputValue::String(v.as_str()))
    }
}

impl<'a> InputValues for &[(&'a str, &'a str)] {
    fn get(&self, name: &str) -> Option<InputValue<'_>> {
        let normalized = nfc_key(name);
        self.iter()
            .find(|(k, _)| *k == name || *k == normalized.as_ref() || nfc_key(k) == normalized)
            .map(|(_, v)| InputValue::String(v))
    }
}

fn nfc_key(s: &str) -> alloc::borrow::Cow<'_, str> {
    #[cfg(feature = "compiled_data")]
    {
        use icu_normalizer::ComposingNormalizer;
        let n = ComposingNormalizer::new_nfc();
        n.normalize(s)
    }
    #[cfg(not(feature = "compiled_data"))]
    {
        alloc::borrow::Cow::Borrowed(s)
    }
}

/// A newtype over a map that carries owned [`InputValue`]s, so tests and
/// callers can mix string and numeric inputs in one container.
///
/// The lifetime on [`InputValue`] is erased to `'static` for all
/// owned-data variants (`Null`, `Bool`, `Number`). String values are
/// stored as owned [`String`]s and lent out as `&str` at lookup time.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct OwnedInputs {
    entries: BTreeMap<String, OwnedValue>,
}

#[derive(Debug, Clone)]
enum OwnedValue {
    Null,
    Bool(bool),
    String(String),
    Number(Decimal),
    Currency {
        value: Decimal,
        currency: String,
    },
    Unit {
        value: Decimal,
        unit: String,
    },
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    Date(Date<Iso>),
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    Time(Time),
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    DateTime(DateTime<Iso>),
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    ZonedDateTime(ZonedDateTime<Iso, UtcOffset>),
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    ZonedTime(ZonedTime<UtcOffset>),
}

impl OwnedInputs {
    /// Create an empty map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a string value.
    pub fn with_str(mut self, name: &str, value: &str) -> Self {
        self.entries
            .insert(name.to_string(), OwnedValue::String(value.to_string()));
        self
    }

    /// Insert a numeric value.
    pub fn with_number(mut self, name: &str, value: impl Into<Decimal>) -> Self {
        self.entries
            .insert(name.to_string(), OwnedValue::Number(value.into()));
        self
    }

    /// Insert a currency value.
    pub fn with_currency(mut self, name: &str, value: impl Into<Decimal>, currency: &str) -> Self {
        self.entries.insert(
            name.to_string(),
            OwnedValue::Currency {
                value: value.into(),
                currency: currency.to_string(),
            },
        );
        self
    }

    /// Insert a unit value.
    pub fn with_unit(mut self, name: &str, value: impl Into<Decimal>, unit: &str) -> Self {
        self.entries.insert(
            name.to_string(),
            OwnedValue::Unit {
                value: value.into(),
                unit: unit.to_string(),
            },
        );
        self
    }

    /// Insert a boolean value.
    pub fn with_bool(mut self, name: &str, value: bool) -> Self {
        self.entries
            .insert(name.to_string(), OwnedValue::Bool(value));
        self
    }

    /// Insert an explicit null.
    pub fn with_null(mut self, name: &str) -> Self {
        self.entries.insert(name.to_string(), OwnedValue::Null);
        self
    }

    /// Insert an ISO date.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    pub fn with_date(mut self, name: &str, value: Date<Iso>) -> Self {
        self.entries
            .insert(name.to_string(), OwnedValue::Date(value));
        self
    }

    /// Insert a time.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    pub fn with_time(mut self, name: &str, value: Time) -> Self {
        self.entries
            .insert(name.to_string(), OwnedValue::Time(value));
        self
    }

    /// Insert an ISO date and time.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    pub fn with_datetime(mut self, name: &str, value: DateTime<Iso>) -> Self {
        self.entries
            .insert(name.to_string(), OwnedValue::DateTime(value));
        self
    }

    /// Insert an ISO date and time with an explicit UTC offset.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    pub fn with_zoned_datetime(mut self, name: &str, value: ZonedDateTime<Iso, UtcOffset>) -> Self {
        self.entries
            .insert(name.to_string(), OwnedValue::ZonedDateTime(value));
        self
    }

    /// Insert a time with an explicit UTC offset.
    #[cfg(all(feature = "unstable", feature = "compiled_data"))]
    pub fn with_zoned_time(mut self, name: &str, value: ZonedTime<UtcOffset>) -> Self {
        self.entries
            .insert(name.to_string(), OwnedValue::ZonedTime(value));
        self
    }
}

impl InputValues for OwnedInputs {
    fn get(&self, name: &str) -> Option<InputValue<'_>> {
        let normalized = nfc_key(name);
        let entry = self
            .entries
            .get(name)
            .or_else(|| self.entries.get(normalized.as_ref()))
            .or_else(|| {
                self.entries
                    .iter()
                    .find(|(k, _)| nfc_key(k.as_str()) == normalized)
                    .map(|(_, v)| v)
            })?;
        Some(match entry {
            OwnedValue::Null => InputValue::Null,
            OwnedValue::Bool(b) => InputValue::Bool(*b),
            OwnedValue::String(s) => InputValue::String(s.as_str()),
            OwnedValue::Number(n) => InputValue::Number(n.clone()),
            OwnedValue::Currency { value, currency } => InputValue::Currency {
                value: value.clone(),
                currency: currency.as_str(),
            },
            OwnedValue::Unit { value, unit } => InputValue::Unit {
                value: value.clone(),
                unit: unit.as_str(),
            },
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            OwnedValue::Date(date) => InputValue::Date(*date),
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            OwnedValue::Time(time) => InputValue::Time(*time),
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            OwnedValue::DateTime(date_time) => InputValue::DateTime(*date_time),
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            OwnedValue::ZonedDateTime(date_time) => InputValue::ZonedDateTime(*date_time),
            #[cfg(all(feature = "unstable", feature = "compiled_data"))]
            OwnedValue::ZonedTime(time) => InputValue::ZonedTime(*time),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_display_string_for_each_variant() {
        assert_eq!(InputValue::Null.to_display_string(), "");
        assert_eq!(InputValue::Bool(true).to_display_string(), "true");
        assert_eq!(InputValue::Bool(false).to_display_string(), "false");
        assert_eq!(InputValue::String("hello").to_display_string(), "hello");
        let d: Decimal = 42_i64.into();
        assert_eq!(InputValue::Number(d).to_display_string(), "42");
    }

    #[test]
    fn as_number_returns_some_only_for_number() {
        assert!(InputValue::Null.as_number().is_none());
        assert!(InputValue::Bool(true).as_number().is_none());
        assert!(InputValue::String("3").as_number().is_none());
        let d: Decimal = 7_i64.into();
        assert!(InputValue::Number(d).as_number().is_some());
    }

    #[test]
    fn slice_lookup_hit_and_miss() {
        let inputs: &[(&str, &str)] = &[("name", "Ada"), ("city", "Paris")];
        match InputValues::get(&inputs, "name") {
            Some(InputValue::String(s)) => assert_eq!(s, "Ada"),
            _ => panic!("expected String"),
        }
        assert!(InputValues::get(&inputs, "missing").is_none());
    }

    #[test]
    fn btreemap_lookup() {
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), "1".to_string());
        m.insert("y".to_string(), "2".to_string());
        match <BTreeMap<String, String> as InputValues>::get(&m, "x") {
            Some(InputValue::String(s)) => assert_eq!(s, "1"),
            _ => panic!("expected String"),
        }
        assert!(<BTreeMap<String, String> as InputValues>::get(&m, "z").is_none());
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn nfc_match_across_forms() {
        // "é" can be written either pre-composed (U+00E9) or decomposed
        // (U+0065 U+0301). Spec says variable-name lookup compares as if
        // both sides were NFC-normalized — so either form must find either.
        let composed = "caf\u{00E9}";
        let decomposed = "caf\u{0065}\u{0301}";
        let inputs: &[(&str, &str)] = &[(decomposed, "value")];
        match InputValues::get(&inputs, composed) {
            Some(InputValue::String(s)) => assert_eq!(s, "value"),
            _ => panic!("expected NFC match"),
        }
    }

    #[test]
    fn owned_inputs_all_variants() {
        let d: Decimal = 99_i64.into();
        let inputs = OwnedInputs::new()
            .with_str("s", "text")
            .with_number("n", d.clone())
            .with_bool("b", true)
            .with_null("z");
        assert!(matches!(
            inputs.get("s"),
            Some(InputValue::String(v)) if v == "text"
        ));
        assert!(matches!(
            inputs.get("n"),
            Some(InputValue::Number(ref x)) if x == &d
        ));
        assert!(matches!(inputs.get("b"), Some(InputValue::Bool(true))));
        assert!(matches!(inputs.get("z"), Some(InputValue::Null)));
        assert!(inputs.get("nope").is_none());
    }

    #[test]
    fn owned_inputs_overwrite_on_duplicate() {
        let inputs = OwnedInputs::new()
            .with_str("k", "first")
            .with_str("k", "second");
        match inputs.get("k") {
            Some(InputValue::String(s)) => assert_eq!(s, "second"),
            _ => panic!(),
        }
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn owned_inputs_nfc_match() {
        let decomposed = "caf\u{0065}\u{0301}";
        let composed = "caf\u{00E9}";
        let inputs = OwnedInputs::new().with_str(decomposed, "owned");
        match inputs.get(composed) {
            Some(InputValue::String(s)) => assert_eq!(s, "owned"),
            _ => panic!("expected NFC match"),
        }
    }
}

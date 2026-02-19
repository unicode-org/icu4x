// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON numbers.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-numbers-full/main/en/numbers.json>

use core::fmt::{Display, Write};
use icu::plurals::PluralElements;
use icu_pattern::{DoublePlaceholder, PatternString};
use icu_provider::DataError;
use itertools::Itertools;
use serde::de::{Deserializer, Error, MapAccess, Unexpected, Visitor};
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};

/// Representation of a UTS-35 number pattern, including positive subpattern (required) and negative
/// subpattern (optional).
#[derive(Debug, PartialEq)]
pub(crate) struct NumberPattern {
    pub(crate) positive: Vec<NumberPatternItem>,
    pub(crate) negative: Option<Vec<NumberPatternItem>>,
}

impl Display for NumberPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.positive {
            f.write_str(i.as_str())?;
        }
        if let Some(n) = self.negative.as_ref() {
            f.write_char(';')?;
            for i in n {
                f.write_str(i.as_str())?;
            }
        }
        Ok(())
    }
}

impl NumberPattern {
    pub(crate) fn try_from_str(s: &str) -> Result<Self, DataError> {
        let (p, n) = s
            .split_once(';')
            .map(|(p, n)| (p, Some(n)))
            .unwrap_or((s, None));

        fn parse_sub_pattern(s: &str) -> Result<Vec<NumberPatternItem>, DataError> {
            let mut items = Vec::new();
            let mut chars = s.chars().peekable();
            let mut in_quote = false;
            let mut string_buffer = String::new();

            fn append_literal(items: &mut Vec<NumberPatternItem>, s: &str) {
                if let Some(NumberPatternItem::Literal(last)) = items.last_mut() {
                    last.push_str(s);
                } else {
                    items.push(NumberPatternItem::Literal(s.to_string()));
                }
            }

            while let Some(c) = chars.next() {
                if in_quote {
                    if c == '\'' {
                        if chars.peek() == Some(&'\'') {
                            // Escaped quote ''
                            string_buffer.push('\'');
                            chars.next();
                        } else {
                            // End of quote
                            in_quote = false;
                            if !string_buffer.is_empty() {
                                append_literal(&mut items, &string_buffer);
                                string_buffer.clear();
                            }
                        }
                    } else {
                        string_buffer.push(c);
                    }
                } else {
                    match c {
                        '\'' => {
                            in_quote = true;
                        }
                        '0' => items.push(NumberPatternItem::MandatoryDigit),
                        '#' => items.push(NumberPatternItem::OptionalDigit),
                        '.' => items.push(NumberPatternItem::DecimalSeparator),
                        ',' => items.push(NumberPatternItem::GroupingSeparator),
                        '¤' => items.push(NumberPatternItem::Currency),
                        '%' => items.push(NumberPatternItem::Percent),
                        '‰' => items.push(NumberPatternItem::PerMille),
                        '+' => items.push(NumberPatternItem::PlusSign),
                        '-' => items.push(NumberPatternItem::MinusSign),
                        'E' => items.push(NumberPatternItem::Exponent),
                        _ => {
                            // Unquoted literal character
                            let mut temp = String::new();
                            temp.push(c);
                            append_literal(&mut items, &temp);
                        }
                    }
                }
            }

            if in_quote {
                return Err(DataError::custom("UnclosedQuote"));
            }
            Ok(items)
        }

        Ok(Self {
            positive: parse_sub_pattern(p)?,
            negative: if let Some(n) = n {
                Some(parse_sub_pattern(n)?)
            } else {
                None
            },
        })
    }
}

impl<'de> Deserialize<'de> for NumberPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Cow::<str>::deserialize(deserializer)?;

        Self::try_from_str(&s).map_err(D::Error::custom)
    }
}

/// An item in a decimal pattern (used during parsing).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NumberPatternItem {
    Literal(String),
    MandatoryDigit,
    OptionalDigit,
    DecimalSeparator,
    GroupingSeparator,
    Currency,
    Percent,
    PerMille,
    PlusSign,
    MinusSign,
    Exponent,
}

impl NumberPatternItem {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Literal(s) => s,
            Self::MandatoryDigit => "0",
            Self::OptionalDigit => "#",
            Self::DecimalSeparator => ".",
            Self::GroupingSeparator => ",",
            Self::Currency => "¤",
            Self::Percent => "%",
            Self::PerMille => "‰",
            Self::PlusSign => "+",
            Self::MinusSign => "-",
            Self::Exponent => "E",
        }
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Symbols {
    // This list is not comprehensive; add more fields when needed
    #[serde(rename = "approximatelySign")]
    pub(crate) approximately_sign: String,
    pub(crate) decimal: String,
    pub(crate) group: String,
    #[serde(rename = "minusSign")]
    pub(crate) minus_sign: String,
    #[serde(rename = "plusSign")]
    pub(crate) plus_sign: String,
    #[serde(rename = "percentSign")]
    pub(crate) percent_sign: String,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct DecimalFormats {
    pub(crate) standard: NumberPattern,
    pub(crate) long: DecimalFormatLength,
    pub(crate) short: DecimalFormatLength,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct DecimalFormatLength {
    #[serde(rename = "decimalFormat")]
    pub(crate) decimal_format: DecimalFormat,
}

#[derive(PartialEq, Debug, Default)]
pub(crate) struct DecimalFormat {
    pub(crate) standard: BTreeMap<u8, PluralElements<NumberPattern>>,
    pub(crate) alpha_next_to_number: BTreeMap<u8, PluralElements<NumberPattern>>,
}

impl<'de> Deserialize<'de> for DecimalFormat {
    fn deserialize<D>(deserializer: D) -> Result<DecimalFormat, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DecimalFormatVisitor)
    }
}

struct DecimalFormatVisitor;
impl<'de> Visitor<'de> for DecimalFormatVisitor {
    type Value = DecimalFormat;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map from keys of the form 10*-count-(zero|one|few|many|other) to compact decimal patterns")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut patterns = BTreeMap::<u8, BTreeMap<String, NumberPattern>>::new();

        while let Some(key) = access.next_key::<String>()? {
            let (magnitude, count) = key.split("-count-").next_tuple().ok_or_else(|| {
                M::Error::invalid_value(Unexpected::Str(&key), &"key to contain -count-")
            })?;

            let mut type_bytes = magnitude.bytes();

            if !(type_bytes.next() == Some(b'1') && type_bytes.all(|b| b == b'0')) {
                return Err(M::Error::custom(format_args!(
                    "Ill-formed type {magnitude}"
                )));
            }
            let log10_type = u8::try_from(magnitude.len() - 1).map_err(|_| {
                M::Error::custom(format_args!("Too many digits in type {magnitude}"))
            })?;

            patterns
                .entry(log10_type)
                .or_default()
                .insert(count.into(), access.next_value()?);
        }

        let standard = patterns
            .iter_mut()
            .filter_map(|(k, v)| {
                Some((
                    *k,
                    PluralElements::new(v.remove("other")?)
                        .with_explicit_zero_value(v.remove("0"))
                        .with_explicit_one_value(v.remove("1"))
                        .with_zero_value(v.remove("zero"))
                        .with_one_value(v.remove("one"))
                        .with_two_value(v.remove("two"))
                        .with_few_value(v.remove("few"))
                        .with_many_value(v.remove("many")),
                ))
            })
            .collect();

        let alpha_next_to_number = patterns
            .iter_mut()
            .filter_map(|(k, v)| {
                Some((
                    *k,
                    PluralElements::new(v.remove("other-alt-alphaNextToNumber")?)
                        .with_explicit_zero_value(v.remove("0-alt-alphaNextToNumber"))
                        .with_explicit_one_value(v.remove("1-alt-alphaNextToNumber"))
                        .with_zero_value(v.remove("zero-alt-alphaNextToNumber"))
                        .with_one_value(v.remove("one-alt-alphaNextToNumber"))
                        .with_two_value(v.remove("two-alt-alphaNextToNumber"))
                        .with_few_value(v.remove("few-alt-alphaNextToNumber"))
                        .with_many_value(v.remove("many-alt-alphaNextToNumber")),
                ))
            })
            .collect();

        Ok(DecimalFormat {
            standard,
            alpha_next_to_number,
        })
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct ShortCompactCurrencyPatterns {
    pub(crate) standard: DecimalFormat,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct CurrencyFormattingPatterns {
    /// Standard pattern
    pub(crate) standard: NumberPattern,

    /// Contains the compact currency patterns for short compact currency formatting
    #[serde(rename = "short")]
    pub(crate) compact_short: Option<ShortCompactCurrencyPatterns>,

    /// Standard alphaNextToNumber pattern
    #[serde(rename = "standard-alphaNextToNumber")]
    pub(crate) standard_alpha_next_to_number: Option<NumberPattern>,

    #[serde(rename = "unitPattern-count-0")]
    pub(crate) pattern_explicit_zero: Option<PatternString<DoublePlaceholder>>,

    #[serde(rename = "unitPattern-count-1")]
    pub(crate) pattern_explicit_one: Option<PatternString<DoublePlaceholder>>,

    #[serde(rename = "unitPattern-count-zero")]
    pub(crate) pattern_zero: Option<PatternString<DoublePlaceholder>>,

    #[serde(rename = "unitPattern-count-one")]
    pub(crate) pattern_one: Option<PatternString<DoublePlaceholder>>,

    #[serde(rename = "unitPattern-count-two")]
    pub(crate) pattern_two: Option<PatternString<DoublePlaceholder>>,

    #[serde(rename = "unitPattern-count-few")]
    pub(crate) pattern_few: Option<PatternString<DoublePlaceholder>>,

    #[serde(rename = "unitPattern-count-many")]
    pub(crate) pattern_many: Option<PatternString<DoublePlaceholder>>,

    #[serde(rename = "unitPattern-count-other")]
    pub(crate) pattern_other: Option<PatternString<DoublePlaceholder>>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct PercentFormattingPatterns {
    /// Standard pattern
    pub(crate) standard: String,
}

#[derive(PartialEq, Debug, Default)]
pub(crate) struct NumberingSystemData {
    /// Map from numbering system to symbols
    pub(crate) symbols: HashMap<String, Symbols>,
    /// Map from numbering system to decimal formats
    pub(crate) formats: HashMap<String, DecimalFormats>,
    /// Map from numbering system to patterns
    pub(crate) currency_patterns: HashMap<String, CurrencyFormattingPatterns>,
    /// Map from numbering system to percent patterns
    pub(crate) percent_patterns: HashMap<String, PercentFormattingPatterns>,
}

pub(crate) struct NumberingSystemDataVisitor;

impl<'de> Visitor<'de> for NumberingSystemDataVisitor {
    type Value = NumberingSystemData;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("formatting data by numbering system")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = NumberingSystemData::default();
        while let Some(key) = access.next_key::<String>()? {
            // Key is of the form: "symbols-numberSystem-latn"
            let (stype, _, numsys) = match key.split('-').next_tuple() {
                Some(v) => v,
                None => continue, // Not what we were looking for; ignore.
            };
            match stype {
                "symbols" => {
                    let value: Symbols = access.next_value()?;
                    result.symbols.insert(numsys.to_string(), value);
                }
                "decimalFormats" => {
                    let value: DecimalFormats = access.next_value()?;
                    result.formats.insert(numsys.to_string(), value);
                }
                "currencyFormats" => {
                    let value: CurrencyFormattingPatterns = access.next_value()?;
                    result.currency_patterns.insert(numsys.to_string(), value);
                }
                "percentFormats" => {
                    let value: PercentFormattingPatterns = access.next_value()?;
                    result.percent_patterns.insert(numsys.to_string(), value);
                }
                _ => {
                    // When needed, consume "scientificFormats", "percentFormats", ...
                    // For now, ignore them.
                }
            }
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for NumberingSystemData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(NumberingSystemDataVisitor)
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Numbers {
    #[serde(rename = "defaultNumberingSystem")]
    pub(crate) default_numbering_system: String,
    #[serde(rename = "minimumGroupingDigits")]
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
    pub(crate) minimum_grouping_digits: u8,
    #[serde(flatten)]
    pub(crate) numsys_data: NumberingSystemData,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LangNumbers {
    pub(crate) numbers: Numbers,
}

pub(crate) type Resource = super::LocaleResource<LangNumbers>;

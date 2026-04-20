// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use icu_provider::prelude::*;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::{VarZeroVec, ZeroMap};

#[cfg(feature = "serde")]
use icu_pattern::DoublePlaceholder;
use icu_pattern::{DoublePlaceholderKey, DoublePlaceholderPattern, PatternItem};

use crate::dimension::currency::options::{CurrencyDisplaySign, Width};
use crate::dimension::currency::CurrencyCode;

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;

icu_provider::data_marker!(
    /// Essential currency data needed for currency formatting. For example, currency patterns.
    CurrencyEssentialsV1,
    CurrencyEssentials<'static>
);

/// This type contains all of the essential data for currency formatting.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone, PartialEq, Debug, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path =  icu_experimental::dimension::provider::currency::essentials))]
#[yoke(prove_covariance_manually)]
pub struct CurrencyEssentials<'data> {
    /// A mapping from 3-letter currency ISO codes to their [`CurrencyPatternConfig`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern_config_map: ZeroMap<'data, UnvalidatedTinyAsciiStr<3>, CurrencyPatternConfig>,

    /// The standard currency pattern used for formatting.
    ///
    /// This pattern uses two placeholders:
    /// - `0`: The numeric currency value.
    /// - `1`: The currency symbol (`¤`).
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<DoublePlaceholder, _>"
        )
    )]
    pub standard_pattern: Cow<'data, DoublePlaceholderPattern>,

    /// Optional negative subpattern for [`Self::standard_pattern`] (CLDR `;` suffix).
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            default,
            deserialize_with = "icu_pattern::deserialize_option_borrowed_cow::<DoublePlaceholder, _>"
        )
    )]
    pub standard_negative_pattern: Option<Cow<'data, DoublePlaceholderPattern>>,

    /// The `standard_alpha_next_to_number` currency pattern used for formatting.
    ///
    /// This pattern uses two placeholders:
    /// - `0`: The numeric currency value.
    /// - `1`: The currency symbol (`¤`).
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<DoublePlaceholder, _>"
        )
    )]
    pub standard_alpha_next_to_number_pattern: Cow<'data, DoublePlaceholderPattern>,

    /// Optional negative subpattern for [`Self::standard_alpha_next_to_number_pattern`].
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            default,
            deserialize_with = "icu_pattern::deserialize_option_borrowed_cow::<DoublePlaceholder, _>"
        )
    )]
    pub standard_alpha_next_to_number_negative_pattern:
        Option<Cow<'data, DoublePlaceholderPattern>>,

    /// CLDR `accounting` pattern — positive subpattern (before `;` when present).
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<DoublePlaceholder, _>"
        )
    )]
    pub accounting_pattern: Cow<'data, DoublePlaceholderPattern>,

    /// CLDR `accounting` pattern — negative subpattern (after `;`), when present.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            default,
            deserialize_with = "icu_pattern::deserialize_option_borrowed_cow::<DoublePlaceholder, _>"
        )
    )]
    pub accounting_negative_pattern: Option<Cow<'data, DoublePlaceholderPattern>>,

    /// CLDR `accounting-alphaNextToNumber` — positive subpattern.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            deserialize_with = "icu_pattern::deserialize_borrowed_cow::<DoublePlaceholder, _>"
        )
    )]
    pub accounting_alpha_next_to_number_pattern: Cow<'data, DoublePlaceholderPattern>,

    /// CLDR `accounting-alphaNextToNumber` — negative subpattern, when present.
    #[cfg_attr(
        feature = "serde",
        serde(
            borrow,
            default,
            deserialize_with = "icu_pattern::deserialize_option_borrowed_cow::<DoublePlaceholder, _>"
        )
    )]
    pub accounting_alpha_next_to_number_negative_pattern:
        Option<Cow<'data, DoublePlaceholderPattern>>,

    /// A list of placeholders (strings), such as currency symbols, referenced by index.
    ///
    /// These values are retrieved using [`PlaceholderValue::Index`] stored in [`CurrencyPatternConfig`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub placeholders: VarZeroVec<'data, str>,

    /// The fallback currency pattern configuration used
    /// when a specific currency's pattern is not found in the currency patterns map.
    pub default_pattern_config: CurrencyPatternConfig,
}

icu_provider::data_struct!(CurrencyEssentials<'_>, #[cfg(feature = "datagen")]);

#[zerovec::make_ule(PatternSelectionULE)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::essentials))]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum PatternSelection {
    /// Use the standard pattern.
    #[default]
    Standard = 0,

    /// Use the `standard_alpha_next_to_number` pattern.
    StandardAlphaNextToNumber = 1,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::essentials))]
#[derive(Copy, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u16)]
pub enum PlaceholderValue {
    /// The index of the place holder in the place holders list.
    /// NOTE: the maximum value is `MAX_PLACEHOLDER_INDEX` which is 2045 (`0b0111_1111_1101`).
    Index(u16),

    /// The place holder is the iso code.
    ISO,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::essentials))]
#[derive(Copy, Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct CurrencyPatternConfig {
    /// Indicates which pattern to use for short currency formatting.
    pub short_pattern_selection: PatternSelection,

    /// Indicates which pattern to use for narrow currency formatting.
    pub narrow_pattern_selection: PatternSelection,

    /// The index of the short pattern place holder in the place holders list.
    /// If the value is `None`, this means that the short pattern does not have a place holder.
    pub short_placeholder_value: Option<PlaceholderValue>,

    /// The index of the narrow pattern place holder in the place holders list.
    /// If the value is `None`, this means that the narrow pattern does not have a place holder.
    pub narrow_placeholder_value: Option<PlaceholderValue>,
}

/// Result of [`CurrencyEssentials::resolve_currency_pattern`].
pub(crate) struct ResolvedCurrencyPattern<'a> {
    pub currency: &'a str,
    pub pattern: &'a DoublePlaceholderPattern,
    /// When true, format the numeric magnitude with [`fixed_decimal::Sign::None`] — the pattern
    /// already encodes parentheses or other sign semantics.
    pub sign_encoded_in_pattern: bool,
}

impl<'a> CurrencyEssentials<'a> {
    fn currency_str_and_selection(
        &'a self,
        width: Width,
        currency: &'a CurrencyCode,
    ) -> (&'a str, PatternSelection) {
        let config = self
            .pattern_config_map
            .get_copied(&currency.0.to_unvalidated())
            .unwrap_or(self.default_pattern_config);

        let placeholder_index = match width {
            Width::Short => config.short_placeholder_value,
            Width::Narrow => config.narrow_placeholder_value,
        };

        let currency = match placeholder_index {
            Some(PlaceholderValue::Index(index)) => self.placeholders.get(index.into()),
            Some(PlaceholderValue::ISO) | None => None,
        }
        .unwrap_or(currency.0.as_str());

        let pattern_selection = match width {
            Width::Short => config.short_pattern_selection,
            Width::Narrow => config.narrow_pattern_selection,
        };
        (currency, pattern_selection)
    }

    /// Resolves the CLDR pattern for short/narrow currency formatting, including optional
    /// negative subpatterns and accounting vs standard pattern choice.
    pub(crate) fn resolve_currency_pattern(
        &'a self,
        width: Width,
        currency: &'a CurrencyCode,
        currency_display_sign: CurrencyDisplaySign,
        value_is_negative: bool,
    ) -> ResolvedCurrencyPattern<'a> {
        let (currency, pattern_selection) = self.currency_str_and_selection(width, currency);
        let (positive, negative_opt) = match currency_display_sign {
            CurrencyDisplaySign::Standard => match pattern_selection {
                PatternSelection::Standard => (
                    self.standard_pattern.as_ref(),
                    self.standard_negative_pattern.as_ref(),
                ),
                PatternSelection::StandardAlphaNextToNumber => (
                    self.standard_alpha_next_to_number_pattern.as_ref(),
                    self.standard_alpha_next_to_number_negative_pattern.as_ref(),
                ),
            },
            CurrencyDisplaySign::Accounting => match pattern_selection {
                PatternSelection::Standard => (
                    self.accounting_pattern.as_ref(),
                    self.accounting_negative_pattern.as_ref(),
                ),
                PatternSelection::StandardAlphaNextToNumber => (
                    self.accounting_alpha_next_to_number_pattern.as_ref(),
                    self.accounting_alpha_next_to_number_negative_pattern
                        .as_ref(),
                ),
            },
        };
        if value_is_negative {
            if let Some(neg) = negative_opt {
                return ResolvedCurrencyPattern {
                    currency,
                    pattern: neg.as_ref(),
                    sign_encoded_in_pattern: true,
                };
            }
        }
        ResolvedCurrencyPattern {
            currency,
            pattern: positive,
            sign_encoded_in_pattern: false,
        }
    }
}

/// Concatenates literals before the first placeholder and after the last placeholder in a
/// [`DoublePlaceholderPattern`].
///
/// Used to reuse CLDR accounting negative framing (for example parentheses) when the formatted
/// monetary value is built from a different placeholder layout than the short `¤` pattern.
pub(crate) fn outer_literal_affixes_double_placeholder(
    pattern: &DoublePlaceholderPattern,
) -> (String, String) {
    let items: Vec<PatternItem<'_, DoublePlaceholderKey>> = pattern.iter().collect();
    let first_ph = items
        .iter()
        .position(|m| matches!(m, PatternItem::Placeholder(_)));
    let last_ph = items
        .iter()
        .rposition(|m| matches!(m, PatternItem::Placeholder(_)));
    let (f, l) = match (first_ph, last_ph) {
        (Some(f), Some(l)) => (f, l),
        _ => return (String::new(), String::new()),
    };
    let mut prefix = String::new();
    for it in &items[..f] {
        if let PatternItem::Literal(s) = it {
            prefix.push_str(s);
        }
    }
    let mut suffix = String::new();
    for it in &items[l + 1..] {
        if let PatternItem::Literal(s) = it {
            suffix.push_str(s);
        }
    }
    (prefix, suffix)
}

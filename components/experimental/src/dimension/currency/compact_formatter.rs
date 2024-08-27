// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// A formatter for monetary values.
///
/// [`CompactCurrencyFormatter`] supports:
///   1. Rendering in the locale's currency system.
///   2. Locale-sensitive grouping separator positions.
///
/// Read more about the options in the [`super::options`] module.
pub struct CompactCurrencyFormatter {
    /// Extended data for the currency formatter.
    extended: DataPayload<CurrencyExtendedDataV1Marker>,

    /// Formatting patterns for each currency plural category.
    patterns: DataPayload<CurrencyPatternsDataV1Marker>,

    /// A [`FixedDecimalFormatter`] to format the currency value.
    fixed_decimal_formatter: FixedDecimalFormatter,

    /// A [`PluralRules`] to determine the plural category of the unit.
    plural_rules: PluralRules,
}
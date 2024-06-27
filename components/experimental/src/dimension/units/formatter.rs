// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use fixed_decimal::FixedDecimal;
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::FixedDecimalFormatter;
use icu_plurals::PluralRules;
use icu_provider::DataPayload;

use super::super::provider::units::UnitsDisplayNameV1Marker;
use super::format::FormattedUnit;
use super::options::UnitsFormatterOptions;
use icu_provider::prelude::*;

extern crate alloc;

/// A formatter for measurement unit values.
///
/// [`UnitsFormatter`] supports:
///   1. Rendering in the locale's units system.
///   2. Locale-sensitive grouping separator positions.
///
/// Read more about the options in the [`super::options`] module.
pub struct UnitsFormatter {
    /// Options bag for the units formatter to determine the behavior of the formatter.
    /// for example: width of the units.
    options: UnitsFormatterOptions,

    // /// Essential data for the units formatter.
    // essential: DataPayload<UnitsEssentialsV1Marker>,
    /// Display name for the units.
    display_name: DataPayload<UnitsDisplayNameV1Marker>,

    /// A [`FixedDecimalFormatter`] to format the unit value.
    fixed_decimal_formatter: FixedDecimalFormatter,

    /// A [`PluralRules`] to determine the plural category of the unit.
    plural_rules: PluralRules,
}

impl UnitsFormatter {
    icu_provider::gen_any_buffer_data_constructors!(
        (locale, unit: &str, options: super::options::UnitsFormatterOptions) -> error: DataError,
        functions: [
            try_new: skip,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    /// Creates a new [`UnitsFormatter`] from compiled locale data and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        locale: &DataLocale,
        unit: &str,
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError> {
        let fixed_decimal_formatter =
            FixedDecimalFormatter::try_new(locale, FixedDecimalFormatterOptions::default())?;

        let plural_rules = PluralRules::try_new_cardinal(locale)?;

        let unit_attribute = DataMarkerAttributes::try_from_str(unit)
            .map_err(|_| DataError::custom("Failed to parse unit"))?;

        let display_name = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    unit_attribute,
                    locale,
                ),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            options,
            display_name,
            fixed_decimal_formatter,
            plural_rules,
        })
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        locale: &DataLocale,
        unit: &str,
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<super::super::provider::units::UnitsDisplayNameV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>
            + DataProvider<icu_plurals::provider::CardinalV1Marker>,
    {
        let fixed_decimal_formatter = FixedDecimalFormatter::try_new_unstable(
            provider,
            locale,
            FixedDecimalFormatterOptions::default(),
        )?;

        let plural_rules = PluralRules::try_new_cardinal_unstable(provider, locale)?;

        let unit_attribute = DataMarkerAttributes::try_from_str(unit)
            .map_err(|_| DataError::custom("Failed to parse unit"))?;

        let display_name = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    unit_attribute,
                    locale,
                ),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            options,
            display_name,
            fixed_decimal_formatter,
            plural_rules,
        })
    }

    /// Formats a [`FixedDecimal`] value for the given unit.
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
        unit: &'l str,
    ) -> FormattedUnit<'l> {
        FormattedUnit {
            value,
            unit,
            options: &self.options,
            display_name: self.display_name.get(),
            fixed_decimal_formatter: &self.fixed_decimal_formatter,
            plural_rules: &self.plural_rules,
        }
    }
}

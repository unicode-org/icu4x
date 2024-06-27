// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::FixedDecimalFormatter;
use icu_provider::DataPayload;

use crate::units::measureunit::MeasureUnit;

use super::super::provider::units::UnitsDisplayNameV1Marker;
use super::format::FormattedUnit;
use super::options::UnitsFormatterOptions;
use icu_provider::prelude::*;

/// A formatter for monetary values.
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

    // TODO: Remove this allow once the `fixed_decimal_formatter` is used.
    #[allow(dead_code)]
    /// A [`FixedDecimalFormatter`] to format the unit value.
    fixed_decimal_formatter: FixedDecimalFormatter,
}

impl UnitsFormatter {
    icu_provider::gen_any_buffer_data_constructors!(
        (locale, options: super::options::UnitsFormatterOptions) -> error: DataError,
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
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError> {
        use icu_decimal::options::FixedDecimalFormatterOptions;

        let fixed_decimal_formatter =
            FixedDecimalFormatter::try_new(locale, FixedDecimalFormatterOptions::default())
                // TODO: replace this `map_err` with `?` once the `FixedDecimalFormatter::try_new` returns a `Result` with `DataError`.
                .map_err(|_| {
                    DataError::custom("Failed to create a FixedDecimalFormatter for UnitsFormatter")
                })?;
        let display_name = crate::provider::Baked
            .load(DataRequest {
                locale,
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            options,
            display_name,
            fixed_decimal_formatter,
        })
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        locale: &DataLocale,
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<super::super::provider::units::UnitsDisplayNameV1Marker>
            + DataProvider<super::super::provider::units::UnitsEssentialsV1Marker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1Marker>,
    {
        let fixed_decimal_formatter = FixedDecimalFormatter::try_new_unstable(
            provider,
            locale,
            FixedDecimalFormatterOptions::default(),
        )
        // TODO: replace this `map_err` with `?` once the `FixedDecimalFormatter::try_new` returns a `Result` with `DataError`.
        .map_err(|_| {
            DataError::custom("Failed to create a FixedDecimalFormatter for UnitsFormatter")
        })?;
        let essential = provider
            .load(DataRequest {
                locale,
                ..Default::default()
            })?
            .take_payload()?;

        Ok(Self {
            options,
            essential,
            fixed_decimal_formatter,
        })
    }

    /// Formats a [`FixedDecimal`] value for the given unit.
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
        unit: MeasureUnit,
    ) -> FormattedUnit<'l> {
        FormattedUnit {
            value,
            unit,
            options: &self.options,
            essential: self.essential.get(),
        }
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use fixed_decimal::FixedDecimal;
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::FixedDecimalFormatter;
use icu_plurals::PluralRules;
use icu_provider::DataPayload;

use super::format::FormattedUnit;
use super::options::{UnitsFormatterOptions, Width};
use crate::dimension::provider::units::UnitsDisplayNameV1Marker;
use icu_provider::prelude::*;
use smallvec::SmallVec;

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
    _options: UnitsFormatterOptions,

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

    // TODO: Remove this function once we have separate markers for different widths.
    #[inline]
    fn attribute(width: Width, unit: &str) -> SmallVec<[u8; 32]> {
        let mut buffer: SmallVec<[u8; 32]> = SmallVec::new();
        let length = match width {
            Width::Short => "short-",
            Width::Narrow => "narrow-",
            Width::Long => "long-",
        };
        buffer.extend_from_slice(length.as_bytes());
        buffer.extend_from_slice(unit.as_bytes());
        buffer
    }

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

        // TODO: Remove this allocation once we have separate markers for different widths.
        let attribute = Self::attribute(options.width, unit);
        let unit_attribute = DataMarkerAttributes::try_from_utf8(&attribute[..attribute.len()])
            .map_err(|_| DataError::custom("Failed to create a data marker"))?;

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
            _options: options,
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

        // TODO: Remove this allocation once we have separate markers for different widths.
        let attribute = Self::attribute(options.width, unit);
        let unit_attribute = DataMarkerAttributes::try_from_utf8(&attribute[..attribute.len()])
            .map_err(|_| DataError::custom("Failed to create a data marker"))?;

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
            _options: options,
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
            display_name: self.display_name.get(),
            fixed_decimal_formatter: &self.fixed_decimal_formatter,
            plural_rules: &self.plural_rules,
        }
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use crate::dimension::units::format::FormattedUnit;
use crate::dimension::units::options::Width;
use crate::measure::category::CategorizedMeasureUnit;
use crate::measure::category::MeasureUnitCategory;
use core::marker::PhantomData;
use fixed_decimal::Decimal;
use icu_decimal::options::DecimalFormatterOptions;
use icu_decimal::DecimalFormatter;
use icu_decimal::DecimalFormatterPreferences;
use icu_locale_core::preferences::{define_preferences, prefs_convert};
use icu_plurals::PluralRules;
use icu_plurals::PluralRulesPreferences;
use icu_provider::marker::DataMarkerExt;
use icu_provider::DataError;
use icu_provider::{
    DataIdentifierBorrowed, DataMarkerAttributes, DataPayload, DataProvider, DataRequest,
};
use smallvec::SmallVec;

extern crate alloc;

define_preferences!(
    /// The preferences for units formatting.
    [Copy]
    CategorizedUnitsFormatterPreferences,
    {
        /// The user's preferred numbering system.
        ///
        /// Corresponds to the `-u-nu` in Unicode Locale Identifier.
        numbering_system: super::super::preferences::NumberingSystem
    }
);
prefs_convert!(
    CategorizedUnitsFormatterPreferences,
    DecimalFormatterPreferences,
    { numbering_system }
);
prefs_convert!(CategorizedUnitsFormatterPreferences, PluralRulesPreferences);

/// A [`CategorizedFormatter`] is used to format specific units.
///
/// This is useful for type inference and for ensuring that the correct units are used.
pub struct CategorizedFormatter<C: MeasureUnitCategory> {
    _category: PhantomData<C>,
    display_name: DataPayload<C::DataMarker>,
    decimal_formatter: DecimalFormatter,
    plural_rules: PluralRules,
}

impl<C: MeasureUnitCategory> CategorizedFormatter<C>
where
    <C as MeasureUnitCategory>::DataMarker: icu_provider::DataMarker,
{
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

    icu_provider::gen_buffer_data_constructors!(
        (
            prefs: CategorizedUnitsFormatterPreferences,
            categorized_unit: CategorizedMeasureUnit<C>,
            options: super::options::UnitsFormatterOptions
        ) -> error: DataError,
        functions: [
            try_new: skip,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    /// Creates a new [`CategorizedFormatter`] from compiled locale data and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        prefs: CategorizedUnitsFormatterPreferences,
        categorized_unit: CategorizedMeasureUnit<C>,
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError>
    where
        crate::provider::Baked: DataProvider<C::DataMarker>,
    {
        let locale = C::DataMarker::make_locale(prefs.locale_preferences);
        let decimal_formatter: DecimalFormatter =
            DecimalFormatter::try_new((&prefs).into(), DecimalFormatterOptions::default())?;

        let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;

        // TODO: Remove this allocation once we have separate markers for different widths.
        let attribute = Self::attribute(options.width, categorized_unit.cldr_id());
        let unit_attribute = DataMarkerAttributes::try_from_utf8(&attribute[..attribute.len()])
            .map_err(|_| DataError::custom("Failed to create a data marker"))?;

        let display_name = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    unit_attribute,
                    &locale,
                ),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            _category: PhantomData,
            display_name,
            decimal_formatter,
            plural_rules,
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<D>(
        provider: &D,
        prefs: CategorizedUnitsFormatterPreferences,
        categorized_unit: CategorizedMeasureUnit<C>,
        options: super::options::UnitsFormatterOptions,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<C::DataMarker>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
        <C as MeasureUnitCategory>::DataMarker: icu_provider::DataMarker,
    {
        let locale = C::DataMarker::make_locale(prefs.locale_preferences);
        let decimal_formatter = DecimalFormatter::try_new_unstable(
            provider,
            (&prefs).into(),
            DecimalFormatterOptions::default(),
        )?;

        let plural_rules = PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?;

        // TODO: Remove this allocation once we have separate markers for different widths.
        let attribute = Self::attribute(options.width, categorized_unit.cldr_id());
        let unit_attribute = DataMarkerAttributes::try_from_utf8(&attribute[..attribute.len()])
            .map_err(|_| DataError::custom("Failed to create a data marker"))?;

        let display_name = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    unit_attribute,
                    &locale,
                ),
                ..Default::default()
            })?
            .payload;

        Ok(Self {
            _category: PhantomData,
            display_name,
            decimal_formatter,
            plural_rules,
        })
    }

    /// Formats a [`Decimal`] value for the given unit.
    pub fn format_fixed_decimal<'l>(&'l self, value: &'l Decimal) -> FormattedUnit<'l> {
        FormattedUnit {
            value,
            display_name: self.display_name.get(),
            decimal_formatter: &self.decimal_formatter,
            plural_rules: &self.plural_rules,
        }
    }
}

#[cfg(feature = "compiled_data")]
#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use fixed_decimal::Decimal;
    use icu_locale_core::locale;
    use writeable::assert_writeable_eq;

    use crate::dimension::units::categorized_formatter::CategorizedFormatter;
    use crate::dimension::units::options::{UnitsFormatterOptions, Width};
    use crate::measure::category::{Area, Duration};

    #[test]
    fn test_area_categorized_formatter() {
        let test_cases = vec![
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions::default(),
                "1,000 m²",
            ),
            (
                locale!("en-US"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1,000 square meters",
            ),
            (
                locale!("fr-FR"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1\u{202f}000\u{a0}mètres carrés",
            ),
            (
                locale!("ar-EG"),
                Area::square_meter(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "١٬٠٠٠ متر مربع",
            ),
        ];

        for (locale, categorized_unit, value_str, options, expected) in test_cases {
            let formatter =
                CategorizedFormatter::<Area>::try_new(locale.into(), categorized_unit, options)
                    .unwrap();
            let signed_decimal = Decimal::from_str(value_str).unwrap();
            let formatted = formatter.format_fixed_decimal(&signed_decimal);
            assert_writeable_eq!(formatted, expected);
        }
    }

    #[test]
    fn test_duration_categorized_formatter() {
        let test_cases = vec![
            (
                locale!("en-US"),
                Duration::second(),
                "1000",
                UnitsFormatterOptions::default(),
                "1,000 sec",
            ),
            (
                locale!("en-US"),
                Duration::second(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1,000 seconds",
            ),
            (
                locale!("fr-FR"),
                Duration::second(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "1\u{202f}000\u{a0}secondes",
            ),
            (
                locale!("ar-EG"),
                Duration::second(),
                "1000",
                UnitsFormatterOptions {
                    width: Width::Long,
                    ..Default::default()
                },
                "١٬٠٠٠ ثانية",
            ),
        ];

        for (locale, categorized_unit, value_str, options, expected) in test_cases {
            let formatter =
                CategorizedFormatter::<Duration>::try_new(locale.into(), categorized_unit, options)
                    .unwrap();
            let signed_decimal = Decimal::from_str(value_str).unwrap();
            let formatted = formatter.format_fixed_decimal(&signed_decimal);
            assert_writeable_eq!(formatted, expected);
        }
    }
}

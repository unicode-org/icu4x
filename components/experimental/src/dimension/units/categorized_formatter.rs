// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use core::marker::PhantomData;
use fixed_decimal::Decimal;

#[cfg(feature = "compiled_data")]
use icu_provider::DataError;

use crate::dimension::units::format::FormattedUnit;
use crate::dimension::units::formatter::UnitsFormatter;
#[cfg(feature = "compiled_data")]
use crate::dimension::units::formatter::UnitsFormatterPreferences;
#[cfg(feature = "compiled_data")]
use crate::dimension::units::options::UnitsFormatterOptions;
#[cfg(feature = "compiled_data")]
use crate::measure::category::CategorizedMeasureUnit;
use crate::measure::category::MeasureUnitCategory;

/// A [`UnitsFormatter`] that is related to a specific category.
///
/// This is useful for type inference and for ensuring that the correct units are used.
pub struct CategorizedFormatter<C: MeasureUnitCategory> {
    _category: PhantomData<C>,
    pub formatter: UnitsFormatter,
}

impl<C: MeasureUnitCategory> CategorizedFormatter<C> {
    /// Creates a new [`CategorizedFormatter`] from compiled locale data and an options bag.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new(
        prefs: UnitsFormatterPreferences,
        categorized_unit: CategorizedMeasureUnit<C>,
        options: UnitsFormatterOptions,
    ) -> Result<Self, DataError> {
        let formatter = UnitsFormatter::try_new(prefs, categorized_unit.cldr_id(), options)?;
        Ok(Self {
            _category: PhantomData,
            formatter,
        })
    }

    /// Formats a [`Decimal`] value for the given unit.
    pub fn format_fixed_decimal<'l>(&'l self, value: &'l Decimal) -> FormattedUnit<'l> {
        self.formatter.format_fixed_decimal(value)
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

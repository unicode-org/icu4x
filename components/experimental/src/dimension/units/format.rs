// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use core::str::FromStr;

use fixed_decimal::FixedDecimal;

use icu_decimal::FixedDecimalFormatter;
use icu_pattern::SinglePlaceholderPattern;
use icu_plurals::PluralRules;
use writeable::{impl_display_with_writeable, Writeable};

use crate::alloc::borrow::ToOwned;
use crate::dimension::provider::units::{Count, UnitsDisplayNameV1};

pub struct FormattedUnit<'l> {
    pub(crate) value: &'l FixedDecimal,
    pub(crate) unit: &'l str,
    // TODO: review using options and essentials.
    // pub(crate) _options: &'l UnitsFormatterOptions,
    // pub(crate) essential: &'l UnitsEssentialsV1<'l>,
    pub(crate) display_name: &'l UnitsDisplayNameV1<'l>,
    pub(crate) fixed_decimal_formatter: &'l FixedDecimalFormatter,
    pub(crate) plural_rules: &'l PluralRules,
}

impl<'l> Writeable for FormattedUnit<'l> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        let plural_category = self.plural_rules.category_for(self.value);
        let count = Count::from(plural_category);
        let mut unit_pattern = None;
        let display_name = self
            .display_name
            .patterns
            .get(&count)
            // TODO(younies): Try to find a test case for testing the following case.
            // As per Unicode TR 35:
            //      https://www.unicode.org/reports/tr35/tr35-55/tr35.html#Multiple_Inheritance
            // If the pattern is not found for the associated `Count`, fall back to the `Count::Other` pattern.
            .or_else(|| {
                if count != Count::Other {
                    self.display_name.patterns.get(&Count::Other)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| unit_pattern.insert("{0} ".to_owned() + self.unit));

        // TODO: once the patterns are implemented to be used in the data side, we do not need this.
        let pattern =
            SinglePlaceholderPattern::from_str(display_name).map_err(|_| core::fmt::Error)?;

        pattern
            .interpolate((self.fixed_decimal_formatter.format(self.value),))
            .write_to(sink)?;

        Ok(())
    }
}

impl_display_with_writeable!(FormattedUnit<'_>);

#[test]
fn test_basic() {
    use icu_locale_core::locale;
    use writeable::assert_writeable_eq;

    use crate::dimension::units::formatter::UnitsFormatter;
    use crate::dimension::units::options::{UnitsFormatterOptions, Width};

    let test_cases = [
        (
            locale!("en-US"),
            "meter",
            "1",
            UnitsFormatterOptions {
                width: Width::Long,
                ..Default::default()
            },
            "1 meter",
        ),
        (
            locale!("en-US"),
            "meter",
            "12345.67",
            UnitsFormatterOptions::default(),
            "12,345.67 m",
        ),
        (
            locale!("en-US"),
            "century",
            "12345.67",
            UnitsFormatterOptions {
                width: Width::Long,
                ..Default::default()
            },
            "12,345.67 centuries",
        ),
        (
            locale!("de-DE"),
            "meter",
            "12345.67",
            UnitsFormatterOptions::default(),
            "12.345,67 m",
        ),
        (
            locale!("ar-EG"),
            "meter",
            "12345.67",
            UnitsFormatterOptions {
                width: Width::Long,
                ..Default::default()
            },
            "١٢٬٣٤٥٫٦٧ متر",
        ),
    ];

    for (locale, unit, value, options, expected) in test_cases {
        let fmt = UnitsFormatter::try_new(&locale.into(), unit, options).unwrap();
        let value = value.parse().unwrap();
        assert_writeable_eq!(fmt.format_fixed_decimal(&value, unit), expected);
    }
}

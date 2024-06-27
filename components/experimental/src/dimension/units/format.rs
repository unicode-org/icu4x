// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use core::str::FromStr;

use fixed_decimal::FixedDecimal;

use icu_decimal::FixedDecimalFormatter;
use icu_pattern::SinglePlaceholderPattern;
use icu_plurals::PluralRules;
use writeable::Writeable;

use crate::dimension::provider::units::{Count, UnitsDisplayNameV1};
use crate::dimension::units::options::{UnitsFormatterOptions, Width};

pub struct FormattedUnit<'l> {
    pub(crate) value: &'l FixedDecimal,
    pub(crate) unit: &'l str,
    pub(crate) options: &'l UnitsFormatterOptions,
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
        let unit_pattern = "{0} ".to_owned() + self.unit;
        let display_name = match self.options.width {
            Width::Short => self.display_name.short.get(&count),
            Width::Long => self.display_name.long.get(&count),
            Width::Narrow => self.display_name.narrow.get(&count),
        }
        .unwrap_or({ unit_pattern.as_str() });

        let pattern =
            SinglePlaceholderPattern::from_str(display_name).map_err(|_| core::fmt::Error)?;

        pattern
            .interpolate((self.fixed_decimal_formatter.format(self.value),))
            .write_to(sink)?;

        Ok(())
    }
}

#[test]
fn test_formatted_unit() {
    use icu_locale_core::locale;
    use writeable::Writeable;

    use crate::dimension::units::formatter::UnitsFormatter;

    let locale = locale!("en-US").into();
    let meter = "meter";
    let fmt = UnitsFormatter::try_new(&locale, meter, Default::default()).unwrap();
    let value = "12345.67".parse().unwrap();
    let formatted_unit = fmt.format_fixed_decimal(&value, meter);
    let mut sink = String::new();
    formatted_unit.write_to(&mut sink).unwrap();
    assert_eq!(sink.as_str(), "12,345.67 m");

    let locale = locale!("de-DE").into();
    let meter = "meter";
    let fmt = UnitsFormatter::try_new(&locale, meter, Default::default()).unwrap();
    let value = "12345.67".parse().unwrap();
    let formatted_unit = fmt.format_fixed_decimal(&value, meter);
    let mut sink = String::new();
    formatted_unit.write_to(&mut sink).unwrap();
    assert_eq!(sink.as_str(), "12.345,67 m");

    let locale = locale!("ar-EG").into();
    let meter = "meter";
    let fmt = UnitsFormatter::try_new(
        &locale,
        meter,
        UnitsFormatterOptions {
            width: Width::Long,
            ..Default::default()
        },
    )
    .unwrap();
    let value = "12345.67".parse().unwrap();
    let formatted_unit = fmt.format_fixed_decimal(&value, meter);
    let mut sink = String::new();
    formatted_unit.write_to(&mut sink).unwrap();
    assert_eq!(sink.as_str(), "١٢٬٣٤٥٫٦٧ متر");
}

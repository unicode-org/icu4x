// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::CurrencyCode;
use crate::compactdecimal::CompactDecimalFormatter;
use crate::dimension::provider::currency_patterns::CurrencyPatternsData;
use crate::dimension::provider::extended_currency::CurrencyExtendedData;
use fixed_decimal::Decimal;
use icu_plurals::PluralRules;
use writeable::Writeable;

pub struct FormattedLongCompactCurrency<'l> {
    pub(crate) signed_fixed_decimal: &'l Decimal,
    // TODO: use this if the display name is not exist and make the extended data optional.
    pub(crate) _currency_code: CurrencyCode,
    pub(crate) extended: &'l CurrencyExtendedData<'l>,
    pub(crate) patterns: &'l CurrencyPatternsData<'l>,
    pub(crate) compact_decimal_formatter: &'l CompactDecimalFormatter,
    pub(crate) plural_rules: &'l PluralRules,
}

writeable::impl_display_with_writeable!(FormattedLongCompactCurrency<'_>);

impl Writeable for FormattedLongCompactCurrency<'_> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        let operands = self.signed_fixed_decimal.into();

        let display_name = self.extended.display_names.get(operands, self.plural_rules);
        let pattern = self.patterns.patterns.get(operands, self.plural_rules);

        let formatted_value = self
            .compact_decimal_formatter
            .format_fixed_decimal(self.signed_fixed_decimal);
        let interpolated = pattern.interpolate((formatted_value, display_name));
        interpolated.write_to(sink)
    }
}

#[cfg(test)]
mod tests {
    use icu_locale_core::locale;
    use tinystr::*;
    use writeable::assert_writeable_eq;

    use crate::dimension::currency::long_compact_formatter::LongCompactCurrencyFormatter;
    use crate::dimension::currency::CurrencyCode;

    #[test]
    pub fn test_en_us() {
        let currency_formatter_prefs = locale!("en-US").into();

        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = LongCompactCurrencyFormatter::try_new(currency_formatter_prefs, &currency_code)
            .unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value, currency_code);
        assert_writeable_eq!(formatted_currency, "12 thousand US dollars");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value, currency_code);
        assert_writeable_eq!(formatted_currency, "-12 thousand US dollars");
    }

    #[test]
    pub fn test_en_us_millions() {
        let currency_formatter_prefs = locale!("en-US").into();

        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = LongCompactCurrencyFormatter::try_new(currency_formatter_prefs, &currency_code)
            .unwrap();

        // Positive case
        let positive_value = "12345000.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value, currency_code);
        assert_writeable_eq!(formatted_currency, "12 million US dollars");

        // Negative case
        let negative_value = "-12345000.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value, currency_code);
        assert_writeable_eq!(formatted_currency, "-12 million US dollars");
    }

    #[test]
    pub fn test_fr_fr() {
        let currency_formatter_prefs = locale!("fr-FR").into();

        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = LongCompactCurrencyFormatter::try_new(currency_formatter_prefs, &currency_code)
            .unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value, currency_code);
        assert_writeable_eq!(formatted_currency, "12 mille dollars des États-Unis");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value, currency_code);
        assert_writeable_eq!(formatted_currency, "-12 mille dollars des États-Unis");
    }

    #[test]
    pub fn test_fr_fr_millions() {
        let currency_formatter_prefs = locale!("fr-FR").into();

        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = LongCompactCurrencyFormatter::try_new(currency_formatter_prefs, &currency_code)
            .unwrap();

        // Positive case
        let positive_value = "12345000.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value, currency_code);
        assert_writeable_eq!(formatted_currency, "12 millions dollars des États-Unis");

        // Negative case
        let negative_value = "-12345000.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value, currency_code);
        assert_writeable_eq!(formatted_currency, "-12 millions dollars des États-Unis");
    }
}

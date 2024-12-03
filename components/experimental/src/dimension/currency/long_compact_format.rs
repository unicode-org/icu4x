// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::CurrencyCode;
use fixed_decimal::{CompactDecimal, SignedFixedDecimal};

use icu_decimal::FixedDecimalFormatter;
use icu_plurals::PluralRules;
use writeable::Writeable;

use core::str::FromStr;

use crate::compactdecimal::CompactDecimalFormatter;
use crate::dimension::provider::currency_patterns::CurrencyPatternsDataV1;
use crate::dimension::provider::extended_currency::CurrencyExtendedDataV1;

pub struct LongCompactFormattedCurrency<'l> {
    pub(crate) signed_fixed_decimal: &'l SignedFixedDecimal,
    // TODO: check if this needs to be here or on the fly.
    // pub(crate) compact_value: &'l CompactDecimal,
    // TODO: use this if the display name is not exist and make the extended data optional.
    pub(crate) _currency_code: CurrencyCode,
    pub(crate) extended: &'l CurrencyExtendedDataV1<'l>,
    pub(crate) patterns: &'l CurrencyPatternsDataV1<'l>,
    pub(crate) compact_decimal_formatter: &'l CompactDecimalFormatter,
    pub(crate) plural_rules: &'l PluralRules,
}

writeable::impl_display_with_writeable!(LongCompactFormattedCurrency<'_>);

impl Writeable for LongCompactFormattedCurrency<'_> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        // TODO: this is the correct one.
        let compact_value = CompactDecimal::from_str(self.signed_fixed_decimal.to_string().as_str()).unwrap();
        let compact_value = CompactDecimal::from_str("+1.20c6").unwrap();
        let compact_operands = (&compact_value).into();
        
        // let compact_operands = self.compact_value.into();
        let display_name = self
            .extended
            .display_names
            .get(compact_operands, self.plural_rules);
        let pattern = self
            .patterns
            .patterns
            .get(compact_operands, self.plural_rules);

        let formatted_value = self
            .compact_decimal_formatter
            .format_compact_decimal(&compact_value).unwrap(); // TODO: handle error


        let interpolated = pattern.interpolate((formatted_value, display_name));
        interpolated.write_to(sink)
    }
}

#[cfg(test)]
mod tests {
    use icu_locale_core::locale;
    use tinystr::*;
    use writeable::assert_writeable_eq;

    use crate::dimension::currency::{
        long_compact_format::LongCompactFormattedCurrency,
        long_compact_formatter::LongCompactCurrencyFormatter, CurrencyCode,
    };

    #[test]
    pub fn test_en_us() {
        let currency_formatter_prefs = locale!("en-US").into();
        let compact_decimal_formatter_prefs = locale!("en-US").into();

        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = LongCompactCurrencyFormatter::try_new(
            currency_formatter_prefs,
            compact_decimal_formatter_prefs,
            &currency_code,
        )
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
}

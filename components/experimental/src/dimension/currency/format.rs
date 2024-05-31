// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;

use writeable::Writeable;

use crate::dimension::currency::formatter::CurrencyCode;
use crate::dimension::currency::options::CurrencyFormatterOptions;
use crate::dimension::currency::options::Width;
use crate::dimension::provider::currency;
use crate::dimension::provider::currency::CurrencyEssentialsV1;

pub struct FormattedCurrency<'l> {
    pub(crate) value: &'l FixedDecimal,
    pub(crate) currency_code: CurrencyCode,
    pub(crate) options: &'l CurrencyFormatterOptions,
    pub(crate) essential: &'l CurrencyEssentialsV1<'l>,
}

impl<'l> Writeable for FormattedCurrency<'l> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        let config = self
            .essential
            .pattern_config_map
            .get_copied(&self.currency_code.0.to_unvalidated())
            .unwrap_or(self.essential.default_pattern_config);

        let placeholder_index = match self.options.width {
            Width::Short => config.short_placeholder_value,
            Width::Narrow => config.narrow_placeholder_value,
        };
        let currency_sign_value = match placeholder_index {
            Some(currency::PlaceholderValue::Index(index)) => self
                .essential
                .placeholders
                .get(index.into())
                .ok_or(core::fmt::Error)?,
            Some(currency::PlaceholderValue::ISO) | None => self.currency_code.0.as_str(),
        };

        let pattern_selection = match self.options.width {
            Width::Short => config.short_pattern_selection,
            Width::Narrow => config.narrow_pattern_selection,
        };
        let pattern = match pattern_selection {
            currency::PatternSelection::Standard => self.essential.standard_pattern.as_ref(),
            currency::PatternSelection::StandardAlphaNextToNumber => self
                .essential
                .standard_alpha_next_to_number_pattern
                .as_ref(),
        }
        .ok_or(core::fmt::Error)?;

        pattern
            .interpolate((self.value, currency_sign_value))
            .write_to(sink)?;

        Ok(())
    }
}

// TODO: add more tests for this module to cover more locales & currencies.
#[cfg(test)]
mod tests {
    use icu_locale_core::locale;
    use tinystr::*;
    use writeable::Writeable;

    use crate::dimension::currency::formatter::{CurrencyCode, CurrencyFormatter};

    #[test]
    pub fn test_en_us() {
        let locale = locale!("en-US");
        let fmt = CurrencyFormatter::try_new(&locale, Default::default()).unwrap();
        let value = "12345.67".parse().unwrap();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let formatted_currency = fmt.format_fixed_decimal(&value, currency_code);
        let mut sink = String::new();
        formatted_currency.write_to(&mut sink).unwrap();
        assert_eq!(sink.as_str(), "$12345.67");
    }

    #[test]
    pub fn test_fr_fr() {
        let locale = locale!("fr-FR");
        let fmt = CurrencyFormatter::try_new(&locale, Default::default()).unwrap();
        let value = "12345.67".parse().unwrap();
        let currency_code = CurrencyCode(tinystr!(3, "EUR"));
        let formatted_currency = fmt.format_fixed_decimal(&value, currency_code);
        let mut sink = String::new();
        formatted_currency.write_to(&mut sink).unwrap();
        assert_eq!(sink.as_str(), "12345.67\u{a0}€");
    }

    #[test]
    pub fn test_ar_eg() {
        let locale = locale!("ar-EG");
        let fmt = CurrencyFormatter::try_new(&locale, Default::default()).unwrap();
        let value = "12345.67".parse().unwrap();
        let currency_code = CurrencyCode(tinystr!(3, "EGP"));
        let formatted_currency = fmt.format_fixed_decimal(&value, currency_code);
        let mut sink = String::new();
        formatted_currency.write_to(&mut sink).unwrap();
        assert_eq!(sink.as_str(), "\u{200f}12345.67\u{a0}ج.م.\u{200f}");
    }
}

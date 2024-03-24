// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;

use writeable::Writeable;

use super::options::Width;
use crate::dimension::{
    options::CurrencyFormatterOptions, provider::currency::CurrencyEssentialsV1,
};

use super::{
    provider::currency::{self},
    CurrencyCode,
};

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

        // TODO: rewrite this so it does not allocate
        sink.write_str(
            pattern
                .interpolate([
                    &self.value.write_to_string(), // placeholder 0 (currency value)
                    currency_sign_value,           // placeholder 1 (currency sign value)
                ])
                .to_string()
                .as_str(),
        )?;

        Ok(())
    }
}

// TODO: add more tests for this module to cover more locales.
#[cfg(test)]
mod tests {
    use icu_locid::locale;
    use tinystr::*;
    use writeable::Writeable;

    use crate::dimension::{CurrencyCode, CurrencyFormatter};

    #[test]
    pub fn test_en_us() {
        let locale = locale!("en-US").into();
        let fmt = CurrencyFormatter::try_new(&locale, Default::default()).unwrap();
        let value = "12345.67".parse().unwrap();
        let currency_code = CurrencyCode {
            0: tinystr!(3, "USD"),
        };
        let formatted_currency = fmt.format_fixed_decimal(&value, currency_code);
        let mut sink = String::new();
        formatted_currency.write_to(&mut sink).unwrap();
        assert_eq!(sink.as_str(), "$12345.67");
    }
}

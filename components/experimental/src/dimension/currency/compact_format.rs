// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    compact_options::{CompactCurrencyFormatterOptions, Width},
    CurrencyCode,
};
use crate::{
    compactdecimal::CompactDecimalFormatter,
    dimension::provider::{
        currency::{self, CurrencyEssentialsV1},
        currency_compact::ShortCurrencyCompactV1,
    },
};
use fixed_decimal::SignedFixedDecimal;
use writeable::Writeable;

use crate::alloc::borrow::ToOwned;

pub struct CompactFormattedCurrency<'l> {
    pub(crate) value: &'l SignedFixedDecimal,
    pub(crate) currency_code: CurrencyCode,
    pub(crate) options: &'l CompactCurrencyFormatterOptions,
    pub(crate) essential: &'l CurrencyEssentialsV1<'l>,
    pub(crate) _short_currency_compact: &'l ShortCurrencyCompactV1<'l>,
    pub(crate) compact_decimal_formatter: &'l CompactDecimalFormatter,
}

writeable::impl_display_with_writeable!(CompactFormattedCurrency<'_>);

impl<'l> Writeable for CompactFormattedCurrency<'l> {
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

        let currency_placeholder = match placeholder_index {
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

        // TODO: add this step in the next PR
        // let plural_category = self.plural_rules.category_for(self.value);
        // let pattern_selection = match pattern_selection {
        //     currency::PatternSelection::Standard => CompactCount::Standard(plural_category),
        //     currency::PatternSelection::StandardAlphaNextToNumber => {
        //         CompactCount::AlphaNextToNumber(plural_category)
        //     }
        // };

        // // TODO: get the i8 for the pattern to get the appropriate pattern from the map.

        // let pattern = match pattern_selection {
        //     CompactCount::Standard(_) => self
        //         .short_currency_compact
        //         .compact_patterns
        //         .get(&(0, CompactCount::Standard(plural_category))),
        //     CompactCount::AlphaNextToNumber(_) => self
        //         .short_currency_compact
        //         .compact_patterns
        //         .get(&(0, CompactCount::AlphaNextToNumber(plural_category))),
        // }
        // .ok_or(core::fmt::Error)?;

        pattern
            .interpolate((
                self.compact_decimal_formatter
                    .format_fixed_decimal(self.value.to_owned()),
                currency_placeholder,
            ))
            .write_to(sink)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use icu_locale_core::locale;
    use tinystr::*;
    use writeable::assert_writeable_eq;

    use crate::dimension::currency::{compact_formatter::CompactCurrencyFormatter, CurrencyCode};

    #[test]
    pub fn test_en_us() {
        let locale = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = CompactCurrencyFormatter::try_new(locale, Default::default()).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value, currency_code);
        assert_writeable_eq!(formatted_currency, "$12K");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value, currency_code);
        assert_writeable_eq!(formatted_currency, "$-12K");
    }

    #[test]
    pub fn test_fr_fr() {
        let locale = locale!("fr-FR").into();
        let currency_code = CurrencyCode(tinystr!(3, "EUR"));
        let fmt = CompactCurrencyFormatter::try_new(locale, Default::default()).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value, currency_code);
        assert_writeable_eq!(formatted_currency, "12\u{a0}k\u{a0}€");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value, currency_code);
        assert_writeable_eq!(formatted_currency, "-12\u{a0}k\u{a0}€");
    }
}

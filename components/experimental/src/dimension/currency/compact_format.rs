// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{compact_options::CompactCurrencyFormatterOptions, CurrencyCode};
use crate::dimension::provider::{
    currency::CurrencyEssentialsV1, currency_compact::ShortCurrencyCompactV1,
};
use fixed_decimal::FixedDecimal;
use icu_decimal::FixedDecimalFormatter;
use icu_plurals::PluralRules;

pub struct CompactFormattedCurrency<'l> {
    pub(crate) value: &'l FixedDecimal,
    pub(crate) currency_code: CurrencyCode,
    pub(crate) options: &'l CompactCurrencyFormatterOptions,
    pub(crate) essential: &'l CurrencyEssentialsV1<'l>,
    pub(crate) short_currency_compact: &'l ShortCurrencyCompactV1<'l>,
    pub(crate) fixed_decimal_formatter: &'l FixedDecimalFormatter,
    pub(crate) plural_rules: &'l PluralRules,
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

        let plural_category = self.plural_rules.category_for(self.value);
        let pattern_selection = match pattern_selection {
            currency::PatternSelection::Standard => CompactCount::Standard(plural_category),
            currency::PatternSelection::StandardAlphaNextToNumber => {
                CompactCount::StandardAlphaNextToNumber(plural_category)
            }
        };

        // TODO: get the i8 for the pattern to get the appropriate pattern from the map.

        let pattern = match pattern_selection {
            currency::PatternSelection::Standard => self
                .short_currency_compact
                .compact_patterns
                .get(&(0, CompactCount::Standard(PluralCategory::One))),
            currency::PatternSelection::StandardAlphaNextToNumber => self
                .essential
                .standard_alpha_next_to_number_pattern
                .as_ref(),
        }
        .ok_or(core::fmt::Error)?;

        pattern
            .interpolate((
                self.fixed_decimal_formatter.format(self.value),
                currency_placeholder,
            ))
            .write_to(sink)?;

        Ok(())
    }
}

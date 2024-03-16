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
            .currency_config_map
            .get_copied(&self.currency_code.0.to_unvalidated())
            .unwrap_or(self.essential.default_config);

        let placeholder_index = match self.options.width {
            Width::Short => config.short_place_holder_index,
            Width::Narrow => config.narrow_place_holder_index,
        };
        let currency_sign_value = match placeholder_index {
            Some(currency::PlaceholderValue::Index(index)) => self
                .essential
                .placeholder_values
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
                .interpolate_to_string([
                    &self.value.write_to_string(), // placeholder 0 (currency value)
                    currency_sign_value,           // placeholder 1 (currency sign value)
                ])
                .as_str(),
        )?;

        Ok(())
    }
}

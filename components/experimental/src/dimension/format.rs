// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;
use icu_pattern::{Pattern, PlaceholderValueProvider};
use tinystr::tinystr;
use writeable::Writeable;
use zerovec::{maps::ZeroVecLike, ule::AsULE};

use crate::dimension::{
    options::CurrencyFormatterOptions, provider::currency::CurrencyEssentialsV1,
};

use super::{
    provider::currency::{self, CurrencyPatterns, PlaceholderValue},
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
        let pattern = CurrencyPatterns::from_unaligned(
            *self
                .essential
                .currency_patterns_map
                .get(&self.currency_code.0.to_unvalidated())
                .unwrap_or(&self.essential.default_pattern.to_unaligned()),
        );

        let placeholder = match self.options.width {
            super::options::Width::Short => match pattern.short_place_holder_index {
                Some(currency::PlaceholderValue::Index(index)) => self
                    .essential
                    .place_holders
                    .get(index.into())
                    .ok_or(core::fmt::Error)?,
                Some(currency::PlaceholderValue::ISO) => self.currency_code.0.to_string().as_str(),
                None => self.currency_code.0.to_string().as_str(),
            },
            super::options::Width::Narrow => match pattern.narrow_place_holder_index {
                Some(currency::PlaceholderValue::Index(index)) => self
                    .essential
                    .place_holders
                    .get(index.into())
                    .ok_or(core::fmt::Error)?,
                Some(currency::PlaceholderValue::ISO) => self.currency_code.0.to_string().as_str(),
                None => self.currency_code.0.to_string().as_str(),
            },
        };

        let pattern = match self.options.width {
            super::options::Width::Short => match pattern.short_pattern_standard {
                currency::PatternSelection::Standard => {
                    self.essential.standard_pattern.ok_or(core::fmt::Error)?
                }
                currency::PatternSelection::StandardAlphaNextToNumber => self
                    .essential
                    .standard_alpha_next_to_number_pattern
                    .ok_or(core::fmt::Error)?,
            },
            super::options::Width::Narrow => match pattern.narrow_pattern_standard {
                currency::PatternSelection::Standard => {
                    self.essential.standard_pattern.ok_or(core::fmt::Error)?
                }
                currency::PatternSelection::StandardAlphaNextToNumber => self
                    .essential
                    .standard_alpha_next_to_number_pattern
                    .ok_or(core::fmt::Error)?,
            },
        };

        // complete

        Ok(())
    }
}

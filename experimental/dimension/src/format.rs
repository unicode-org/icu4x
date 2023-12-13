// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;

use crate::{options::CurrencyFormatterOptions, provider::CurrencyEssentialsV1};

pub struct FormattedCurrency<'l> {
    pub(crate) value: &'l FixedDecimal,
    pub(crate) options: &'l CurrencyFormatterOptions,
    pub(crate) symbols: &'l CurrencyEssentialsV1<'l>,
}

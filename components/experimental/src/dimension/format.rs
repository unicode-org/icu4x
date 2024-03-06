// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;
use writeable::Writeable;

use crate::dimension::{options::CurrencyFormatterOptions, provider::CurrencyEssentialsV1};

use super::CurrencyCode;

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
        todo!("Implement FormattedCurrency::write_to")
    }
}

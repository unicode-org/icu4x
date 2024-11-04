// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::CurrencyCode;
use fixed_decimal::{CompactDecimal, FixedDecimal};

use icu_decimal::FixedDecimalFormatter;
use icu_plurals::PluralRules;
use writeable::Writeable;

use crate::compactdecimal::CompactDecimalFormatter;
use crate::dimension::provider::currency_patterns::CurrencyPatternsDataV1;
use crate::dimension::provider::extended_currency::CurrencyExtendedDataV1;

pub struct LongCompactFormattedCurrency<'l> {
    pub(crate) decimal_value: &'l FixedDecimal,
    pub(crate) compact_value: &'l CompactDecimal,
    // TODO: use this if the display name is not exist and make the extended data optional.
    pub(crate) _currency_code: CurrencyCode,
    pub(crate) extended: &'l CurrencyExtendedDataV1<'l>,
    pub(crate) patterns: &'l CurrencyPatternsDataV1<'l>,
    pub(crate) fixed_decimal_formatter: &'l FixedDecimalFormatter,
    pub(crate) compact_decimal_formatter: &'l CompactDecimalFormatter,
    pub(crate) plural_rules: &'l PluralRules,
}

writeable::impl_display_with_writeable!(LongCompactFormattedCurrency<'_>);

impl Writeable for LongCompactFormattedCurrency<'_> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        let decimal_operands = self.decimal_value.into();
        // let compact_operands = self.compact_value.into();
        let display_name = self.extended.display_names.get(decimal_operands, self.plural_rules);
        let pattern = self.patterns.patterns.get(decimal_operands, self.plural_rules);
        let formatted_value = self.fixed_decimal_formatter.format(self.decimal_value);
        let interpolated = pattern.interpolate((formatted_value, display_name));
        interpolated.write_to(sink)
    }
}

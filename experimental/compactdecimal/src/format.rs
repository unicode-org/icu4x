// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::CompactDecimal;
use icu_decimal::{FormattedFixedDecimal};
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::provider::DecimalSymbolsV1;
use writeable::Writeable;
use zerovec::maps::ZeroMap2d;

use crate::compactdecimal::CompactDecimalFormatter;
use crate::provider::{CompactDecimalPatternDataV1, Count, PatternULE};
use crate::CompactDecimalError;


/// An intermediate structure returned by [`CompactDecimalFormatter`](crate::CompactDecimalFormatter).
/// Use [`Writeable`][Writeable] to render the formatted decimal to a string or buffer.
#[derive(Debug)]
pub struct FormattedCompactDecimal<'l> {
    pub(crate) formatter: &'l CompactDecimalFormatter,
    pub(crate) value: &'l CompactDecimal,
    pub(crate) options: &'l FixedDecimalFormatterOptions,
    pub(crate) symbols: &'l DecimalSymbolsV1<'l>,
    pub(crate) compact_data: &'l CompactDecimalPatternDataV1<'l>,
}

impl<'l> FormattedCompactDecimal<'l> {
    fn log10_type(&self) -> i16 {
        let significand = self.value.significand();
        let exponent = self.value.exponent();
        significand.nonzero_magnitude_start() + exponent
    }
}

impl<'l> Writeable for FormattedCompactDecimal<'l> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        let plural_map =
            self.compact_data
                .patterns
                .iter0()
                .filter(|cursor| i16::from(*cursor.key0()) <= self.log10_type())
                .last();        
        let expected_exponent = plural_map.and_then(|map| map.get1(&Count::Other).and_then(|pattern| Some(i16::from(pattern.exponent)))).unwrap_or(0);
        if self.value.exponent() != expected_exponent {
            return Err(core::fmt::Error);
        }

        if self.value.exponent() == 0 {
            self.formatter.fixed_decimal_format.format(&self.value.significand()).write_to(sink)
        } else {
            Ok(())
        }
    }
}

writeable::impl_display_with_writeable!(FormattedCompactDecimal<'_>);

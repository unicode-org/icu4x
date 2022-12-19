// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Lower-level types for decimal formatting.

use crate::grouper;
use crate::options::*;
use crate::provider::*;
use fixed_decimal::FixedDecimal;
use fixed_decimal::Sign;
use writeable::Writeable;

/// An intermediate structure returned by [`FixedDecimalFormatter`](crate::FixedDecimalFormatter).
/// Use [`Writeable`][Writeable] to render the formatted decimal to a string or buffer.
#[derive(Debug, PartialEq, Clone)]
pub struct FormattedFixedDecimal<'l> {
    pub(crate) value: &'l FixedDecimal,
    pub(crate) options: &'l FixedDecimalFormatterOptions,
    pub(crate) symbols: &'l DecimalSymbolsV1<'l>,
}

impl<'l> FormattedFixedDecimal<'l> {
    fn get_affixes(&self) -> Option<&AffixesV1> {
        match self.value.sign() {
            Sign::None => None,
            Sign::Negative => Some(&self.symbols.minus_sign_affixes),
            Sign::Positive => Some(&self.symbols.plus_sign_affixes),
        }
    }
}

impl<'l> Writeable for FormattedFixedDecimal<'l> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        let affixes = self.get_affixes();
        if let Some(affixes) = affixes {
            sink.write_str(&affixes.prefix)?;
        }
        let range = self.value.magnitude_range();
        let upper_magnitude = *range.end();
        for m in range.rev() {
            if m == -1 {
                sink.write_str(&self.symbols.decimal_separator)?;
            }
            #[allow(clippy::indexing_slicing)] // digit_at in 0..=9
            sink.write_char(self.symbols.digits[self.value.digit_at(m) as usize])?;
            if grouper::check(
                upper_magnitude,
                m,
                self.options.grouping_strategy,
                &self.symbols.grouping_sizes,
            ) {
                sink.write_str(&self.symbols.grouping_separator)?;
            }
        }
        if let Some(affixes) = affixes {
            sink.write_str(&affixes.suffix)?;
        }
        Ok(())
    }
}

writeable::impl_display_with_writeable!(FormattedFixedDecimal<'_>);

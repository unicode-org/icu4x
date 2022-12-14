use fixed_decimal::CompactDecimal;
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::provider::DecimalSymbolsV1;
use writeable::Writeable;

use crate::provider::{CompactDecimalPatternDataV1, Count};
use crate::CompactDecimalError;


/// An intermediate structure returned by [`CompactDecimalFormatter`](crate::CompactDecimalFormatter).
/// Use [`Writeable`][Writeable] to render the formatted decimal to a string or buffer.
#[derive(Debug, PartialEq, Clone)]
pub struct FormattedCompactDecimal<'l> {
    pub(crate) value: &'l CompactDecimal,
    pub(crate) options: &'l FixedDecimalFormatterOptions,
    pub(crate) symbols: &'l DecimalSymbolsV1<'l>,
    pub(crate) compact_data: &'l CompactDecimalPatternDataV1<'l>,
}

impl<'l> FormattedCompactDecimal<'l> {
    fn check_exponent(&self) -> Result<(), CompactDecimalError> {
        let significand = self.value.significand();
        let exponent = self.value.exponent();
        let log10_type = significand.nonzero_magnitude_start() + exponent;
        let expected_exponent = self
            .compact_data
            .patterns
            .iter0()
            .filter(|cursor| i16::from(*cursor.key0()) <= log10_type)
            .last()
            .and_then(|cursor| cursor.get1(&Count::Other))
            .and_then(|pattern| Some(i16::from(pattern.exponent)))
            .unwrap_or(0);
        (expected_exponent == exponent)
            .then(|| ())
            .ok_or_else(|| CompactDecimalError::Exponent {
                actual: exponent,
                expected: expected_exponent,
                log10_type,
            })
    }
}

impl<'l> Writeable for FormattedCompactDecimal<'l> {
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

writeable::impl_display_with_writeable!(FormattedCompactDecimal<'_>);

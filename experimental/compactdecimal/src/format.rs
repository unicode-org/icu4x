// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use fixed_decimal::{CompactDecimal, FixedDecimal};
use writeable::Writeable;
use zerovec::maps::ZeroMap2dCursor;

use crate::compactdecimal::CompactDecimalFormatter;
use crate::provider::{Count, PatternULE};

/// An intermediate structure returned by [`CompactDecimalFormatter`](crate::CompactDecimalFormatter).
/// Use [`Writeable`][Writeable] to render the formatted decimal to a string or buffer.
#[derive(Debug)]
pub struct FormattedCompactDecimal<'l> {
    pub(crate) formatter: &'l CompactDecimalFormatter,
    pub(crate) value: Cow<'l, CompactDecimal>,
    pub(crate) plural_map: Option<ZeroMap2dCursor<'l, 'l, i8, Count, PatternULE>>,
}

impl<'l> Writeable for FormattedCompactDecimal<'l> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        if self.value.exponent() == 0 {
            self.formatter
                .fixed_decimal_format
                .format(self.value.significand())
                .write_to(sink)
        } else {
            let plural_map = self.plural_map.as_ref().ok_or(core::fmt::Error)?;
            let chosen_pattern = (|| {
                if self.value.significand() == &FixedDecimal::from(1) {
                    if let Some(pattern) = plural_map.get1(&Count::Explicit1) {
                        return Some(pattern);
                    }
                }
                let plural_category = self
                    .formatter
                    .plural_rules
                    .category_for(self.value.significand());
                plural_map
                    .get1(&plural_category.into())
                    .or_else(|| plural_map.get1(&Count::Other))
            })()
            .ok_or(core::fmt::Error)?;
            match chosen_pattern.index {
                u8::MAX => sink.write_str(&chosen_pattern.literal_text),
                _ => {
                    let i = usize::from(chosen_pattern.index);
                    sink.write_str(
                        chosen_pattern
                            .literal_text
                            .get(..i)
                            .ok_or(core::fmt::Error)?,
                    )?;
                    self.formatter
                        .fixed_decimal_format
                        .format(self.value.significand())
                        .write_to(sink)?;
                    sink.write_str(
                        chosen_pattern
                            .literal_text
                            .get(i..)
                            .ok_or(core::fmt::Error)?,
                    )
                }
            }
        }
    }
}

writeable::impl_display_with_writeable!(FormattedCompactDecimal<'_>);

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use fixed_decimal::{CompactDecimal, FixedDecimal};
use icu_pattern::SinglePlaceholderPattern;
use icu_plurals::PluralCategory;
use writeable::Writeable;
use zerovec::maps::ZeroMap2dCursor;

use crate::compactdecimal::compactdecimal::CompactDecimalFormatter;
use crate::compactdecimal::provider::PatternULE;

/// An intermediate structure returned by [`CompactDecimalFormatter`](super::CompactDecimalFormatter).
/// Use [`Writeable`][Writeable] to render the formatted decimal to a string or buffer.
#[derive(Debug)]
pub struct FormattedCompactDecimal<'l> {
    pub(crate) formatter: &'l CompactDecimalFormatter,
    pub(crate) value: Cow<'l, CompactDecimal>,
    pub(crate) plural_map: Option<ZeroMap2dCursor<'l, 'l, i8, PluralCategory, PatternULE>>,
}

impl FormattedCompactDecimal<'_> {
    /// Access the resolved [`CompactDecimal`] after formatting.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::FixedDecimal;
    /// use icu::experimental::compactdecimal::CompactDecimalFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let short_english = CompactDecimalFormatter::try_new_short(
    ///     &locale!("en").into(),
    ///     Default::default(),
    /// )
    /// .unwrap();
    ///
    /// let formatted_compact_decimal = short_english.format_i64(2207);
    ///
    /// assert_writeable_eq!(formatted_compact_decimal, "2.2K");
    /// assert_eq!(
    ///     formatted_compact_decimal.get_compact_decimal().to_string(),
    ///     "2.2c3"
    /// );
    /// ```
    pub fn get_compact_decimal(&self) -> &CompactDecimal {
        &self.value
    }
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
                    if let Some(pattern) = plural_map.get1(&PluralCategory::Explicit1) {
                        return Some(pattern);
                    }
                }
                let plural_category = self
                    .formatter
                    .plural_rules
                    .category_for(self.value.significand());
                plural_map
                    .get1(&plural_category)
                    .or_else(|| plural_map.get1(&PluralCategory::Other))
            })()
            .ok_or(core::fmt::Error)?;
            SinglePlaceholderPattern::from_ref_store_unchecked(&chosen_pattern.pattern)
                .interpolate([self
                    .formatter
                    .fixed_decimal_format
                    .format(self.value.significand())])
                .write_to(sink)
        }
    }
}

writeable::impl_display_with_writeable!(FormattedCompactDecimal<'_>);

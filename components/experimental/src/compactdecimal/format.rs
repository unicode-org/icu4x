// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use fixed_decimal::CompactDecimal;
use icu_pattern::SinglePlaceholderPattern;
use icu_plurals::provider::PluralElementsPackedULE;
use writeable::Writeable;

use crate::compactdecimal::formatter::CompactDecimalFormatter;

/// An intermediate structure returned by [`CompactDecimalFormatter`](super::CompactDecimalFormatter).
/// Use [`Writeable`][Writeable] to render the formatted decimal to a string or buffer.
#[derive(Debug)]
pub struct FormattedCompactDecimal<'l> {
    pub(crate) formatter: &'l CompactDecimalFormatter,
    pub(crate) value: Cow<'l, CompactDecimal>,
    pub(crate) plural_map: &'l PluralElementsPackedULE<SinglePlaceholderPattern>,
}

impl FormattedCompactDecimal<'_> {
    /// Access the resolved [`CompactDecimal`] after formatting.
    ///
    /// # Examples
    ///
    /// ```
    /// use fixed_decimal::Decimal;
    /// use icu::experimental::compactdecimal::CompactDecimalFormatter;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let short_english = CompactDecimalFormatter::try_new_short(
    ///     locale!("en").into(),
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

impl Writeable for FormattedCompactDecimal<'_> {
    fn write_to<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        self.plural_map
            .get(
                self.value.significand().into(),
                &self.formatter.plural_rules,
            )
            .1
            .interpolate([self
                .formatter
                .decimal_formatter
                .format(self.value.significand())])
            .write_to(sink)
    }
}

writeable::impl_display_with_writeable!(FormattedCompactDecimal<'_>);

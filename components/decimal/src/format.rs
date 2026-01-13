// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Lower-level types for decimal formatting.

use super::Cow;
use core::fmt::Write;
use writeable::Part;

use crate::grouper;
use crate::options::*;
use crate::parts;
use crate::provider::*;
use fixed_decimal::UnsignedDecimal;
use writeable::PartsWrite;
use writeable::Writeable;

/// An intermediate structure returned by [`DecimalFormatter`](crate::DecimalFormatter).
/// Use [`Writeable`][Writeable] to render the formatted decimal to a string or buffer.
#[derive(Debug, PartialEq, Clone)]
pub struct FormattedDecimal<'l>(pub(crate) FormattedSign<'l, FormattedUnsignedDecimal<'l>>);

#[doc(hidden)]
/// Building block for formatted numbers
#[derive(Debug, PartialEq, Clone)]
pub struct FormattedUnsignedDecimal<'l> {
    pub(crate) value: Cow<'l, UnsignedDecimal>,
    pub(crate) options: &'l DecimalFormatterOptions,
    pub(crate) symbols: &'l DecimalSymbols<'l>,
    pub(crate) digits: &'l [char; 10],
}

#[doc(hidden)]
/// Building block for formatted numbers
#[derive(Debug, PartialEq, Clone)]
pub struct FormattedSign<'l, T> {
    pub(crate) value: T,
    pub(crate) sign: Option<(Part, &'l str, &'l str)>,
}

impl<T: Writeable> Writeable for FormattedSign<'_, T> {
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> core::fmt::Result {
        if let Some((part, prefix, _)) = self.sign {
            sink.with_part(part, |w| w.write_str(prefix))?;
        }
        self.value.write_to_parts(sink)?;
        if let Some((part, _, suffix)) = self.sign {
            sink.with_part(part, |w| w.write_str(suffix))?;
        }
        Ok(())
    }
}

impl Writeable for FormattedUnsignedDecimal<'_> {
    fn write_to_parts<W>(&self, w: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: writeable::PartsWrite + ?Sized,
    {
        let range = self.value.magnitude_range();
        let upper_magnitude = *range.end();
        let mut range = range.rev().peekable();
        w.with_part(parts::INTEGER, |w| {
            while let Some(m) = range.next_if(|&m| m >= 0) {
                #[expect(clippy::indexing_slicing)] // digit_at in 0..=9
                w.write_char(self.digits[self.value.digit_at(m) as usize])?;
                if grouper::check(
                    upper_magnitude,
                    m,
                    self.options.grouping_strategy.unwrap_or_default(),
                    self.symbols.grouping_sizes,
                ) {
                    w.with_part(parts::GROUP, |w| {
                        w.write_str(self.symbols.grouping_separator())
                    })?;
                }
            }
            Ok(())
        })?;
        if range.peek().is_some() {
            w.with_part(parts::DECIMAL, |w| {
                w.write_str(self.symbols.decimal_separator())
            })?;
            w.with_part(parts::FRACTION, |w| {
                for m in range.by_ref() {
                    #[expect(clippy::indexing_slicing)] // digit_at in 0..=9
                    w.write_char(self.digits[self.value.digit_at(m) as usize])?;
                }
                Ok(())
            })?;
        }
        Ok(())
    }
}

impl Writeable for FormattedDecimal<'_> {
    fn write_to_parts<W>(&self, sink: &mut W) -> core::result::Result<(), core::fmt::Error>
    where
        W: writeable::PartsWrite + ?Sized,
    {
        self.0.write_to_parts(sink)
    }
}

writeable::impl_display_with_writeable!(FormattedDecimal<'_>, #[cfg(feature = "alloc")]);
writeable::impl_display_with_writeable!(FormattedUnsignedDecimal<'_>, #[cfg(feature = "alloc")]);

/// This trait is implemented for compatibility with [`fmt!`](alloc::fmt)
/// To create a string, [`Writeable::write_to_string`] is usually more efficient
impl<T: Writeable> core::fmt::Display for FormattedSign<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        writeable::Writeable::write_to(&self, f)
    }
}
#[cfg(feature = "alloc")]
impl<T: Writeable> FormattedSign<'_, T> {
    /// Converts the given value to a `String`.
    ///
    /// Under the hood, this uses an efficient [`Writeable`] implementation.
    /// However, in order to avoid allocating a string, it is more efficient
    /// to use [`Writeable`] directly.
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> writeable::_internal::String {
        writeable::Writeable::write_to_string(self).into_owned()
    }
}

#[cfg(test)]
mod tests {
    use icu_locale_core::locale;
    use writeable::assert_writeable_eq;

    use crate::DecimalFormatter;

    #[test]
    pub fn test_es_mx() {
        let locale = locale!("es-MX").into();
        let fmt = DecimalFormatter::try_new(locale, Default::default()).unwrap();
        let fd = "12345.67".parse().unwrap();
        assert_writeable_eq!(fmt.format(&fd), "12,345.67");
    }
}

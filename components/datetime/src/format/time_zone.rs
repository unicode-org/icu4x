// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::{
    input::TimeZoneInput,
    time_zone::{FormatTimeZone, FormatTimeZoneWithFallback, TimeZoneFormatter},
    DateTimeError,
};
use writeable::Writeable;

/// [`FormattedTimeZone`] is a intermediate structure which can be retrieved as an output from [`TimeZoneFormatter`].
#[derive(Debug, Copy, Clone)]
pub struct FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    pub(crate) time_zone_format: &'l TimeZoneFormatter,
    pub(crate) time_zone: &'l T,
}

impl<'l, T> Writeable for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    /// Format time zone with fallbacks.
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let mut sink = writeable::adapters::CoreWriteAsPartsWrite(sink);

        let r = match self.write_no_fallback(&mut sink) {
            Ok(fmt_result) => Ok(fmt_result?),
            Err(_) => self
                .time_zone_format
                .fallback_unit
                .format_with_last_resort_fallback(
                    &mut sink,
                    self.time_zone,
                    &self.time_zone_format.data_payloads,
                )?,
        };

        debug_assert!(r.is_ok(), "{r:?}");
        Ok(())
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'l, T> fmt::Display for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

impl<'l, T> FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    /// Write time zone with no fallback.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::time_zone::TimeZoneFormatter;
    /// use icu::datetime::DateTimeError;
    /// use icu::locid::locale;
    /// use icu::timezone::CustomTimeZone;
    /// use tinystr::tinystr;
    ///
    /// let mut tzf =
    ///     TimeZoneFormatter::try_new(&locale!("en").into(), Default::default())
    ///         .unwrap();
    /// let mut buf = String::new();
    ///
    /// let mut time_zone = "Z".parse::<CustomTimeZone>().unwrap();
    /// time_zone.time_zone_id = Some(tinystr!(8, "gblon").into());
    ///
    /// // There are no non-fallback formats enabled:
    /// assert!(matches!(
    ///     tzf.format(&time_zone).write_no_fallback(&mut buf),
    ///     Err(DateTimeError::UnsupportedOptions)
    /// ));
    /// assert!(buf.is_empty());
    ///
    /// // Enable a non-fallback format:
    /// tzf.include_generic_location_format().unwrap();
    /// assert!(matches!(
    ///     tzf.format(&time_zone).write_no_fallback(&mut buf),
    ///     Ok(Ok(_))
    /// ));
    /// assert_eq!("London Time", buf);
    ///
    /// // Errors still occur if the time zone is not supported:
    /// buf.clear();
    /// time_zone.time_zone_id = Some(tinystr!(8, "zzzzz").into());
    /// assert!(matches!(
    ///     tzf.format(&time_zone).write_no_fallback(&mut buf),
    ///     Err(DateTimeError::UnsupportedOptions)
    /// ));
    ///
    /// // Use the `Writable` trait instead to enable infallible formatting:
    /// writeable::assert_writeable_eq!(tzf.format(&time_zone), "GMT");
    /// ```
    pub fn write_no_fallback<W>(&self, mut w: &mut W) -> Result<fmt::Result, DateTimeError>
    where
        W: core::fmt::Write + ?Sized,
    {
        for unit in self.time_zone_format.format_units.iter() {
            match unit.format(
                &mut writeable::adapters::CoreWriteAsPartsWrite(&mut w),
                self.time_zone,
                &self.time_zone_format.data_payloads,
            ) {
                Ok(r) => return Ok(r),
                Err(DateTimeError::UnsupportedOptions) => continue,
                Err(e) => return Err(e),
            }
        }
        Err(DateTimeError::UnsupportedOptions)
    }
}

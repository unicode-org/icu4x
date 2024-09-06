// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::format::datetime::DateTimeWriteError;
use crate::{
    input::{ExtractedTimeZoneInput, TimeZoneInput},
    time_zone::{
        FormatTimeZone, FormatTimeZoneError, FormatTimeZoneWithFallback, TimeZoneFormatter,
    },
};
use writeable::Writeable;

/// [`FormattedTimeZone`] is a intermediate structure which can be retrieved as an output from [`TimeZoneFormatter`].
#[derive(Debug, Copy, Clone)]
pub struct FormattedTimeZone<'l> {
    pub(crate) time_zone_format: &'l TimeZoneFormatter,
    // Note: CustomTimeZone is being used as an ExtractedTimeZoneInput
    pub(crate) time_zone: ExtractedTimeZoneInput,
}

impl<'l> Writeable for FormattedTimeZone<'l> {
    /// Format time zone with fallbacks.
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let mut sink = writeable::adapters::CoreWriteAsPartsWrite(sink);

        let r = match self.write_no_fallback(&mut sink)? {
            Ok(()) => Ok(()),
            Err(_) => self
                .time_zone_format
                .fallback_unit
                .format_with_last_resort_fallback(
                    &mut sink,
                    self.time_zone,
                    self.time_zone_format.data_payloads.as_borrowed(),
                )?,
        };

        debug_assert!(r.is_ok(), "{r:?}");
        Ok(())
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'l> fmt::Display for FormattedTimeZone<'l> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

impl<'l> FormattedTimeZone<'l> {
    /// Write time zone with no fallback.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::time_zone::TimeZoneFormatter;
    /// use icu::datetime::DateTimeWriteError;
    /// use icu::locale::locale;
    /// use icu::timezone::{CustomTimeZone, TimeZoneBcp47Id};
    /// use tinystr::tinystr;
    ///
    /// let mut tzf =
    ///     TimeZoneFormatter::try_new(&locale!("en").into(), Default::default())
    ///         .unwrap();
    /// let mut buf = String::new();
    ///
    /// let mut time_zone = "Z".parse::<CustomTimeZone>().unwrap();
    /// time_zone.time_zone_id = Some(TimeZoneBcp47Id(tinystr!(8, "gblon")));
    ///
    /// // There are no non-fallback formats enabled:
    /// assert!(matches!(
    ///     tzf.format(&time_zone).write_no_fallback(&mut buf),
    ///     Ok(Err(DateTimeWriteError::MissingTimeZoneSymbol(..)))
    /// ));
    /// assert!(buf.is_empty());
    ///
    /// // Enable a non-fallback format:
    /// tzf.include_generic_location_format().unwrap();
    /// assert!(matches!(
    ///     tzf.format(&time_zone).write_no_fallback(&mut buf),
    ///     Ok(Ok(()))
    /// ));
    /// assert_eq!("London Time", buf);
    ///
    /// // Errors still occur if the time zone is not supported:
    /// buf.clear();
    /// time_zone.time_zone_id = Some(TimeZoneBcp47Id(tinystr!(8, "zzzzz")));
    /// assert!(matches!(
    ///     tzf.format(&time_zone).write_no_fallback(&mut buf),
    ///     Ok(Err(DateTimeWriteError::MissingTimeZoneSymbol(..)))
    /// ));
    ///
    /// // Use the `Writable` trait instead to enable infallible formatting:
    /// writeable::assert_writeable_eq!(tzf.format(&time_zone), "GMT");
    /// ```
    pub fn write_no_fallback<W>(
        &self,
        mut w: &mut W,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        for unit in self.time_zone_format.format_units.iter() {
            match unit.format(
                &mut writeable::adapters::CoreWriteAsPartsWrite(&mut w),
                &self.time_zone,
                self.time_zone_format.data_payloads.as_borrowed(),
            )? {
                Ok(()) => return Ok(Ok(())),
                Err(FormatTimeZoneError::NameNotFound) => continue,
                Err(FormatTimeZoneError::MissingZoneSymbols) => {
                    return Ok(Err(DateTimeWriteError::MissingZoneSymbols))
                }
                Err(FormatTimeZoneError::MissingInputField(s)) => {
                    return Ok(Err(DateTimeWriteError::MissingInputField(s)))
                }
            }
        }
        Ok(Err(DateTimeWriteError::MissingTimeZoneSymbol(
            self.time_zone.time_zone_id(),
            self.time_zone.metazone_id(),
        )))
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use crate::{AsCalendar, Calendar, Date, Iso, RangeError};
use icu_locale_core::preferences::extensions::unicode::keywords::CalendarAlgorithm;
use ixdtf::encoding::Utf8;
use ixdtf::parsers::IxdtfParser;
use ixdtf::records::IxdtfParseRecord;
use ixdtf::ParseError as Rfc9557Error;

/// An error returned from parsing an RFC 9557 string to an `icu::calendar` type.
#[derive(Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum ParseError {
    /// Syntax error.
    #[displaydoc("Syntax error in the RFC 9557 string: {0}")]
    Syntax(Rfc9557Error),
    /// Value is out of range.
    #[displaydoc("Value out of range: {0}")]
    Range(RangeError),
    /// The RFC 9557 string is missing fields required for parsing into the chosen type.
    MissingFields,
    /// The RFC 9557 string specifies an unknown calendar.
    UnknownCalendar,
    /// Expected a different calendar.
    #[displaydoc("Expected calendar {0:?} but found calendar {1:?}")]
    MismatchedCalendar(CalendarAlgorithm, CalendarAlgorithm),
}

impl From<RangeError> for ParseError {
    fn from(value: RangeError) -> Self {
        Self::Range(value)
    }
}

impl From<Rfc9557Error> for ParseError {
    fn from(value: Rfc9557Error) -> Self {
        Self::Syntax(value)
    }
}

impl FromStr for Date<Iso> {
    type Err = ParseError;
    fn from_str(rfc_9557_str: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(rfc_9557_str, Iso)
    }
}

impl<A: AsCalendar> Date<A> {
    /// Creates a [`Date`] in the given calendar from an RFC 9557 string.
    ///
    /// Returns an error if the string has a calendar annotation that does not
    /// match the calendar argument, unless the argument is [`Iso`].
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::{Date, Gregorian};
    ///
    /// let date = Date::try_from_str("2024-07-17", Gregorian).unwrap();
    /// let date =
    ///     Date::try_from_str("2024-07-17[u-ca=gregory]", Gregorian).unwrap();
    /// let _ =
    ///     Date::try_from_str("2024-07-17[u-ca=hebrew]", Gregorian).unwrap_err();
    ///
    /// assert_eq!(date.era_year().year, 2024);
    /// assert_eq!(date.month().number(), 7);
    /// assert_eq!(date.day_of_month().0, 17);
    /// ```
    pub fn try_from_str(rfc_9557_str: &str, calendar: A) -> Result<Self, ParseError> {
        Self::try_from_utf8(rfc_9557_str.as_bytes(), calendar)
    }

    /// Creates a [`Date`] in the given calendar from an RFC 9557 string.
    ///
    /// Returns an error if the string has a calendar annotation that does not
    /// match the calendar argument.
    ///
    /// See [`Self::try_from_str()`].
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    pub fn try_from_utf8(rfc_9557_str: &[u8], calendar: A) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(rfc_9557_str).parse()?;
        Self::try_from_ixdtf_record(&ixdtf_record, calendar)
    }

    #[doc(hidden)]
    pub fn try_from_ixdtf_record(
        ixdtf_record: &IxdtfParseRecord<'_, Utf8>,
        calendar: A,
    ) -> Result<Self, ParseError> {
        let date_record = ixdtf_record.date.ok_or(ParseError::MissingFields)?;
        let iso = Date::try_new_iso_without_range_check(
            date_record.year,
            date_record.month,
            date_record.day,
        )?;

        if let Some(ixdtf_calendar) = ixdtf_record.calendar {
            if let Some(expected_calendar) = calendar.as_calendar().calendar_algorithm() {
                if let Some(parsed_calendar) =
                    icu_locale_core::extensions::unicode::Value::try_from_utf8(ixdtf_calendar)
                        .ok()
                        .and_then(|v| CalendarAlgorithm::try_from(&v).ok())
                {
                    if parsed_calendar != expected_calendar {
                        return Err(ParseError::MismatchedCalendar(
                            expected_calendar,
                            parsed_calendar,
                        ));
                    }
                }
            }
        }
        Ok(iso.to_calendar(calendar))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cal::{Hijri, HijriTabularEpoch, HijriTabularLeapYears};

    #[test]
    fn test_parse_out_of_iso_year_range() {
        // This test ensures that dates with large years, representable in a calendar
        // like Islamic-tabular, can roundtrip through the IXDTF string format.
        // It specifically verifies that the `Date::try_new_iso_without_range_check`
        // function, used internally for IXDTF parsing, does not perform ISO year
        // range validity checks (e.g., against VALID_YEAR_RANGE).

        // Double check that +501000-01-01 is outside of the valid year range for ISO
        assert!(Date::try_new_iso(501000, 1, 1).is_err());

        // This date is within the RD range but outside of ISO's valid year range
        // as verified above. It should parse.
        //
        // We picked Hijri here since Hijri lunar years are *much* shorter than solar years
        // and the difference is more stark.
        let valid_date_string = "+501000-01-01[u-ca=islamic-tbla]";
        let calendar =
            Hijri::new_tabular(HijriTabularLeapYears::TypeII, HijriTabularEpoch::Thursday);

        Date::try_from_str(valid_date_string, calendar)
            .expect("Should parse date string successfully");

        // This date is outside of the valid year range, and should fail to parse
        let invalid_date_string = "+501912-01-01[u-ca=islamic-tbla]";

        Date::try_from_str(invalid_date_string, calendar).expect_err("Should be out of range");
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

use crate::{AnyCalendar, AnyCalendarKind, Date, DateTime, Iso, RangeError, TaggedDate, Time};
use ixdtf::parsers::records::IxdtfParseRecord;
use ixdtf::parsers::IxdtfParser;
use ixdtf::ParseError as IxdtfError;

/// An error returned from parsing an IXDTF string to an `icu_calendar` type.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseError {
    /// Syntax error in the IXDTF string.
    Syntax(IxdtfError),
    /// Value is out of range.
    Range(RangeError),
    /// The IXDTF is missing fields required for parsing into the chosen type.
    MissingFields,
    /// The IXDTF specifies an unknown calendar.
    UnknownCalendar,
}

impl From<RangeError> for ParseError {
    fn from(value: RangeError) -> Self {
        Self::Range(value)
    }
}

impl From<IxdtfError> for ParseError {
    fn from(value: IxdtfError) -> Self {
        Self::Syntax(value)
    }
}

impl AnyCalendarKind {
    fn try_from_ixdtf_record(ixdtf_record: &IxdtfParseRecord) -> Result<Option<Self>, ParseError> {
        let Some(calendar_id) = ixdtf_record.calendar else {
            return Ok(None);
        };
        match AnyCalendarKind::get_for_bcp47_bytes(calendar_id) {
            Some(kind) => Ok(Some(kind)),
            None => Err(ParseError::UnknownCalendar),
        }
    }
}

impl Date<Iso> {
    /// Creates a [`Date`] in the ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// Ignores any calendar annotations in the string. This might not be what
    /// you want! Consider using [`TaggedDate::try_from_str`].
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    ///
    /// let date = Date::try_iso_from_str("2024-07-17").unwrap();
    ///
    /// assert_eq!(date.year().era_year_or_extended(), 2024);
    /// assert_eq!(
    ///     date.month().standard_code,
    ///     icu::calendar::types::MonthCode(tinystr::tinystr!(4, "M07"))
    /// );
    /// assert_eq!(date.day_of_month().0, 17);
    /// ```
    pub fn try_iso_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_iso_from_utf8(ixdtf_str.as_bytes())
    }

    /// Creates a [`Date`] in the ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// See [`Self::try_iso_from_str()`].
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    pub fn try_iso_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        Self::try_from_ixdtf_record(&ixdtf_record)
    }

    fn try_from_ixdtf_record(ixdtf_record: &IxdtfParseRecord) -> Result<Self, ParseError> {
        let date_record = ixdtf_record.date.ok_or(ParseError::MissingFields)?;
        let date = Self::try_new_iso(date_record.year, date_record.month, date_record.day)?;
        Ok(date)
    }
}

impl FromStr for Date<Iso> {
    type Err = ParseError;
    fn from_str(ixdtf_str: &str) -> Result<Self, Self::Err> {
        Self::try_iso_from_str(ixdtf_str)
    }
}

impl TaggedDate {
    /// Creates a [`TaggedDate`] from an IXDTF syntax string, retaining the
    /// calendar annotation if present.
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::TaggedDate;
    ///
    /// let date = TaggedDate::try_from_str("2024-10-28[u-ca=hebrew]")
    ///     .unwrap()
    ///     .to_any_date();
    ///
    /// assert_eq!(date.year().era_year_or_extended(), 5785);
    /// assert_eq!(
    ///     date.month().standard_code,
    ///     icu::calendar::types::MonthCode(tinystr::tinystr!(4, "M01"))
    /// );
    /// assert_eq!(date.day_of_month().0, 26);
    /// ```
    pub fn try_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_from_utf8(ixdtf_str.as_bytes())
    }

    /// Creates a [`TaggedDate`] from an IXDTF syntax string, retaining the
    /// calendar annotation if present.
    ///
    /// See [`Self::try_from_str()`].
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    pub fn try_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        Self::try_from_ixdtf_record(&ixdtf_record)
    }

    fn try_from_ixdtf_record(ixdtf_record: &IxdtfParseRecord) -> Result<Self, ParseError> {
        let date_record = ixdtf_record.date.ok_or(ParseError::MissingFields)?;
        let date = Date::try_new_iso(date_record.year, date_record.month, date_record.day)?;
        let kind = AnyCalendarKind::try_from_ixdtf_record(ixdtf_record)?;
        Ok(Self::from_iso_date_and_kind(date, kind))
    }
}

impl FromStr for TaggedDate {
    type Err = ParseError;
    fn from_str(ixdtf_str: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(ixdtf_str)
    }
}

impl Time {
    /// Creates a [`Time`] from an IXDTF syntax string of a time.
    ///
    /// Does not support parsing an IXDTF string with a date and time; for that, use [`DateTime`].
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Time;
    ///
    /// let time = Time::try_from_str("16:01:17.045").unwrap();
    ///
    /// assert_eq!(time.hour.number(), 16);
    /// assert_eq!(time.minute.number(), 1);
    /// assert_eq!(time.second.number(), 17);
    /// assert_eq!(time.nanosecond.number(), 45000000);
    /// ```
    pub fn try_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_from_utf8(ixdtf_str.as_bytes())
    }

    /// Creates a [`Time`] in the ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// See [`Self::try_from_str()`].
    pub fn try_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse_time()?;
        Self::try_from_ixdtf_record(&ixdtf_record)
    }

    fn try_from_ixdtf_record(ixdtf_record: &IxdtfParseRecord) -> Result<Self, ParseError> {
        let time_record = ixdtf_record.time.ok_or(ParseError::MissingFields)?;
        let time = Self::try_new(
            time_record.hour,
            time_record.minute,
            time_record.second,
            time_record.nanosecond,
        )?;
        Ok(time)
    }
}

impl FromStr for Time {
    type Err = ParseError;
    fn from_str(ixdtf_str: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(ixdtf_str)
    }
}

impl DateTime<Iso> {
    /// Creates a [`DateTime`] in the ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// Ignores any calendar annotations in the string.
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    ///
    /// let datetime = DateTime::try_iso_from_str("2024-07-17T16:01:17.045").unwrap();
    ///
    /// assert_eq!(datetime.date.year().era_year_or_extended(), 2024);
    /// assert_eq!(
    ///     datetime.date.month().standard_code,
    ///     icu::calendar::types::MonthCode(tinystr::tinystr!(4, "M07"))
    /// );
    /// assert_eq!(datetime.date.day_of_month().0, 17);
    ///
    /// assert_eq!(datetime.time.hour.number(), 16);
    /// assert_eq!(datetime.time.minute.number(), 1);
    /// assert_eq!(datetime.time.second.number(), 17);
    /// assert_eq!(datetime.time.nanosecond.number(), 45000000);
    /// ```
    pub fn try_iso_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_iso_from_utf8(ixdtf_str.as_bytes())
    }

    /// Creates a [`DateTime`] in the ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// See [`Self::try_iso_from_str()`].
    pub fn try_iso_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        Self::try_from_ixdtf_record(&ixdtf_record)
    }

    fn try_from_ixdtf_record(ixdtf_record: &IxdtfParseRecord) -> Result<Self, ParseError> {
        let date = Date::<Iso>::try_from_ixdtf_record(ixdtf_record)?;
        let time = Time::try_from_ixdtf_record(ixdtf_record)?;
        Ok(Self::new(date, time))
    }
}

impl FromStr for DateTime<Iso> {
    type Err = ParseError;
    fn from_str(ixdtf_str: &str) -> Result<Self, Self::Err> {
        Self::try_iso_from_str(ixdtf_str)
    }
}

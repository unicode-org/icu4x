// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{AnyCalendar, Date, Iso, RangeError};
use ixdtf::parsers::records::{DateRecord, IxdtfParseRecord};
use ixdtf::parsers::IxdtfParser;
use ixdtf::ParserError;

/// An error returned from parsing an IXDTF string to an `icu_calendar` type.
#[derive(Debug)]
pub enum FromIxdtfError {
    /// Syntax error in the IXDTF string.
    Ixdtf(ParserError),
    /// Value is out of range of the `icu_calendar` type.
    Range(RangeError),
    /// The IXDTF is missing fields required for parsing into the chosen type.
    Missing,
    /// The IXDTF specifies a calendar unknown to `icu_calendar`.
    UnknownCalendar,
}

impl From<RangeError> for FromIxdtfError {
    fn from(value: RangeError) -> Self {
        Self::Range(value)
    }
}

impl From<ParserError> for FromIxdtfError {
    fn from(value: ParserError) -> Self {
        Self::Ixdtf(value)
    }
}

impl TryFrom<DateRecord> for Date<Iso> {
    type Error = RangeError;
    fn try_from(value: DateRecord) -> Result<Self, Self::Error> {
        Self::try_new_iso_date(value.year, value.month, value.day)
    }
}

impl Date<Iso> {
    /// Creates a [`Date`] in the ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// Ignores any calendar annotations in the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    ///
    /// let date = Date::try_iso_from_str("2024-07-17").unwrap();
    ///
    /// assert_eq!(date.year().number, 2024);
    /// assert_eq!(
    ///     date.month().code,
    ///     icu::calendar::types::MonthCode(tinystr::tinystr!(4, "M07"))
    /// );
    /// assert_eq!(date.day_of_month().0, 17);
    /// ```
    pub fn try_iso_from_str(ixdtf_str: &str) -> Result<Self, FromIxdtfError> {
        let ixdtf_record = IxdtfParser::from_str(ixdtf_str).parse()?;
        Self::try_from_ixdtf_record(&ixdtf_record)
    }

    fn try_from_ixdtf_record(ixdtf_record: &IxdtfParseRecord) -> Result<Self, FromIxdtfError> {
        let date_record = ixdtf_record.date.ok_or(FromIxdtfError::Missing)?;
        let date = Self::try_from(date_record)?;
        Ok(date)
    }
}

impl AnyCalendar {
    fn try_from_ixdtf_record(ixdtf_record: &IxdtfParseRecord) -> Result<Self, FromIxdtfError> {
        let calendar_id = ixdtf_record.calendar.unwrap_or(b"iso");
        let calendar_kind = crate::AnyCalendarKind::get_for_bcp47_bytes(calendar_id)
            .ok_or(FromIxdtfError::UnknownCalendar)?;
        let calendar = AnyCalendar::new(calendar_kind);
        Ok(calendar)
    }
}

impl Date<AnyCalendar> {
    /// Creates a [`Date`] in any calendar from an IXDTF syntax string using compiled data.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    ///
    /// let date = Date::try_from_str("2024-07-17[u-ca=hebrew]").unwrap();
    ///
    /// assert_eq!(date.year().number, 5784);
    /// assert_eq!(
    ///     date.month().code,
    ///     icu::calendar::types::MonthCode(tinystr::tinystr!(4, "M10"))
    /// );
    /// assert_eq!(date.day_of_month().0, 11);
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn try_from_str(ixdtf_str: &str) -> Result<Self, FromIxdtfError> {
        let ixdtf_record = IxdtfParser::from_str(ixdtf_str).parse()?;
        let iso_date = Date::<Iso>::try_from_ixdtf_record(&ixdtf_record)?;
        let calendar = AnyCalendar::try_from_ixdtf_record(&ixdtf_record)?;
        let date = iso_date.to_any().to_calendar(calendar);
        Ok(date)
    }
}

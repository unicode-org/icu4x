// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{CustomTimeZone, CustomZonedDateTime, GmtOffset, InvalidOffsetError};
use icu_calendar::{AnyCalendar, Iso, RangeError};
use ixdtf::{
    parsers::records::{TimeZoneRecord, UTCOffsetRecord},
    ParseError as IxdtfParseError,
};

#[cfg(feature = "compiled_data")]
use crate::TimeZoneIdMapper;
#[cfg(feature = "compiled_data")]
use icu_calendar::{Date, Time};
#[cfg(feature = "compiled_data")]
use ixdtf::parsers::{records::IxdtfParseRecord, IxdtfParser};

/// The error type for parsing IXDTF syntax strings in `icu_timezone`.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum ParseError {
    /// Syntax error for IXDTF string.
    Syntax(IxdtfParseError),
    /// Parsed record is out of valid date range.
    Range(RangeError),
    /// There was no time zone offset or IANA identifer found.
    MissingTimeZone,
    /// There were missing fields required to parse component.
    MissingFields,
    /// There were two offsets provided that were not consistent with each other.
    InconsistentTimeZoneOffsets,
    /// There was an invalid Offset.
    InvalidOffsetError,
    /// There was an invalid IANA identifier provided.
    InvalidIanaIdentifier,
    /// An unknown calendar was provided.
    UnknownCalendar,
}

impl From<IxdtfParseError> for ParseError {
    fn from(value: IxdtfParseError) -> Self {
        Self::Syntax(value)
    }
}

impl From<RangeError> for ParseError {
    fn from(value: RangeError) -> Self {
        Self::Range(value)
    }
}

impl From<InvalidOffsetError> for ParseError {
    fn from(_: InvalidOffsetError) -> Self {
        Self::InvalidOffsetError
    }
}

impl TryFrom<UTCOffsetRecord> for GmtOffset {
    type Error = InvalidOffsetError;

    fn try_from(value: UTCOffsetRecord) -> Result<Self, Self::Error> {
        let hour_seconds = i32::from(value.hour) * 3600;
        let minute_seconds = i32::from(value.minute) * 60;
        Self::try_from_offset_seconds(
            i32::from(value.sign as i8) * (hour_seconds + minute_seconds + i32::from(value.second)),
        )
    }
}

// ==== CustomTimeZone methods and traits ====

impl CustomTimeZone {
    /// Create a [`CustomTimeZone`] from an IXDTF syntax string.
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// # Examples
    ///
    /// An IXDTF string can provide a time zone in two parts: the DateTime UTC Offset or the Time Zone
    /// Annotation. A DateTime UTC Offset is the time offset as laid out by RFC3339; meanwhile, the Time
    /// Zone Annotation is the annotation laid out by RFC9557 and is defined as a UTC offset or IANA Time
    /// Zone identifier.
    ///
    /// ## DateTime UTC Offsets
    ///
    /// Below is an example of a time zone from a DateTime UTC Offset. The syntax here is familiar to a RFC3339
    /// DateTime string.
    ///
    /// ```
    /// use icu_timezone::{CustomTimeZone, GmtOffset};
    ///
    /// let tz_from_offset = CustomTimeZone::try_from_ixdtf_str("2024-08-08T12:08:19-05:00").unwrap();
    ///
    /// assert_eq!(tz_from_offset, CustomTimeZone {
    ///     gmt_offset: Some(GmtOffset::try_from_offset_seconds(-18000).unwrap()),
    ///     time_zone_id: None,
    ///     metazone_id: None,
    ///     zone_variant: None,
    /// });
    ///
    /// ```
    ///
    /// ## Time Zone Annotations
    ///
    /// Below is an example of a time zone being provided by a time zone annotation.
    ///
    /// ```
    /// use icu_timezone::{CustomTimeZone, GmtOffset, TimeZoneBcp47Id};
    /// use tinystr::tinystr;
    ///
    /// let tz_from_offset_annotation = CustomTimeZone::try_from_ixdtf_str("2024-08-08T12:08:19[-05:00]").unwrap();
    /// let tz_from_iana_annotation = CustomTimeZone::try_from_ixdtf_str("2024-08-08T12:08:19[America/Chicago]").unwrap();
    ///
    /// assert_eq!(tz_from_offset_annotation, CustomTimeZone {
    ///     gmt_offset: Some(GmtOffset::try_from_offset_seconds(-18000).unwrap()),
    ///     time_zone_id: None,
    ///     metazone_id: None,
    ///     zone_variant: None,
    /// });
    ///
    /// assert_eq!(tz_from_iana_annotation, CustomTimeZone {
    ///     gmt_offset: None,
    ///     time_zone_id: Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))),
    ///     metazone_id: None,
    ///     zone_variant: None,
    /// });
    ///
    /// ```
    ///
    /// ## DateTime UTC Offset and Time Zone Annotations.
    ///
    /// An IXDTF string may contain both a DateTime UTC Offset and Time Zone Annotation. This is fine as long as
    /// the time zone parts can be deemed as inconsistent or unknown consistency.
    ///
    /// ### DateTime UTC Offset with IANA identifier annotation
    ///
    /// In cases where the DateTime UTC Offset is provided and the IANA identifier, these will be returned without
    /// verifying internal consistency.
    ///
    /// ```
    /// use icu_timezone::{CustomTimeZone, GmtOffset, TimeZoneBcp47Id};
    /// use tinystr::tinystr;
    ///
    /// let consistent_tz_from_both = CustomTimeZone::try_from_ixdtf_str("2024-08-08T12:08:19-05:00[America/Chicago]").unwrap();
    ///
    /// assert_eq!(consistent_tz_from_both, CustomTimeZone {
    ///     gmt_offset: Some(GmtOffset::try_from_offset_seconds(-18000).unwrap()),
    ///     time_zone_id: Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))),
    ///     metazone_id: None,
    ///     zone_variant: None,
    /// });
    ///
    /// let inconsistent_tz_from_both = CustomTimeZone::try_from_ixdtf_str("2024-08-08T12:08:19-05:00[America/Los_Angeles]").unwrap();
    ///
    /// assert_eq!(inconsistent_tz_from_both, CustomTimeZone {
    ///     gmt_offset: Some(GmtOffset::try_from_offset_seconds(-18000).unwrap()),
    ///     time_zone_id: Some(TimeZoneBcp47Id(tinystr!(8, "uslax"))),
    ///     metazone_id: None,
    ///     zone_variant: None,
    /// });
    ///
    /// ```
    ///
    /// ### DateTime UTC offset with UTC Offset annotation.
    ///
    /// These annotations must always be consistent as they should be either the same value or are inconsistent.
    ///
    /// ```
    /// use icu_timezone::{ParseError, CustomTimeZone, GmtOffset, TimeZoneBcp47Id};
    /// use tinystr::tinystr;
    ///
    /// let consistent_tz_from_both = CustomTimeZone::try_from_ixdtf_str("2024-08-08T12:08:19-05:00[-05:00]").unwrap();
    ///
    /// assert_eq!(consistent_tz_from_both, CustomTimeZone {
    ///     gmt_offset: Some(GmtOffset::try_from_offset_seconds(-18000).unwrap()),
    ///     time_zone_id: None,
    ///     metazone_id: None,
    ///     zone_variant: None,
    /// });
    ///
    ///
    /// let inconsistent_tz_from_both = CustomTimeZone::try_from_ixdtf_str("2024-08-08T12:08:19-05:00[+05:00]");
    ///
    /// assert_eq!(inconsistent_tz_from_both, Err(ParseError::InconsistentTimeZoneOffsets));
    ///
    /// ```
    ///
    #[cfg(feature = "compiled_data")]
    pub fn try_from_ixdtf_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_from_ixdtf_utf8(ixdtf_str.as_bytes())
    }

    /// Create a [`CustomTimeZone`] from IXDTF syntax utf8 bytes.
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// See [`Self::try_from_ixdtf_str`].
    #[cfg(feature = "compiled_data")]
    pub fn try_from_ixdtf_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        Self::try_from_ixdtf_record(&ixdtf_record)
    }

    #[cfg(feature = "compiled_data")]
    fn try_from_ixdtf_record(ixdtf_record: &IxdtfParseRecord) -> Result<Self, ParseError> {
        match ixdtf_record {
            IxdtfParseRecord {
                offset: Some(offset),
                tz: None,
                ..
            } => Self::try_from(offset),
            IxdtfParseRecord {
                offset: None,
                tz: Some(time_zone_annotation),
                ..
            } => Self::try_from(&time_zone_annotation.tz),
            IxdtfParseRecord {
                offset: Some(offset),
                tz: Some(time_zone_annotation),
                ..
            } => {
                let custom_from_offset = Self::try_from(offset)?;
                let custom_from_annotation = Self::try_from(&time_zone_annotation.tz)?;

                if custom_from_annotation.gmt_offset.is_some()
                    && custom_from_annotation.gmt_offset != custom_from_offset.gmt_offset
                {
                    return Err(ParseError::InconsistentTimeZoneOffsets);
                }

                Ok(Self {
                    gmt_offset: custom_from_offset.gmt_offset,
                    time_zone_id: custom_from_annotation.time_zone_id,
                    metazone_id: None,
                    zone_variant: None,
                })
            }
            _ => Err(ParseError::MissingTimeZone),
        }
    }
}

impl TryFrom<&UTCOffsetRecord> for CustomTimeZone {
    type Error = ParseError;

    fn try_from(value: &UTCOffsetRecord) -> Result<Self, Self::Error> {
        Ok(Self::new_with_offset(GmtOffset::try_from(*value)?))
    }
}

#[cfg(feature = "compiled_data")]
impl TryFrom<&TimeZoneRecord<'_>> for CustomTimeZone {
    type Error = ParseError;

    fn try_from(value: &TimeZoneRecord) -> Result<Self, Self::Error> {
        match value {
            TimeZoneRecord::Name(iana_identifier) => {
                let mapper = TimeZoneIdMapper::new();
                let bcp47_id = mapper
                    .as_borrowed()
                    .iana_bytes_to_bcp47(iana_identifier)
                    .ok_or(ParseError::InvalidIanaIdentifier)?;
                Ok(Self::new_with_bcp47_id(bcp47_id))
            }
            TimeZoneRecord::Offset(offset) => Self::try_from(offset),
            _ => Err(ParseError::MissingTimeZone),
        }
    }
}

// ==== CustomZonedDateTime methods and traits ====

impl CustomZonedDateTime<Iso> {
    /// Create a [`CustomZonedDateTime`] in ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// ```
    /// use icu_timezone::{CustomZonedDateTime, CustomTimeZone, GmtOffset, TimeZoneBcp47Id};
    /// use tinystr::tinystr;
    ///
    /// let zoneddatetime = CustomZonedDateTime::try_iso_from_ixdtf_str("2024-08-08T12:08:19-05:00[America/Chicago]").unwrap();
    ///
    /// assert_eq!(zoneddatetime.date.year().number, 2024);
    /// assert_eq!(
    ///     zoneddatetime.date.month().code,
    ///     icu::calendar::types::MonthCode(tinystr::tinystr!(4, "M08"))
    /// );
    /// assert_eq!(zoneddatetime.date.day_of_month().0, 8);
    ///
    /// assert_eq!(zoneddatetime.time.hour.number(), 12);
    /// assert_eq!(zoneddatetime.time.minute.number(), 8);
    /// assert_eq!(zoneddatetime.time.second.number(), 19);
    /// assert_eq!(zoneddatetime.time.nanosecond.number(), 0);
    /// assert_eq!(zoneddatetime.zone, CustomTimeZone {
    ///     gmt_offset: Some(GmtOffset::try_from_offset_seconds(-18000).unwrap()),
    ///     time_zone_id: Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))),
    ///     metazone_id: None,
    ///     zone_variant: None,
    /// });
    ///
    /// ```
    ///
    /// For more information on time zone parsing, see [`CustomTimeZone::try_from_ixdtf_str`].
    #[cfg(feature = "compiled_data")]
    pub fn try_iso_from_ixdtf_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_iso_from_ixdtf_utf8(ixdtf_str.as_bytes())
    }

    /// Create a [`CustomZonedDateTime`] in ISO-8601 calendar from IXDTF syntax utf8 bytes.
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// See [`Self::try_iso_from_ixdtf_str`].
    #[cfg(feature = "compiled_data")]
    pub fn try_iso_from_ixdtf_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        Self::try_iso_from_ixdtf_record(&ixdtf_record)
    }

    #[cfg(feature = "compiled_data")]
    fn try_iso_from_ixdtf_record(ixdtf_record: &IxdtfParseRecord) -> Result<Self, ParseError> {
        let date_record = ixdtf_record.date.ok_or(ParseError::MissingFields)?;
        let date = Date::try_new_iso_date(date_record.year, date_record.month, date_record.day)?;
        let time_record = ixdtf_record.time.ok_or(ParseError::MissingFields)?;
        let time = Time::try_new(
            time_record.hour,
            time_record.minute,
            time_record.second,
            time_record.nanosecond,
        )?;
        let time_zone = CustomTimeZone::try_from_ixdtf_record(ixdtf_record)?;

        Ok(Self {
            date,
            time,
            zone: time_zone,
        })
    }
}

impl CustomZonedDateTime<AnyCalendar> {
    /// Create a [`CustomZonedDateTime`] in ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// ```
    /// use icu_timezone::{CustomZonedDateTime, CustomTimeZone, GmtOffset, TimeZoneBcp47Id};
    /// use tinystr::tinystr;
    ///
    /// let zoneddatetime = CustomZonedDateTime::try_from_ixdtf_str("2024-08-08T12:08:19-05:00[America/Chicago][u-ca=hebrew]").unwrap();
    ///
    /// assert_eq!(zoneddatetime.date.year().number, 5784);
    /// assert_eq!(
    ///     zoneddatetime.date.month().code,
    ///     icu::calendar::types::MonthCode(tinystr::tinystr!(4, "M11"))
    /// );
    /// assert_eq!(zoneddatetime.date.day_of_month().0, 4);
    ///
    /// assert_eq!(zoneddatetime.time.hour.number(), 12);
    /// assert_eq!(zoneddatetime.time.minute.number(), 8);
    /// assert_eq!(zoneddatetime.time.second.number(), 19);
    /// assert_eq!(zoneddatetime.time.nanosecond.number(), 0);
    /// assert_eq!(zoneddatetime.zone, CustomTimeZone {
    ///     gmt_offset: Some(GmtOffset::try_from_offset_seconds(-18000).unwrap()),
    ///     time_zone_id: Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))),
    ///     metazone_id: None,
    ///     zone_variant: None,
    /// });
    ///
    /// ```
    ///
    /// For more information on time zone parsing, see [`CustomTimeZone::try_from_ixdtf_str`].
    #[cfg(feature = "compiled_data")]
    pub fn try_from_ixdtf_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_from_ixdtf_utf8(ixdtf_str.as_bytes())
    }

    /// Create a [`CustomZonedDateTime`] in any calendar from IXDTF syntax utf8 bytes.
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// See [`Self::try_from_ixdtf_str`].
    #[cfg(feature = "compiled_data")]
    pub fn try_from_ixdtf_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        Self::try_from_ixdtf_record(&ixdtf_record)
    }

    #[cfg(feature = "compiled_data")]
    fn try_from_ixdtf_record(ixdtf_record: &IxdtfParseRecord) -> Result<Self, ParseError> {
        let iso_zdt = CustomZonedDateTime::<Iso>::try_iso_from_ixdtf_record(ixdtf_record)?;

        // Find the calendar (based off icu_calendar's AnyCalendar try_from)
        let calendar_id = ixdtf_record.calendar.unwrap_or(b"iso");
        let calendar_kind = icu_calendar::AnyCalendarKind::get_for_bcp47_bytes(calendar_id)
            .ok_or(ParseError::UnknownCalendar)?;
        let calendar = AnyCalendar::new(calendar_kind);

        Ok(Self {
            date: iso_zdt.date.to_any().to_calendar(calendar),
            time: iso_zdt.time,
            zone: iso_zdt.zone,
        })
    }
}

#[cfg(test)]
mod test {
    use ixdtf::parsers::IxdtfParser;

    use crate::{GmtOffset, InvalidOffsetError};

    #[test]
    fn max_possible_ixdtf_utc_offset() {
        let ixdtf_record =
            IxdtfParser::from_utf8("2024-08-08T12:08:19+23:59:60.999999999".as_bytes())
                .parse()
                .unwrap();
        let result = GmtOffset::try_from(ixdtf_record.offset.unwrap());
        assert_eq!(result, Err(InvalidOffsetError));
    }
}

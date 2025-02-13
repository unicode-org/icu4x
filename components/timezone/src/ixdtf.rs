// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    provider::{names::IanaToBcp47MapV3, ZoneOffsetPeriodV1},
    time_zone::models,
    DateTime, InvalidOffsetError, Time, TimeZoneBcp47Id, TimeZoneIdMapper,
    TimeZoneIdMapperBorrowed, TimeZoneInfo, UtcOffset, ZoneOffsetCalculator, ZonedDateTime,
};
use core::str::FromStr;
use icu_calendar::{AnyCalendarKind, AsCalendar, Date, DateError, Iso, RangeError};
use icu_provider::prelude::*;
use ixdtf::{
    parsers::{
        records::{
            DateRecord, IxdtfParseRecord, TimeRecord, TimeZoneAnnotation, TimeZoneRecord,
            UtcOffsetRecord, UtcOffsetRecordOrZ,
        },
        IxdtfParser,
    },
    ParseError as IxdtfParseError,
};

/// The error type for parsing IXDTF syntax strings in `icu_timezone`.
#[derive(Debug, PartialEq, displaydoc::Display)]
#[non_exhaustive]
pub enum ParseError {
    /// Syntax error for IXDTF string.
    #[displaydoc("Syntax error in the IXDTF string: {0}")]
    Syntax(IxdtfParseError),
    /// Parsed record is out of valid date range.
    #[displaydoc("Value out of range: {0}")]
    Range(RangeError),
    /// Parsed date and time records were not a valid ISO date.
    #[displaydoc("Parsed date and time records were not a valid ISO date: {0}")]
    Date(DateError),
    /// There were missing fields required to parse component.
    MissingFields,
    /// There were two offsets provided that were not consistent with each other.
    InconsistentTimeZoneOffsets,
    /// There was an invalid Offset.
    InvalidOffsetError,
    /// The set of time zone fields was not expected for the given type.
    /// For example, if a named time zone was present with offset-only parsing,
    /// or an offset was present with named-time-zone-only parsing.
    #[displaydoc("The set of time zone fields was not expected for the given type")]
    MismatchedTimeZoneFields,
    /// An unknown calendar was provided.
    UnknownCalendar,
    /// Expected a different calendar.
    #[displaydoc("Expected calendar {0} but found calendar {1}")]
    MismatchedCalendar(AnyCalendarKind, AnyCalendarKind),
    /// A timezone calculation is required to interpret this string, which is not supported.
    ///
    /// # Example
    /// ```
    /// use icu::calendar::Iso;
    /// use icu::timezone::{ZonedDateTimeParser, ParseError};
    ///
    /// // This timestamp is in UTC, and requires a time zone calculation in order to display a Zurich time.
    /// assert_eq!(
    ///     ZonedDateTimeParser::new().parse_loose("2024-08-12T12:32:00Z[Europe/Zurich]", Iso).unwrap_err(),
    ///     ParseError::RequiresCalculation,
    /// );
    ///
    /// // These timestamps are in local time
    /// ZonedDateTimeParser::new().parse_loose("2024-08-12T14:32:00+02:00[Europe/Zurich]", Iso).unwrap();
    /// ZonedDateTimeParser::new().parse_loose("2024-08-12T14:32:00[Europe/Zurich]", Iso).unwrap();
    /// ```
    #[displaydoc(
        "A timezone calculation is required to interpret this string, which is not supported"
    )]
    RequiresCalculation,
}

impl core::error::Error for ParseError {}

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

impl From<DateError> for ParseError {
    fn from(value: DateError) -> Self {
        Self::Date(value)
    }
}

impl From<InvalidOffsetError> for ParseError {
    fn from(_: InvalidOffsetError) -> Self {
        Self::InvalidOffsetError
    }
}

impl From<icu_calendar::ParseError> for ParseError {
    fn from(value: icu_calendar::ParseError) -> Self {
        match value {
            icu_calendar::ParseError::MissingFields => Self::MissingFields,
            icu_calendar::ParseError::Range(r) => Self::Range(r),
            icu_calendar::ParseError::Syntax(s) => Self::Syntax(s),
            icu_calendar::ParseError::UnknownCalendar => Self::UnknownCalendar,
            _ => unreachable!(),
        }
    }
}

impl UtcOffset {
    fn try_from_utc_offset_record(record: UtcOffsetRecord) -> Result<Self, ParseError> {
        let hour_seconds = i32::from(record.hour) * 3600;
        let minute_seconds = i32::from(record.minute) * 60;
        Self::try_from_seconds(
            i32::from(record.sign as i8)
                * (hour_seconds + minute_seconds + i32::from(record.second)),
        )
        .map_err(Into::into)
    }
}

#[derive(Debug)]
/// A parser for [`ZonedDateTime`] based on IXDTF strings.
///
/// Any function that takes a calendar argument  returns an error if the
/// string has a calendar annotation that does not match the calendar
/// argument.
///
/// ✨ *Enabled with the `ixdtf` Cargo feature.*
pub struct ZonedDateTimeParser {
    mapper: TimeZoneIdMapper,
    offsets: ZoneOffsetCalculator,
}

impl ZonedDateTimeParser {
    /// Creates a new [`ZonedDateTimeParser`] from compiled data.
    #[cfg(feature = "compiled_data")]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mapper: TimeZoneIdMapper::new().static_to_owned(),
            offsets: ZoneOffsetCalculator::new(),
        }
    }

    icu_provider::gen_buffer_data_constructors!(
        () -> error: DataError,
        functions: [
            new: skip,
                        try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, DataError>
    where
        P: ?Sized + DataProvider<ZoneOffsetPeriodV1> + DataProvider<IanaToBcp47MapV3>,
    {
        Ok(Self {
            mapper: TimeZoneIdMapper::try_new_unstable(provider)?,
            offsets: ZoneOffsetCalculator::try_new_unstable(provider)?,
        })
    }
}

struct Intermediate<'a> {
    offset: Option<UtcOffsetRecord>,
    is_z: bool,
    iana_identifier: Option<&'a [u8]>,
    date: DateRecord,
    time: TimeRecord,
}

impl<'a> Intermediate<'a> {
    fn try_from_ixdtf_record(ixdtf_record: &'a IxdtfParseRecord) -> Result<Self, ParseError> {
        let (offset, is_z, iana_identifier) = match ixdtf_record {
            // empty
            IxdtfParseRecord {
                offset: None,
                tz: None,
                ..
            } => (None, false, None),
            // -0800
            IxdtfParseRecord {
                offset: Some(UtcOffsetRecordOrZ::Offset(offset)),
                tz: None,
                ..
            } => (Some(*offset), false, None),
            // Z
            IxdtfParseRecord {
                offset: Some(UtcOffsetRecordOrZ::Z),
                tz: None,
                ..
            } => (None, true, None),
            // [-0800]
            IxdtfParseRecord {
                offset: None,
                tz:
                    Some(TimeZoneAnnotation {
                        tz: TimeZoneRecord::Offset(offset),
                        ..
                    }),
                ..
            } => (Some(*offset), false, None),
            // -0800[-0800]
            IxdtfParseRecord {
                offset: Some(UtcOffsetRecordOrZ::Offset(offset)),
                tz:
                    Some(TimeZoneAnnotation {
                        tz: TimeZoneRecord::Offset(offset1),
                        ..
                    }),
                ..
            } => {
                if offset != offset1 {
                    return Err(ParseError::InconsistentTimeZoneOffsets);
                }
                (Some(*offset), false, None)
            }
            // -0800[America/Los_Angeles]
            IxdtfParseRecord {
                offset: Some(UtcOffsetRecordOrZ::Offset(offset)),
                tz:
                    Some(TimeZoneAnnotation {
                        tz: TimeZoneRecord::Name(iana_identifier),
                        ..
                    }),
                ..
            } => (Some(*offset), false, Some(*iana_identifier)),
            // Z[-0800]
            IxdtfParseRecord {
                offset: Some(UtcOffsetRecordOrZ::Z),
                tz:
                    Some(TimeZoneAnnotation {
                        tz: TimeZoneRecord::Offset(offset),
                        ..
                    }),
                ..
            } => (Some(*offset), true, None),
            // Z[America/Los_Angeles]
            IxdtfParseRecord {
                offset: Some(UtcOffsetRecordOrZ::Z),
                tz:
                    Some(TimeZoneAnnotation {
                        tz: TimeZoneRecord::Name(iana_identifier),
                        ..
                    }),
                ..
            } => (None, true, Some(*iana_identifier)),
            // [America/Los_Angeles]
            IxdtfParseRecord {
                offset: None,
                tz:
                    Some(TimeZoneAnnotation {
                        tz: TimeZoneRecord::Name(iana_identifier),
                        ..
                    }),
                ..
            } => (None, false, Some(*iana_identifier)),
            // non_exhaustive match: maybe something like [u-tz=uslax] in the future
            IxdtfParseRecord {
                tz: Some(TimeZoneAnnotation { tz, .. }),
                ..
            } => {
                debug_assert!(false, "unexpected TimeZoneRecord: {tz:?}");
                (None, false, None)
            }
        };
        let IxdtfParseRecord {
            date: Some(date),
            time: Some(time),
            ..
        } = *ixdtf_record
        else {
            // Date or time was missing
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        Ok(Self {
            offset,
            is_z,
            iana_identifier,
            date,
            time,
        })
    }

    fn offset_only(self) -> Result<UtcOffset, ParseError> {
        let None = self.iana_identifier else {
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        if self.is_z {
            if let Some(offset) = self.offset {
                if offset != UtcOffsetRecord::zero() {
                    return Err(ParseError::RequiresCalculation);
                }
            }
            return Ok(UtcOffset::zero());
        }
        let Some(offset) = self.offset else {
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        UtcOffset::try_from_utc_offset_record(offset)
    }

    fn location_only(
        self,
        mapper: TimeZoneIdMapperBorrowed<'_>,
    ) -> Result<TimeZoneInfo<models::AtTime>, ParseError> {
        let None = self.offset else {
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        let Some(iana_identifier) = self.iana_identifier else {
            if self.is_z {
                return Err(ParseError::RequiresCalculation);
            }
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        let time_zone_id = mapper.iana_bytes_to_bcp47(iana_identifier);
        let date = Date::<Iso>::try_new_iso(self.date.year, self.date.month, self.date.day)?;
        let time = Time::try_new(
            self.time.hour,
            self.time.minute,
            self.time.second,
            self.time.nanosecond,
        )?;
        let offset = match time_zone_id.as_str() {
            "utc" | "gmt" => Some(UtcOffset::zero()),
            _ => None,
        };
        Ok(time_zone_id.with_offset(offset).at_time((date, time)))
    }

    fn loose(
        self,
        mapper: TimeZoneIdMapperBorrowed<'_>,
    ) -> Result<TimeZoneInfo<models::AtTime>, ParseError> {
        let time_zone_id = match self.iana_identifier {
            Some(iana_identifier) => {
                if self.is_z {
                    return Err(ParseError::RequiresCalculation);
                }
                mapper.iana_bytes_to_bcp47(iana_identifier)
            }
            None if self.is_z => TimeZoneBcp47Id(tinystr::tinystr!(8, "utc")),
            None => TimeZoneBcp47Id::unknown(),
        };
        let offset = match self.offset {
            Some(offset) => {
                if self.is_z && offset != UtcOffsetRecord::zero() {
                    return Err(ParseError::RequiresCalculation);
                }
                Some(UtcOffset::try_from_utc_offset_record(offset)?)
            }
            None => match time_zone_id.as_str() {
                "utc" | "gmt" => Some(UtcOffset::zero()),
                _ if self.is_z => Some(UtcOffset::zero()),
                _ => None,
            },
        };
        let date = Date::<Iso>::try_new_iso(self.date.year, self.date.month, self.date.day)?;
        let time = Time::try_new(
            self.time.hour,
            self.time.minute,
            self.time.second,
            self.time.nanosecond,
        )?;
        Ok(time_zone_id.with_offset(offset).at_time((date, time)))
    }

    fn full(
        self,
        mapper: TimeZoneIdMapperBorrowed<'_>,
        zone_offset_calculator: &ZoneOffsetCalculator,
    ) -> Result<TimeZoneInfo<models::Full>, ParseError> {
        let Some(offset) = self.offset else {
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        let Some(iana_identifier) = self.iana_identifier else {
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        let time_zone_id = mapper.iana_bytes_to_bcp47(iana_identifier);
        let date = Date::try_new_iso(self.date.year, self.date.month, self.date.day)?;
        let time = Time::try_new(
            self.time.hour,
            self.time.minute,
            self.time.second,
            self.time.nanosecond,
        )?;
        let offset = UtcOffset::try_from_utc_offset_record(offset)?;
        time_zone_id
            .with_offset(Some(offset))
            .at_time((date, time))
            .try_infer_zone_variant(zone_offset_calculator)
            .ok_or(ParseError::InvalidOffsetError)
    }
}

impl ZonedDateTimeParser {
    /// Create a [`ZonedDateTime`] in any calendar from an IXDTF syntax string.
    ///
    /// Returns an error if the string has a calendar annotation that does not
    /// match the calendar argument, unless the argument is [`Iso`].
    ///
    /// This function is "strict": the string should have only an offset and no named time zone.
    pub fn parse_offset_only<A: AsCalendar>(
        &self,
        ixdtf_str: &str,
        calendar: A,
    ) -> Result<ZonedDateTime<A, UtcOffset>, ParseError> {
        self.parse_offset_only_from_utf8(ixdtf_str.as_bytes(), calendar)
    }

    /// Create a [`ZonedDateTime`] in any calendar from IXDTF syntax UTF-8 bytes.
    ///
    /// See [`Self::parse_offset_only`].
    pub fn parse_offset_only_from_utf8<A: AsCalendar>(
        &self,
        ixdtf_str: &[u8],
        calendar: A,
    ) -> Result<ZonedDateTime<A, UtcOffset>, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let date = Date::try_from_ixdtf_record(&ixdtf_record, calendar)?;
        let time = Time::try_from_ixdtf_record(&ixdtf_record)?;
        let zone = Intermediate::try_from_ixdtf_record(&ixdtf_record)?.offset_only()?;
        Ok(ZonedDateTime { date, time, zone })
    }

    /// Create a [`ZonedDateTime`] in any calendar from an IXDTF syntax string.
    ///
    /// Returns an error if the string has a calendar annotation that does not
    /// match the calendar argument, unless the argument is [`Iso`].
    ///
    /// This function is "strict": the string should have only a named time zone and no offset.
    pub fn parse_location_only<A: AsCalendar>(
        &self,
        ixdtf_str: &str,
        calendar: A,
    ) -> Result<ZonedDateTime<A, TimeZoneInfo<models::AtTime>>, ParseError> {
        self.parse_location_only_from_utf8(ixdtf_str.as_bytes(), calendar)
    }

    /// Create a [`ZonedDateTime`] in any calendar from IXDTF syntax UTF-8 bytes.
    ///
    /// See [`Self::parse_location_only`].
    pub fn parse_location_only_from_utf8<A: AsCalendar>(
        &self,
        ixdtf_str: &[u8],
        calendar: A,
    ) -> Result<ZonedDateTime<A, TimeZoneInfo<models::AtTime>>, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let date = Date::try_from_ixdtf_record(&ixdtf_record, calendar)?;
        let time = Time::try_from_ixdtf_record(&ixdtf_record)?;
        let zone = Intermediate::try_from_ixdtf_record(&ixdtf_record)?
            .location_only(self.mapper.as_borrowed())?;
        Ok(ZonedDateTime { date, time, zone })
    }

    /// Create a [`ZonedDateTime`] in any calendar from an IXDTF syntax string.
    ///
    /// Returns an error if the string has a calendar annotation that does not
    /// match the calendar argument, unless the argument is [`Iso`].
    ///
    /// This function is "loose": the string can have an offset, and named time zone, both, or
    /// neither. If the named time zone is missing, it is returned as Etc/Unknown.
    ///
    /// The zone variant is _not_ calculated with this function. If you need it, use
    /// [`Self::parse`].
    pub fn parse_loose<A: AsCalendar>(
        &self,
        ixdtf_str: &str,
        calendar: A,
    ) -> Result<ZonedDateTime<A, TimeZoneInfo<models::AtTime>>, ParseError> {
        self.parse_loose_from_utf8(ixdtf_str.as_bytes(), calendar)
    }

    /// Create a [`ZonedDateTime`] in any calendar from IXDTF syntax UTF-8 bytes.
    ///
    /// See [`Self::parse_loose`].
    pub fn parse_loose_from_utf8<A: AsCalendar>(
        &self,
        ixdtf_str: &[u8],
        calendar: A,
    ) -> Result<ZonedDateTime<A, TimeZoneInfo<models::AtTime>>, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let date = Date::try_from_ixdtf_record(&ixdtf_record, calendar)?;
        let time = Time::try_from_ixdtf_record(&ixdtf_record)?;
        let zone =
            Intermediate::try_from_ixdtf_record(&ixdtf_record)?.loose(self.mapper.as_borrowed())?;
        Ok(ZonedDateTime { date, time, zone })
    }

    /// Create a [`ZonedDateTime`] in any calendar from an IXDTF syntax string.
    ///
    /// Returns an error if the string has a calendar annotation that does not
    /// match the calendar argument, unless the argument is [`Iso`].
    ///
    /// The string should have both an offset and a named time zone.
    ///
    /// For more information on IXDTF, see the [`ixdtf`] crate.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use icu_calendar::cal::Hebrew;
    /// use icu_timezone::{
    ///     ZonedDateTimeParser, TimeZoneBcp47Id, TimeZoneInfo, UtcOffset, ZoneVariant,
    /// };
    /// use tinystr::tinystr;
    ///
    /// let zoneddatetime = ZonedDateTimeParser::new()
    ///     .parse("2024-08-08T12:08:19-05:00[America/Chicago][u-ca=hebrew]", Hebrew)
    ///     .unwrap();
    ///
    /// assert_eq!(zoneddatetime.date.year().extended_year, 5784);
    /// assert_eq!(
    ///     zoneddatetime.date.month().standard_code,
    ///     icu::calendar::types::MonthCode(tinystr::tinystr!(4, "M11"))
    /// );
    /// assert_eq!(zoneddatetime.date.day_of_month().0, 4);
    ///
    /// assert_eq!(zoneddatetime.time.hour.number(), 12);
    /// assert_eq!(zoneddatetime.time.minute.number(), 8);
    /// assert_eq!(zoneddatetime.time.second.number(), 19);
    /// assert_eq!(zoneddatetime.time.nanosecond.number(), 0);
    /// assert_eq!(
    ///     zoneddatetime.zone.time_zone_id(),
    ///     TimeZoneBcp47Id(tinystr!(8, "uschi"))
    /// );
    /// assert_eq!(
    ///     zoneddatetime.zone.offset(),
    ///     Some(UtcOffset::try_from_seconds(-18000).unwrap())
    /// );
    /// assert_eq!(zoneddatetime.zone.zone_variant(), ZoneVariant::Daylight);
    /// let (_, _) = zoneddatetime.zone.local_time();
    /// ```
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
    /// use icu_calendar::Iso;
    /// use icu_timezone::{ZonedDateTimeParser, TimeZoneInfo, UtcOffset};
    ///
    /// let tz_from_offset = ZonedDateTimeParser::new()
    ///     .parse_offset_only("2024-08-08T12:08:19-05:00", Iso)
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     tz_from_offset.zone,
    ///     UtcOffset::try_from_seconds(-18000).unwrap()
    /// );
    /// ```
    ///
    /// ## Time Zone Annotations
    ///
    /// Below is an example of a time zone being provided by a time zone annotation.
    ///
    /// ```
    /// use icu_calendar::Iso;
    /// use icu_timezone::{
    ///     ZonedDateTimeParser, TimeZoneBcp47Id, TimeZoneInfo, UtcOffset, ZoneVariant,
    /// };
    /// use tinystr::tinystr;
    ///
    /// let tz_from_offset_annotation = ZonedDateTimeParser::new()
    ///     .parse_offset_only("2024-08-08T12:08:19[-05:00]", Iso)
    ///     .unwrap();
    /// let tz_from_iana_annotation = ZonedDateTimeParser::new()
    ///     .parse_location_only("2024-08-08T12:08:19[America/Chicago]", Iso)
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     tz_from_offset_annotation.zone,
    ///     UtcOffset::try_from_seconds(-18000).unwrap()
    /// );
    ///
    /// assert_eq!(
    ///     tz_from_iana_annotation.zone.time_zone_id(),
    ///     TimeZoneBcp47Id(tinystr!(8, "uschi"))
    /// );
    /// assert_eq!(tz_from_iana_annotation.zone.offset(), None);
    /// ```
    ///
    /// ## DateTime UTC Offset and Time Zone Annotations.
    ///
    /// An IXDTF string may contain both a DateTime UTC Offset and Time Zone Annotation. This is fine as long as
    /// the time zone parts can be deemed as inconsistent or unknown consistency.
    ///
    /// ### DateTime UTC Offset with IANA identifier annotation
    ///
    /// In cases where the DateTime UTC Offset is provided and the IANA identifier, some validity checks are performed.
    ///
    /// ```
    /// use icu_calendar::Iso;
    /// use icu_timezone::{TimeZoneInfo, ZonedDateTimeParser, UtcOffset, TimeZoneBcp47Id, ZoneVariant, ParseError};
    /// use tinystr::tinystr;
    ///
    /// let consistent_tz_from_both = ZonedDateTimeParser::new().parse("2024-08-08T12:08:19-05:00[America/Chicago]", Iso).unwrap();
    ///
    ///
    /// assert_eq!(consistent_tz_from_both.zone.time_zone_id(), TimeZoneBcp47Id(tinystr!(8, "uschi")));
    /// assert_eq!(consistent_tz_from_both.zone.offset(), Some(UtcOffset::try_from_seconds(-18000).unwrap()));
    /// assert_eq!(consistent_tz_from_both.zone.zone_variant(), ZoneVariant::Daylight);
    /// let (_, _) = consistent_tz_from_both.zone.local_time();
    ///
    /// // We know that America/Los_Angeles never used a -05:00 offset at any time of the year 2024
    /// assert_eq!(
    ///     ZonedDateTimeParser::new().parse("2024-08-08T12:08:19-05:00[America/Los_Angeles]", Iso).unwrap_err(),
    ///     ParseError::InvalidOffsetError
    /// );
    ///
    /// // We don't know that America/Los_Angeles didn't use standard time (-08:00) in August
    /// assert!(
    ///     ZonedDateTimeParser::new().parse("2024-08-08T12:08:19-08:00[America/Los_Angeles]", Iso).is_ok()
    /// );
    /// ```
    ///
    /// ### DateTime UTC offset with UTC Offset annotation.
    ///
    /// These annotations must always be consistent as they should be either the same value or are inconsistent.
    ///
    /// ```
    /// use icu_calendar::Iso;
    /// use icu_timezone::{
    ///     ZonedDateTimeParser, ParseError, TimeZoneBcp47Id, TimeZoneInfo, UtcOffset,
    /// };
    /// use tinystr::tinystr;
    ///
    /// let consistent_tz_from_both = ZonedDateTimeParser::new()
    ///     .parse_offset_only("2024-08-08T12:08:19-05:00[-05:00]", Iso)
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     consistent_tz_from_both.zone,
    ///     UtcOffset::try_from_seconds(-18000).unwrap()
    /// );
    ///
    /// let inconsistent_tz_from_both = ZonedDateTimeParser::new()
    ///     .parse_offset_only("2024-08-08T12:08:19-05:00[+05:00]", Iso);
    ///
    /// assert!(matches!(
    ///     inconsistent_tz_from_both,
    ///     Err(ParseError::InconsistentTimeZoneOffsets)
    /// ));
    /// ```
    pub fn parse<A: AsCalendar>(
        &self,
        ixdtf_str: &str,
        calendar: A,
    ) -> Result<ZonedDateTime<A, TimeZoneInfo<models::Full>>, ParseError> {
        self.parse_from_utf8(ixdtf_str.as_bytes(), calendar)
    }

    /// Create a [`ZonedDateTime`] in any calendar from IXDTF syntax UTF-8 bytes.
    ///
    /// See [`Self::parse`].
    pub fn parse_from_utf8<A: AsCalendar>(
        &self,
        ixdtf_str: &[u8],
        calendar: A,
    ) -> Result<ZonedDateTime<A, TimeZoneInfo<models::Full>>, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let date = Date::try_from_ixdtf_record(&ixdtf_record, calendar)?;
        let time = Time::try_from_ixdtf_record(&ixdtf_record)?;
        let zone = Intermediate::try_from_ixdtf_record(&ixdtf_record)?
            .full(self.mapper.as_borrowed(), &self.offsets)?;

        Ok(ZonedDateTime { date, time, zone })
    }
}

impl FromStr for DateTime<Iso> {
    type Err = ParseError;
    fn from_str(ixdtf_str: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(ixdtf_str, Iso)
    }
}

impl<A: AsCalendar> DateTime<A> {
    /// Creates a [`DateTime`] in any calendar from an IXDTF syntax string.
    ///
    /// Returns an error if the string has a calendar annotation that does not
    /// match the calendar argument, unless the argument is [`Iso`].
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::cal::Hebrew;
    /// use icu::timezone::DateTime;
    ///
    /// let datetime =
    ///     DateTime::try_from_str("2024-07-17T16:01:17.045[u-ca=hebrew]", Hebrew).unwrap();
    ///
    /// assert_eq!(datetime.date.year().era_year_or_extended(), 5784);
    /// assert_eq!(
    ///     datetime.date.month().standard_code,
    ///     icu::calendar::types::MonthCode(tinystr::tinystr!(4, "M10"))
    /// );
    /// assert_eq!(datetime.date.day_of_month().0, 11);
    ///
    /// assert_eq!(datetime.time.hour.number(), 16);
    /// assert_eq!(datetime.time.minute.number(), 1);
    /// assert_eq!(datetime.time.second.number(), 17);
    /// assert_eq!(datetime.time.nanosecond.number(), 45000000);
    /// ```
    pub fn try_from_str(ixdtf_str: &str, calendar: A) -> Result<Self, ParseError> {
        Self::try_from_utf8(ixdtf_str.as_bytes(), calendar)
    }

    /// Creates a [`DateTime`] in any calendar from an IXDTF syntax string.
    ///
    /// See [`Self::try_from_str()`].
    ///
    /// ✨ *Enabled with the `ixdtf` Cargo feature.*
    pub fn try_from_utf8(ixdtf_str: &[u8], calendar: A) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let date = Date::try_from_ixdtf_record(&ixdtf_record, calendar)?;
        let time = Time::try_from_ixdtf_record(&ixdtf_record)?;
        Ok(Self { date, time })
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
    /// use icu::timezone::Time;
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::TimeZoneBcp47Id;

    #[test]
    fn max_possible_ixdtf_utc_offset() {
        assert_eq!(
            ZonedDateTimeParser::new()
                .parse_offset_only("2024-08-08T12:08:19+23:59:60.999999999", Iso)
                .unwrap_err(),
            ParseError::InvalidOffsetError
        );
    }

    #[test]
    fn zone_calculations() {
        ZonedDateTimeParser::new()
            .parse_offset_only("2024-08-08T12:08:19Z", Iso)
            .unwrap();
        assert_eq!(
            ZonedDateTimeParser::new()
                .parse_offset_only("2024-08-08T12:08:19Z[+08:00]", Iso)
                .unwrap_err(),
            ParseError::RequiresCalculation
        );
        assert_eq!(
            ZonedDateTimeParser::new()
                .parse_offset_only("2024-08-08T12:08:19Z[Europe/Zurich]", Iso)
                .unwrap_err(),
            ParseError::MismatchedTimeZoneFields
        );
    }

    #[test]
    fn future_zone() {
        let result = ZonedDateTimeParser::new()
            .parse_loose("2024-08-08T12:08:19[Future/Zone]", Iso)
            .unwrap();
        assert_eq!(result.zone.time_zone_id(), TimeZoneBcp47Id::unknown());
        assert_eq!(result.zone.offset(), None);
    }
}

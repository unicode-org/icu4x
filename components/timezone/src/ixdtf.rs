// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    time_zone::models, CustomZonedDateTime, InvalidOffsetError, TimeZoneBcp47Id, TimeZoneIdMapper,
    TimeZoneInfo, UtcOffset, ZoneOffsetCalculator, ZoneVariant,
};
use alloc::str::FromStr;
use icu_calendar::{AnyCalendar, Date, DateError, DateTime, Iso, RangeError, Time};
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
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum ParseError {
    /// Syntax error for IXDTF string.
    Syntax(IxdtfParseError),
    /// Parsed record is out of valid date range.
    Range(RangeError),
    /// Parsed date and time records were not a valid ISO date.
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
    MismatchedTimeZoneFields,
    /// An unknown calendar was provided.
    UnknownCalendar,
    /// A timezone calculation is required to interpret this string, which is not supported.
    ///
    /// # Example
    /// ```
    /// use icu::timezone::{CustomZonedDateTime, ParseError};
    ///
    /// // This timestamp is in UTC, and requires a time zone calculation in order to display a Zurich time.
    /// assert_eq!(
    ///     CustomZonedDateTime::try_loose_iso_from_str("2024-08-12T12:32:00Z[Europe/Zurich]").unwrap_err(),
    ///     ParseError::RequiresCalculation,
    /// );
    ///
    /// // These timestamps are in local time
    /// CustomZonedDateTime::try_loose_iso_from_str("2024-08-12T14:32:00+02:00[Europe/Zurich]").unwrap();
    /// CustomZonedDateTime::try_loose_iso_from_str("2024-08-12T14:32:00[Europe/Zurich]").unwrap();
    /// ```
    RequiresCalculation,
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

struct Intermediate<'a> {
    offset: Option<UtcOffsetRecord>,
    iana_identifier: Option<&'a [u8]>,
    date: DateRecord,
    time: TimeRecord,
}

impl<'a> Intermediate<'a> {
    fn try_from_ixdtf_record(ixdtf_record: &'a IxdtfParseRecord) -> Result<Self, ParseError> {
        let (offset, iana_identifier) = match ixdtf_record {
            // empty
            // -0800
            // Z
            IxdtfParseRecord {
                offset, tz: None, ..
            } => (offset.map(UtcOffsetRecordOrZ::resolve_rfc_9557), None),
            // [-0800]
            IxdtfParseRecord {
                offset: None,
                tz:
                    Some(TimeZoneAnnotation {
                        tz: TimeZoneRecord::Offset(offset),
                        ..
                    }),
                ..
            } => (Some(*offset), None),
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
                (Some(*offset), None)
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
            } => (Some(*offset), Some(*iana_identifier)),
            // Z[-0800]
            // Z[America/Los_Angeles]
            IxdtfParseRecord {
                offset: Some(UtcOffsetRecordOrZ::Z),
                tz: Some(_),
                ..
            } => return Err(ParseError::RequiresCalculation),
            // [America/Los_Angeles]
            IxdtfParseRecord {
                offset: None,
                tz:
                    Some(TimeZoneAnnotation {
                        tz: TimeZoneRecord::Name(iana_identifier),
                        ..
                    }),
                ..
            } => (None, Some(*iana_identifier)),
            // non_exhaustive match: maybe something like [u-tz=uslax] in the future
            IxdtfParseRecord {
                tz: Some(TimeZoneAnnotation { tz, .. }),
                ..
            } => {
                debug_assert!(false, "unexpected TimeZoneRecord: {tz:?}");
                (None, None)
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
            iana_identifier,
            date,
            time,
        })
    }

    fn offset_only(self) -> Result<UtcOffset, ParseError> {
        let Some(offset) = self.offset else {
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        let None = self.iana_identifier else {
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        UtcOffset::try_from_utc_offset_record(offset)
    }

    fn location_only(self) -> Result<TimeZoneInfo<models::AtTime>, ParseError> {
        let None = self.offset else {
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        let Some(iana_identifier) = self.iana_identifier else {
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        let time_zone_id = TimeZoneIdMapper::new().iana_bytes_to_bcp47(iana_identifier);
        let iso = DateTime::<Iso>::try_new_iso(
            self.date.year,
            self.date.month,
            self.date.day,
            self.time.hour,
            self.time.minute,
            self.time.second,
        )?;
        Ok(time_zone_id.with_offset(None).at_time((iso.date, iso.time)))
    }

    fn loose(self) -> Result<TimeZoneInfo<models::AtTime>, ParseError> {
        let offset = match self.offset {
            Some(offset) => Some(UtcOffset::try_from_utc_offset_record(offset)?),
            None => None,
        };
        let time_zone_id = match self.iana_identifier {
            Some(iana_identifier) => TimeZoneIdMapper::new().iana_bytes_to_bcp47(iana_identifier),
            None => TimeZoneBcp47Id::unknown(),
        };
        let iso = DateTime::<Iso>::try_new_iso(
            self.date.year,
            self.date.month,
            self.date.day,
            self.time.hour,
            self.time.minute,
            self.time.second,
        )?;
        Ok(time_zone_id
            .with_offset(offset)
            .at_time((iso.date, iso.time)))
    }

    fn full(self) -> Result<TimeZoneInfo<models::Full>, ParseError> {
        let Some(offset) = self.offset else {
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        let Some(iana_identifier) = self.iana_identifier else {
            return Err(ParseError::MismatchedTimeZoneFields);
        };
        let time_zone_id = TimeZoneIdMapper::new().iana_bytes_to_bcp47(iana_identifier);
        let iso = DateTime::<Iso>::try_new_iso(
            self.date.year,
            self.date.month,
            self.date.day,
            self.time.hour,
            self.time.minute,
            self.time.second,
        )?;
        let offset = UtcOffset::try_from_utc_offset_record(offset)?;
        let zone_offset_calculator = ZoneOffsetCalculator::new();
        let zone_variant =
            match zone_offset_calculator.compute_offsets_from_time_zone(time_zone_id, &iso) {
                Some((std_offset, dst_offset)) => {
                    if offset == std_offset {
                        ZoneVariant::standard()
                    } else if Some(offset) == dst_offset {
                        ZoneVariant::daylight()
                    } else {
                        return Err(ParseError::InvalidOffsetError);
                    }
                }
                None => {
                    // time_zone_id not found; Etc/Unknown?
                    debug_assert_eq!(time_zone_id.0.as_str(), "unk");
                    ZoneVariant::standard()
                }
            };
        Ok(offset
            .with_id(time_zone_id)
            .at_time((iso.date, iso.time))
            .with_zone_variant(zone_variant))
    }
}

// ==== TimeZoneInfo methods and traits ====

// ==== CustomZonedDateTime methods and traits ====

impl CustomZonedDateTime<Iso, UtcOffset> {
    /// Create a [`CustomZonedDateTime`] in ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// This function is "strict": the string should have only an offset and no named time zone.
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_offset_only_iso_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_offset_only_iso_from_utf8(ixdtf_str.as_bytes())
    }

    /// Create a [`CustomZonedDateTime`] in ISO-8601 calendar from IXDTF syntax utf8 bytes.
    ///
    /// This function is "strict": the string should have only an offset and no named time zone.
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_offset_only_iso_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let intermediate = Intermediate::try_from_ixdtf_record(&ixdtf_record)?;
        let time_zone = intermediate.offset_only()?;
        Self::try_iso_from_ixdtf_record(&ixdtf_record, time_zone)
    }
}

impl CustomZonedDateTime<Iso, TimeZoneInfo<models::AtTime>> {
    /// Create a [`CustomZonedDateTime`] in ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// This function is "strict": the string should have only a named time zone and no offset.
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_location_only_iso_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_location_only_iso_from_utf8(ixdtf_str.as_bytes())
    }

    /// Create a [`CustomZonedDateTime`] in ISO-8601 calendar from IXDTF syntax utf8 bytes.
    ///
    /// This function is "strict": the string should have only a named time zone and no offset.
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_location_only_iso_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let intermediate = Intermediate::try_from_ixdtf_record(&ixdtf_record)?;
        let time_zone = intermediate.location_only()?;
        Self::try_iso_from_ixdtf_record(&ixdtf_record, time_zone)
    }
}

impl CustomZonedDateTime<Iso, TimeZoneInfo<models::AtTime>> {
    /// Create a [`CustomZonedDateTime`] in ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// This function is "loose": the string can have an offset, and named time zone, both, or
    /// neither. If the named time zone is missing, it is returned as Etc/Unknown.
    ///
    /// The zone variant is _not_ calculated with this function. If you need it, use
    /// [`CustomZonedDateTime::try_iso_from_str`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_loose_iso_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_loose_iso_from_utf8(ixdtf_str.as_bytes())
    }

    /// Create a [`CustomZonedDateTime`] in ISO-8601 calendar from IXDTF syntax utf8 bytes.
    ///
    /// This function is "loose": the string can have an offset, and named time zone, both, or
    /// neither. If the named time zone is missing, it is returned as Etc/Unknown.
    ///
    /// The zone variant is _not_ calculated with this function. If you need it, use
    /// [`CustomZonedDateTime::try_iso_from_utf8`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_loose_iso_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let intermediate = Intermediate::try_from_ixdtf_record(&ixdtf_record)?;
        let time_zone = intermediate.loose()?;
        Self::try_iso_from_ixdtf_record(&ixdtf_record, time_zone)
    }
}

impl CustomZonedDateTime<Iso, TimeZoneInfo<models::Full>> {
    /// Create a [`CustomZonedDateTime`] in ISO-8601 calendar from an IXDTF syntax string.
    ///
    /// The string should have both an offset and a named time zone.
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    ///
    /// ```
    /// use icu_timezone::{CustomZonedDateTime, TimeZoneInfo, UtcOffset, TimeZoneBcp47Id, ZoneVariant};
    /// use tinystr::tinystr;
    ///
    /// let zoneddatetime = CustomZonedDateTime::try_iso_from_str("2024-08-08T12:08:19-05:00[America/Chicago]").unwrap();
    ///
    /// assert_eq!(zoneddatetime.date.year().extended_year, 2024);
    /// assert_eq!(
    ///     zoneddatetime.date.month().standard_code,
    ///     icu::calendar::types::MonthCode(tinystr::tinystr!(4, "M08"))
    /// );
    /// assert_eq!(zoneddatetime.date.day_of_month().0, 8);
    ///
    /// assert_eq!(zoneddatetime.time.hour.number(), 12);
    /// assert_eq!(zoneddatetime.time.minute.number(), 8);
    /// assert_eq!(zoneddatetime.time.second.number(), 19);
    /// assert_eq!(zoneddatetime.time.nanosecond.number(), 0);
    /// assert_eq!(zoneddatetime.zone.time_zone_id, TimeZoneBcp47Id(tinystr!(8, "uschi")));
    /// assert_eq!(zoneddatetime.zone.offset, Some(UtcOffset::try_from_seconds(-18000).unwrap()));
    /// assert_eq!(zoneddatetime.zone.zone_variant, ZoneVariant::daylight());
    /// let (_, _) = zoneddatetime.zone.local_time;
    /// ```
    ///
    /// For more information on date, time, and time zone parsing,
    /// see [`CustomZonedDateTime::try_from_str`].
    pub fn try_iso_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_iso_from_utf8(ixdtf_str.as_bytes())
    }

    /// Create a [`CustomZonedDateTime`] in ISO-8601 calendar from IXDTF syntax utf8 bytes.
    ///
    /// The string should have both an offset and a named time zone.
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    ///
    /// See [`Self::try_iso_from_str`].
    pub fn try_iso_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let intermediate = Intermediate::try_from_ixdtf_record(&ixdtf_record)?;
        let time_zone = intermediate.full()?;
        Self::try_iso_from_ixdtf_record(&ixdtf_record, time_zone)
    }
}

impl<Z> CustomZonedDateTime<Iso, Z> {
    fn try_iso_from_ixdtf_record(
        ixdtf_record: &IxdtfParseRecord,
        zone: Z,
    ) -> Result<Self, ParseError> {
        let date_record = ixdtf_record.date.ok_or(ParseError::MissingFields)?;
        let date = Date::try_new_iso(date_record.year, date_record.month, date_record.day)?;
        let time_record = ixdtf_record.time.ok_or(ParseError::MissingFields)?;
        let time = Time::try_new(
            time_record.hour,
            time_record.minute,
            time_record.second,
            time_record.nanosecond,
        )?;

        Ok(Self { date, time, zone })
    }
}

impl FromStr for CustomZonedDateTime<Iso, TimeZoneInfo<models::Full>> {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_iso_from_str(s)
    }
}

impl CustomZonedDateTime<AnyCalendar, UtcOffset> {
    /// Create a [`CustomZonedDateTime`] in any calendar from an IXDTF syntax string.
    ///
    /// This function is "strict": the string should have only an offset and no named time zone.
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_offset_only_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_offset_only_from_utf8(ixdtf_str.as_bytes())
    }

    /// Create a [`CustomZonedDateTime`] in any calendar from IXDTF syntax utf8 bytes.
    ///
    /// This function is "strict": the string should have only an offset and no named time zone.
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_offset_only_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let intermediate = Intermediate::try_from_ixdtf_record(&ixdtf_record)?;
        let time_zone = intermediate.offset_only()?;
        Self::try_from_ixdtf_record(&ixdtf_record, time_zone)
    }
}

impl CustomZonedDateTime<AnyCalendar, TimeZoneInfo<models::AtTime>> {
    /// Create a [`CustomZonedDateTime`] in any calendar from an IXDTF syntax string.
    ///
    /// This function is "strict": the string should have only a named time zone and no offset.
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_location_only_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_location_only_from_utf8(ixdtf_str.as_bytes())
    }

    /// Create a [`CustomZonedDateTime`] in any calendar from IXDTF syntax utf8 bytes.
    ///
    /// This function is "strict": the string should have only a named time zone and no offset.
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_location_only_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let intermediate = Intermediate::try_from_ixdtf_record(&ixdtf_record)?;
        let time_zone = intermediate.location_only()?;
        Self::try_from_ixdtf_record(&ixdtf_record, time_zone)
    }
}

impl CustomZonedDateTime<AnyCalendar, TimeZoneInfo<models::AtTime>> {
    /// Create a [`CustomZonedDateTime`] in any calendar from an IXDTF syntax string.
    ///
    /// This function is "loose": the string can have an offset, and named time zone, both, or
    /// neither. If the named time zone is missing, it is returned as Etc/Unknown.
    ///
    /// The zone variant is _not_ calculated with this function. If you need it, use
    /// [`CustomZonedDateTime::try_from_str`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_loose_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_loose_from_utf8(ixdtf_str.as_bytes())
    }

    /// Create a [`CustomZonedDateTime`] in any calendar from IXDTF syntax utf8 bytes.
    ///
    /// This function is "loose": the string can have an offset, and named time zone, both, or
    /// neither. If the named time zone is missing, it is returned as Etc/Unknown.
    ///
    /// The zone variant is _not_ calculated with this function. If you need it, use
    /// [`CustomZonedDateTime::try_from_utf8`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    pub fn try_loose_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let intermediate = Intermediate::try_from_ixdtf_record(&ixdtf_record)?;
        let time_zone = intermediate.loose()?;
        Self::try_from_ixdtf_record(&ixdtf_record, time_zone)
    }
}

impl CustomZonedDateTime<AnyCalendar, TimeZoneInfo<models::Full>> {
    /// Create a [`CustomZonedDateTime`] in any calendar from an IXDTF syntax string.
    ///
    /// The string should have both an offset and a named time zone.
    ///
    /// For more information on IXDTF, see the [`ixdtf`] crate.
    ///
    /// This is a convenience constructor that uses compiled data. For custom data providers,
    /// use [`ixdtf`] and/or the other primitives in this crate such as [`TimeZoneIdMapper`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use icu_timezone::{CustomZonedDateTime, TimeZoneInfo, UtcOffset, TimeZoneBcp47Id, ZoneVariant};
    /// use tinystr::tinystr;
    ///
    /// let zoneddatetime = CustomZonedDateTime::try_from_str("2024-08-08T12:08:19-05:00[America/Chicago][u-ca=hebrew]").unwrap();
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
    /// assert_eq!(zoneddatetime.zone.time_zone_id, TimeZoneBcp47Id(tinystr!(8, "uschi")));
    /// assert_eq!(zoneddatetime.zone.offset, Some(UtcOffset::try_from_seconds(-18000).unwrap()));
    /// assert_eq!(zoneddatetime.zone.zone_variant, ZoneVariant::daylight());
    /// let (_, _) = zoneddatetime.zone.local_time;
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
    /// use icu_timezone::{TimeZoneInfo, CustomZonedDateTime, UtcOffset};
    ///
    /// let tz_from_offset = CustomZonedDateTime::try_offset_only_from_str("2024-08-08T12:08:19-05:00").unwrap();
    ///
    /// assert_eq!(tz_from_offset.zone, UtcOffset::try_from_seconds(-18000).unwrap());
    /// ```
    ///
    /// ## Time Zone Annotations
    ///
    /// Below is an example of a time zone being provided by a time zone annotation.
    ///
    /// ```
    /// use icu_timezone::{TimeZoneInfo, CustomZonedDateTime, UtcOffset, TimeZoneBcp47Id, ZoneVariant};
    /// use tinystr::tinystr;
    ///
    /// let tz_from_offset_annotation = CustomZonedDateTime::try_offset_only_from_str("2024-08-08T12:08:19[-05:00]").unwrap();
    /// let tz_from_iana_annotation = CustomZonedDateTime::try_location_only_from_str("2024-08-08T12:08:19[America/Chicago]").unwrap();
    ///
    /// assert_eq!(tz_from_offset_annotation.zone, UtcOffset::try_from_seconds(-18000).unwrap());
    ///
    /// assert_eq!(tz_from_iana_annotation.zone.time_zone_id, TimeZoneBcp47Id(tinystr!(8, "uschi")));
    /// assert_eq!(tz_from_iana_annotation.zone.offset, None);
    /// let () = tz_from_iana_annotation.zone.zone_variant;
    /// let (_, _) = tz_from_iana_annotation.zone.local_time;
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
    /// use icu_timezone::{TimeZoneInfo, CustomZonedDateTime, UtcOffset, TimeZoneBcp47Id, ZoneVariant, ParseError};
    /// use tinystr::tinystr;
    ///
    /// let consistent_tz_from_both = CustomZonedDateTime::try_from_str("2024-08-08T12:08:19-05:00[America/Chicago]").unwrap();
    ///
    ///
    /// assert_eq!(consistent_tz_from_both.zone.time_zone_id, TimeZoneBcp47Id(tinystr!(8, "uschi")));
    /// assert_eq!(consistent_tz_from_both.zone.offset, Some(UtcOffset::try_from_seconds(-18000).unwrap()));
    /// assert_eq!(consistent_tz_from_both.zone.zone_variant, ZoneVariant::daylight());
    /// let (_, _) = consistent_tz_from_both.zone.local_time;
    ///
    /// // We know that America/Los_Angeles never used a -05:00 offset at any time of the year 2024
    /// assert_eq!(
    ///     CustomZonedDateTime::try_from_str("2024-08-08T12:08:19-05:00[America/Los_Angeles]").unwrap_err(),
    ///     ParseError::InvalidOffsetError
    /// );
    ///
    /// // We don't know that America/Los_Angeles didn't use standard time (-08:00) in August
    /// assert!(
    ///     CustomZonedDateTime::try_from_str("2024-08-08T12:08:19-08:00[America/Los_Angeles]").is_ok()
    /// );
    /// ```
    ///
    /// ### DateTime UTC offset with UTC Offset annotation.
    ///
    /// These annotations must always be consistent as they should be either the same value or are inconsistent.
    ///
    /// ```
    /// use icu_timezone::{ParseError, TimeZoneInfo, CustomZonedDateTime, UtcOffset, TimeZoneBcp47Id};
    /// use tinystr::tinystr;
    ///
    /// let consistent_tz_from_both = CustomZonedDateTime::try_offset_only_from_str("2024-08-08T12:08:19-05:00[-05:00]").unwrap();
    ///
    /// assert_eq!(consistent_tz_from_both.zone, UtcOffset::try_from_seconds(-18000).unwrap());
    ///
    ///
    /// let inconsistent_tz_from_both = CustomZonedDateTime::try_offset_only_from_str("2024-08-08T12:08:19-05:00[+05:00]");
    ///
    /// assert!(matches!(inconsistent_tz_from_both, Err(ParseError::InconsistentTimeZoneOffsets)));
    /// ```
    pub fn try_from_str(ixdtf_str: &str) -> Result<Self, ParseError> {
        Self::try_from_utf8(ixdtf_str.as_bytes())
    }

    /// Create a [`CustomZonedDateTime`] in any calendar from IXDTF syntax utf8 bytes.
    ///
    /// The string should have both an offset and a named time zone.
    ///
    /// ✨ *Enabled with the `compiled_data` and `ixdtf` Cargo features.*
    ///
    /// See [`Self::try_from_str`].
    pub fn try_from_utf8(ixdtf_str: &[u8]) -> Result<Self, ParseError> {
        let ixdtf_record = IxdtfParser::from_utf8(ixdtf_str).parse()?;
        let intermediate = Intermediate::try_from_ixdtf_record(&ixdtf_record)?;
        let time_zone = intermediate.full()?;
        Self::try_from_ixdtf_record(&ixdtf_record, time_zone)
    }
}

impl<Z> CustomZonedDateTime<AnyCalendar, Z> {
    fn try_from_ixdtf_record(ixdtf_record: &IxdtfParseRecord, zone: Z) -> Result<Self, ParseError> {
        let iso_zdt = CustomZonedDateTime::<Iso, Z>::try_iso_from_ixdtf_record(ixdtf_record, zone)?;

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

impl FromStr for CustomZonedDateTime<AnyCalendar, TimeZoneInfo<models::Full>> {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TimeZoneBcp47Id;

    #[test]
    fn max_possible_ixdtf_utc_offset() {
        assert_eq!(
            CustomZonedDateTime::try_offset_only_iso_from_str(
                "2024-08-08T12:08:19+23:59:60.999999999"
            )
            .unwrap_err(),
            ParseError::InvalidOffsetError
        );
    }

    #[test]
    fn zone_calculations() {
        CustomZonedDateTime::try_offset_only_iso_from_str("2024-08-08T12:08:19Z").unwrap();
        assert_eq!(
            CustomZonedDateTime::try_offset_only_iso_from_str("2024-08-08T12:08:19Z[+08:00]")
                .unwrap_err(),
            ParseError::RequiresCalculation
        );
        assert_eq!(
            CustomZonedDateTime::try_offset_only_iso_from_str(
                "2024-08-08T12:08:19Z[Europe/Zurich]"
            )
            .unwrap_err(),
            ParseError::RequiresCalculation
        );
    }

    #[test]
    fn future_zone() {
        let result =
            CustomZonedDateTime::try_loose_from_str("2024-08-08T12:08:19[Future/Zone]").unwrap();
        assert_eq!(result.zone.time_zone_id, TimeZoneBcp47Id::unknown());
        assert_eq!(result.zone.offset, None);
    }
}

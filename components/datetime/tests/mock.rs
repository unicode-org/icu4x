// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Some useful parsing functions for tests.

use icu_calendar::Gregorian;
use icu_timezone::{
    models, TimeZoneIdMapper, TimeZoneInfo, ZoneOffsetCalculator, ZoneVariant, ZonedDateTime,
};

/// Parse a [`DateTime`] and [`TimeZoneInfo`] from a string.
///
/// This utility is for easily creating dates, not a complete robust solution. The
/// string must take a specific form of the ISO 8601 format:
/// `YYYY-MM-DDThh:mm:ssZ`,
/// `YYYY-MM-DDThh:mm:ss±hh`,
/// `YYYY-MM-DDThh:mm:ss±hhmm`,
/// `YYYY-MM-DDThh:mm:ss±hh:mm`,
///
/// # Examples
///
/// ```
/// use icu::datetime::mock;
///
/// let (date, zone) =
///     mock::parse_zoned_gregorian_from_str("2020-10-14T13:21:00+05:30")
///         .expect("Failed to parse a zoned datetime.");
/// ```
pub fn parse_zoned_gregorian_from_str(
    input: &str,
) -> ZonedDateTime<Gregorian, TimeZoneInfo<models::Full>> {
    match ZonedDateTime::try_from_str(
        input,
        Gregorian,
        TimeZoneIdMapper::new(),
        &ZoneOffsetCalculator::new(),
    ) {
        Ok(zdt) => zdt,
        Err(icu_timezone::ParseError::MismatchedTimeZoneFields) => {
            match ZonedDateTime::try_loose_from_str(input, Gregorian, TimeZoneIdMapper::new()) {
                Ok(zdt) => {
                    ZonedDateTime {
                        date: zdt.date,
                        time: zdt.time,
                        // For fixture tests, set the zone variant to standard here
                        zone: zdt.zone.with_zone_variant(ZoneVariant::Standard),
                    }
                }
                Err(e) => panic!("could not parse input: {input}: {e:?}"),
            }
        }
        Err(e) => panic!("could not parse input: {input}: {e:?}"),
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{cmp, fmt};

use icu_calendar::{types::RataDie, Date, Iso};

const ZONE_NAME_EPOCH: RataDie = calendrical_calculations::iso::const_fixed_from_iso(1970, 1, 1);
const QUARTER_HOURS_IN_DAY_I64: i64 = 24 * 4;
const QUARTER_HOURS_IN_DAY_I32: i32 = 24 * 4;
#[allow(overflowing_literals)] // this is intentional
const MIN_QUARTER_HOURS: i64 = 0xFF000000i32 as i64;
const MAX_QUARTER_HOURS: i64 = 0x00FFFFFFi32 as i64;

use crate::{DateTime, Hour, Minute, Nanosecond, Second, Time};

extern crate std;

/// Internal intermediate type for interfacing with [`ZoneNameTimestamp`].
#[derive(Debug, Copy, Clone)]
struct ZoneNameTimestampParts {
    quarter_hours_since_local_unix_epoch: i32,
    metadata: u8,
}

impl ZoneNameTimestampParts {
    /// Recovers the DateTime from these parts.
    fn date_time(self) -> DateTime<Iso> {
        let qh = self.quarter_hours_since_local_unix_epoch;
        // Note: the `as` casts below are trivially save because the rem_euclid is in range
        let (days, remainder) = (qh.div_euclid(QUARTER_HOURS_IN_DAY_I32) as i64, qh.rem_euclid(QUARTER_HOURS_IN_DAY_I32) as u8);
        let (hours, minutes) = (remainder / 4, (remainder % 4) * 15);
        std::eprintln!("recovered: {days} {hours} {minutes}");
        DateTime {
            date: Date::from_rata_die(ZONE_NAME_EPOCH + days, Iso),
            time: Time {
                hour: Hour::try_from(hours).unwrap_or_else(|_| {
                    debug_assert!(false, "ZoneNameTimestampParts: out of range: {self:?}");
                    Hour::zero()
                }),
                minute: Minute::try_from(minutes).unwrap_or_else(|_| {
                    debug_assert!(false, "ZoneNameTimestampParts: out of range: {self:?}");
                    Minute::zero()
                }),
                second: Second::zero(),
                subsecond: Nanosecond::zero(),
            },
        }
    }

    /// Creates an instance of this type with all invariants upheld.
    fn from_saturating_date_time_with_metadata(date_time: &DateTime<Iso>, metadata: u8) -> Self {
        // Note: RataDie should be in range for this multiplication.
        let qh_days = (date_time.date.to_rata_die() - ZONE_NAME_EPOCH) * QUARTER_HOURS_IN_DAY_I64;
        // Note: Hour is 0 to 23 in a u8 so it should be in range for this multiplication.
        let qh_hours = date_time.time.hour.number() * 4;
        let qh_minutes = date_time.time.minute.number() / 15;
        let qh_total = qh_days + (qh_hours as i64) + (qh_minutes as i64);
        std::eprintln!("qh: {qh_days} {qh_hours} {qh_minutes} {qh_total}");
        let qh_clamped = cmp::max(cmp::min(qh_total, MAX_QUARTER_HOURS), MIN_QUARTER_HOURS);
        let qh_i32 = match i32::try_from(qh_clamped) {
            Ok(x) => x,
            Err(_) => {
                debug_assert!(
                    false,
                    "ZoneNameTimestampParts: saturation invariants not upheld: {date_time:?}"
                );
                0
            }
        };
        let metadata = if metadata > 0x7F { 0 } else { metadata };
        ZoneNameTimestampParts {
            quarter_hours_since_local_unix_epoch: qh_i32,
            metadata,
        }
    }
}

/// The moment in time for resolving a time zone name.
///
/// When a location-based time zone, such as "America/Ciudad_Juarez", switches
/// to a whole different set of rules, the time zone display name, sometimes
/// called the "metazone", can also change. This type is used for picking the
/// set of time zone display names.
#[derive(Copy, Clone)]
pub struct ZoneNameTimestamp(u32);

impl ZoneNameTimestamp {
    /// Recovers the local datetime for this [`ZoneNameTimestamp`].
    ///
    /// For more information, see [`Self::from_date_time_iso()`].
    pub fn to_date_time_iso(self) -> DateTime<Iso> {
        let parts = self.to_parts();
        std::eprintln!("to: {} => {parts:?}", self.0);
        parts.date_time()
    }

    /// Creates an instance of [`ZoneNameTimestamp`] from a local datetime.
    ///
    /// The datetime might be clamped and might lose precision.
    ///
    /// Note: Currently, this type cannot represent ambiguous times in the
    /// period after a time zone transition. For example, if a "fall back"
    /// time zone transition occurs at 02:00, then the times 01:00-02:00
    /// occur twice. To ensure that you get the correct time zone display
    /// name _after_ a transition, you can pick any time later in the day.
    ///
    /// # Examples
    ///
    /// DateTime does _not_ necessarily roundtrip:
    ///
    /// ```
    /// use icu::time::zone::ZoneNameTimestamp;
    /// use icu::calendar::Date;
    /// use icu::time::{Time, DateTime};
    ///
    /// let date_time = DateTime {
    ///     date: Date::try_new_iso(2025, 4, 30).unwrap(),
    ///     time: Time::try_new(13, 58, 16, 500000000).unwrap(),
    /// };
    ///
    /// let zone_name_timestamp = ZoneNameTimestamp::from_date_time_iso(&date_time);
    ///
    /// let recovered_date_time = zone_name_timestamp.to_date_time_iso();
    ///
    /// // The datetime doesn't roundtrip:
    /// assert_ne!(date_time, recovered_date_time);
    ///
    /// // The exact behavior is subject to change. For illustration only:
    /// assert_eq!(recovered_date_time.date, date_time.date);
    /// assert_eq!(recovered_date_time.time.hour, date_time.time.hour);
    /// assert_eq!(recovered_date_time.time.minute.number(), 45); // rounded down
    /// assert_eq!(recovered_date_time.time.second.number(), 0); // always zero
    /// assert_eq!(recovered_date_time.time.subsecond.number(), 0); // always zero
    /// ```
    pub fn from_date_time_iso(date_time: &DateTime<Iso>) -> Self {
        let metadata = 0; // currently unused (reserved)
        let parts =
            ZoneNameTimestampParts::from_saturating_date_time_with_metadata(date_time, metadata);
        std::eprintln!("from: {parts:?}");
        Self::from_parts(parts)
    }

    fn to_parts(self) -> ZoneNameTimestampParts {
        let metadata = ((self.0 & 0x7F000000) >> 24) as u8;
        // TODO(rust 1.87): use cast_signed()
        let qh_masked_i32 = self.0 as i32;
        #[allow(overflowing_literals)] // intentional
        let qh_recovered = ((qh_masked_i32 >> 7) & 0xFF000000) | qh_masked_i32;
        ZoneNameTimestampParts {
            quarter_hours_since_local_unix_epoch: qh_recovered,
            metadata,
        }
    }

    fn from_parts(parts: ZoneNameTimestampParts) -> Self {
        let metadata_shifted = (parts.metadata as u32) << 24;
        // TODO(rust 1.87): use cast_unsigned()
        let qh_masked = (parts.quarter_hours_since_local_unix_epoch as u32) & 0x80FFFFFF;
        debug_assert_eq!(
            metadata_shifted & qh_masked,
            0
        );
        Self(metadata_shifted | qh_masked)
    }
}

impl fmt::Debug for ZoneNameTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts = self.to_parts();
        f.debug_struct("ZoneNameTimestamp")
            .field("date_time", &parts.date_time())
            .field("metadata", &parts.metadata)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_roundtrip() {
        #[derive(Debug)]
        struct TestCase {
            input: DateTime<Iso>,
            output: DateTime<Iso>,
        }
        for test_case in [
            TestCase {
                input: "1970-01-01T00:00".parse().unwrap(),
                output: "1970-01-01T00:00".parse().unwrap(),
            },
            TestCase {
                input: "1970-01-01T00:01".parse().unwrap(),
                output: "1970-01-01T00:00".parse().unwrap(),
            },
            TestCase {
                input: "1970-01-01T00:15".parse().unwrap(),
                output: "1970-01-01T00:15".parse().unwrap(),
            },
            TestCase {
                input: "1970-01-01T00:29".parse().unwrap(),
                output: "1970-01-01T00:15".parse().unwrap(),
            },
            TestCase {
                input: "1969-12-31T23:59".parse().unwrap(),
                output: "1969-12-31T23:45".parse().unwrap(),
            },
            TestCase {
                input: "1969-12-31T12:00".parse().unwrap(),
                output: "1969-12-31T12:00".parse().unwrap(),
            },
            TestCase {
                input: "1900-07-15T12:34".parse().unwrap(),
                output: "1900-07-15T12:30".parse().unwrap(),
            },
            TestCase {
                input: "2000-07-15T12:34".parse().unwrap(),
                output: "2000-07-15T12:30".parse().unwrap(),
            },
            TestCase {
                input: "2100-07-15T12:34".parse().unwrap(),
                output: "2100-07-15T12:30".parse().unwrap(),
            },
            TestCase {
                // Min Value
                input: "1491-07-08T08:00".parse().unwrap(),
                output: "1491-07-08T08:00".parse().unwrap(),
            },
            TestCase {
                input: "1491-07-08T07:59".parse().unwrap(),
                output: "1491-07-08T08:00".parse().unwrap(),
            },
            TestCase {
                input: "1000-01-01T00:00".parse().unwrap(),
                output: "1491-07-08T08:00".parse().unwrap(),
            },
        ] {
            let znt = ZoneNameTimestamp::from_date_time_iso(&test_case.input);
            let actual = znt.to_date_time_iso();
            assert_eq!(test_case.output, actual, "{test_case:?}");
        }
    }
}

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{cmp, fmt};

use icu_calendar::{types::RataDie, Date, Iso};
use zerovec::{maps::ZeroMapKV, ule::AsULE, ZeroSlice, ZeroVec};

/// The epoch for time zone names. This is set to 1970-01-01 since the TZDB often drops data before then.
const ZONE_NAME_EPOCH: RataDie = calendrical_calculations::iso::const_fixed_from_iso(1970, 1, 1);
const QUARTER_HOURS_IN_DAY_I64: i64 = 24 * 4;
const QUARTER_HOURS_IN_DAY_U32: u32 = 24 * 4;
const MIN_QUARTER_HOURS: i64 = 0;
const MAX_QUARTER_HOURS: i64 = 0xFFFFFF;

use crate::{DateTime, Hour, Minute, Nanosecond, Second, Time};

extern crate std;

/// Internal intermediate type for interfacing with [`ZoneNameTimestamp`].
#[derive(Debug, Copy, Clone)]
struct ZoneNameTimestampParts {
    quarter_hours_since_local_unix_epoch: u32,
    metadata: u8,
}

impl ZoneNameTimestampParts {
    /// Recovers the DateTime from these parts.
    fn date_time(self) -> DateTime<Iso> {
        let qh = self.quarter_hours_since_local_unix_epoch;
        // Note: the `as` casts below are trivially save because the rem_euclid is in range
        let (days, remainder) = (
            (qh / QUARTER_HOURS_IN_DAY_U32) as i64,
            (qh % QUARTER_HOURS_IN_DAY_U32) as u8,
        );
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
        let qh_u32 = match u32::try_from(qh_clamped) {
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
            quarter_hours_since_local_unix_epoch: qh_u32,
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
///
/// # Examples
///
/// ```
/// use icu::time::zone::IanaParser;
/// use icu::time::zone::ZoneNameTimestamp;
/// use icu::datetime::NoCalendarFormatter;
/// use icu::datetime::fieldsets::zone::GenericLong;
/// use icu::locale::locale;
/// use writeable::assert_writeable_eq;
///
/// let juba = IanaParser::new().parse("America/Chihuahua");
///
/// let zone_formatter = NoCalendarFormatter::try_new(
///     locale!("en-US").into(),
///     GenericLong,
/// )
/// .unwrap();
///
/// let time_zone_info_2015 = juba.without_offset().at_date_time_iso(&"2015-01-01T00:00".parse().unwrap());
/// let time_zone_info_2025 = juba.without_offset().at_date_time_iso(&"2025-01-01T00:00".parse().unwrap());
///
/// assert_eq!(
///     time_zone_info_2015.zone_name_timestamp(),
///     ZoneNameTimestamp::from_date_time_iso(&"2015-01-01T00:00".parse().unwrap())
/// );
/// assert_eq!(
///     time_zone_info_2025.zone_name_timestamp(),
///     ZoneNameTimestamp::from_date_time_iso(&"2025-01-01T00:00".parse().unwrap())
/// );
///
/// let name_2015 = zone_formatter.format(&time_zone_info_2015);
/// let name_2025 = zone_formatter.format(&time_zone_info_2025);
///
/// assert_writeable_eq!(name_2015, "South Sudan Time");
/// assert_writeable_eq!(name_2025, "def");
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
        let metadata = ((self.0 & 0xFF000000) >> 24) as u8;
        let qh_recovered = self.0 & 0x00FFFFFF;
        ZoneNameTimestampParts {
            quarter_hours_since_local_unix_epoch: qh_recovered,
            metadata,
        }
    }

    fn from_parts(parts: ZoneNameTimestampParts) -> Self {
        let metadata_shifted = (parts.metadata as u32) << 24;
        debug_assert!(parts.quarter_hours_since_local_unix_epoch <= 0x00FFFFFF);
        let qh_masked = parts.quarter_hours_since_local_unix_epoch & 0x00FFFFFF;
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

impl AsULE for ZoneNameTimestamp {
    type ULE = <u32 as AsULE>::ULE;
    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        self.0.to_unaligned()
    }
    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(u32::from_unaligned(unaligned))
    }
}

impl<'a> ZeroMapKV<'a> for ZoneNameTimestamp {
    type Container = ZeroVec<'a, Self>;
    type Slice = ZeroSlice<Self>;
    type GetType = <Self as AsULE>::ULE;
    type OwnedType = Self;
}

#[cfg(feature = "serde")]
impl serde::Serialize for ZoneNameTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[cfg(feature = "alloc")]
        if serializer.is_human_readable() {
            let date_time = self.to_date_time_iso();
            let year = date_time.date.extended_year();
            let month = date_time.date.month().month_number();
            let day = date_time.date.day_of_month().0;
            let hour = date_time.time.hour.number();
            let minute = date_time.time.minute.number();
            // don't serialize the metadata for now
            return serializer.serialize_str(&alloc::format!(
                "{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}"
            ));
        }
        serializer.serialize_u32(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ZoneNameTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[cfg(feature = "alloc")]
        if deserializer.is_human_readable() {
            use serde::de::Error;
            let e0 = D::Error::custom("invalid");
            let e1 = |_| D::Error::custom("invalid");
            let e2 = |_| D::Error::custom("invalid");
            let e3 = |_| D::Error::custom("invalid");

            let parts = alloc::borrow::Cow::<'de, str>::deserialize(deserializer)?;
            if parts.len() != 16 {
                return Err(e0);
            }
            let year = parts[0..4].parse::<i32>().map_err(e1)?;
            let month = parts[5..7].parse::<u8>().map_err(e1)?;
            let day = parts[8..10].parse::<u8>().map_err(e1)?;
            let hour = parts[11..13].parse::<u8>().map_err(e1)?;
            let minute = parts[14..16].parse::<u8>().map_err(e1)?;
            return Ok(Self::from_date_time_iso(&DateTime {
                date: Date::try_new_iso(year, month, day).map_err(e2)?,
                time: Time::try_new(hour, minute, 0, 0).map_err(e3)?,
            }));
        }
        u32::deserialize(deserializer).map(Self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_packing() {
        #[derive(Debug)]
        struct TestCase {
            input: DateTime<Iso>,
            output: DateTime<Iso>,
        }
        for test_case in [
            // Behavior at the epoch
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
            // Min Value Clamping
            TestCase {
                input: "1969-12-31T23:59".parse().unwrap(),
                output: "1970-01-01T00:00".parse().unwrap(),
            },
            TestCase {
                input: "1969-12-31T12:00".parse().unwrap(),
                output: "1970-01-01T00:00".parse().unwrap(),
            },
            TestCase {
                input: "1900-07-15T12:34".parse().unwrap(),
                output: "1970-01-01T00:00".parse().unwrap(),
            },
            // Max Value Clamping
            TestCase {
                input: "2448-06-25T15:45".parse().unwrap(),
                output: "2448-06-25T15:45".parse().unwrap(),
            },
            TestCase {
                input: "2448-06-25T16:00".parse().unwrap(),
                output: "2448-06-25T15:45".parse().unwrap(),
            },
            TestCase {
                input: "2448-06-26T00:00".parse().unwrap(),
                output: "2448-06-25T15:45".parse().unwrap(),
            },
            TestCase {
                input: "2500-01-01T00:00".parse().unwrap(),
                output: "2448-06-25T15:45".parse().unwrap(),
            },
            // Other cases
            TestCase {
                input: "2025-04-30T15:18:25".parse().unwrap(),
                output: "2025-04-30T15:15".parse().unwrap(),
            },
        ] {
            let znt = ZoneNameTimestamp::from_date_time_iso(&test_case.input);
            let actual = znt.to_date_time_iso();
            assert_eq!(test_case.output, actual, "{test_case:?}");
        }
    }

    #[test]
    fn test_metadata_noop() {
        let raw = (0x12345678u32).to_unaligned();
        let znt = ZoneNameTimestamp::from_unaligned(raw);
        let roundtrip_znt = ZoneNameTimestamp::from_date_time_iso(&znt.to_date_time_iso());
        let roundtrip_raw = roundtrip_znt.to_unaligned();

        // [0..3] is the datetime. [3] is the metadata.
        assert_eq!(raw.0[0..3], roundtrip_raw.0[0..3]);
        assert_eq!(raw.0[3], 0x12);
        assert_eq!(roundtrip_raw.0[3], 0);
    }
}

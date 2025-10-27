// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use icu_calendar::Iso;
use zerovec::ule::AsULE;

use crate::{zone::UtcOffset, DateTime, ZonedDateTime};

/// The moment in time for resolving a time zone name.
///
/// **What is this for?** Most software deals with _time zone transitions_,
/// computing the UTC offset on a given point in time. In ICU4X, we deal with
/// _time zone display names_. Whereas time zone transitions occur multiple
/// times per year in some time zones, the set of display names changes more
/// rarely. For example, ICU4X needs to know when a region switches from
/// Eastern Time to Central Time.
///
/// This type can only represent display name changes after 1970, and only to
/// a coarse (15-minute) granularity, which is sufficient for CLDR and TZDB
/// data within that time frame.
///
/// # Examples
///
/// The region of Metlakatla (Alaska) switched between Pacific Time and
/// Alaska Time multiple times between 2010 and 2025.
///
/// ```
/// use icu::calendar::Iso;
/// use icu::datetime::fieldsets::zone::GenericLong;
/// use icu::datetime::NoCalendarFormatter;
/// use icu::locale::locale;
/// use icu::time::zone::IanaParser;
/// use icu::time::zone::ZoneNameTimestamp;
/// use icu::time::ZonedDateTime;
/// use writeable::assert_writeable_eq;
///
/// let metlakatla = IanaParser::new().parse("America/Metlakatla");
///
/// let zone_formatter =
///     NoCalendarFormatter::try_new(locale!("en-US").into(), GenericLong)
///         .unwrap();
///
/// let time_zone_info_2010 = metlakatla
///     .without_offset()
///     .with_zone_name_timestamp(ZoneNameTimestamp::from_zoned_date_time_iso(
///         ZonedDateTime::try_offset_only_from_str("2010-01-01T00:00Z", Iso)
///             .unwrap(),
///     ));
/// let time_zone_info_2025 = metlakatla
///     .without_offset()
///     .with_zone_name_timestamp(ZoneNameTimestamp::from_zoned_date_time_iso(
///         ZonedDateTime::try_offset_only_from_str("2025-01-01T00:00Z", Iso)
///             .unwrap(),
///     ));
///
/// // Check the display names:
/// let name_2010 = zone_formatter.format(&time_zone_info_2010);
/// let name_2025 = zone_formatter.format(&time_zone_info_2025);
///
/// assert_writeable_eq!(name_2010, "Pacific Time");
/// assert_writeable_eq!(name_2025, "Alaska Time");
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoneNameTimestamp(u32);

impl ZoneNameTimestamp {
    /// Recovers the UTC datetime for this [`ZoneNameTimestamp`].
    ///
    /// This will always return a [`ZonedDateTime`] with [`UtcOffset::zero()`]
    pub fn to_zoned_date_time_iso(self) -> ZonedDateTime<Iso, UtcOffset> {
        ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
            match self.0 as i64 * 15 * 60 * 1000 {
                // See `from_zoned_date_time_iso`
                63593100000 => 63593070000,
                307622700000 => 307622400000,
                576042300000 => 576041460000,
                576044100000 => 576043260000,
                594180900000 => 594180060000,
                607491900000 => 607491060000,
                1601741700000 => 1601740860000,
                1633191300000 => 1633190460000,
                1664640900000 => 1664640060000,
                ms => ms,
            },
            UtcOffset::zero(),
        )
    }

    /// Creates an instance of [`ZoneNameTimestamp`] from a zoned datetime.
    ///
    /// The datetime might be clamped and might lose precision.
    ///
    /// # Examples
    ///
    /// ZonedDateTime does _not_ necessarily roundtrip:
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::time::zone::ZoneNameTimestamp;
    /// use icu::time::{ZonedDateTime, Time, zone::UtcOffset};
    ///
    /// let zoned_date_time = ZonedDateTime {
    ///     date: Date::try_new_iso(2025, 4, 30).unwrap(),
    ///     time: Time::try_new(13, 58, 16, 500000000).unwrap(),
    ///     zone: UtcOffset::zero(),
    /// };
    ///
    /// let zone_name_timestamp = ZoneNameTimestamp::from_zoned_date_time_iso(zoned_date_time);
    ///
    /// let recovered_zoned_date_time = zone_name_timestamp.to_zoned_date_time_iso();
    ///
    /// // The datetime doesn't roundtrip:
    /// assert_ne!(zoned_date_time, recovered_zoned_date_time);
    ///
    /// // The exact behavior is subject to change. For illustration only:
    /// assert_eq!(recovered_zoned_date_time.date, zoned_date_time.date);
    /// assert_eq!(recovered_zoned_date_time.time.hour, zoned_date_time.time.hour);
    /// assert_eq!(recovered_zoned_date_time.time.minute.number(), 45); // rounded down
    /// assert_eq!(recovered_zoned_date_time.time.second.number(), 0); // always zero
    /// assert_eq!(recovered_zoned_date_time.time.subsecond.number(), 0); // always zero
    /// ```
    pub fn from_zoned_date_time_iso(zoned_date_time: ZonedDateTime<Iso, UtcOffset>) -> Self {
        let ms = match zoned_date_time.to_epoch_milliseconds_utc() {
            // Values that are not multiples of 15, that we map to the next multiple
            // of 15 (which is always 00:15 or 00:45, values that are otherwise unused).
            63593070000..63593100000 => 63593100000,
            307622400000..307622700000 => 307622700000,
            576041460000..576042300000 => 576042300000,
            576043260000..576044100000 => 576044100000,
            594180060000..594180900000 => 594180900000,
            607491060000..607491900000 => 607491900000,
            1601740860000..1601741700000 => 1601741700000,
            1633190460000..1633191300000 => 1633191300000,
            1664640060000..1664640900000 => 1664640900000,
            ms => ms,
        };
        let qh = ms / 1000 / 60 / 15;
        let qh_clamped = qh.clamp(Self::far_in_past().0 as i64, Self::far_in_future().0 as i64);
        // Valid cast as the value is clamped to u32 values.
        Self(qh_clamped as u32)
    }

    /// Recovers the UTC datetime for this [`ZoneNameTimestamp`].
    #[deprecated(
        since = "2.1.0",
        note = "returns a UTC DateTime, which is the wrong type. Use `to_zoned_date_time_iso` instead"
    )]
    pub fn to_date_time_iso(self) -> DateTime<Iso> {
        let ZonedDateTime {
            date,
            time,
            zone: _utc_offset_zero,
        } = self.to_zoned_date_time_iso();
        DateTime { date, time }
    }

    /// Creates an instance of [`ZoneNameTimestamp`] from a UTC datetime.
    ///
    /// The datetime might be clamped and might lose precision.
    #[deprecated(
        since = "2.1.0",
        note = "implicitly interprets the DateTime as UTC. Use `from_zoned_date_time_iso` instead."
    )]
    pub fn from_date_time_iso(DateTime { date, time }: DateTime<Iso>) -> Self {
        Self::from_zoned_date_time_iso(ZonedDateTime {
            date,
            time,
            zone: UtcOffset::zero(),
        })
    }

    /// Returns a [`ZoneNameTimestamp`] for a time far in the past.
    pub fn far_in_past() -> Self {
        Self(0)
    }

    /// Returns a [`ZoneNameTimestamp`] for a time far in the future.
    pub fn far_in_future() -> Self {
        Self(0xFFFFFF)
    }
}

impl fmt::Debug for ZoneNameTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.to_zoned_date_time_iso(), f)
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

#[cfg(feature = "alloc")]
impl<'a> zerovec::maps::ZeroMapKV<'a> for ZoneNameTimestamp {
    type Container = zerovec::ZeroVec<'a, Self>;
    type Slice = zerovec::ZeroSlice<Self>;
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
            let date_time = self.to_zoned_date_time_iso();
            let year = date_time.date.era_year().year;
            let month = date_time.date.month().month_number();
            let day = date_time.date.day_of_month().0;
            let hour = date_time.time.hour.number();
            let minute = date_time.time.minute.number();
            let second = date_time.time.second.number();
            let mut s = alloc::format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}");
            if second != 0 {
                use alloc::fmt::Write;
                let _infallible = write!(&mut s, ":{second:02}");
            }
            // don't serialize the metadata for now
            return serializer.serialize_str(&s);
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
            return Ok(Self::from_zoned_date_time_iso(ZonedDateTime {
                date: icu_calendar::Date::try_new_iso(year, month, day).map_err(e2)?,
                time: crate::Time::try_new(hour, minute, 0, 0).map_err(e3)?,
                zone: UtcOffset::zero(),
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
            input: &'static str,
            output: &'static str,
        }
        for test_case in [
            // Behavior at the epoch
            TestCase {
                input: "1970-01-01T00:00Z",
                output: "1970-01-01T00:00Z",
            },
            TestCase {
                input: "1970-01-01T00:01Z",
                output: "1970-01-01T00:00Z",
            },
            TestCase {
                input: "1970-01-01T00:15Z",
                output: "1970-01-01T00:15Z",
            },
            TestCase {
                input: "1970-01-01T00:29Z",
                output: "1970-01-01T00:15Z",
            },
            // Min Value Clamping
            TestCase {
                input: "1969-12-31T23:59Z",
                output: "1970-01-01T00:00Z",
            },
            TestCase {
                input: "1969-12-31T12:00Z",
                output: "1970-01-01T00:00Z",
            },
            TestCase {
                input: "1900-07-15T12:34Z",
                output: "1970-01-01T00:00Z",
            },
            // Max Value Clamping
            TestCase {
                input: "2448-06-25T15:45Z",
                output: "2448-06-25T15:45Z",
            },
            TestCase {
                input: "2448-06-25T16:00Z",
                output: "2448-06-25T15:45Z",
            },
            TestCase {
                input: "2448-06-26T00:00Z",
                output: "2448-06-25T15:45Z",
            },
            TestCase {
                input: "2500-01-01T00:00Z",
                output: "2448-06-25T15:45Z",
            },
            // Offset adjusments
            TestCase {
                input: "2025-10-10T10:15+02",
                output: "2025-10-10T08:15Z",
            },
            // Other cases
            TestCase {
                input: "2025-04-30T15:18:25Z",
                output: "2025-04-30T15:15Z",
            },
        ] {
            let znt = ZoneNameTimestamp::from_zoned_date_time_iso(
                ZonedDateTime::try_offset_only_from_str(test_case.input, Iso).unwrap(),
            );
            let actual = znt.to_zoned_date_time_iso();
            assert_eq!(
                ZonedDateTime::try_offset_only_from_str(test_case.output, Iso).unwrap(),
                actual,
                "{test_case:?}"
            );
        }
    }
}

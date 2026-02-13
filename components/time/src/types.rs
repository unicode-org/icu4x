// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::{types::RataDie, AsCalendar, Date, Iso, RangeError};

use crate::zone::UtcOffset;

/// This macro defines a struct for 0-based date fields: hours, minutes, seconds
/// and fractional seconds. Each unit is bounded by a range. The traits implemented
/// here will return a Result on whether or not the unit is in range from the given
/// input.
macro_rules! dt_unit {
    ($name:ident, $storage:ident, $value:expr, $(#[$docs:meta])+) => {
        $(#[$docs])+
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
        pub struct $name(pub(crate) $storage);

        impl $name {
            /// Gets the numeric value for this component.
            pub const fn number(self) -> $storage {
                self.0
            }

            /// Creates a new value at 0.
            pub const fn zero() -> $name {
                Self(0)
            }

            /// Returns whether the value is zero.
            #[inline]
            pub fn is_zero(self) -> bool {
                self.0 == 0
            }
        }

        impl TryFrom<$storage> for $name {
            type Error = RangeError;

            fn try_from(input: $storage) -> Result<Self, Self::Error> {
                if input > $value {
                    Err(RangeError {
                        field: stringify!($name),
                        min: 0,
                        max: $value,
                        value: input as i32,
                    })
                } else {
                    Ok(Self(input))
                }
            }
        }

        impl TryFrom<usize> for $name {
            type Error = RangeError;

            fn try_from(input: usize) -> Result<Self, Self::Error> {
                if input > $value {
                    Err(RangeError {
                        field: "$name",
                        min: 0,
                        max: $value,
                        value: input as i32,
                    })
                } else {
                    Ok(Self(input as $storage))
                }
            }
        }

        impl From<$name> for $storage {
            fn from(input: $name) -> Self {
                input.0
            }
        }

        impl From<$name> for usize {
            fn from(input: $name) -> Self {
                input.0 as Self
            }
        }
    };
}

dt_unit!(
    Hour,
    u8,
    23,
    /// An ISO-8601 hour component, for use with ISO calendars.
    ///
    /// Must be within inclusive bounds `[0, 23]`.
);

dt_unit!(
    Minute,
    u8,
    59,
    /// An ISO-8601 minute component, for use with ISO calendars.
    ///
    /// Must be within inclusive bounds `[0, 59]`.
);

dt_unit!(
    Second,
    u8,
    60,
    /// An ISO-8601 second component, for use with ISO calendars.
    ///
    /// Must be within inclusive bounds `[0, 60]`. `60` accommodates for leap seconds.
);

dt_unit!(
    Nanosecond,
    u32,
    999_999_999,
    /// A fractional second component, stored as nanoseconds.
    ///
    /// Must be within inclusive bounds `[0, 999_999_999]`."
);

/// A representation of a time-of-day in hours, minutes, seconds, and nanoseconds
///
/// **The primary definition of this type is in the [`icu_time`](https://docs.rs/icu_time) crate. Other ICU4X crates re-export it for convenience.**
///
/// This type supports the range [00:00:00.000000000, 23:59:60.999999999].
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct Time {
    /// Hour
    pub hour: Hour,

    /// Minute
    pub minute: Minute,

    /// Second
    pub second: Second,

    /// Subsecond
    pub subsecond: Nanosecond,
}

impl Time {
    /// Construct a new [`Time`], without validating that all components are in range
    pub const fn new(hour: Hour, minute: Minute, second: Second, subsecond: Nanosecond) -> Self {
        Self {
            hour,
            minute,
            second,
            subsecond,
        }
    }

    /// Construct a new [`Time`] representing the start of the day (00:00:00.000)
    pub const fn start_of_day() -> Self {
        Self {
            hour: Hour(0),
            minute: Minute(0),
            second: Second(0),
            subsecond: Nanosecond(0),
        }
    }

    /// Construct a new [`Time`] representing noon (12:00:00.000)
    pub const fn noon() -> Self {
        Self {
            hour: Hour(12),
            minute: Minute(0),
            second: Second(0),
            subsecond: Nanosecond(0),
        }
    }

    /// Construct a new [`Time`], whilst validating that all components are in range
    pub fn try_new(hour: u8, minute: u8, second: u8, nanosecond: u32) -> Result<Self, RangeError> {
        Ok(Self {
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
            subsecond: nanosecond.try_into()?,
        })
    }

    pub(crate) const fn seconds_since_midnight(self) -> u32 {
        (self.hour.0 as u32 * 60 + self.minute.0 as u32) * 60 + self.second.0 as u32
    }
}

/// A date and time for a given calendar.
///
/// **The primary definition of this type is in the [`icu_time`](https://docs.rs/icu_time) crate. Other ICU4X crates re-export it for convenience.**
///
/// This type exists as an input type for datetime formatting and should only be constructed
/// to pass to a datetime formatter.
///
/// # Semantics
///
/// This type represents the date and time that are *displayed* to a user. It does not identify
/// the absolute time that an event happens, nor does it represent the general concept of a
/// "local date time", which would require time zone and leap second information for operations
/// like validation, arithmetic, and comparisons.
///
/// Hence, while this type implements [`PartialEq`]/[`Eq`] (equal [`DateTime`]s will *display*
/// equally), it does not implement [`PartialOrd`]/[`Ord`], arithmetic, and it is possible to
/// create [`DateTime`]s that do not exist for a particular timezone.
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct DateTime<A: AsCalendar> {
    /// The date
    pub date: Date<A>,
    /// The time
    pub time: Time,
}

impl<A, B> PartialEq<DateTime<B>> for DateTime<A>
where
    A: AsCalendar,
    B: AsCalendar,
    Date<A>: PartialEq<Date<B>>,
{
    fn eq(&self, other: &DateTime<B>) -> bool {
        self.date.eq(&other.date) && self.time.eq(&other.time)
    }
}

impl<A: AsCalendar> Eq for DateTime<A> where Date<A>: Eq {}

impl<A: AsCalendar> Clone for DateTime<A>
where
    Date<A>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            date: self.date.clone(),
            time: self.time,
        }
    }
}

impl<A: AsCalendar> Copy for DateTime<A> where Date<A>: Copy {}

/// A date and time for a given calendar, local to a specified time zone.
///
/// **The primary definition of this type is in the [`icu_time`](https://docs.rs/icu_time) crate. Other ICU4X crates re-export it for convenience.**
///
/// This type exists as an input type for datetime formatting and should only be constructed
/// to pass to a datetime formatter.
///
/// # Semantics
///
/// This type represents the date, time, and time zone that are *displayed* to a user.
/// It does not identify the absolute time that an event happens, nor does it represent
/// the general concept of a "zoned date time", which would require time zone and leap
/// second information for operations like validation, arithmetic, and comparisons[^1].
///
/// Hence, while this type implements [`PartialEq`]/[`Eq`] (equal [`ZonedDateTime`]s will
/// *display* equally), it does not implement [`PartialOrd`]/[`Ord`], arithmetic, and it
/// is possible to create [`ZonedDateTime`]s that do not exist.
///
/// [^1]: [`ZonedDateTime<UtcOffset>`] is an exception to this, as the whole time zone
///       identity is part of the type, and there are no local time discontinuities.
///       Therefore it actually identifies an absolute time and implements [`PartialOrd`].
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct ZonedDateTime<A: AsCalendar, Z> {
    /// The date, local to the time zone
    pub date: Date<A>,
    /// The time, local to the time zone
    pub time: Time,
    /// The time zone
    pub zone: Z,
}

impl<A, B, Y, Z> PartialEq<ZonedDateTime<B, Z>> for ZonedDateTime<A, Y>
where
    A: AsCalendar,
    B: AsCalendar,
    Date<A>: PartialEq<Date<B>>,
    Y: PartialEq<Z>,
{
    fn eq(&self, other: &ZonedDateTime<B, Z>) -> bool {
        self.date.eq(&other.date) && self.time.eq(&other.time) && self.zone.eq(&other.zone)
    }
}

impl<A: AsCalendar, Z> Eq for ZonedDateTime<A, Z>
where
    Date<A>: Eq,
    Z: Eq,
{
}

impl<A: AsCalendar, Z> Clone for ZonedDateTime<A, Z>
where
    Date<A>: Clone,
    Z: Clone,
{
    fn clone(&self) -> Self {
        Self {
            date: self.date.clone(),
            time: self.time,
            zone: self.zone.clone(),
        }
    }
}

impl<A: AsCalendar, Z> Copy for ZonedDateTime<A, Z>
where
    Date<A>: Copy,
    Z: Copy,
{
}

const UNIX_EPOCH: RataDie = calendrical_calculations::gregorian::fixed_from_gregorian(1970, 1, 1);

impl Ord for ZonedDateTime<Iso, UtcOffset> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let mut srd = self.date.to_rata_die();
        let mut ord = other.date.to_rata_die();

        // If the RDs are three days apart, even with maximum/minimum
        // times and offsets, the UTC days will still be at at least
        // one day apart
        if srd + 3 <= ord {
            return core::cmp::Ordering::Less;
        }
        if srd - 3 >= ord {
            return core::cmp::Ordering::Greater;
        }

        let mut ss = self.time.seconds_since_midnight() as i32 - self.zone.to_seconds();
        let mut os = other.time.seconds_since_midnight() as i32 - other.zone.to_seconds();

        // the seconds can wrap into the day

        if ss < 0 {
            srd -= 1;
            ss += 24 * 60 * 60;
        }
        if ss > 24 * 60 * 60 {
            srd += 1;
            ss -= 24 * 60 * 60;
        }

        if os < 0 {
            ord -= 1;
            os += 24 * 60 * 60;
        }
        if os > 24 * 60 * 60 {
            ord += 1;
            os -= 24 * 60 * 60;
        }

        // the subseconds cannot wrap into the seconds

        srd.cmp(&ord)
            .then(ss.cmp(&os))
            .then(self.time.subsecond.cmp(&other.time.subsecond))
    }
}
impl PartialOrd for ZonedDateTime<Iso, UtcOffset> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ZonedDateTime<Iso, UtcOffset> {
    /// Creates a [`ZonedDateTime`] from an absolute time, in milliseconds since the UNIX Epoch,
    /// and a UTC offset.
    ///
    /// This constructor returns a [`ZonedDateTime`] that supports only the localized offset
    /// time zone style.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::cal::Iso;
    /// use icu::time::zone::UtcOffset;
    /// use icu::time::ZonedDateTime;
    ///
    /// let iso_str = "2025-04-30T17:45-0700";
    /// let timestamp = 1746060300000; // milliseconds since UNIX epoch
    /// let offset: UtcOffset = "-0700".parse().unwrap();
    ///
    /// let zdt_from_timestamp =
    ///     ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
    ///         timestamp, offset,
    ///     );
    ///
    /// // Check that it equals the same as the parse result:
    /// let zdt_from_str =
    ///     ZonedDateTime::try_offset_only_from_str(iso_str, Iso).unwrap();
    /// assert_eq!(zdt_from_timestamp, zdt_from_str);
    /// ```
    ///
    /// Negative timestamps are supported:
    ///
    /// ```
    /// use icu::calendar::cal::Iso;
    /// use icu::time::zone::UtcOffset;
    /// use icu::time::ZonedDateTime;
    ///
    /// let iso_str = "1920-01-02T03:04:05.250+0600";
    /// let timestamp = -1577847354750; // milliseconds since UNIX epoch
    /// let offset: UtcOffset = "+0600".parse().unwrap();
    ///
    /// let zdt_from_timestamp =
    ///     ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
    ///         timestamp, offset,
    ///     );
    ///
    /// // Check that it equals the same as the parse result:
    /// let zdt_from_str =
    ///     ZonedDateTime::try_offset_only_from_str(iso_str, Iso).unwrap();
    /// assert_eq!(zdt_from_timestamp, zdt_from_str);
    /// ```
    ///
    /// When epoch milliseconds exceed the representable date range, the date component
    /// saturates to the maximum or minimum representable date in the ISO calendar
    ///
    /// ```
    /// use icu::calendar::cal::Iso;
    /// use icu::time::zone::UtcOffset;
    /// use icu::time::ZonedDateTime;
    ///
    /// let max_offset = UtcOffset::try_from_seconds(50400).unwrap(); // +14 hours
    /// let zdt_max = ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
    ///         i64::MAX,
    ///         max_offset
    ///     );
    ///
    /// let min_offset = UtcOffset::try_from_seconds(-43200).unwrap(); // -12 hours
    /// let zdt_min = ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
    ///         i64::MIN,
    ///         min_offset
    ///     );
    /// ```
    pub fn from_epoch_milliseconds_and_utc_offset(
        epoch_milliseconds: i64,
        utc_offset: UtcOffset,
    ) -> Self {
        let (utc_epoch_days, utc_time_millisecs) = (
            epoch_milliseconds.div_euclid(86400000),
            epoch_milliseconds.rem_euclid(86400000),
        );
        let offset_millisecs = 1000 * (utc_offset.to_seconds() as i64);
        let local_time_millisecs = utc_time_millisecs + offset_millisecs;
        let day_adjustment = local_time_millisecs.div_euclid(86400000);
        let final_time_millisecs = local_time_millisecs.rem_euclid(86400000);
        let rata_die = UNIX_EPOCH + utc_epoch_days + day_adjustment;
        #[expect(clippy::unwrap_used)] // these values are derived via modulo operators
        let time = Time::try_new(
            (final_time_millisecs / 3600000) as u8,
            ((final_time_millisecs % 3600000) / 60000) as u8,
            ((final_time_millisecs % 60000) / 1000) as u8,
            ((final_time_millisecs % 1000) as u32) * 1000000,
        )
        .unwrap();
        ZonedDateTime {
            date: Date::from_rata_die(rata_die, Iso),
            time,
            zone: utc_offset,
        }
    }
}

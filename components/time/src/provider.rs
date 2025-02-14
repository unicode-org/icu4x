// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use crate::Time;
use calendrical_calculations::rata_die::RataDie;
use core::ops::Deref;
use icu_calendar::{Date, Iso};
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use zerovec::maps::ZeroMapKV;
use zerovec::ule::{AsULE, ULE};
use zerovec::{ZeroMap2d, ZeroSlice, ZeroVec};

pub mod names;
pub mod windows;

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub struct Baked;

#[cfg(feature = "compiled_data")]
#[allow(unused_imports)]
const _: () = {
    use icu_time_data::*;
    pub mod icu {
        pub use crate as time;
    }
    make_provider!(Baked);
    impl_bcp47_to_iana_map_v1!(Baked);
    impl_iana_to_bcp47_map_v3!(Baked);
    impl_windows_zones_to_bcp47_map_v1!(Baked);
    impl_zone_offset_period_v1!(Baked);
};

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    names::Bcp47ToIanaMapV1::INFO,
    names::IanaToBcp47MapV3::INFO,
    windows::WindowsZonesToBcp47MapV1::INFO,
    ZoneOffsetPeriodV1::INFO,
];

/// TimeZone ID in BCP47 format
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, yoke::Yokeable, ULE, Hash)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_time::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct TimeZone(pub TinyAsciiStr<8>);

impl TimeZone {
    /// The synthetic `Etc/Unknown` time zone.
    ///
    /// This is the result of parsing unknown zones. It's important that such parsing does not
    /// fail, as new zones are added all the time, and ICU4X might not be up to date.
    pub const fn unknown() -> Self {
        Self(tinystr::tinystr!(8, "unk"))
    }
}

impl Deref for TimeZone {
    type Target = TinyAsciiStr<8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsULE for TimeZone {
    type ULE = Self;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        self
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        unaligned
    }
}

impl<'a> zerovec::maps::ZeroMapKV<'a> for TimeZone {
    type Container = ZeroVec<'a, TimeZone>;
    type Slice = ZeroSlice<TimeZone>;
    type GetType = TimeZone;
    type OwnedType = TimeZone;
}

/// Storage type for storing UTC offsets as eights of an hour.
pub type EighthsOfHourOffset = i8;
/// Storage type for storing `(Date<Iso>, Time)`.
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct MinutesSinceEpoch(pub i32);

impl From<(Date<Iso>, Time)> for MinutesSinceEpoch {
    fn from((d, t): (Date<Iso>, Time)) -> MinutesSinceEpoch {
        Self(
            ((Iso::to_fixed(d) - Self::EPOCH) as i32 * 24 + t.hour.number() as i32) * 60
                + t.minute.number() as i32,
        )
    }
}

impl MinutesSinceEpoch {
    // This is 1970-01-01, but that is coincidental to anything UNIX and could be changed to 0 in the future.
    const EPOCH: RataDie = RataDie::new(719163);
}

impl AsULE for MinutesSinceEpoch {
    type ULE = <i32 as AsULE>::ULE;

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(i32::from_unaligned(unaligned))
    }

    fn to_unaligned(self) -> Self::ULE {
        self.0.to_unaligned()
    }
}

impl<'a> ZeroMapKV<'a> for MinutesSinceEpoch {
    type Container = ZeroVec<'a, Self>;
    type Slice = ZeroSlice<Self>;
    type GetType = <Self as AsULE>::ULE;
    type OwnedType = Self;
}

#[cfg(feature = "serde")]
impl serde::Serialize for MinutesSinceEpoch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            let minute = self.0 % 60;
            let hour = self.0 / 60 % 24;
            let days = self.0 / 60 / 24;
            let date = Iso::from_fixed(Self::EPOCH + days as i64);
            let year = date.year().extended_year;
            let month = date.month().month_number();
            let day = date.day_of_month().0;
            serializer.serialize_str(&alloc::format!(
                "{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}"
            ))
        } else {
            serializer.serialize_i32(self.0)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MinutesSinceEpoch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        if deserializer.is_human_readable() {
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
            Ok(Self::from((
                Date::try_new_iso(year, month, day).map_err(e2)?,
                Time::try_new(hour, minute, 0, 0).map_err(e3)?,
            )))
        } else {
            i32::deserialize(deserializer).map(Self)
        }
    }
}

/// An ICU4X mapping to the time zone offsets at a given period.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(marker(ZoneOffsetPeriodV1, "time_zone/offset_period@1", singleton))]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_time::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct ZoneOffsetPeriod<'data>(
    /// The default mapping between period and offsets. The second level key is a wall-clock time encoded as
    /// [`MinutesSinceEpoch`]. It represents when the offsets started to be used.
    ///
    /// The values are the standard offset, and the daylight offset *relative to the standard offset*. As such,
    /// if the second value is 0, there is no daylight time.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ZeroMap2d<'data, TimeZone, MinutesSinceEpoch, (EighthsOfHourOffset, EighthsOfHourOffset)>,
);

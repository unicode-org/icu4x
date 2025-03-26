// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for resolving and manipulating time zones.
//!
//! # Fields
//!
//! In ICU4X, a [formattable time zone] consists of up to four different fields:
//!
//! 1. The time zone ID
//! 2. The offset from UTC
//! 3. A timestamp, as time zone names can change over time
//! 4. The zone variant, representing concepts such as Standard, Summer, Daylight, and Ramadan time
//!
//! ## Time Zone
//!
//! The time zone ID corresponds to a time zone from the time zone database. The time zone ID
//! usually corresponds to the largest city in the time zone.
//!
//! There are two mostly-interchangeable standards for time zone IDs:
//!
//! 1. IANA time zone IDs, like `"America/Chicago"`
//! 2. BCP-47 time zone IDs, like `"uschi"`
//!
//! ICU4X uses BCP-47 time zone IDs for all of its APIs. To get a BCP-47 time zone from an
//! IANA time zone, use [`IanaParser`].
//!
//! ## UTC Offset
//!
//! The UTC offset precisely states the time difference between the time zone in question and
//! Coordinated Universal Time (UTC).
//!
//! In localized strings, it is often rendered as "UTC-6", meaning 6 hours less than UTC (some locales
//! use the term "GMT" instead of "UTC").
//!
//! ## Timestamp
//!
//! Some time zones change names over time, such as when changing "metazone". For example, Portugal changed from
//! "Western European Time" to "Central European Time" and back in the 1990s, without changing time zone ID
//! (`Europe/Lisbon`, `ptlis`). Therefore, a timestamp is needed to resolve such generic time zone names.
//!
//! It is not required to set the timestamp on [`TimeZoneInfo`]. If it is not set, some string
//! formats may be unsupported.
//!
//! ## Zone Variant
//!
//! Many zones use different names and offsets in the summer than in the winter. In ICU4X,
//! this is called the _zone variant_.
//!
//! CLDR has two zone variants, named `"standard"` and `"daylight"`. However, the mapping of these
//! variants to specific observed offsets varies from time zone to time zone, and they may not
//! consistently represent winter versus summer time.
//!
//! Note: It is not required to set the zone variant on [`TimeZoneInfo`]. If it is not set, some string
//! formats may be unsupported.
//!
//! # Examples
//!
//! ```
//! use icu::calendar::Date;
//! use icu::time::zone::IanaParser;
//! use icu::time::zone::TimeZoneVariant;
//! use icu::time::Time;
//! use icu::time::TimeZone;
//! use icu::locale::subtags::subtag;
//!
//! // Parse the IANA ID
//! let id = IanaParser::new().parse("America/Chicago");
//!
//! // Alternatively, use the BCP47 ID directly
//! let id = TimeZone(subtag!("uschi"));
//!
//! // Create a TimeZoneInfo<Base> by associating the ID with an offset
//! let time_zone = id.with_offset("-0600".parse().ok());
//!
//! // Extend to a TimeZoneInfo<AtTime> by adding a local time
//! let time_zone_at_time = time_zone
//!     .at_time((Date::try_new_iso(2023, 12, 2).unwrap(), Time::midnight()));
//!
//! // Extend to a TimeZoneInfo<Full> by adding a zone variant
//! let time_zone_with_variant =
//!     time_zone_at_time.with_zone_variant(TimeZoneVariant::Standard);
//! ```

pub mod iana;
mod offset;
pub mod windows;

#[doc(inline)]
pub use offset::InvalidOffsetError;
pub use offset::UtcOffset;
pub use offset::VariantOffsets;
pub use offset::VariantOffsetsCalculator;
pub use offset::VariantOffsetsCalculatorBorrowed;

#[doc(no_inline)]
pub use iana::IanaParser;
#[doc(no_inline)]
pub use windows::WindowsParser;

use crate::{scaffold::IntoOption, Time};
use core::fmt;
use core::ops::Deref;
use icu_calendar::{Date, Iso};
use icu_locale_core::subtags::{subtag, Subtag};
use icu_provider::prelude::yoke;
use zerovec::ule::{AsULE, ULE};
use zerovec::{ZeroSlice, ZeroVec};

/// Time zone data model choices.
pub mod models {
    use super::*;
    mod private {
        pub trait Sealed {}
    }

    /// Trait encoding a particular data model for time zones.
    ///
    /// <div class="stab unstable">
    /// 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this
    /// trait, please consider using a type from the implementors listed below.
    /// </div>
    pub trait TimeZoneModel: private::Sealed {
        /// The zone variant, if required for this time zone model.
        type TimeZoneVariant: IntoOption<TimeZoneVariant> + fmt::Debug + Copy;
        /// The local time, if required for this time zone model.
        type LocalTime: IntoOption<(Date<Iso>, Time)> + fmt::Debug + Copy;
    }

    /// A time zone containing a time zone ID and optional offset.
    #[derive(Debug, PartialEq, Eq)]
    #[non_exhaustive]
    pub struct Base;

    impl private::Sealed for Base {}
    impl TimeZoneModel for Base {
        type TimeZoneVariant = ();
        type LocalTime = ();
    }

    /// A time zone containing a time zone ID, optional offset, and local time.
    #[derive(Debug, PartialEq, Eq)]
    #[non_exhaustive]
    pub struct AtTime;

    impl private::Sealed for AtTime {}
    impl TimeZoneModel for AtTime {
        type TimeZoneVariant = ();
        type LocalTime = (Date<Iso>, Time);
    }

    /// A time zone containing a time zone ID, optional offset, local time, and zone variant.
    #[derive(Debug, PartialEq, Eq)]
    #[non_exhaustive]
    pub struct Full;

    impl private::Sealed for Full {}
    impl TimeZoneModel for Full {
        type TimeZoneVariant = TimeZoneVariant;
        type LocalTime = (Date<Iso>, Time);
    }
}

/// A CLDR time zone identity.
///
/// **The primary definition of this type is in the [`icu_time`](https://docs.rs/icu_time) crate. Other ICU4X crates re-export it for convenience.**
///
/// This can be created directly from BCP-47 strings, or it can be parsed from IANA IDs.
///
/// CLDR uses difference equivalence classes than IANA. For example, `Europe/Oslo` is
/// an alias to `Europe/Berlin` in IANA (because they agree since 1970), but these are
/// different identities in CLDR, as we want to be able to say "Norway Time" and
/// "Germany Time". On the other hand `Europe/Belfast` and `Europe/London` are the same
/// CLDR identity ("UK Time").
///
/// ```
/// use icu::time::zone::{IanaParser, TimeZone};
/// use icu::locale::subtags::subtag;
///
/// let parser = IanaParser::new();
/// assert_eq!(parser.parse("Europe/Oslo"), TimeZone(subtag!("noosl")));
/// assert_eq!(
///     parser.parse("Europe/Berlin"),
///     TimeZone(subtag!("deber"))
/// );
/// assert_eq!(
///     parser.parse("Europe/Belfast"),
///     TimeZone(subtag!("gblon"))
/// );
/// assert_eq!(
///     parser.parse("Europe/London"),
///     TimeZone(subtag!("gblon"))
/// );
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, yoke::Yokeable, ULE, Hash)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_time::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // This is a stable newtype
pub struct TimeZone(pub Subtag);

impl TimeZone {
    /// The synthetic `Etc/Unknown` time zone.
    ///
    /// This is the result of parsing unknown zones. It's important that such parsing does not
    /// fail, as new zones are added all the time, and ICU4X might not be up to date.
    pub const fn unknown() -> Self {
        Self(subtag!("unk"))
    }
}

/// This module exists so we can cleanly reexport TimeZoneVariantULE from the provider module, whilst retaining a public stable TimeZoneVariant type.
pub(crate) mod ule {
    /// A time zone variant, such as Standard Time, or Daylight/Summer Time.
    ///
    /// This should not generally be constructed by client code. Instead, use
    /// * [`TimeZoneVariant::from_rearguard_isdst`]
    /// * [`TimeZoneInfo::infer_zone_variant`](crate::TimeZoneInfo::infer_zone_variant)
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[zerovec::make_ule(TimeZoneVariantULE)]
    #[repr(u8)]
    #[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
    #[cfg_attr(feature = "datagen", databake(path = icu_time))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
    #[non_exhaustive]
    pub enum TimeZoneVariant {
        /// The variant corresponding to `"standard"` in CLDR.
        ///
        /// The semantics vary from time zone to time zone. The time zone display
        /// name of this variant may or may not be called "Standard Time".
        ///
        /// This is the variant with the lower UTC offset.
        Standard = 0,
        /// The variant corresponding to `"daylight"` in CLDR.
        ///
        /// The semantics vary from time zone to time zone. The time zone display
        /// name of this variant may or may not be called "Daylight Time".
        ///
        /// This is the variant with the higher UTC offset.
        Daylight = 1,
    }
}
pub use ule::TimeZoneVariant;

impl Deref for TimeZone {
    type Target = Subtag;

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

/// A utility type that can hold time zone information.
///
/// **The primary definition of this type is in the [`icu_time`](https://docs.rs/icu_time) crate. Other ICU4X crates re-export it for convenience.**
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // these four fields fully cover the needs of UTS 35
pub struct TimeZoneInfo<Model: models::TimeZoneModel> {
    time_zone_id: TimeZone,
    offset: Option<UtcOffset>,
    local_time: Model::LocalTime,
    zone_variant: Model::TimeZoneVariant,
}

impl<Model: models::TimeZoneModel> Clone for TimeZoneInfo<Model> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Model: models::TimeZoneModel> Copy for TimeZoneInfo<Model> {}

impl<Model: models::TimeZoneModel> TimeZoneInfo<Model> {
    /// The BCP47 time-zone identifier.
    pub fn time_zone_id(self) -> TimeZone {
        self.time_zone_id
    }

    /// The UTC offset, if known.
    ///
    /// This field is not enforced to be consistent with the time zone id.
    pub fn offset(self) -> Option<UtcOffset> {
        self.offset
    }
}

impl<Model> TimeZoneInfo<Model>
where
    Model: models::TimeZoneModel<LocalTime = (Date<Iso>, Time)>,
{
    /// The time at which to interpret the time zone.
    pub fn local_time(self) -> (Date<Iso>, Time) {
        self.local_time
    }
}

impl<Model> TimeZoneInfo<Model>
where
    Model: models::TimeZoneModel<TimeZoneVariant = TimeZoneVariant>,
{
    /// The time variant e.g. daylight or standard, if known.
    ///
    /// This field is not enforced to be consistent with the time zone id and offset.
    pub fn zone_variant(self) -> TimeZoneVariant {
        self.zone_variant
    }
}

impl TimeZone {
    /// Associates this [`TimeZone`] with a UTC offset, returning a [`TimeZoneInfo`].
    pub const fn with_offset(self, offset: Option<UtcOffset>) -> TimeZoneInfo<models::Base> {
        TimeZoneInfo {
            offset,
            time_zone_id: self,
            local_time: (),
            zone_variant: (),
        }
    }

    /// Converts this [`TimeZone`] into a [`TimeZoneInfo`] without an offset.
    pub const fn without_offset(self) -> TimeZoneInfo<models::Base> {
        TimeZoneInfo {
            offset: None,
            time_zone_id: self,
            local_time: (),
            zone_variant: (),
        }
    }
}

impl TimeZoneInfo<models::Base> {
    /// Creates a time zone info with no information.
    pub const fn unknown() -> Self {
        TimeZone::unknown().with_offset(None)
    }

    /// Creates a new [`TimeZoneInfo`] for the UTC time zone.
    pub const fn utc() -> Self {
        TimeZone(subtag!("utc")).with_offset(Some(UtcOffset::zero()))
    }

    /// Sets a local time on this time zone.
    pub const fn at_time(self, local_time: (Date<Iso>, Time)) -> TimeZoneInfo<models::AtTime> {
        TimeZoneInfo {
            offset: self.offset,
            time_zone_id: self.time_zone_id,
            local_time,
            zone_variant: (),
        }
    }
}

impl TimeZoneInfo<models::AtTime> {
    /// Sets a zone variant on this time zone.
    pub const fn with_zone_variant(
        self,
        zone_variant: TimeZoneVariant,
    ) -> TimeZoneInfo<models::Full> {
        TimeZoneInfo {
            offset: self.offset,
            time_zone_id: self.time_zone_id,
            local_time: self.local_time,
            zone_variant,
        }
    }

    /// Sets the zone variant by calculating it using a [`VariantOffsetsCalculator`].
    ///
    /// If `offset()` is `None`, or if it doesn't match either of the
    /// timezone's standard or daylight offset around `local_time()`,
    /// the variant will be set to [`TimeZoneVariant::Standard`] and the time zone
    /// to [`TimeZone::unknown()`].
    pub fn infer_zone_variant(
        self,
        calculator: VariantOffsetsCalculatorBorrowed,
    ) -> TimeZoneInfo<models::Full> {
        let Some(offset) = self.offset else {
            return TimeZone::unknown()
                .with_offset(self.offset)
                .at_time(self.local_time)
                .with_zone_variant(TimeZoneVariant::Standard);
        };
        let Some(zone_variant) = calculator
            .compute_offsets_from_time_zone(self.time_zone_id, self.local_time)
            .and_then(|os| {
                if os.standard == offset {
                    Some(TimeZoneVariant::Standard)
                } else if os.daylight == Some(offset) {
                    Some(TimeZoneVariant::Daylight)
                } else {
                    None
                }
            })
        else {
            return TimeZone::unknown()
                .with_offset(self.offset)
                .at_time(self.local_time)
                .with_zone_variant(TimeZoneVariant::Standard);
        };
        self.with_zone_variant(zone_variant)
    }
}

impl TimeZoneVariant {
    /// Creates a zone variant from a TZDB `isdst` flag, if it is known that the TZDB was built with
    /// `DATAFORM=rearguard`.
    ///
    /// If it is known that the database was *not* built with `rearguard`, a caller can try to adjust
    /// for the differences. This is a moving target, for example the known differences for 2025a are:
    ///
    /// * `Europe/Dublin` since 1968-10-27
    /// * `Africa/Windhoek` between 1994-03-20 and 2017-10-24
    /// * `Africa/Casablanca` and `Africa/El_Aaiun` since 2018-10-28
    ///
    /// If the TZDB build mode is unknown or variable, use [`TimeZoneInfo::infer_zone_variant`].
    pub const fn from_rearguard_isdst(isdst: bool) -> Self {
        if isdst {
            TimeZoneVariant::Daylight
        } else {
            TimeZoneVariant::Standard
        }
    }
}

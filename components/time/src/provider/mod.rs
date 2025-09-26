// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]
#![allow(clippy::type_complexity)]

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use crate::zone::{UtcOffset, ZoneNameTimestamp};
use icu_provider::prelude::*;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::ule::vartuple::VarTupleULE;
use zerovec::ule::{AsULE, NichedOption, RawBytesULE};
use zerovec::{VarZeroVec, ZeroSlice, ZeroVec};

pub use crate::zone::TimeZone;
pub mod iana;
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
    impl_timezone_identifiers_iana_extended_v1!(Baked);
    impl_timezone_identifiers_iana_core_v1!(Baked);
    impl_timezone_identifiers_windows_v1!(Baked);
    impl_timezone_periods_v1!(Baked);
};

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    iana::TimezoneIdentifiersIanaExtendedV1::INFO,
    iana::TimezoneIdentifiersIanaCoreV1::INFO,
    windows::TimezoneIdentifiersWindowsV1::INFO,
    TimezonePeriodsV1::INFO,
];

const SECONDS_TO_EIGHTS_OF_HOURS: i32 = 60 * 60 / 8;

/// A time zone variant used to identify a display name in CLDR.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[zerovec::make_ule(TimeZoneVariantULE)]
#[repr(u8)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_time::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(not(feature = "alloc"), zerovec::skip_derive(ZeroMapKV))]
#[non_exhaustive]
pub enum TimeZoneVariant {
    /// The variant corresponding to `"standard"` in CLDR.
    ///
    /// The semantics vary from time zone to time zone. The time zone display
    /// name of this variant may or may not be called "Standard Time".
    Standard = 0,
    /// The variant corresponding to `"daylight"` in CLDR.
    ///
    /// The semantics vary from time zone to time zone. The time zone display
    /// name of this variant may or may not be called "Daylight Time".
    Daylight = 1,
}

/// Metadata about a metazone membership
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[non_exhaustive]
pub enum MetazoneMembershipKind {
    /// This zone is equivalent to the metazone's golden time zone.
    BehavesLikeGolden,
    /// This zone uses variants that the golden zone does not use.
    /// This happens for example for London, Dublin, Troll (all in GMT), Windhoek (in WAT).
    CustomVariants,
    /// This zone uses different transitions than the golden zone.
    /// This happens for example for Phoenix, Regina, Algiers, Brisbane (no DST),
    /// or Chisinau (transitions at different times, not implemented yet).
    CustomTransitions,
}

/// Represents the different offsets in use for a time zone
// warning: stable (deprecated) type through reexport in crate::zone::offset
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct VariantOffsets {
    /// The standard offset.
    pub standard: UtcOffset,
    /// The daylight-saving offset, if used.
    pub daylight: Option<UtcOffset>,
}

impl VariantOffsets {
    /// Creates a new [`VariantOffsets`] from a [`UtcOffset`] representing standard time.
    pub fn from_standard(standard: UtcOffset) -> Self {
        Self {
            standard,
            daylight: None,
        }
    }
}

/// A [`VariantOffsets`] and a [`MetazoneMembershipKind`] packed into one byte.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct VariantOffsetsWithMetazoneMembershipKind {
    /// The offsets. Currently uses 3 bits.
    pub offsets: VariantOffsets,
    /// Metazone membership metadata. Currently uses 2 bits.
    pub mzmsk: MetazoneMembershipKind,
}

impl AsULE for VariantOffsetsWithMetazoneMembershipKind {
    type ULE = [i8; 2];

    fn from_unaligned([std, dst]: Self::ULE) -> Self {
        Self {
            offsets: VariantOffsets {
                standard: UtcOffset::from_seconds_unchecked(if std == i8::MAX {
                    // Special bit pattern for value that appears in TZDB but is not
                    // representable by our schema.
                    -2670
                } else {
                    std as i32 * SECONDS_TO_EIGHTS_OF_HOURS
                        + match std % 8 {
                            // 7.5, 37.5, representing 10, 40
                            1 | 5 => 150,
                            -1 | -5 => -150,
                            // 22.5, 52.5, representing 20, 50
                            3 | 7 => -150,
                            -3 | -7 => 150,
                            // 0, 15, 30, 45
                            _ => 0,
                        }
                }),
                daylight: match dst as u8 & 0b0011_1111 {
                    0 => None,
                    1 => Some(0),
                    2 => Some(1800),
                    3 => Some(3600),
                    4 => Some(5400),
                    5 => Some(7200),
                    6 => Some(-3600),
                    x => {
                        debug_assert!(false, "unknown DST encoding {x}");
                        None
                    }
                }
                .map(|d| {
                    UtcOffset::from_seconds_unchecked(std as i32 * SECONDS_TO_EIGHTS_OF_HOURS + d)
                }),
            },
            mzmsk: match (dst as u8 & 0b1100_0000) >> 6 {
                0b00 => MetazoneMembershipKind::BehavesLikeGolden,
                0b10 => MetazoneMembershipKind::CustomTransitions,
                0b01 => MetazoneMembershipKind::CustomVariants,
                x => {
                    debug_assert!(false, "unknown MetazoneMembershipKind encoding {x}");
                    MetazoneMembershipKind::BehavesLikeGolden
                }
            },
        }
    }

    fn to_unaligned(self) -> Self::ULE {
        let offset = self.offsets.standard.to_seconds();
        [
            if offset == -2670 {
                // Special bit pattern for value that appears in TZDB but is not
                // representable by our schema.
                i8::MAX
            } else {
                debug_assert_eq!(offset.abs() % 60, 0);
                let scaled = match offset.abs() / 60 % 60 {
                    0 | 15 | 30 | 45 => offset / SECONDS_TO_EIGHTS_OF_HOURS,
                    10 | 40 => {
                        // stored as 7.5, 37.5, truncating div
                        offset / SECONDS_TO_EIGHTS_OF_HOURS
                    }
                    20 | 50 => {
                        // stored as 22.5, 52.5, need to add one
                        offset / SECONDS_TO_EIGHTS_OF_HOURS + offset.signum()
                    }
                    _ => {
                        debug_assert!(false, "{offset:?}");
                        offset / SECONDS_TO_EIGHTS_OF_HOURS
                    }
                };
                debug_assert!(i8::MIN as i32 <= scaled && scaled < i8::MAX as i32);
                scaled as i8
            },
            match self
                .offsets
                .daylight
                .map(|o| o.to_seconds() - self.offsets.standard.to_seconds())
            {
                None => 0,
                Some(0) => 1,
                Some(1800) => 2,
                Some(3600) => 3,
                Some(5400) => 4,
                Some(7200) => 5,
                Some(-3600) => 6,
                Some(x) => {
                    debug_assert!(false, "unhandled DST value {x}");
                    0
                }
            } | (match self.mzmsk {
                MetazoneMembershipKind::BehavesLikeGolden => 0b00u8,
                MetazoneMembershipKind::CustomTransitions => 0b10,
                MetazoneMembershipKind::CustomVariants => 0b01,
            } << 6) as i8,
        ]
    }
}

#[cfg(all(feature = "alloc", feature = "serde"))]
impl serde::Serialize for VariantOffsetsWithMetazoneMembershipKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_unaligned().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for VariantOffsetsWithMetazoneMembershipKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <_>::deserialize(deserializer).map(Self::from_unaligned)
    }
}

#[test]
fn offsets_ule() {
    #[track_caller]
    fn assert_round_trip(offset: UtcOffset) {
        let variants = VariantOffsets::from_standard(offset);
        assert_eq!(
            variants,
            VariantOffsets::from_unaligned(VariantOffsets::to_unaligned(variants))
        );
    }

    assert_round_trip(UtcOffset::try_from_str("+01:00").unwrap());
    assert_round_trip(UtcOffset::try_from_str("+01:15").unwrap());
    assert_round_trip(UtcOffset::try_from_str("+01:30").unwrap());
    assert_round_trip(UtcOffset::try_from_str("+01:45").unwrap());

    assert_round_trip(UtcOffset::try_from_str("+01:10").unwrap());
    assert_round_trip(UtcOffset::try_from_str("+01:20").unwrap());
    assert_round_trip(UtcOffset::try_from_str("+01:40").unwrap());
    assert_round_trip(UtcOffset::try_from_str("+01:50").unwrap());

    assert_round_trip(UtcOffset::try_from_str("-01:00").unwrap());
    assert_round_trip(UtcOffset::try_from_str("-01:15").unwrap());
    assert_round_trip(UtcOffset::try_from_str("-01:30").unwrap());
    assert_round_trip(UtcOffset::try_from_str("-01:45").unwrap());

    assert_round_trip(UtcOffset::try_from_str("-01:10").unwrap());
    assert_round_trip(UtcOffset::try_from_str("-01:20").unwrap());
    assert_round_trip(UtcOffset::try_from_str("-01:40").unwrap());
    assert_round_trip(UtcOffset::try_from_str("-01:50").unwrap());
}

#[cfg(feature = "alloc")]
impl<'a> zerovec::maps::ZeroMapKV<'a> for VariantOffsets {
    type Container = ZeroVec<'a, Self>;
    type Slice = ZeroSlice<Self>;
    type GetType = <Self as AsULE>::ULE;
    type OwnedType = Self;
}

#[cfg(all(feature = "alloc", feature = "serde"))]
impl serde::Serialize for VariantOffsets {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            use alloc::fmt::Write;
            let mut r = alloc::format!(
                "{:+02}:{:02}",
                self.standard.hours_part(),
                self.standard.minutes_part(),
            );
            if self.standard.seconds_part() != 0 {
                let _infallible = write!(&mut r, ":{:02}", self.standard.seconds_part());
            }
            if let Some(dst) = self.daylight {
                let _infallible = write!(
                    &mut r,
                    "/{:+02}:{:02}",
                    dst.hours_part(),
                    dst.minutes_part(),
                );

                if dst.seconds_part() != 0 {
                    let _infallible = write!(&mut r, ":{:02}", dst.seconds_part());
                }
            }

            serializer.serialize_str(&r)
        } else {
            self.to_unaligned().serialize(serializer)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for VariantOffsets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        if deserializer.is_human_readable() {
            let raw = <&str>::deserialize(deserializer)?;
            Ok(if let Some((std, dst)) = raw.split_once('/') {
                Self {
                    standard: UtcOffset::try_from_str(std)
                        .map_err(|_| D::Error::custom("invalid offset"))?,
                    daylight: Some(
                        UtcOffset::try_from_str(dst)
                            .map_err(|_| D::Error::custom("invalid offset"))?,
                    ),
                }
            } else {
                Self {
                    standard: UtcOffset::try_from_str(raw)
                        .map_err(|_| D::Error::custom("invalid offset"))?,
                    daylight: None,
                }
            })
        } else {
            <_>::deserialize(deserializer).map(Self::from_unaligned)
        }
    }
}

/// Metazone ID in a compact format
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
pub type MetazoneId = core::num::NonZeroU8;

/// Data struct for the [`TimezonePeriodsV1`] marker.
#[derive(PartialEq, Debug, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_time::provider))]
pub struct TimezonePeriods<'a> {
    /// Index of `TimeZone`s into `list`.
    pub index: ZeroTrieSimpleAscii<ZeroVec<'a, u8>>,
    /// Each entry contains at least one period, which implicitly starts at the UNIX epoch.
    /// This is stored in the first tuple element.
    ///
    /// If more periods are required the second tuple element contains them, along with their
    /// starting timestamp. These entries are ordered chronologically.
    ///
    /// The values (`(u8, Option<MetazoneId>)`) are an index into the `offsets` list for the offset
    /// that the zone observes in that period, and optionally whether it is part of a metazone.
    pub list: VarZeroVec<
        'a,
        VarTupleULE<
            (u8, NichedOption<MetazoneId, 1>),
            ZeroSlice<(Timestamp24, u8, NichedOption<MetazoneId, 1>)>,
        >,
    >,

    /// The deduplicated list of offsets.
    ///
    /// There are currently 99 unique VariantOffsetsWithMetazoneMembershipKind, so storing the index as a u8 is plenty enough.
    pub offsets: ZeroVec<'a, VariantOffsetsWithMetazoneMembershipKind>,
}

/// Encodes ZoneNameTimestamp in 3 bytes by dropping the unused metadata
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
pub struct Timestamp24(pub ZoneNameTimestamp);

impl AsULE for Timestamp24 {
    type ULE = RawBytesULE<3>;
    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        let RawBytesULE([a, b, c, _]) = self.0.to_unaligned();
        RawBytesULE([a, b, c])
    }
    #[inline]
    fn from_unaligned(RawBytesULE([a, b, c]): Self::ULE) -> Self {
        Self(ZoneNameTimestamp::from_unaligned(RawBytesULE([a, b, c, 0])))
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
struct TimeZonePeriodsSerde<'a> {
    #[serde(borrow)]
    pub index: ZeroTrieSimpleAscii<ZeroVec<'a, u8>>,
    #[serde(borrow)]
    pub list: VarZeroVec<
        'a,
        VarTupleULE<
            (u8, NichedOption<MetazoneId, 1>),
            ZeroSlice<(Timestamp24, u8, NichedOption<MetazoneId, 1>)>,
        >,
    >,

    pub offsets: ZeroVec<'a, VariantOffsetsWithMetazoneMembershipKind>,
}

#[cfg(feature = "datagen")]
impl serde::Serialize for TimezonePeriods<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        if serializer.is_human_readable() {
            let mut map = serializer.serialize_map(None)?;
            for (tz, idx) in self.index.iter() {
                if let Some(value) = self.list.get(idx) {
                    map.serialize_entry(
                        &tz,
                        &[ZoneNameTimestamp::far_in_past()]
                            .into_iter()
                            .chain(value.variable.iter().map(|(t, _, _)| t.0))
                            .map(|t| {
                                use icu_locale_core::subtags::Subtag;

                                #[allow(clippy::unwrap_used)] // JSON debug format
                                let (os, mz_info) = self
                                    .get(TimeZone(Subtag::try_from_str(&tz).unwrap()), t)
                                    .unwrap();
                                (
                                    t,
                                    (
                                        os,
                                        mz_info.map(|i| {
                                            (
                                                i.id,
                                                match i.kind {
                                                    MetazoneMembershipKind::BehavesLikeGolden => {
                                                        [].as_slice()
                                                    }
                                                    MetazoneMembershipKind::CustomVariants => {
                                                        &["custom variants"]
                                                    }
                                                    MetazoneMembershipKind::CustomTransitions => {
                                                        &["custom transitions"]
                                                    }
                                                },
                                            )
                                        }),
                                    ),
                                )
                            })
                            .collect::<alloc::collections::BTreeMap<_, _>>(),
                    )?;
                }
            }
            map.end()
        } else {
            TimeZonePeriodsSerde {
                list: self.list.clone(),
                index: self.index.clone(),
                offsets: self.offsets.clone(),
            }
            .serialize(serializer)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TimezonePeriods<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        if deserializer.is_human_readable() {
            // TODO(#6752): Add human-readable deserialization for this data
            Err(D::Error::custom("not yet supported; see icu4x#6752"))
        } else {
            let TimeZonePeriodsSerde {
                index,
                list,
                offsets,
            } = TimeZonePeriodsSerde::deserialize(deserializer)?;
            Ok(Self {
                index,
                list,
                offsets,
            })
        }
    }
}

/// Information about a metazone membership
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct MetazoneInfo {
    /// The metazone ID.
    pub id: MetazoneId,
    /// The kind.
    pub kind: MetazoneMembershipKind,
}

impl TimezonePeriods<'_> {
    /// Gets the information for a time zone at at timestamp
    ///
    /// If the timezone is in a metazone, returns the metazone ID as well as the offsets
    /// that the metazone's golden zone currently uses.
    pub fn get(
        &self,
        time_zone_id: TimeZone,
        timestamp: ZoneNameTimestamp,
    ) -> Option<(VariantOffsets, Option<MetazoneInfo>)> {
        let (os_idx, NichedOption(mz)) =
            self.find_period(self.index.get(time_zone_id.as_str())?, timestamp)?;

        let os = self.offsets.get(os_idx as usize)?;

        let Some(mz) = mz else {
            return Some((os.offsets, None));
        };

        Some((
            os.offsets,
            Some(MetazoneInfo {
                id: mz,
                kind: os.mzmsk,
            }),
        ))
    }

    // Given an index in `list`, returns the values at the `timestamp`
    fn find_period(
        &self,
        idx: usize,
        timestamp: ZoneNameTimestamp,
    ) -> Option<(u8, NichedOption<MetazoneId, 1>)> {
        use zerovec::ule::vartuple::VarTupleULE;
        use zerovec::ule::AsULE;
        let &VarTupleULE {
            sized: first,
            variable: ref rest,
        } = self.list.get(idx)?;

        let i = match rest.binary_search_by(|(t, ..)| t.cmp(&Timestamp24(timestamp))) {
            Err(0) => return Some(<(u8, NichedOption<MetazoneId, 1>)>::from_unaligned(first)),
            Err(i) => i - 1,
            Ok(i) => i,
        };
        let (_, os, mz) = rest.get(i)?;
        Some((os, mz))
    }
}

icu_provider::data_struct!(
    TimezonePeriods<'_>,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// An ICU4X mapping to timezone offset data and metazones at a given period.
    TimezonePeriodsV1,
    TimezonePeriods<'static>,
    is_singleton = true,
    has_checksum = true
);

impl AsULE for VariantOffsets {
    type ULE = [i8; 2];

    fn from_unaligned([std, dst]: Self::ULE) -> Self {
        fn decode(encoded: i8) -> i32 {
            encoded as i32 * SECONDS_TO_EIGHTS_OF_HOURS
                + match encoded % 8 {
                    // 7.5, 37.5, representing 10, 40
                    1 | 5 => 150,
                    -1 | -5 => -150,
                    // 22.5, 52.5, representing 20, 50
                    3 | 7 => -150,
                    -3 | -7 => 150,
                    // 0, 15, 30, 45
                    _ => 0,
                }
        }

        Self {
            standard: UtcOffset::from_seconds_unchecked(decode(std)),
            daylight: (dst != 0).then(|| UtcOffset::from_seconds_unchecked(decode(std + dst))),
        }
    }

    fn to_unaligned(self) -> Self::ULE {
        fn encode(offset: i32) -> i8 {
            debug_assert_eq!(offset.abs() % 60, 0);
            let scaled = match offset.abs() / 60 % 60 {
                0 | 15 | 30 | 45 => offset / SECONDS_TO_EIGHTS_OF_HOURS,
                10 | 40 => {
                    // stored as 7.5, 37.5, truncating div
                    offset / SECONDS_TO_EIGHTS_OF_HOURS
                }
                20 | 50 => {
                    // stored as 22.5, 52.5, need to add one
                    offset / SECONDS_TO_EIGHTS_OF_HOURS + offset.signum()
                }
                _ => {
                    debug_assert!(false, "{offset:?}");
                    offset / SECONDS_TO_EIGHTS_OF_HOURS
                }
            };
            debug_assert!(i8::MIN as i32 <= scaled && scaled <= i8::MAX as i32);
            scaled as i8
        }
        [
            encode(self.standard.to_seconds()),
            self.daylight
                .map(|d| encode(d.to_seconds() - self.standard.to_seconds()))
                .unwrap_or_default(),
        ]
    }
}

// remove in 3.0
#[cfg(feature = "alloc")]
pub(crate) mod legacy {
    use super::*;
    use zerovec::ZeroMap2d;

    icu_provider::data_marker!(
        /// The default mapping between period and offsets. The second level key is a wall-clock time encoded as
        /// [`ZoneNameTimestamp`]. It represents when the offsets started to be used.
        TimezoneVariantsOffsetsV1,
        "timezone/variants/offsets/v1",
        ZeroMap2d<'static, TimeZone, ZoneNameTimestamp, VariantOffsets>,
        is_singleton = true
    );
}
